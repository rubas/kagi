# Contributing

This is a small personal project. Bug reports and feature requests via Issues are welcome. Pull requests may or may not be accepted at my discretion.

## Local setup

Requires Rust (edition 2024) and [Task](https://taskfile.dev).

```bash
task test          # unit and integration tests
task fmt:check     # formatting
task lint          # clippy
```

## Testing against Kagi

Live tests hit the real Kagi service and need a session token:

```bash
export KAGI_SESSION_TOKEN="..."
task test:live
```

## Releases

Releases are automated. Bump the version in `Cargo.toml`, merge to `main`, and the CI creates the GitHub release.
