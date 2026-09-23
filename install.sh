#!/usr/bin/env sh
# Install or update kagi-search, kagi-maps, kagi-summarize, and the kagi skill.
#
# usage: install.sh [--check] [--force] [<version-tag>]
#
# Without a tag it installs the latest release. When the installed
# kagi-search --version already matches, it says so and changes nothing;
# --force installs anyway. --check installs nothing: it exits 0 when the
# install is current and 100 when an update is available.
set -eu

usage="usage: install.sh [--check] [--force] [<version-tag>]"
repo="${KAGI_INSTALL_REPO:-rubas/kagi}"
version="${KAGI_INSTALL_VERSION:-}"
verify="${KAGI_INSTALL_VERIFY:-auto}"
check=false
force=false

for arg; do
  case "$arg" in
  --check) check=true ;;
  --force) force=true ;;
  -*)
    echo "$usage" >&2
    exit 1
    ;;
  *) version="$arg" ;;
  esac
done

# KAGI_INSTALL_VERIFY: auto (default) verifies the GitHub build provenance
# attestations when gh is available and warns otherwise; require fails without
# verification; skip disables it.
case "$verify" in
auto | require | skip) ;;
*)
  echo "invalid KAGI_INSTALL_VERIFY value: $verify (expected auto, require, or skip)" >&2
  exit 1
  ;;
esac

os="$(uname -s)"
arch="$(uname -m)"

case "$os/$arch" in
Linux/x86_64)
  archive="kagi-linux-x86_64.tar.gz"
  root="kagi-linux-x86_64"
  ;;
Darwin/arm64)
  archive="kagi-macos-aarch64.tar.gz"
  root="kagi-macos-aarch64"
  ;;
*)
  echo "unsupported platform: $os/$arch" >&2
  exit 1
  ;;
esac

if [ -z "$version" ]; then
  if [ -n "${KAGI_INSTALL_BASE_URL:-}" ]; then
    echo "KAGI_INSTALL_BASE_URL needs a version tag" >&2
    exit 1
  fi
  # The web redirect of /releases/latest names the tag; unlike the API it needs
  # no token and has no rate limit.
  url="$(curl -fsSIL -o /dev/null -w '%{url_effective}' "https://github.com/${repo}/releases/latest")" || exit 1
  case "$url" in
  */releases/tag/*) version="${url##*/}" ;;
  *)
    echo "no release found at https://github.com/${repo}/releases/latest" >&2
    exit 1
    ;;
  esac
fi

bin_dir="${HOME}/.local/bin"
target="${version#v}"
installed="$("${bin_dir}/kagi-search" --version 2>/dev/null)" || installed=""
installed="${installed#kagi-search }"

if [ "$installed" = "$target" ]; then
  echo "kagi ${target} is current"
  if $check || ! $force; then exit 0; fi
else
  echo "kagi: ${installed:-not installed} -> ${target}"
  if $check; then exit 100; fi
fi

# KAGI_INSTALL_BASE_URL lets the release workflow smoke-test this script
# against just-built local artifacts (file:// URL) before publishing.
base_url="${KAGI_INSTALL_BASE_URL:-https://github.com/${repo}/releases/download/${version}}"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
trap 'exit 1' INT TERM

curl -fsSL "${base_url}/${archive}" -o "$tmp/$archive"
curl -fsSL "${base_url}/kagi-skills.tar.gz" -o "$tmp/kagi-skills.tar.gz"

if [ "$verify" != "skip" ]; then
  if command -v gh >/dev/null 2>&1; then
    gh attestation verify "$tmp/$archive" --repo "$repo"
    gh attestation verify "$tmp/kagi-skills.tar.gz" --repo "$repo"
  elif [ "$verify" = "require" ]; then
    echo "KAGI_INSTALL_VERIFY=require but the gh CLI is not available to verify attestations" >&2
    exit 1
  else
    echo "warning: gh CLI not found; skipping release attestation verification" >&2
    echo "warning: install the GitHub CLI or set KAGI_INSTALL_VERIFY=require to fail instead" >&2
  fi
fi

tar -xzf "$tmp/$archive" -C "$tmp"
tar -xzf "$tmp/kagi-skills.tar.gz" -C "$tmp"

# Fail before touching any installed files if either archive's layout drifted.
for required in \
  "$tmp/$root/bin/kagi-search" \
  "$tmp/$root/bin/kagi-maps" \
  "$tmp/$root/bin/kagi-summarize" \
  "$tmp/kagi-skills/kagi/SKILL.md"; do
  if [ ! -f "$required" ]; then
    echo "unexpected archive layout: missing ${required#"$tmp"/}" >&2
    exit 1
  fi
done

agents_root="${HOME}/.agents/skills"
claude_root="${HOME}/.claude/skills"
gemini_root="${HOME}/.gemini/antigravity-cli/skills"

install -d "$bin_dir" "$agents_root" "$claude_root" "$gemini_root"

# The three per-binary skill dirs of v0.4 and earlier are legacy; remove them.
rm -rf \
  "${agents_root}/kagi" "${claude_root}/kagi" "${gemini_root}/kagi" \
  "${agents_root}/kagi-search" "${agents_root}/kagi-maps" "${agents_root}/kagi-summarize" \
  "${claude_root}/kagi-search" "${claude_root}/kagi-maps" "${claude_root}/kagi-summarize" \
  "${gemini_root}/kagi-search" "${gemini_root}/kagi-maps" "${gemini_root}/kagi-summarize"

install -d "${agents_root}/kagi" "${claude_root}/kagi" "${gemini_root}/kagi"

install -m 755 "$tmp/$root/bin/kagi-search" "${bin_dir}/kagi-search"
install -m 755 "$tmp/$root/bin/kagi-maps" "${bin_dir}/kagi-maps"
install -m 755 "$tmp/$root/bin/kagi-summarize" "${bin_dir}/kagi-summarize"

install -m 644 "$tmp/kagi-skills/kagi/SKILL.md" "${agents_root}/kagi/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi/SKILL.md" "${claude_root}/kagi/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi/SKILL.md" "${gemini_root}/kagi/SKILL.md"

echo "installed kagi-search, kagi-maps and kagi-summarize to ${bin_dir}"
echo "installed skills to ${agents_root}, ${claude_root} and ${gemini_root}"
