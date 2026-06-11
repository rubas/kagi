use assert_cmd::Command;
use predicates::prelude::*;

fn token() -> Option<String> {
    kagi::resolve_session_token_for_tests().ok()
}

#[test]
#[ignore = "live test requiring a real Kagi session token"]
fn live_search_returns_results() {
    let Some(token) = token() else {
        eprintln!("skipping live_search_returns_results: no token found via env or config file");
        return;
    };

    let output = Command::new(assert_cmd::cargo::cargo_bin!("kagi-search"))
        .env("KAGI_SESSION_TOKEN", token)
        .args(["--json", "--limit", "3", "rust programming language"])
        .output()
        .unwrap();

    assert!(output.status.success(), "kagi-search failed: {output:?}");
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let results = json["results"].as_array().unwrap();
    assert!(
        !results.is_empty(),
        "live search returned zero results; an empty array would also mask a token failure"
    );
}

#[test]
#[ignore = "live test requiring a real Kagi session token"]
fn live_search_rejects_bogus_token() {
    // An invalid session must fail loudly, not exit 0 with empty results.
    Command::new(assert_cmd::cargo::cargo_bin!("kagi-search"))
        .env("KAGI_SESSION_TOKEN", "bogus-token-for-live-test")
        .args(["--json", "--limit", "3", "rust"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid or expired session token"));
}

#[test]
#[ignore = "live test requiring a real Kagi session token"]
fn live_summarize_returns_json() {
    let Some(token) = token() else {
        eprintln!("skipping live_summarize_returns_json: no token found via env or config file");
        return;
    };

    Command::new(assert_cmd::cargo::cargo_bin!("kagi-summarize"))
        .env("KAGI_SESSION_TOKEN", token)
        .args(["--json", "https://www.rust-lang.org/learn"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"summary\""));
}

#[test]
#[ignore = "live test requiring a real Kagi session token"]
fn live_maps_returns_json() {
    let Some(token) = token() else {
        eprintln!("skipping live_maps_returns_json: no token found via env or config file");
        return;
    };

    Command::new(assert_cmd::cargo::cargo_bin!("kagi-maps"))
        .env("KAGI_SESSION_TOKEN", token)
        .args([
            "--json",
            "--limit",
            "3",
            "--ll",
            "47.3769,8.5417",
            "--zoom",
            "13",
            "coffee zurich",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"results\""));
}
