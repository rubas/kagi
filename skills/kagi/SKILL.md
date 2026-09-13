---
name: kagi
description: "Use when you need ranked web hits with a region, lens, or site filter, a place or address, or a summary of one URL. For a synthesized answer use the search skill."
allowed-tools: Bash
argument-hint: "<query or url>"
---

# Kagi

Three binaries, one session token (`~/.config/kagi/session-token`;
`KAGI_SESSION_TOKEN` overrides it):

- `kagi-search <query>` finds web sources.
- `kagi-maps <query>` finds places, businesses, and addresses.
- `kagi-summarize <url>` summarizes one explicit URL server-side when the
  user requests a URL summary, including pages too large for WebFetch.

Output is plain text; `-j/--json` prints compact JSON. Queries are
positional and multiple words join automatically. Each binary's `--help`
lists every flag.

## Search

Search is iterative, not one-shot. For ranked-link requests, present the
hits directly. For a source-backed answer, read the source page itself; no
extra permission is needed.

- No hit: change one axis per retry. Synonyms, fewer terms, another lens, a
  wider time window. After two failed retries, report what you searched and
  what remains unknown. Do not infer absence from the whole web.
- Fresh topics get `--sort recency` with `--time day` or `week`. Evergreen
  topics get no time filter; recency buries the canonical page.
- Fetch the source page directly when a snippet is not enough to verify a
  claim. Use `kagi-summarize` only for a requested URL summary.
- Every fact you take from a result keeps its URL.

| User intent               | Options                                      |
| ------------------------- | -------------------------------------------- |
| Recent news or events     | `--sort recency --time day` or `--time week` |
| Programming or tech docs  | `--lens programming`                         |
| Community discussions     | `--lens forums`                              |
| Academic or research PDFs | `--lens pdfs`                                |
| One site only             | `--site example.com`                         |
| A date window             | `--from YYYY-MM-DD --to YYYY-MM-DD`          |
| Swiss or local results    | `--region ch`                                |
| Fewer, focused results    | `--limit 5`                                  |

```bash
kagi-search 'SBB Fahrplan' --region ch --sort recency --time week
kagi-search 'elixir genserver timeout' --lens programming --limit 5
kagi-search 'memory leak' --site github.com --filetype rs --json
```

## Maps

```bash
kagi-maps 'coffee zurich' --ll 47.3769,8.5417 --zoom 13
kagi-maps 'bookstore near bern' --sort rating --json
```

## Summarize

`--type summary` (default) for an overview, `--type takeaway` for bullet
points, `--lang DE` for another language. Use it only when the user asks for
a summary of a URL, or when that page is too large to read directly.

```bash
kagi-summarize 'https://example.com/article' --type takeaway
kagi-summarize 'https://example.com/article' --lang DE --json
```
