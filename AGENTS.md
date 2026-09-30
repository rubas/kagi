# kagi

Three Unix-style CLIs for Kagi: `kagi-search`, `kagi-maps`, and `kagi-summarize`. `README.md`
covers install, the token, and usage.

## Gates

- `task ci` runs `task check`, the release-profile check, `nix build`, cargo-machete, and
  cargo-deny. GitHub Actions runs all of it except `task test:nix`. Run `task ci` yourself after
  you touch `flake.nix`, `flake.lock`, or `Cargo.toml`, because nothing else builds the Nix
  package. This includes a lock-only input refresh.
- `task lint`, and with it `task check` and `task ci`, runs `zizmor`. The `nix develop` shell does
  not include it, so put `zizmor` on `PATH` first.
- `task test:live` calls the real Kagi service. Run it when you change the request path or a
  parser: `src/cli.rs`, `src/client.rs`, or `src/parse.rs`. It needs a session token (see
  `README.md`) and fails without one.

## Rules

- Keep three separate binaries. Do not add a combined command again.
- Never hardcode a session token.
- `--sort` means two things. `kagi-search` sends it to Kagi as the `order` parameter. `kagi-maps`
  gets the whole result page, sorts it locally, then cuts it to `--limit`.
- `install.sh` is the one installer. `task install` and the release smoke test run it on local
  archives. `task install` packs the local build like a release archive, so it runs only on the
  two release platforms.
- The repo ships no agent skill. The `search` skill in rubas/dotfiles covers these CLIs.
- A version bump is the release trigger. On each push to `main`, `release.yml` reads `version`
  from `Cargo.toml`. When the tag `v<version>` does not exist, it tags and publishes. The tag
  decides, not the parent commit, so a push of many commits or a cancelled run does not lose a
  release. A tag that someone deletes comes back on the next push. To recover a failed build or
  release, run `release.yml` through `workflow_dispatch`.

- `main` requires signed commits, and the bot cannot sign a commit. It also cannot read the rule,
  so a PR with commits from a plain `git push` shows only `BLOCKED`. Commit through the GraphQL
  mutation `createCommitOnBranch`, which GitHub signs: stage the change, then run
  `gh-signed-commit "<headline>" ["<body>"]` from rubas/dotfiles. It also creates the branch on
  GitHub.

## Pitfalls

- `flake.nix` repeats the package version as a literal. Bump it in the same commit as
  `Cargo.toml`, or `nix build` makes a package with the old version.
- `install.sh` runs under `sh` on Linux with GNU tools and on macOS with BSD tools. Use POSIX sh
  and only flags that both sets have: no `realpath --relative-to`, no `ln -T`.
- `ci.yml` runs its checks only on a pull request from a branch of this repo, so a fork PR never
  reaches the incus runners. A fork PR shows no checks. That is the gate, not a broken run.
