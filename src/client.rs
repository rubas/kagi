use std::path::{Path, PathBuf};
use std::time::Duration;

use wreq::Client;
use wreq_util::Emulation;

use crate::cli::{MapsArgs, MapsOrder, MapsSort, SearchArgs, SummarizeArgs};
use crate::error::Error;
use crate::parse::{
    MapsOutput, MapsResult, SearchOutput, SummarizeOutput, parse_maps_results,
    parse_search_results, parse_summary_stream,
};

const SEARCH_URL: &str = "https://kagi.com/html/search";
const MAPS_SEARCH_URL: &str = "https://kagi.com/maps/api/v1/search";
const SUMMARIZE_URL: &str = "https://kagi.com/mother/summary_labs";
const SESSION_TOKEN_ENV: &str = "KAGI_SESSION_TOKEN";
const XDG_CONFIG_ENV: &str = "XDG_CONFIG_HOME";
const SESSION_TOKEN_RELATIVE_PATH: &str = "kagi/session-token";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const SUMMARIZE_TOTAL_TIMEOUT: Duration = Duration::from_secs(300);

pub async fn search(args: &SearchArgs) -> Result<SearchOutput, Error> {
    let token = session_token()?;
    let client = build_client()?;
    let query = build_query(args);
    let query_params = build_search_query_params(args, &query);

    let response = client
        .get(SEARCH_URL)
        .query(&query_params)
        .header("Cookie", format!("kagi_session={token}"))
        .send()
        .await
        .map_err(|error| Error::new(format!("Search request failed: {error}")))?;

    // An invalid or expired session answers with a redirect to
    // kagi.com/welcome, which serves 200; catch it by the final URL because
    // the status check below never fires for it.
    if response.url().path() != "/html/search" {
        return Err(Error::new("invalid or expired session token"));
    }

    let status = response.status();
    if status == 401 || status == 403 {
        return Err(Error::new("invalid or expired session token"));
    }
    if status == 429 {
        return Err(Error::new("rate limited"));
    }
    if !status.is_success() {
        return Err(Error::new(format!("HTTP {status}")));
    }

    let html = response
        .text()
        .await
        .map_err(|error| Error::new(format!("Failed to read response: {error}")))?;

    parse_search_results(&html, args.limit).map_err(Error::from)
}

pub async fn summarize(args: &SummarizeArgs) -> Result<SummarizeOutput, Error> {
    let token = session_token()?;
    let client = build_client()?;

    let response = client
        .get(SUMMARIZE_URL)
        .query(&[
            ("url", args.url.as_str()),
            ("stream", "1"),
            ("target_language", args.lang.as_str()),
            ("summary_type", args.summary_type.as_api_value()),
        ])
        .header("Accept", "application/vnd.kagi.stream")
        .header("Cookie", format!("kagi_session={token}"))
        .header("Referer", "https://kagi.com/summarizer")
        // Summarizing a long PDF or video can stream for minutes, so the
        // shared 30-second client deadline is wrong here. The server's
        // progress chunks reset the read timeout, so a stalled connection
        // still fails fast while an active stream runs to completion.
        .timeout(SUMMARIZE_TOTAL_TIMEOUT)
        .read_timeout(REQUEST_TIMEOUT)
        .send()
        .await
        .map_err(|error| Error::new(format!("Summarize request failed: {error}")))?;

    let status = response.status();
    if status == 401 || status == 403 {
        return Err(Error::new("invalid or expired session token"));
    }
    if status == 429 {
        return Err(Error::new("rate limited"));
    }
    if !status.is_success() {
        return Err(Error::new(format!("HTTP {status}")));
    }

    let body = response
        .bytes()
        .await
        .map_err(|error| Error::new(format!("Failed to read response: {error}")))?;

    parse_summary_stream(&body).map_err(Error::from)
}

pub async fn maps(args: &MapsArgs) -> Result<MapsOutput, Error> {
    let token = session_token()?;
    let client = build_client()?;
    let query = args.query.join(" ");
    let query_params = build_maps_query_params(args, &query);

    let response = client
        .get(MAPS_SEARCH_URL)
        .query(&query_params)
        .header("Cookie", format!("kagi_session={token}"))
        .header("Referer", "https://kagi.com/maps")
        .send()
        .await
        .map_err(|error| Error::new(format!("Maps request failed: {error}")))?;

    let status = response.status();
    if status == 401 || status == 403 {
        return Err(Error::new("invalid or expired session token"));
    }
    if status == 429 {
        return Err(Error::new("rate limited"));
    }
    if !status.is_success() {
        return Err(Error::new(format!("HTTP {status}")));
    }

    let body = response
        .bytes()
        .await
        .map_err(|error| Error::new(format!("Failed to read response: {error}")))?;
    let mut output = parse_maps_results(&body, usize::MAX).map_err(Error::from)?;
    sort_maps_results(&mut output.results, args.sort, args.order);
    output.results.truncate(args.limit);

    Ok(output)
}

