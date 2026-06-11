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
curl -fSL https://github.com/rubas/kagi/releases/download/v0.4.0/install.sh | sh -s v0.4.0
```

This installs:

- `kagi-search`, `kagi-maps`, and `kagi-summarize` to `~/.local/bin`
- skills to `~/.agents/skills/kagi-search`, `~/.agents/skills/kagi-maps`, `~/.agents/skills/kagi-summarize`
- skills to `~/.claude/skills/kagi-search`, `~/.claude/skills/kagi-maps`, `~/.claude/skills/kagi-summarize`
- skills to `~/.gemini/antigravity-cli/skills/kagi-search`, `~/.gemini/antigravity-cli/skills/kagi-maps`, `~/.gemini/antigravity-cli/skills/kagi-summarize`

Supported platforms: Linux x86_64 and macOS aarch64.

When the GitHub CLI (`gh`) is available, the installer verifies the build
provenance attestations of both archives before installing anything; without
`gh` it warns and continues. Set `KAGI_INSTALL_VERIFY=require` to fail instead,
or `KAGI_INSTALL_VERIFY=skip` to disable verification. For a fully verifiable
install path, prefer the Nix flake below: `flake.lock` pins every input by
hash.

### From source

```bash
cargo install --git https://github.com/rubas/kagi.git
```

For local development with the required native build tools:

```bash
nix develop
task ci
```

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

Options:

- `--limit <N>`: maximum results, default `10`
- `--region <REGION>`: region code such as `ch`, `us`, `de`, or `no_region`
- `--lens <LENS>`: `default`, `programming`, `forums`, `pdfs`, `non-commercial`, `world-news`
- `--sort <SORT>`: `recency`, `website`, `ad-trackers`
- `--time <RANGE>`: `day`, `week`, `month`, `year`
- `--from <YYYY-MM-DD>`: start date, cannot be combined with `--time`
- `--to <YYYY-MM-DD>`: end date, cannot be combined with `--time`
- `--site <DOMAIN>`: append a `site:` filter to the query
- `--filetype <EXT>`: append a `filetype:` filter to the query
- `--verbatim`: disable query expansion
- `--output <text|json>`: explicit output mode
- `-j, --json`: shortcut for JSON output
- `-h, --help`: print help
- `-V, --version`: print version

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

Options:

- `--limit <N>`: maximum results, default `10`
- `--ll <LAT,LON>`: search origin coordinate, such as `47.3769,8.5417`
- `--bbox <WEST,SOUTH,EAST,NORTH>`: map bounding box
- `--zoom <N>`: map zoom level passed to Kagi Maps as `z`
- `--sort <SORT>`: `relevance`, `rating`, `distance`, `price`
- `--order <ORDER>`: `asc`, `desc` (requires `--sort`)
- `--output <text|json>`: explicit output mode
- `-j, --json`: shortcut for JSON output
- `-h, --help`: print help
- `-V, --version`: print version

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

Options:

- `--type <summary|takeaway>`: summary mode, default `summary`
- `--lang <LANG>`: target language, default `EN`
- `--output <text|json>`: explicit output mode
- `-j, --json`: shortcut for JSON output
- `-h, --help`: print help
- `-V, --version`: print version

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

This repo ships companion skills for Claude Code and other agents:

- [`skills/search/SKILL.md`](skills/search/SKILL.md)
- [`skills/maps/SKILL.md`](skills/maps/SKILL.md)
- [`skills/summarize/SKILL.md`](skills/summarize/SKILL.md)

## License

[MIT](LICENSE)
