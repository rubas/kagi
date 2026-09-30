# kagi

Three Unix-style CLIs for Kagi: `kagi-search`, `kagi-maps`, and `kagi-summarize`. `README.md`
covers install, the token, and usage.

## Gates

- `task test:live` calls the real Kagi service. Run it when you change the request path or a
  parser: `src/cli.rs`, `src/client.rs`, or `src/parse.rs`. It needs a session token (see
  `README.md`) and fails without one.

## Rules

- Keep three separate binaries. Do not add a combined command again.
- Never hardcode a session token.
- `--sort` means two things. `kagi-search` sends it to Kagi as the `order` parameter. `kagi-maps`
  gets the whole result page, sorts it locally, then cuts it to `--limit`.
- Our machines install the CLIs with mise (`github:rubas/kagi`). mise picks the release archive
  by the OS and architecture in its name, `kagi-linux-x86_64.tar.gz` or
  `kagi-macos-aarch64.tar.gz`, and finds the binaries in its `bin/` dir. A change to an archive
  name or layout breaks the install on every machine.
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

- `ci.yml` runs its checks only on a pull request from a branch of this repo, so a fork PR never
  reaches the incus runners. A fork PR shows no checks. That is the gate, not a broken run.
