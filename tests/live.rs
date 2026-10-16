use assert_cmd::Command;
use predicates::prelude::*;

// Each binary resolves the session token itself, so a missing or unusable
// token fails these tests instead of skipping them.

#[test]
#[ignore = "live test requiring a real Kagi session token"]
fn live_search_returns_results() {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("kagi-search"))
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
    Command::new(assert_cmd::cargo::cargo_bin!("kagi-summarize"))
        .args(["--json", "https://www.rust-lang.org/learn"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"summary\""));
}

#[test]
#[ignore = "live test requiring a real Kagi session token"]
fn live_maps_returns_json() {
    Command::new(assert_cmd::cargo::cargo_bin!("kagi-maps"))
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
