use kagi::{parse_search_results, parse_summary_stream};

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

    assert_eq!(output.results.len(), 2);
    assert_eq!(output.results[0].title, "Grouped One");
    assert_eq!(output.results[0].url, "https://grouped.com/1");
    assert_eq!(output.results[0].snippet, "Grouped description one.");
}

#[test]
fn detects_captcha_fixture() {
    let html = include_str!("fixtures/search/captcha.html");
    let error = parse_search_results(html, 10).unwrap_err();
    assert!(error.contains("CAPTCHA"));
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
