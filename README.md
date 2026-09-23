# kagi

Unofficial Unix-style CLI tools for [Kagi Search](https://kagi.com). Not affiliated with or endorsed by Kagi Inc.

Each binary does one job:

- `kagi-search` finds sources
- `kagi-maps` finds places and addresses
- `kagi-summarize` summarizes one explicit URL

Output is plain text by default and compact JSON with `--json`, so it works well for terminal use and agentic pipelines.

Requires a Kagi account with an active session token.

## Install

### From a GitHub release

```bash
curl -fsSL https://github.com/rubas/kagi/releases/latest/download/install.sh | sh
```

The installer puts `kagi-search`, `kagi-maps`, and `kagi-summarize` in `~/.local/bin` and the
`kagi` skill in `~/.agents/skills/kagi`. It links the skill into the skill directory of each agent
whose configuration directory exists:

| Agent directory             | Link                                                                      |
| --------------------------- | ------------------------------------------------------------------------- |
| `~/.claude`                 | `~/.claude/skills/kagi` -> `../../.agents/skills/kagi`                    |
| `~/.codex`                  | `~/.codex/skills/kagi` -> `../../.agents/skills/kagi`                     |
| `~/.gemini/antigravity-cli` | `~/.gemini/antigravity-cli/skills/kagi` -> `../../../.agents/skills/kagi` |
| `~/.pi/agent`               | `~/.pi/agent/skills/kagi` -> `../../../.agents/skills/kagi`               |

A link replaces a skill directory that an older installer copied there. When a configuration
directory or its `skills` directory is a symlink, the link target is the relative path between the
resolved directories, the same path `realpath --relative-to` gives. A `skills` directory that links
to `~/.agents/skills` already holds the skill and gets no link.

Run the same command again to update. When `~/.local/bin/kagi-search --version` already shows the
target version, the installer says so and changes nothing.

Put a release tag or an option after `sh -s --`:

```bash
# Install a given release.
curl -fsSL https://github.com/rubas/kagi/releases/latest/download/install.sh | sh -s -- v0.5.3
# Show the installed and the latest version, and install nothing.
curl -fsSL https://github.com/rubas/kagi/releases/latest/download/install.sh | sh -s -- --check
```

- `--check` exits 0 when the install is current and 100 when an update is available.
- `--force` installs again when the version already matches.

Supported platforms: Linux x86_64 and macOS aarch64.

When the GitHub CLI (`gh`) is available, the installer verifies the build provenance attestation of
the release archive before it changes a file; without `gh` it warns and continues. For a fully
verifiable install path, prefer the Nix flake below: `flake.lock` pins every input by hash.

The release also attests `install.sh`. To verify the installer before you run it:

```bash
curl -fsSLO https://github.com/rubas/kagi/releases/latest/download/install.sh &&
  gh attestation verify install.sh --repo rubas/kagi &&
  KAGI_INSTALL_VERIFY=require sh install.sh
```

| Variable                | Effect                                                                                                                  |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `KAGI_INSTALL_VERIFY`   | `auto` (default) verifies when `gh` is available. `require` fails without `gh`. `skip` does not verify.                 |
| `KAGI_INSTALL_BASE_URL` | Downloads the archive from this URL instead of the GitHub release, for example `file:///tmp/kagi`. Needs a version tag. |
| `KAGI_INSTALL_VERSION`  | The version tag when no argument gives one.                                                                             |
| `KAGI_INSTALL_REPO`     | The GitHub repository, `rubas/kagi` by default.                                                                         |

### From source

```bash
cargo install --git https://github.com/rubas/kagi.git
```

For local development with the required native build tools:

```bash
nix develop
task check
```

`task lint` also runs [zizmor](https://github.com/zizmorcore/zizmor) over the
workflows, and the dev shell does not include it. Install `zizmor` separately or
that step fails.

### With Nix flakes

Install the CLIs directly:

```bash
nix profile install github:rubas/kagi
```

Or enable the Home Manager module to install the CLIs and companion skills:

```nix
{
  inputs.kagi.url = "github:rubas/kagi";

  outputs = { kagi, ... }: {
    homeConfigurations.example = home-manager.lib.homeManagerConfiguration {
      modules = [
        kagi.homeManagerModules.default
        {
          programs.kagi.enable = true;
        }
      ];
    };
  };
}
```

## Authentication

You need a Kagi session token. The binaries check these sources in order:

1. `KAGI_SESSION_TOKEN` environment variable (ignored when empty)
2. `$XDG_CONFIG_HOME/kagi/session-token`, falling back to
   `~/.config/kagi/session-token` when `XDG_CONFIG_HOME` is unset, empty, or
   relative
3. Fail with an error if neither is set

Create the token file with owner-only permissions (paste the token, then
Ctrl-D):

```bash
install -d -m 700 ~/.config/kagi
(umask 077; cat > ~/.config/kagi/session-token)
```

The token is a full kagi.com session cookie, so the binaries refuse a token
file that is group- or world-readable (`chmod 600` fixes it). The permission
check does not apply to `KAGI_SESSION_TOKEN`. The binaries never embed or
store secrets.

## Binaries

### `kagi-search`

Search the web with Kagi.

Usage:

```bash
kagi-search [OPTIONS] <QUERY>...
```

Run the binary with `--help` for all flags.

Examples:

```bash
kagi-search 'rust async runtime' --lens programming --limit 5
kagi-search 'SBB Fahrplan' --region ch --sort recency --time week
kagi-search 'memory leak' --site github.com --filetype rs --json
```

### `kagi-maps`

Search Kagi Maps for places, businesses, points of interest, and addresses.

Usage:

```bash
kagi-maps [OPTIONS] <QUERY>...
```

Run the binary with `--help` for all flags.

Examples:

```bash
kagi-maps 'coffee zurich' --ll 47.3769,8.5417 --zoom 13
kagi-maps 'bookstore near bern' --sort rating --json
```

### `kagi-summarize`

Summarize one explicit URL with Kagi.

Usage:

```bash
kagi-summarize [OPTIONS] <URL>
```

Run the binary with `--help` for all flags.

Examples:

```bash
kagi-summarize 'https://www.rust-lang.org/learn'
kagi-summarize 'https://www.rust-lang.org/learn' --type takeaway
kagi-summarize 'https://www.rust-lang.org/learn' --lang DE --json
```

## Output

`kagi-search` returns:

- text: numbered results plus optional `Related:` terms
- json: `{ "results": [...], "related": [...] }`

`kagi-maps` returns:

- text: numbered places with address, coordinates, rating, phone, and URL when present
- json: `{ "results": [...] }`

`kagi-summarize` returns:

- text: raw markdown summary
- json: `{ "summary": "..." }`

## Skills

This repo ships one companion skill for Claude Code and other agents:

- [`skills/kagi/SKILL.md`](skills/kagi/SKILL.md): one skill covering search, maps, and summarize

## License

[MIT](LICENSE)
