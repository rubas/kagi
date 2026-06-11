//! Fixture-level parser tests for Kagi search HTML, maps JSON, and summarizer streams.
//!
//! These tests cover local parsing contracts only. They do not perform live Kagi requests.

use kagi::{parse_maps_results, parse_search_results, parse_summary_stream};

#[test]
fn parses_standard_results_fixture() {
    let html = include_str!("fixtures/search/basic.html");
    let output = parse_search_results(html, 2).unwrap();

    assert_eq!(output.results.len(), 2);
    assert_eq!(output.results[0].title, "Example One");
    assert_eq!(output.results[0].url, "https://example.com/1");
    assert_eq!(output.results[0].snippet, "First result description.");
    assert_eq!(output.results[1].title, "Example Two");
    assert_eq!(
        output.related,
        vec![
            "related term one".to_string(),
            "related term two".to_string()
        ]
    );
}

#[test]
fn parses_grouped_results_fixture() {
    let html = include_str!("fixtures/search/grouped.html");
    let output = parse_search_results(html, 10).unwrap();

    let urls: Vec<&str> = output
        .results
        .iter()
        .map(|result| result.url.as_str())
        .collect();
    assert_eq!(
        urls,
        [
            "https://grouped.com/1",
            "https://grouped.com/2",
            "https://standard.com/1",
            "https://grouped.com/3",
            "https://standard.com/2",
        ]
    );
    assert_eq!(output.results[0].title, "Grouped One");
    assert_eq!(output.results[0].snippet, "Grouped description one.");
    assert_eq!(output.results[2].title, "Standard One");
    assert_eq!(output.results[2].snippet, "Standard description one.");
}

#[test]
fn grouped_results_keep_document_order_under_limit() {
    let html = include_str!("fixtures/search/grouped.html");
    let output = parse_search_results(html, 3).unwrap();

    let urls: Vec<&str> = output
        .results
        .iter()
        .map(|result| result.url.as_str())
        .collect();
    assert_eq!(
        urls,
        [
            "https://grouped.com/1",
            "https://grouped.com/2",
            "https://standard.com/1",
        ]
    );
}

#[test]
fn detects_captcha_fixture() {
    let html = include_str!("fixtures/search/captcha.html");
    let error = parse_search_results(html, 10).unwrap_err();
    assert!(error.contains("CAPTCHA"));
}

#[test]
fn parses_maps_results_fixture() {
    let body = include_bytes!("fixtures/maps/search.json");
    let output = parse_maps_results(body, 1).unwrap();

    assert_eq!(output.results.len(), 1);
    assert_eq!(output.results[0].name, "Example Coffee");
    assert_eq!(
        output.results[0].address.as_deref(),
        Some("Example Street 1")
    );
    assert_eq!(output.results[0].coordinates.latitude, 47.3726576);
    assert_eq!(output.results[0].coordinates.longitude, 8.5262939);
    assert_eq!(output.results[0].phone.as_deref(), Some("+41 44 000 00 00"));
    assert_eq!(output.results[0].rating, Some(4.7));
    assert_eq!(output.results[0].review_count, Some(477));
    assert_eq!(output.results[0].price.as_deref(), Some("$$"));
}

#[test]
fn parses_summary_stream_fixture() {
    let body = decode_nul_fixture(include_str!("fixtures/summary/stream.txt"));
    let output = parse_summary_stream(&body).unwrap();
    assert_eq!(output.summary, "# Summary\nThis is the summary.");
}

#[test]
fn parses_summary_stream_output_data_fallback() {
    let body = decode_nul_fixture(include_str!("fixtures/summary/fallback.txt"));
    let output = parse_summary_stream(&body).unwrap();
    assert_eq!(output.summary, "Fallback content.");
}

#[test]
fn reports_summary_error_state() {
    let body = decode_nul_fixture(include_str!("fixtures/summary/error.txt"));
    let error = parse_summary_stream(&body).unwrap_err();
    assert!(error.contains("sorry"));
}

fn decode_nul_fixture(input: &str) -> Vec<u8> {
    input.replace("[NUL]", "\0").into_bytes()
}
