---
name: summarize
description: |
  Covers: URL summarization via the kagi-summarize CLI (human-readable or JSON output).
  Consult when: The user provides a specific URL and wants a summary or key takeaways from it.
  Also use as the fallback when a page is too large to fetch directly (WebFetch
  `maxContentLength` errors, multi-MB pages) — summarization runs server-side, so page
  size doesn't matter.
  Use this skill whenever the user asks you to "summarize this page", "what does this article say",
  "give me the key points from this URL", "TLDR this link", or provides a URL they want distilled -
  even if they don't explicitly say "summarize."
  Not covered: Web search (use the search skill). Fetching raw HTML (use WebFetch; if
  WebFetch fails on size, come back here).
allowed-tools: Bash
argument-hint: "<url>"
---

# Summarize

Summarize a single URL using `kagi-summarize`. Outputs a markdown summary in plain text by default, or compact JSON with `-j`/`--json`.

Use it for a URL the user provided or one already in play — including as the fallback when a direct fetch fails because the page is too large. Do not search for URLs to summarize unless explicitly asked.

## Usage

```bash
kagi-summarize [OPTIONS] <URL>
```

## Options

| Flag                    | Description             | Default                  |
| ----------------------- | ----------------------- | ------------------------ |
| `--type TYPE`           | `summary` or `takeaway` | `summary`                |
| `--lang LANG`           | Target language code    | `EN`                     |
| `-j, --json`            | JSON output             | text                     |
| `--session-token TOKEN` | Explicit token override | env `KAGI_SESSION_TOKEN` |

## When to Use Each Type

| User intent                 | Option                         |
| --------------------------- | ------------------------------ |
| General overview of a page  | `--type summary` (default)     |
| Bullet-point key takeaways  | `--type takeaway`              |
| Summary in another language | `--lang DE`, `--lang FR`, etc. |

## Examples

```bash
# Default summary
kagi-summarize 'https://example.com/article'

# Key takeaways
kagi-summarize 'https://example.com/article' --type takeaway

# Summary in German, JSON output
kagi-summarize 'https://example.com/article' --lang DE --json
```

## Output

### Text (default)

Raw markdown summary printed directly to stdout.

### JSON (`--json`)

```json
{ "summary": "markdown content here" }
```
