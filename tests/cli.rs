use assert_cmd::Command;
use predicates::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn search_bin() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("kagi-search"))
}

fn summarize_bin() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("kagi-summarize"))
}

fn maps_bin() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("kagi-maps"))
}

#[test]
fn search_help_lists_search_flags() {
    search_bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--limit"))
        .stdout(predicate::str::contains("--lens"))
        .stdout(predicate::str::contains("QUERY"));
}

#[test]
fn search_rejects_time_and_date_range_together() {
    search_bin()
        .args(["rust", "--time", "week", "--from", "2026-03-01"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn search_rejects_invalid_date() {
    search_bin()
        .args(["rust", "--from", "20260301"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected YYYY-MM-DD"));
}

#[test]
fn search_requires_session_token_before_network() {
    let temp_home = std::env::temp_dir().join(format!(
        "kagi-no-token-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_home).unwrap();

    search_bin()
        .env_remove("KAGI_SESSION_TOKEN")
        .env("XDG_CONFIG_HOME", temp_home.join("xdg-config"))
        .args(["rust"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing session token"));
}

#[test]
fn summarize_requires_url() {
    summarize_bin()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"))
        .stderr(predicate::str::contains("<URL>"));
}

#[test]
fn summarize_help_lists_summary_flags() {
    summarize_bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--type"))
        .stdout(predicate::str::contains("--lang"))
        .stdout(predicate::str::contains("URL"));
}

#[test]
fn maps_help_lists_maps_flags() {
    maps_bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--ll"))
        .stdout(predicate::str::contains("--bbox"))
        .stdout(predicate::str::contains("--zoom"))
        .stdout(predicate::str::contains("QUERY"));
}

#[test]
fn maps_rejects_invalid_coordinates() {
    maps_bin()
        .args(["coffee", "--ll", "100,8"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected LAT,LON"));
}

#[test]
fn maps_accepts_antimeridian_bbox() {
    // Box around Fiji crossing the 180° meridian: west=170, east=-170.
    // Must reach the session-token check, not bounce off bbox validation.
    let temp_home = std::env::temp_dir().join(format!(
        "kagi-maps-antimeridian-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_home).unwrap();

    maps_bin()
        .env_remove("KAGI_SESSION_TOKEN")
        .env("XDG_CONFIG_HOME", temp_home.join("xdg-config"))
        .args(["coffee", "--bbox", "170,-10,-170,10"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing session token"));
}

#[test]
fn maps_rejects_degenerate_bbox() {
    maps_bin()
        .args(["coffee", "--bbox", "10,0,10,5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("WEST and EAST must differ"));
}

#[test]
fn maps_rejects_inverted_latitude_bbox() {
    maps_bin()
        .args(["coffee", "--bbox", "0,10,5,5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("SOUTH < NORTH"));
}

#[test]
fn maps_requires_session_token_before_network() {
    let temp_home = std::env::temp_dir().join(format!(
        "kagi-maps-no-token-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_home).unwrap();

    maps_bin()
        .env_remove("KAGI_SESSION_TOKEN")
        .env("XDG_CONFIG_HOME", temp_home.join("xdg-config"))
        .args(["coffee"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing session token"));
}
