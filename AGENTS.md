# kagi

## Goal

Unofficial Unix-style CLIs for Kagi: `kagi-search`, `kagi-maps`, `kagi-summarize`.
Each binary ships a companion agent skill from `skills/`.

## Gates

- `task check` (fmt:check, lint, test) before you push.
- `task ci` adds the release-profile check, `nix build`, cargo-machete, and
  cargo-deny. GitHub Actions runs every one of those except `task test:nix`, so
  run `task ci` yourself after you touch `flake.nix`, `flake.lock`, or
  `Cargo.toml`. A lock-only input refresh still ships a Nix build nothing else
  checks.
- `task lint` calls `zizmor`. The `nix develop` shell does not ship it, so put
  `zizmor` on `PATH` before you run lint or check.
- `task test:live` hits the real Kagi service. Run it when you change the
  request path or a parser: `src/cli.rs`, `src/client.rs`, or `src/parse.rs`. It
  needs `KAGI_SESSION_TOKEN` or `~/.config/kagi/session-token` and fails without
  one. `task test:live:advisory` gives the same signal without failing the task.

## Layout

- `src/cli.rs`: argument definitions and the `as_api_value` tables that turn
  `--lens`, `--sort`, `--time`, and `--type` into raw Kagi parameter values.
- `src/client.rs`: token resolution, HTTP client, Kagi request parameters.
- `src/parse.rs`: search HTML, maps JSON, summarize stream.
- `skills/search`, `skills/maps`, `skills/summarize` install under the binary
  names `kagi-search`, `kagi-maps`, `kagi-summarize`. A rename changes all of
  these together:
  - `install.sh` and the `install` task in `Taskfile.yml`
  - `postInstall` and `skillNames` in `flake.nix`
  - `.github/workflows/release.yml`
  - the `name:` front matter in each `skills/*/SKILL.md`
  - the documented install paths in `README.md`
- `tests/live.rs` runs only under `--ignored`.

## Decisions

- Three separate binaries. Do not reintroduce a combined command.
- Never hardcode a session token. The client reads `KAGI_SESSION_TOKEN` (empty
  counts as unset), then `$XDG_CONFIG_HOME/kagi/session-token`, falling back to
  `~/.config/kagi/session-token`. It refuses a token file that is group- or
  world-readable.
- `--sort` means two different things. `kagi-search` sends it to Kagi as the
  `order` parameter. `kagi-maps` fetches the whole page, sorts locally, then
  truncates to `--limit`.
- A version bump is the release trigger: when a commit on `main` changes
  `version` in `Cargo.toml`, `release.yml` tags `v<version>` and publishes. No
  bump, no release. Push a tag by hand only to recover a failed run, through
  `workflow_dispatch` on `release.yml`.

## Pitfalls

- `flake.nix` repeats the package version as a literal. Bump it in the same
  commit as `Cargo.toml`, or `nix build` produces a package with the old
  version.
- The README pins the installer URL to a release tag. Update it with the version
  bump, or the documented install command points at the previous release.
- `ci.yml` skips a pull request whose author is not the repository owner. A
  contributor PR shows no checks; that is the gate, not a broken run.
