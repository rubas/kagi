# Changelog

All notable changes to this project are documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/).

## [0.4.0] - 2026-06-11

### Breaking

- `kagi-maps --json` now emits `review_count` instead of the accidental
  camelCase `reviewCount`, matching every other key in the schema.
- A session token file that is group- or world-readable is refused with an
  error telling you to `chmod 600` it. `KAGI_SESSION_TOKEN` is unaffected.
- `kagi-maps --order` without `--sort` is now a usage error; it silently did
  nothing before.

### Fixed

- `kagi-search` no longer reports an invalid or expired session token as a
  successful empty result. The redirect to kagi.com/welcome is detected and
  reported as a token error, and an unrecognized response page (markup change
  or block page) is an error instead of zero results.
- Search results preserve Kagi's ranking: standard and grouped results are
  collected in one document-order pass instead of appending grouped rows
  behind all standard rows (#7).
- `kagi-summarize` no longer aborts long-running summaries (large PDFs,
  videos) at 30 seconds. The stream now has a 30-second read timeout between
  chunks and a 5-minute total ceiling.
- The token file is found at `~/.config/kagi/session-token` when
  `XDG_CONFIG_HOME` is unset, and an empty or relative `XDG_CONFIG_HOME` no
  longer resolves the token file against the working directory.
- An empty `KAGI_SESSION_TOKEN` no longer shadows the token file, and
  whitespace around the env value is trimmed instead of corrupting the
  Cookie header.
- Piping output into a closed reader (e.g. `head -1`) exits cleanly instead
  of panicking with a broken-pipe backtrace.
- Control characters in server-derived text (titles, snippets, summaries,
  maps fields) are stripped in text output so embedded terminal escape
  sequences cannot rewrite the terminal.
- `kagi-maps --sort price` orders mixed price formats ("$$", "€20–60")
  by price level instead of UTF-8 byte length, and
  `--sort relevance --order asc` now actually reverses the API's
  descending relevance order.
- The shipped search and summarize skills no longer document a
  `--session-token` flag that does not exist; a test now checks every
  documented flag against `--help`.

### Changed

- The HTTP client refuses non-HTTPS redirects (`https_only`), so the session
  cookie can never travel in cleartext.
- All three binaries run on a current-thread Tokio runtime instead of
  spawning a worker thread per core for a single sequential request.
- `install.sh` verifies GitHub build provenance attestations when the `gh`
  CLI is available (`KAGI_INSTALL_VERIFY=require|skip` to tighten or opt
  out), validates the archive layout before touching installed files, and
  the release workflow attests both archives and smoke-tests the installer
  against the built artifacts before publishing.
- CI runs a weekly `cargo deny` advisory sweep against the merged lockfile.

## [0.3.0] - 2026-05-17

### Added

- `kagi-maps` binary: place and address search via the Kagi Maps API
  (`--limit`, `--ll`, `--bbox`, `--zoom`, `--sort`, `--order`, `--output`,
  `-j/--json`).
- Companion skill at `skills/maps/SKILL.md` plus install, release, and
  Home Manager wiring for the new binary.

### Changed

- Migrated HTTP client from `rquest` / `rquest-util` (yanked from crates.io)
  to the renamed `wreq` 5.3.0 / `wreq-util` 2.2.6.
- Bumped `scraper` 0.26 → 0.27 and `assert_cmd` 2.2.1 → 2.2.2.
- `--bbox` now accepts boxes that cross the antimeridian
  (e.g. `170,-10,-170,10`). Only `WEST == EAST` is rejected; latitude
  still requires `SOUTH < NORTH`.

## [0.2.1] - 2026-05-11

- Initial public release with `kagi-search` and `kagi-summarize`, Nix
  flake packaging, and Home Manager module.