fn build_client() -> Result<Client, Error> {
    Client::builder()
        .emulation(Emulation::Chrome131)
        .cookie_store(true)
        .redirect(wreq::redirect::Policy::limited(10))
        .https_only(true)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|error| Error::new(format!("Failed to build HTTP client: {error}")))
}

fn session_token() -> Result<String, Error> {
    // A set-but-empty variable (a typical missing CI secret) must not shadow
    // the token file, and stray whitespace would break the Cookie header.
    if let Some(value) = std::env::var_os(SESSION_TOKEN_ENV) {
        let value = value.to_string_lossy();
        let token = value.trim();
        if !token.is_empty() {
            return Ok(token.to_string());
        }
    }

    if let Some(token) = read_session_token_file()? {
        return Ok(token);
    }

    Err(Error::new(format!(
        "missing session token; export {SESSION_TOKEN_ENV} or store it in ~/.config/kagi/session-token (or $XDG_CONFIG_HOME/kagi/session-token)"
    )))
}

pub fn resolve_session_token_for_tests() -> Result<String, Error> {
    session_token()
}

fn read_session_token_file() -> Result<Option<String>, Error> {
    for path in session_token_file_candidates() {
        let metadata = match std::fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(Error::new(format!(
                    "failed to read session token file {}: {error}",
                    path.display()
                )));
            }
        };

        // The token is a full kagi.com session cookie; refuse to use it when
        // other local users can read it, mirroring ssh and .pgpass.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o077 != 0 {
                return Err(Error::new(format!(
                    "session token file {} is group/world-readable; run: chmod 600 {}",
                    path.display(),
                    path.display()
                )));
            }
        }
        #[cfg(not(unix))]
        let _ = &metadata;

        let contents = std::fs::read_to_string(&path).map_err(|error| {
            Error::new(format!(
                "failed to read session token file {}: {error}",
                path.display()
            ))
        })?;
        let token = contents.trim();
        if !token.is_empty() {
            return Ok(Some(token.to_string()));
        }
    }

    Ok(None)
}

fn session_token_file_candidates() -> Vec<PathBuf> {
    // Per the XDG Base Directory spec, a missing, empty, or relative
    // XDG_CONFIG_HOME counts as unset and falls back to ~/.config.
    let xdg_config_home = std::env::var_os(XDG_CONFIG_ENV)
        .filter(|value| Path::new(value).is_absolute())
        .map(PathBuf::from);

    let config_home =
        xdg_config_home.or_else(|| std::env::home_dir().map(|home| home.join(".config")));

    config_home
        .map(|dir| dir.join(SESSION_TOKEN_RELATIVE_PATH))
        .into_iter()
        .collect()
}

fn build_query(args: &SearchArgs) -> String {
    let mut query = args.query.join(" ");

    if let Some(site) = &args.site {
        query.push_str(&format!(" site:{site}"));
    }
    if let Some(filetype) = &args.filetype {
        query.push_str(&format!(" filetype:{filetype}"));
    }

    query
}

fn build_search_query_params(args: &SearchArgs, query: &str) -> Vec<(&'static str, String)> {
    let mut params = vec![("q", query.to_string())];

    if let Some(region) = &args.region {
        params.push(("r", region.clone()));
    }
    if let Some(lens) = args.lens {
        params.push(("l", lens.as_api_value().to_string()));
    }
    if let Some(sort) = args.sort {
        params.push(("order", sort.as_api_value().to_string()));
    }
    if let Some(time) = args.time {
        params.push(("dr", time.as_api_value().to_string()));
    }
    if let Some(from_date) = &args.from {
        params.push(("from_date", from_date.clone()));
    }
    if let Some(to_date) = &args.to {
        params.push(("to_date", to_date.clone()));
    }
    if args.verbatim {
        params.push(("verbatim", "1".to_string()));
    }

    params
}

fn build_maps_query_params(args: &MapsArgs, query: &str) -> Vec<(&'static str, String)> {
    let mut params = vec![("q", query.to_string())];

    if let Some(ll) = &args.ll {
        params.push(("ll", ll.clone()));
    }
    if let Some(bbox) = &args.bbox {
        params.push(("bbox", bbox.clone()));
    }
    if let Some(zoom) = args.zoom {
        params.push(("z", zoom.to_string()));
    }

    params
}

