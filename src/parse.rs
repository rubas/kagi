use scraper::{Html, Selector};
use serde::Serialize;

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

pub fn parse_search_results(html: &str, limit: usize) -> Result<SearchOutput, String> {
    let doc = Html::parse_document(html);

    let has_search_app = Selector::parse("#search-app")
        .ok()
        .is_some_and(|selector| doc.select(&selector).next().is_some());
    let has_search_results = Selector::parse(".search-result")
        .ok()
        .is_some_and(|selector| doc.select(&selector).next().is_some());
    let has_grouped_results = Selector::parse(".sr-group .__srgi")
        .ok()
        .is_some_and(|selector| doc.select(&selector).next().is_some());

    let lower = html.to_lowercase();
    if !has_search_app
        && !has_search_results
        && !has_grouped_results
        && (lower.contains("cf-challenge")
            || lower.contains("captcha")
            || lower.contains("challenge-platform")
            || lower.contains("just a moment"))
    {
        return Err("Blocked by CAPTCHA/challenge".into());
    }

    let mut results = Vec::new();

    if let Ok(selector) = Selector::parse(".search-result") {
        let title_selector = Selector::parse(".__sri_title_link").ok();
        let description_selector = Selector::parse(".__sri-desc").ok();

        for element in doc.select(&selector) {
            if results.len() >= limit {
                break;
            }

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

    if let Ok(selector) = Selector::parse(".sr-group .__srgi") {
        let title_selector = Selector::parse(".__srgi-title a").ok();
        let description_selector = Selector::parse(".__sri-desc").ok();

        for element in doc.select(&selector) {
            if results.len() >= limit {
                break;
            }

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

    results.truncate(limit);

    Ok(SearchOutput { results, related })
}

pub fn parse_summary_stream(body: &[u8]) -> Result<SummarizeOutput, String> {
    if body.is_empty() {
        return Err("Empty response from summarizer".into());
    }

    let chunks: Vec<&[u8]> = body.split(|&byte| byte == 0).collect();
    let last_chunk = chunks
        .iter()
        .rev()
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
        return Err(format!("Summarizer error: {reply}"));
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
