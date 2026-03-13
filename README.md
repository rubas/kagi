# kagi

Unofficial Unix-style CLI tools for [Kagi Search](https://kagi.com). Not affiliated with or endorsed by Kagi Inc.

Each binary does one job:

- `kagi-search` finds sources
- `kagi-summarize` summarizes one explicit URL

Output is plain text by default and compact JSON with `--json`, so it works well for terminal use and agentic pipelines.

Requires a Kagi account with an active session token.

## Install

### From a GitHub release

```bash
curl -fSL https://github.com/rubas/kagi/releases/download/v0.1.5/install.sh | sh -s v0.1.5
```

This installs:

- `kagi-search` and `kagi-summarize` to `~/.local/bin`
- skills to `~/.agents/skills/kagi-search`, `~/.agents/skills/kagi-summarize`
- skills to `~/.claude/skills/kagi-search`, `~/.claude/skills/kagi-summarize`

Supported platforms: Linux x86_64 and macOS aarch64.

### From source

```bash
cargo install --git https://github.com/rubas/kagi.git
```

## Authentication

You need a Kagi session token. The binaries check these sources in order:

1. `KAGI_SESSION_TOKEN` environment variable
2. `$XDG_CONFIG_HOME/kagi/session-token` file
3. Fail with an error if neither is set

The binaries never embed or store secrets.

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

## Skills

This repo ships companion skills for Claude Code and other agents:

- [`skills/search/SKILL.md`](skills/search/SKILL.md)
- [`skills/summarize/SKILL.md`](skills/summarize/SKILL.md)

## License

[MIT](LICENSE)
