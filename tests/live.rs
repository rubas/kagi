use assert_cmd::Command;
use predicates::prelude::*;

fn token() -> Option<String> {
    kagi::resolve_session_token_for_tests()
        .ok()
        .map(|value| value.to_string_lossy().into_owned())
}

#[test]
#[ignore = "live test requiring a real Kagi session token"]
fn live_search_returns_json() {
    let Some(token) = token() else {
        eprintln!("skipping live_search_returns_json: no token found via env or XDG config");
        return;
    };

    Command::new(assert_cmd::cargo::cargo_bin!("kagi-search"))
        .env("KAGI_SESSION_TOKEN", token)
        .args(["--json", "--limit", "3", "rust programming language"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"results\""));
}

#[test]
#[ignore = "live test requiring a real Kagi session token"]
fn live_summarize_returns_json() {
    let Some(token) = token() else {
        eprintln!("skipping live_summarize_returns_json: no token found via env or XDG config");
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
        eprintln!("skipping live_maps_returns_json: no token found via env or XDG config");
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
