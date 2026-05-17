# Changelog

All notable changes to this project are documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/).

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
