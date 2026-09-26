# kagi

Three Unix-style CLIs for Kagi, `kagi-search`, `kagi-maps`, and `kagi-summarize`, and one agent
skill, `skills/kagi`, for all three. `README.md` covers install, the token, and usage.

## Gates

- `task ci` runs `task check`, the release-profile check, `nix build`, cargo-machete, and
  cargo-deny. GitHub Actions runs all of it except `task test:nix`. Run `task ci` yourself after
  you touch `flake.nix`, `flake.lock`, or `Cargo.toml`, because nothing else builds the Nix
  package. This includes a lock-only input refresh.
- `task test:live` calls the real Kagi service. Run it when you change the request path or a
  parser: `src/cli.rs`, `src/client.rs`, or `src/parse.rs`. It needs a session token (see
  `README.md`) and fails without one.

## Rules

- Keep three separate binaries. Do not add a combined command again.
- Never hardcode a session token.
- A rename of the skill changes all of these together:
  - `install.sh` and the `install` task in `Taskfile.yml`
  - `postInstall` and `skillNames` in `flake.nix`
  - `.github/workflows/release.yml`
  - the `name:` front matter in `skills/kagi/SKILL.md`
  - the install paths in `README.md`
- `--sort` means two things. `kagi-search` sends it to Kagi as the `order` parameter. `kagi-maps`
  gets the whole result page, sorts it locally, then cuts it to `--limit`.
- `install.sh` is the one installer. `task install` and the release smoke test run it on local
  archives. It installs the skill once to `~/.agents/skills/kagi` and links it into the skill
  directory of each agent. Each link target is the path `realpath -m --relative-to` gives between
  the physical paths, the same link a dotfiles fan-out from `~/.agents/skills` makes. Keep the two
  identical, or the installer and the fan-out replace each other's links on every run.
  `task install` packs the local build like a release archive, so it runs only on the two release
  platforms.
- A version bump is the release trigger. On each push to `main`, `release.yml` reads `version`
  from `Cargo.toml`. When the tag `v<version>` does not exist, it tags and publishes. The tag
  decides, not the parent commit, so a push of many commits or a cancelled run does not lose a
  release. A tag that someone deletes comes back on the next push. To recover a failed build or
  release, run `release.yml` through `workflow_dispatch`.

## Pitfalls

- `flake.nix` repeats the package version as a literal. Bump it in the same commit as
  `Cargo.toml`, or `nix build` makes a package with the old version.
- `install.sh` runs under `sh` on Linux with GNU tools and on macOS with BSD tools. Use POSIX sh
  and only flags that both sets have: no `realpath --relative-to`, no `ln -T`.
- `ci.yml` skips a pull request whose author is not the repository owner. A contributor PR shows
  no checks. That is the gate, not a broken run.
