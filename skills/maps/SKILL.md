---
name: kagi-maps
description: |
  Covers: Place and address search via the kagi-maps CLI.
  Consult when: You need to find businesses, points of interest, addresses, or local map results.
  Not covered: General web search (use the kagi-search skill). Summarizing URLs (use the kagi-summarize skill).
allowed-tools: Bash
argument-hint: "<query>"
---

# Maps

Search Kagi Maps using `kagi-maps`. Outputs place results in plain text by default, or compact JSON with `-j`/`--json`.

## Usage

```bash
kagi-maps [OPTIONS] <QUERY>...
```

The query is positional - multiple words are joined automatically.

## Options

| Flag             | Description                                | Default               |
| ---------------- | ------------------------------------------ | --------------------- |
| `--limit N`      | Maximum results                            | 10                    |
| `--ll LAT,LON`   | Search origin coordinate                   | -                     |
| `--bbox W,S,E,N` | Bounding box as west, south, east, north   | -                     |
| `--zoom N`       | Map zoom level passed as `z`               | -                     |
| `--sort SORT`    | `relevance`, `rating`, `distance`, `price` | Kagi order            |
| `--order ORDER`  | `asc`, `desc` (requires `--sort`)          | sort-specific default |
| `--output MODE`  | `text`, `json`                             | text                  |
| `-j, --json`     | JSON output                                | text                  |

## Examples

```bash
kagi-maps 'coffee zurich' --ll 47.3769,8.5417 --zoom 13
kagi-maps 'bookstore near bern' --sort rating --json
```