fn sort_maps_results(results: &mut [MapsResult], sort: Option<MapsSort>, order: Option<MapsOrder>) {
    let Some(sort) = sort else {
        return;
    };

    let order = order.unwrap_or(default_maps_order(sort));
    let multiplier = match order {
        MapsOrder::Asc => 1.0,
        MapsOrder::Desc => -1.0,
    };

    match sort {
        MapsSort::Relevance => {
            // The API returns results in descending relevance.
            if matches!(order, MapsOrder::Asc) {
                results.reverse();
            }
        }
        MapsSort::Rating => results
            .sort_by(|left, right| compare_optional_f64(left.rating, right.rating, multiplier)),
        MapsSort::Distance => results
            .sort_by(|left, right| compare_optional_f64(left.distance, right.distance, multiplier)),
        MapsSort::Price => results.sort_by(|left, right| {
            compare_optional_f64(
                left.price.as_deref().and_then(price_sort_key),
                right.price.as_deref().and_then(price_sort_key),
                multiplier,
            )
        }),
    }
}

/// The maps API returns prices in two shapes: a repeated currency-symbol
/// level ("$", "$$$") or an amount range ("€20–60", "€100+"). Map both onto
/// the 1–4 level scale: symbol runs by their count, amounts by the common
/// per-person thresholds (<15 → 1, <30 → 2, <60 → 3, else 4) with the amount
/// folded in as a sub-level fraction so amounts stay ordered among
/// themselves.
fn price_sort_key(price: &str) -> Option<f64> {
    let digits_start = price.find(|c: char| c.is_ascii_digit());
    if let Some(start) = digits_start {
        let rest = &price[start..];
        let end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        let amount: f64 = rest[..end].parse().ok()?;
        let level = match amount {
            amount if amount < 15.0 => 1.0,
            amount if amount < 30.0 => 2.0,
            amount if amount < 60.0 => 3.0,
            _ => 4.0,
        };
        return Some(level + (amount / 10_000.0).min(0.99));
    }

    let symbols = price.chars().filter(|c| !c.is_whitespace()).count();
    (symbols > 0).then_some(symbols as f64)
}

fn default_maps_order(sort: MapsSort) -> MapsOrder {
    match sort {
        MapsSort::Relevance | MapsSort::Rating => MapsOrder::Desc,
        MapsSort::Distance | MapsSort::Price => MapsOrder::Asc,
    }
}

fn compare_optional_f64(
    left: Option<f64>,
    right: Option<f64>,
    multiplier: f64,
) -> std::cmp::Ordering {
    match (left, right) {
        (Some(left), Some(right)) => (left * multiplier)
            .partial_cmp(&(right * multiplier))
            .unwrap_or(std::cmp::Ordering::Equal),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_maps_results;

    fn fixture_results() -> Vec<MapsResult> {
        let body = include_bytes!("../tests/fixtures/maps/search.json");
        parse_maps_results(body, usize::MAX).unwrap().results
    }

    fn prices(results: &[MapsResult]) -> Vec<&str> {
        results
            .iter()
            .filter_map(|result| result.price.as_deref())
            .collect()
    }

    #[test]
    fn price_sort_orders_symbol_runs_and_amount_ranges_together() {
        let mut results = fixture_results();

        sort_maps_results(&mut results, Some(MapsSort::Price), None);
        assert_eq!(prices(&results), ["$", "$$", "€20–60", "$$$", "€100+"]);

        sort_maps_results(&mut results, Some(MapsSort::Price), Some(MapsOrder::Desc));
        assert_eq!(prices(&results), ["€100+", "$$$", "€20–60", "$$", "$"]);
    }

    #[test]
    fn relevance_sort_honors_ascending_order() {
        let mut results = fixture_results();
        let api_order: Vec<String> = results.iter().map(|result| result.name.clone()).collect();

        sort_maps_results(&mut results, Some(MapsSort::Relevance), None);
        let unchanged: Vec<String> = results.iter().map(|result| result.name.clone()).collect();
        assert_eq!(unchanged, api_order);

        sort_maps_results(
            &mut results,
            Some(MapsSort::Relevance),
            Some(MapsOrder::Asc),
        );
        let reversed: Vec<String> = results.iter().map(|result| result.name.clone()).collect();
        let expected: Vec<String> = api_order.into_iter().rev().collect();
        assert_eq!(reversed, expected);
    }
}
