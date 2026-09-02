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
- `kagi-summarize <url>` summarizes one explicit URL server-side, so page
  size does not matter. It is the fallback when WebFetch hits its size limit.

Output is plain text; `-j/--json` prints compact JSON. Queries are
positional and multiple words join automatically. Each binary's `--help`
lists every flag.

## Search

Search is iterative, not one-shot. Present results directly; do not chain
search into summarize unless the user asks.

- No hit: change one axis per retry. Synonyms, fewer terms, another lens, a
  wider time window. After two failed retries, say the open web likely does
  not have it.
- Fresh topics get `--sort recency` with `--time day` or `week`. Evergreen
  topics get no time filter; recency buries the canonical page.
- A promising hit that needs more than its snippet goes to `kagi-summarize`,
  not to a raw fetch.
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
points, `--lang DE` for another language. Do not search for URLs to
summarize unless the user asks.

```bash
kagi-summarize 'https://example.com/article' --type takeaway
kagi-summarize 'https://example.com/article' --lang DE --json
```
