# kagi

Unix-style Kagi CLI tools.

Each binary does one job:

- `kagi-search` finds sources
- `kagi-summarize` summarizes one explicit URL

Output is plain text by default and compact JSON with `--json`, so it works well for terminal use and agentic pipelines.

## Authentication

Authentication precedence:

1. `KAGI_SESSION_TOKEN`
2. `$XDG_CONFIG_HOME/kagi/session-token`
3. otherwise fail

For normal local use, the config-file fallback is:

- `$XDG_CONFIG_HOME/kagi/session-token`

The binaries never embed secrets.

## Install

From a local checkout:

```bash
task install
```

That installs:

- `kagi-search` and `kagi-summarize` to `~/.local/bin`
- skills to `~/.agents/skills/kagi-search`, `~/.agents/skills/kagi-summarize`
- skills to `~/.claude/skills/kagi-search`, `~/.claude/skills/kagi-summarize`

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

`kagi-summarize` returns:

- text: raw markdown summary
- json: `{ "summary": "..." }`

## Notes

- Both binaries fail fast if no session token is available.
- Local fallback token file uses XDG config resolution.
- `kagi-search` and `kagi-summarize` are intentionally separate commands.
- The repo does not store secrets in code or config files.

## Telemetry

Both binaries export OpenTelemetry traces to the same Incus/SigNoz stack used by your other tools.
Tracing setup is best-effort: if exporter initialization fails, the command still runs normally.

Captured fields include:

- command status and duration
- search query, result count, related count
- summarize URL, summary type, summary length
- error type for request, timeout, HTTP status, and parse failures

## Release

To produce a release, bump the version in `Cargo.toml` in the PR that you want to release and merge that PR to `main`.
A workflow on `main` creates the matching `vX.Y.Z` tag automatically, and that tag triggers the release workflow.
The GitHub release notes include copy-paste install commands for each platform.

## Skills

This repo also ships companion skills:

- [`skills/search/SKILL.md`](skills/search/SKILL.md)
- [`skills/summarize/SKILL.md`](skills/summarize/SKILL.md)
