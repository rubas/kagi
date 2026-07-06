---
name: kagi-search
description: |
  Covers: Web search via the kagi-search CLI (human-readable or JSON output).
  Consult when: You need to search the web for current information, look something up, find documentation,
  check recent news or events, research a topic, or get search results for any query.
  Use this skill whenever the user asks you to "search for", "look up", "find out",
  "what's the latest on", "google", or wants web results - even if they don't explicitly say "search."
  Not covered: Summarizing a URL (use the kagi-summarize skill). Fetching raw HTML (use WebFetch).
allowed-tools: Bash
argument-hint: "<query>"
---

# Search

Search the web using `kagi-search`. Outputs numbered results in plain text by default, or compact JSON with `-j`/`--json`.

Present results directly to the user. Do not chain search into summarize unless the user explicitly asks.

## Usage

```bash
kagi-search [OPTIONS] <QUERY>...
```

The query is positional - multiple words are joined automatically.

## Options

| Flag              | Description                                                                | Default |
| ----------------- | -------------------------------------------------------------------------- | ------- |
| `--limit N`       | Maximum results                                                            | 10      |
| `--region REGION` | Country code (`ch`, `us`, `de`, ...) or `no_region`                        | -       |
| `--lens LENS`     | `default`, `programming`, `forums`, `pdfs`, `non-commercial`, `world-news` | -       |
| `--sort SORT`     | `recency`, `website`, `ad-trackers`                                        | -       |
| `--time TIME`     | `day`, `week`, `month`, `year` (conflicts with `--from`/`--to`)            | -       |
| `--from DATE`     | Start date `YYYY-MM-DD` (conflicts with `--time`)                          | -       |
| `--to DATE`       | End date `YYYY-MM-DD` (conflicts with `--time`)                            | -       |
| `--site DOMAIN`   | Restrict to a domain                                                       | -       |
| `--filetype EXT`  | Restrict to a file extension                                               | -       |
| `--verbatim`      | Disable query expansion                                                    | off     |
| `-j, --json`      | JSON output                                                                | text    |

## Option Selection Guide

| User intent                  | Options                                      |
| ---------------------------- | -------------------------------------------- |
| Recent news/events           | `--sort recency --time day` or `--time week` |
| Programming/tech docs        | `--lens programming`                         |
| Community discussions        | `--lens forums`                              |
| Academic/research PDFs       | `--lens pdfs`                                |
| Ad-free/independent sources  | `--lens non-commercial`                      |
| Results from a specific site | `--site example.com`                         |
| Results within a date window | `--from YYYY-MM-DD --to YYYY-MM-DD`          |
| Swiss/local results          | `--region ch`                                |
| Fewer, focused results       | `--limit 3` or `--limit 5`                   |
| Machine-readable output      | `--json` (for further processing)            |

## Examples

```bash
# Swiss news from the past week
kagi-search 'SBB Fahrplan' --region ch --sort recency --time week

# Programming docs, limited results
kagi-search 'elixir genserver timeout' --lens programming --limit 5

# GitHub issues about a specific bug
kagi-search 'memory leak production' --site github.com --sort recency

# PDF whitepapers
kagi-search 'transformer architecture attention' --lens pdfs --limit 5

# Rust files mentioning a pattern
kagi-search 'memory leak' --site github.com --filetype rs --json

# Date range search
kagi-search 'security vulnerability' --from 2026-01-01 --to 2026-03-01
```

## Output

### Text (default)

```
1. Steve Jobs - Wikipedia
   https://en.wikipedia.org/wiki/Steve_Jobs
   Steven Paul Jobs was an American businessman...

Related: steve jobs death, steve jobs quotes
```

### JSON (`--json`)

```json
{ "results": [{ "url": "...", "title": "...", "snippet": "..." }], "related": ["term1"] }
```
