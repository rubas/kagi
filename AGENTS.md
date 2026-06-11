# kagi - Project Rules

Rust CLI repo for `kagi-search`, `kagi-maps`, and `kagi-summarize`.

## Testing

- Run `task test` for normal local verification.
- Run `task test:live` when you change real request-path code:
  - token resolution
  - HTTP client behavior
  - Kagi request parameters
  - maps request parameters
  - search HTML parsing
  - maps JSON parsing
  - summarize stream parsing
  - timeout handling
- `task test:live` uses the real Kagi service and requires either:
  - `KAGI_SESSION_TOKEN`
  - or `${XDG_CONFIG_HOME:-$HOME/.config}/kagi/session-token` (mode 600)
- If you only want signal without a failing task, use `task test:live:advisory`.

## Secrets

- Never hardcode session tokens in code or docs.
- Runtime token sources are:
  - `KAGI_SESSION_TOKEN` (ignored when empty)
  - `${XDG_CONFIG_HOME:-$HOME/.config}/kagi/session-token`, refused unless
    file mode is owner-only (600)

## CLI Contract

- Keep `kagi-search`, `kagi-maps`, and `kagi-summarize` separate.
- Do not reintroduce a combined multi-purpose command.

## Release Flow

- Releases are created when a commit merged to `main` bumps the package version in `Cargo.toml`.
- The `release` workflow detects the version bump on `main`, creates the matching `v<version>` tag, and publishes the GitHub release in the same run.
- `workflow_dispatch` on `release.yml` remains the manual fallback for recovery runs against an existing tag or ref.
- Do not push release tags by hand unless the workflow failed and you are recovering it deliberately.

Exact sequence:

1. Make the intended version bump in `Cargo.toml` in the same PR as the releaseable changes.
2. Open the PR.
3. Merge the PR to `main`.
4. The `release` workflow compares the new `Cargo.toml` version on `main` with the previous commit.
5. If the version changed, it creates and pushes `vX.Y.Z`, builds the archives from that tag, and publishes the release.

- No version bump in `Cargo.toml` means no release.
- One version bump merged to `main` means one release attempt.
- Do not tag feature branches or intermediate commits.
- Bump `Cargo.toml` only when you want the merge to produce a release.
