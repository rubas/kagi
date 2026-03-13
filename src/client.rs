use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

use rquest::Client;
use rquest_util::Emulation;

use crate::cli::{SearchArgs, SummarizeArgs};
use crate::error::Error;
use crate::parse::{SearchOutput, SummarizeOutput, parse_search_results, parse_summary_stream};

const SEARCH_URL: &str = "https://kagi.com/html/search";
const SUMMARIZE_URL: &str = "https://kagi.com/mother/summary_labs";
const SESSION_TOKEN_ENV: &str = "KAGI_SESSION_TOKEN";
const XDG_CONFIG_ENV: &str = "XDG_CONFIG_HOME";
const SESSION_TOKEN_RELATIVE_PATH: &str = "kagi/session-token";

pub async fn search(args: &SearchArgs) -> Result<SearchOutput, Error> {
    let token = session_token()?;
    let client = build_client()?;
    let query = build_query(args);
    let query_params = build_search_query_params(args, &query);

    let response = client
        .get(SEARCH_URL)
        .query(&query_params)
        .header(
            "Cookie",
            format!("kagi_session={}", token.to_string_lossy()),
        )
        .send()
        .await
        .map_err(|error| Error::new(format!("Search request failed: {error}")))?;

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
        .header(
            "Cookie",
            format!("kagi_session={}", token.to_string_lossy()),
        )
        .header("Referer", "https://kagi.com/summarizer")
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

fn build_client() -> Result<Client, Error> {
    Client::builder()
        .emulation(Emulation::Chrome131)
        .cookie_store(true)
        .redirect(rquest::redirect::Policy::limited(10))
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|error| Error::new(format!("Failed to build HTTP client: {error}")))
}

fn session_token() -> Result<OsString, Error> {
    if let Some(token) = std::env::var_os(SESSION_TOKEN_ENV) {
        return Ok(token);
    }

    if let Some(token) = read_session_token_file()? {
        return Ok(token);
    }

    Err(Error::new(format!(
        "missing session token; export {SESSION_TOKEN_ENV} or store it in $XDG_CONFIG_HOME/kagi/session-token"
    )))
}

pub fn resolve_session_token_for_tests() -> Result<OsString, Error> {
    session_token()
}

fn read_session_token_file() -> Result<Option<OsString>, Error> {
    for path in session_token_file_candidates() {
        match std::fs::read_to_string(&path) {
            Ok(contents) => {
                let token = contents.trim();
                if !token.is_empty() {
                    return Ok(Some(OsString::from(token)));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(Error::new(format!(
                    "failed to read session token file {}: {error}",
                    path.display()
                )));
            }
        }
    }

    Ok(None)
}

fn session_token_file_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(xdg_config_home) = std::env::var_os(XDG_CONFIG_ENV) {
        paths.push(PathBuf::from(xdg_config_home).join(SESSION_TOKEN_RELATIVE_PATH));
    }

    paths
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
