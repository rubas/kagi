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

fn temp_dir(label: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "kagi-{label}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn search_requires_session_token_before_network() {
    let temp_home = temp_dir("no-token");

    search_bin()
        .env_remove("KAGI_SESSION_TOKEN")
        .env("XDG_CONFIG_HOME", temp_home.join("xdg-config"))
        .args(["rust"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing session token"));
}

#[test]
fn search_treats_empty_env_token_as_missing() {
    let temp_home = temp_dir("empty-env-token");

    search_bin()
        .env("KAGI_SESSION_TOKEN", "")
        .env("XDG_CONFIG_HOME", temp_home.join("xdg-config"))
        .args(["rust"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing session token"));
}

#[test]
fn search_ignores_relative_xdg_config_home() {
    // An empty or relative XDG_CONFIG_HOME counts as unset per the XDG spec;
    // it must not resolve the token file against the working directory.
    let home = temp_dir("relative-xdg-home");
    let cwd = temp_dir("relative-xdg-cwd");
    std::fs::create_dir_all(cwd.join("kagi")).unwrap();
    std::fs::write(cwd.join("kagi/session-token"), "cwd-token\n").unwrap();

    search_bin()
        .env_remove("KAGI_SESSION_TOKEN")
        .env("XDG_CONFIG_HOME", "")
        .env("HOME", &home)
        .current_dir(&cwd)
        .args(["rust"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing session token"));
}

#[cfg(unix)]
#[test]
fn search_rejects_world_readable_token_file() {
    use std::os::unix::fs::PermissionsExt;

    // The file is found through the ~/.config fallback (XDG_CONFIG_HOME
    // unset) and rejected for its permissions before any network use.
    let home = temp_dir("world-readable-home");
    let token_dir = home.join(".config/kagi");
    std::fs::create_dir_all(&token_dir).unwrap();
    let token_file = token_dir.join("session-token");
    std::fs::write(&token_file, "file-token\n").unwrap();
    std::fs::set_permissions(&token_file, std::fs::Permissions::from_mode(0o644)).unwrap();

    search_bin()
        .env_remove("KAGI_SESSION_TOKEN")
        .env_remove("XDG_CONFIG_HOME")
        .env("HOME", &home)
        .args(["rust"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("group/world-readable"))
        .stderr(predicate::str::contains("chmod 600"));
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
    let temp_home = temp_dir("maps-antimeridian");

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
    let temp_home = temp_dir("maps-no-token");

    maps_bin()
        .env_remove("KAGI_SESSION_TOKEN")
        .env("XDG_CONFIG_HOME", temp_home.join("xdg-config"))
        .args(["coffee"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing session token"));
}

#[test]
fn maps_rejects_order_without_sort() {
    maps_bin()
        .args(["coffee", "--order", "asc"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--sort"));
}

/// Every flag a SKILL.md options table documents must exist in the
/// binary's --help output, so installed agents never emit invocations
/// that fail to parse.
fn assert_skill_flags_exist(skill_markdown: &str, mut command: Command) {
    let help = command.arg("--help").output().unwrap();
    assert!(help.status.success());
    let help = String::from_utf8(help.stdout).unwrap();

    let mut checked = 0;
    for line in skill_markdown.lines() {
        let Some(rest) = line.strip_prefix("| `") else {
            continue;
        };
        let Some(cell) = rest.split('`').next() else {
            continue;
        };
        for token in cell.split([' ', ',']) {
            if token.starts_with("--") {
                assert!(
                    help.contains(token),
                    "SKILL.md documents {token}, which is missing from --help"
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "no flags found in SKILL.md options table");
}

#[test]
fn search_skill_documents_only_real_flags() {
    assert_skill_flags_exist(include_str!("../skills/search/SKILL.md"), search_bin());
}

#[test]
fn maps_skill_documents_only_real_flags() {
    assert_skill_flags_exist(include_str!("../skills/maps/SKILL.md"), maps_bin());
}

#[test]
fn summarize_skill_documents_only_real_flags() {
    assert_skill_flags_exist(
        include_str!("../skills/summarize/SKILL.md"),
        summarize_bin(),
    );
}
