# Changelog

All notable changes to this project are documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/).

## [0.5.3] - 2026-09-23

### Fixed

- `kagi-maps` accepts a negative `--ll` or `--bbox` value as a separate
  argument, for example `--ll -33.8688,151.2093`. Before, clap read the value
  as an unknown flag (#27).
- `kagi-search` reports `unrecognized Kagi response page` when it finds result
  cards but can parse none of them. Before, it reported zero results (#28).
- `kagi-search` no longer reports a CAPTCHA block for a normal search page
  that contains the word captcha, for example a zero-result page for such a
  query, or any such page with `--limit 0` (#29).

### Changed

- `clap` 4.6.7, and a lockfile refresh of the transitive tree.
- The Nix build and the dev shell take rustc 1.98.1 from nixpkgs again.
  `rust-overlay` is gone from `flake.nix` and `flake.lock`.

## [0.5.2] - 2026-09-21

### Changed

- The `kagi` skill description narrows to ranked hits with a region, lens, or
  site filter, places, and URL summaries. It points to the search skill for a
  synthesized answer.
- The `kagi` skill reads a source page directly. It uses `kagi-summarize` only
  when the user asks for a summary of a URL or the page is too large to read.
  After two failed search retries, it reports the coverage and the open
  questions.
- Release binaries build with thin LTO and the default optimization level
  instead of `opt-level = "z"`.

## [0.5.1] - 2026-08-29

0.5.0 was never published: its release build still packaged the three old
skill directories. 0.5.1 ships the same change.

### Breaking

- One `kagi` skill replaces the `kagi-search`, `kagi-maps`, and
  `kagi-summarize` skills. `install.sh`, the Nix package, and the Home Manager
  module install only `kagi`. The installer removes the three old skill
  directories. Update any reference to the old skill names.

### Changed

- The skill descriptions say when to use the skill. The README no longer
  repeats the option lists; `--help` is the reference.

## [0.4.1] - 2026-08-28

### Changed

- Move to the maintained `wreq` line: `wreq` 0.16.1 and `wreq-util` 0.2.0.
  Upstream reset the version numbers. It yanked the 5.x and 2.2.x lines, and
  it froze the 6.0.0-rc series. The old code that kagi used now has the
  number 0.15.3. Development continues on 0.16. This is the true upgrade,
  not the renumber.
- `wreq` 0.16 replaces the BoringSSL binding with `btls`. It moves the HTTP
  core into `wreq-proto`, and it rebuilds the client on Tower middleware. The
  CLI, the output, and the Chrome 131 profile do not change.
- The Nix build and the dev shell now take the Rust toolchain from
  `rust-overlay`. `wreq` 0.16 needs rustc 1.98, but nixpkgs-unstable still
  has 1.97.1.
- `Cargo.toml` now declares `rust-version = "1.98"`.
- `clap` 4.6.3 to 4.6.6, and a lockfile refresh of the transitive tree.
- `flake.lock`: refresh `nixpkgs` and `crane`, and add `rust-overlay`.

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
