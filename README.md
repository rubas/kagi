# kagi

Unofficial Unix-style command-line tools for [Kagi Search](https://kagi.com). Kagi Inc. does not
make or endorse them.

Each binary does one job:

- `kagi-search` finds sources
- `kagi-maps` finds places and addresses
- `kagi-summarize` summarizes one explicit URL

Output is plain text by default and compact JSON with `--json`, for scripts and agents. You need a
Kagi account and its session token.

## Install

Releases ship Linux x86_64 and macOS aarch64 builds. Install them with [mise](https://mise.jdx.dev),
which verifies the GitHub attestation of the release archive:

```bash
mise use -g github:rubas/kagi
```

Without mise, build from source:

```bash
cargo install --git https://github.com/rubas/kagi
```

## Authentication

You need a Kagi session token. The binaries check these sources in order:

1. The `KAGI_SESSION_TOKEN` environment variable, when it is not empty.
2. `$XDG_CONFIG_HOME/kagi/session-token`, or `~/.config/kagi/session-token` when `XDG_CONFIG_HOME`
   is unset, empty, or relative.

Without either, they stop with an error.

Create the token file with owner-only permissions (paste the token, then
Ctrl-D):

```bash
install -d -m 700 ~/.config/kagi
(umask 077; cat > ~/.config/kagi/session-token)
```

The token is a full kagi.com session cookie, so the binaries refuse a token file that is group- or
world-readable (`chmod 600` fixes it). This check does not apply to `KAGI_SESSION_TOKEN`. The
binaries never embed or store secrets.

## Binaries

Each binary lists all its flags with `--help`.

### `kagi-search`

Searches the web. The text output is numbered results and, when Kagi has them, `Related:` terms.
The JSON is `{ "results": [...], "related": [...] }`.

```bash
kagi-search 'rust async runtime' --lens programming --limit 5
kagi-search 'SBB Fahrplan' --region ch --sort recency --time week
kagi-search 'memory leak' --site github.com --filetype rs --json
```

### `kagi-maps`

Searches Kagi Maps for places, businesses, and addresses. The text output is numbered places with
the address, coordinates, rating, phone, and URL that Kagi has. The JSON is `{ "results": [...] }`.

```bash
kagi-maps 'coffee zurich' --ll 47.3769,8.5417 --zoom 13
kagi-maps 'bookstore near bern' --sort rating --json
```

### `kagi-summarize`

Summarizes one explicit URL. The text output is the raw Markdown summary. The JSON is
`{ "summary": "..." }`.

```bash
kagi-summarize 'https://www.rust-lang.org/learn'
kagi-summarize 'https://www.rust-lang.org/learn' --type takeaway
kagi-summarize 'https://www.rust-lang.org/learn' --lang DE --json
```

## Development

```bash
task check
```

`task lint` also runs [zizmor](https://github.com/zizmorcore/zizmor) over the workflows, so install
`zizmor` first.

## License

[MIT](LICENSE)
