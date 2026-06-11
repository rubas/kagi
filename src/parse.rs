use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub snippet: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SearchOutput {
    pub results: Vec<SearchResult>,
    pub related: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SummarizeOutput {
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapsCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MapsResult {
    pub name: String,
    pub address: Option<String>,
    pub coordinates: MapsCoordinates,
    pub phone: Option<String>,
    pub url: Option<String>,
    pub source: Option<String>,
    pub id: Option<String>,
    pub rating: Option<f64>,
    pub review_count: Option<u64>,
    pub price: Option<String>,
    pub distance: Option<f64>,
    pub hours_now: Option<String>,
    pub types: Option<Vec<String>>,
    pub links: Option<Value>,
    pub images: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MapsOutput {
    pub results: Vec<MapsResult>,
}

#[derive(Debug, Deserialize)]
struct MapsApiResponse {
    pois: Vec<MapsApiPoi>,
}

#[derive(Debug, Deserialize)]
struct MapsApiPoi {
    name: String,
    address: Option<String>,
    coordinates: MapsCoordinates,
    phone: Option<String>,
    url: Option<String>,
    source: Option<String>,
    id: Option<String>,
    id_k: Option<String>,
    rating: Option<f64>,
    #[serde(rename = "reviewCount")]
    review_count: Option<u64>,
    price: Option<String>,
    distance: Option<f64>,
    hours_now: Option<String>,
    types: Option<Vec<String>>,
    links: Option<Value>,
    images: Option<Value>,
}

pub fn parse_search_results(html: &str, limit: usize) -> Result<SearchOutput, String> {
    let doc = Html::parse_document(html);

    let mut results = Vec::new();

    // Standard and grouped results interleave on real pages, so collect both in
    // one document-order pass; separate passes would push grouped rows behind
    // standard rows and the limit would cut Kagi's top-ranked results.
    if let Ok(selector) = Selector::parse(".search-result, .sr-group .__srgi") {
        let standard_title_selector = Selector::parse(".__sri_title_link").ok();
        let grouped_title_selector = Selector::parse(".__srgi-title a").ok();
        let description_selector = Selector::parse(".__sri-desc").ok();

        for element in doc.select(&selector) {
            if results.len() >= limit {
                break;
            }

            let is_grouped = element.value().classes().any(|class| class == "__srgi");
            let title_selector = if is_grouped {
                &grouped_title_selector
            } else {
                &standard_title_selector
            };

            let Some((title, url)) = title_selector.as_ref().and_then(|selector| {
                let link = element.select(selector).next()?;
                let href = link.value().attr("href")?.to_string();
                let title = link.text().collect::<String>().trim().to_string();
                Some((title, href))
            }) else {
                continue;
            };

            let snippet = description_selector
                .as_ref()
                .and_then(|selector| {
                    let description = element.select(selector).next()?;
                    Some(description.text().collect::<String>().trim().to_string())
                })
                .unwrap_or_default();

            results.push(SearchResult {
                url,
                title,
                snippet,
            });
        }
    }

    let mut related = Vec::new();
    if let Ok(selector) = Selector::parse(".related-searches a span") {
        for element in doc.select(&selector) {
            let text = element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                related.push(text);
            }
        }
    }

    // Zero results is only trustworthy on a recognized result page. A
    // genuine zero-result page still renders the search shell; a CAPTCHA
    // interstitial, the kagi.com/welcome page, or a markup redesign does
    // not, and silently reporting "no results" for those misleads callers.
    if results.is_empty() {
        let lower = html.to_lowercase();
        if lower.contains("cf-challenge")
            || lower.contains("captcha")
            || lower.contains("challenge-platform")
            || lower.contains("just a moment")
        {
            return Err("Blocked by CAPTCHA/challenge".into());
        }

        let has_search_shell = ["._0_main-search-results", ".footer-search-results"]
            .iter()
            .any(|shell_selector| {
                Selector::parse(shell_selector)
                    .ok()
                    .is_some_and(|selector| doc.select(&selector).next().is_some())
            });
        if !has_search_shell {
            return Err("unrecognized Kagi response page (markup change or block page)".into());
        }
    }

    Ok(SearchOutput { results, related })
}

pub fn parse_maps_results(body: &[u8], limit: usize) -> Result<MapsOutput, String> {
    let response: MapsApiResponse = serde_json::from_slice(body)
        .map_err(|error| format!("Failed to parse maps JSON: {error}"))?;

    let results = response
        .pois
        .into_iter()
        .take(limit)
        .map(|poi| MapsResult {
            name: poi.name,
            address: poi.address,
            coordinates: poi.coordinates,
            phone: poi.phone,
            url: poi.url,
            source: poi.source,
            id: poi.id_k.or(poi.id),
            rating: poi.rating,
            review_count: poi.review_count,
            price: poi.price,
            distance: poi.distance,
            hours_now: poi.hours_now,
            types: poi.types,
            links: poi.links,
            images: poi.images,
        })
        .collect();

    Ok(MapsOutput { results })
}

pub fn parse_summary_stream(body: &[u8]) -> Result<SummarizeOutput, String> {
    if body.is_empty() {
        return Err("Empty response from summarizer".into());
    }

    let last_chunk = body
        .rsplit(|&byte| byte == 0)
        .find(|chunk| chunk.iter().any(|byte| !byte.is_ascii_whitespace()))
        .ok_or("No data chunks in response")?;

    let text = std::str::from_utf8(last_chunk)
        .map_err(|error| format!("Invalid UTF-8 in response: {error}"))?
        .trim();

    let json_str = if let Some(rest) = text.strip_prefix("final:") {
        rest.trim()
    } else if let Some(rest) = text.strip_prefix("new_message.json:") {
        rest.trim()
    } else {
        text
    };

    let json: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|error| format!("Failed to parse summary JSON: {error}"))?;

    if json["state"].as_str() == Some("error") {
        let reply = json["reply"].as_str().unwrap_or("Unknown error");
        // The reply is server-derived text that ends up on a terminal.
        return Err(format!(
            "Summarizer error: {}",
            crate::output::sanitize(reply)
        ));
    }

    let markdown = json["md"]
        .as_str()
        .or_else(|| json["output_data"]["markdown"].as_str())
        .ok_or("Missing markdown in response")?
        .to_string();

    if markdown.is_empty() {
        return Err("Empty summary returned".into());
    }

    Ok(SummarizeOutput { summary: markdown })
}
