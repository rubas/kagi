# Changelog

All notable changes to this project are documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/).

## [0.5.4] - 2026-09-23

### Added

- `install.sh` installs the latest release when you give no version tag:
  `curl -fsSL https://github.com/rubas/kagi/releases/latest/download/install.sh | sh`.
  It reads the tag from the web redirect of `releases/latest` and needs no
  `gh`. A pinned tag, `sh -s -- v0.5.4`, still works.
- `install.sh --check` prints the installed and the target version and
  installs nothing. It exits 0 when the installed version matches the target
  and 100 when kagi is not installed or has a different version.
- `install.sh --force` installs again when the version already matches.
- The release attests `install.sh`. Check it with
  `gh attestation verify install.sh --repo rubas/kagi` before you run it.

### Changed

- `install.sh` does nothing when `~/.local/bin/kagi-search --version` already
  shows the target version. Before, each run installed again. Add `--force`
  to reinstall the same version.
- The skill installs once to `~/.agents/skills/kagi`. The installer links it
  into `~/.claude/skills`, `~/.codex/skills`, `~/.gemini/antigravity-cli/skills`,
  and `~/.pi/agent/skills` when that agent's config directory exists. The link
  target is relative, for example `../../.agents/skills/kagi`, and resolves
  through a symlinked config or `skills` directory.
- `~/.claude/skills/kagi` and `~/.gemini/antigravity-cli/skills/kagi` become
  symlinks instead of copies. Do not edit the skill there; the next install
  replaces `~/.agents/skills/kagi`. Codex and pi get the skill for the first
  time.
- An agent without a config directory gets no link. Before, the installer
  created `~/.claude/skills` and `~/.gemini/antigravity-cli/skills` in any
  case. The next release or `--force` links an agent set up after the install
  and restores a skill or link removed after it.
- The installer downloads and verifies only the platform archive, which
  contains the skill. It no longer downloads `kagi-skills.tar.gz`. The release
  still publishes it.
- The installer also removes the v0.4 per-binary skill directories from
  `~/.codex/skills` and `~/.pi/agent/skills`.
- `task install` installs the local build through `install.sh --force` in the
  release layout. It runs only on Linux x86_64 and macOS aarch64.

## [0.5.3] - 2026-09-23

### Fixed

- `kagi-maps` accepts a negative `--ll` or `--bbox` value as a separate
  argument, for example `--ll -33.8688,151.2093`. Before, clap read the value
  as an unknown flag (#27). A flag right after `--ll` or `--bbox` now counts as
  its value, so `--ll --limit 5` fails as an invalid coordinate instead of a
  missing value.
- `kagi-search` reports `unrecognized Kagi response page` when it finds result
  cards but can parse none of them. Before, it reported zero results (#28).
- `kagi-search` no longer reports a CAPTCHA block for a normal search page
  that contains the word captcha, for example a zero-result page for such a
  query, or any such page with `--limit 0` (#29).

### Changed

- `clap` 4.6.7, and a lockfile refresh of the transitive tree.
- The Nix build and the dev shell take rustc 1.98.1 from nixpkgs again.
  `rust-overlay` is gone from `flake.nix` and `flake.lock`.
- A failed platform build no longer publishes a partial GitHub release. The
  tag stays without a release until a `workflow_dispatch` run recovers it.

## [0.5.2] - 2026-09-21

### Changed

- The `kagi` skill description narrows to ranked hits with a region, lens, or
  site filter, places, and URL summaries. It points to the search skill for a
  synthesized answer.
- The `kagi` skill reads a source page directly. It uses `kagi-summarize` only
  when the user asks for a summary of a URL or the page is too large to read.
  After two failed search retries, it reports the coverage and the open
  questions.
- Release binaries build with thin LTO, 16 codegen units, and the default
  optimization level instead of fat LTO, one codegen unit, and
  `opt-level = "z"`.

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
