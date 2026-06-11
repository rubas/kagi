#!/usr/bin/env sh
set -eu

repo="${KAGI_INSTALL_REPO:-rubas/kagi}"
version="${1:-${KAGI_INSTALL_VERSION:-}}"

if [ -z "$version" ]; then
  echo "usage: install.sh <version-tag>" >&2
  echo "example: install.sh v0.1.5" >&2
  exit 1
fi

# KAGI_INSTALL_BASE_URL lets the release workflow smoke-test this script
# against just-built local artifacts (file:// URL) before publishing.
base_url="${KAGI_INSTALL_BASE_URL:-https://github.com/${repo}/releases/download/${version}}"

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

tmp="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp"
}
trap cleanup EXIT INT TERM

curl -fSL "${base_url}/${archive}" -o "$tmp/$archive"
curl -fSL "${base_url}/kagi-skills.tar.gz" -o "$tmp/kagi-skills.tar.gz"

# Verify GitHub build provenance attestations before trusting the archives.
# KAGI_INSTALL_VERIFY: auto (default) verifies when gh is available and warns
# otherwise; require fails without verification; skip disables it.
verify="${KAGI_INSTALL_VERIFY:-auto}"
case "$verify" in
auto | require | skip) ;;
*)
  echo "invalid KAGI_INSTALL_VERIFY value: $verify (expected auto, require, or skip)" >&2
  exit 1
  ;;
esac

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
  "$tmp/kagi-skills/kagi-search/SKILL.md" \
  "$tmp/kagi-skills/kagi-maps/SKILL.md" \
  "$tmp/kagi-skills/kagi-summarize/SKILL.md"; do
  if [ ! -f "$required" ]; then
    echo "unexpected archive layout: missing ${required#"$tmp"/}" >&2
    exit 1
  fi
done

bin_dir="${HOME}/.local/bin"
agents_root="${HOME}/.agents/skills"
claude_root="${HOME}/.claude/skills"
gemini_root="${HOME}/.gemini/antigravity-cli/skills"

install -d "$bin_dir" "$agents_root" "$claude_root" "$gemini_root"

rm -rf \
  "${agents_root}/kagi-search" \
  "${agents_root}/kagi-maps" \
  "${agents_root}/kagi-summarize" \
  "${claude_root}/kagi-search" \
  "${claude_root}/kagi-maps" \
  "${claude_root}/kagi-summarize" \
  "${gemini_root}/kagi-search" \
  "${gemini_root}/kagi-maps" \
  "${gemini_root}/kagi-summarize"

install -d \
  "${agents_root}/kagi-search" \
  "${agents_root}/kagi-maps" \
  "${agents_root}/kagi-summarize" \
  "${claude_root}/kagi-search" \
  "${claude_root}/kagi-maps" \
  "${claude_root}/kagi-summarize" \
  "${gemini_root}/kagi-search" \
  "${gemini_root}/kagi-maps" \
  "${gemini_root}/kagi-summarize"

install -m 755 "$tmp/$root/bin/kagi-search" "${bin_dir}/kagi-search"
install -m 755 "$tmp/$root/bin/kagi-maps" "${bin_dir}/kagi-maps"
install -m 755 "$tmp/$root/bin/kagi-summarize" "${bin_dir}/kagi-summarize"

install -m 644 "$tmp/kagi-skills/kagi-search/SKILL.md" "${agents_root}/kagi-search/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-maps/SKILL.md" "${agents_root}/kagi-maps/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-summarize/SKILL.md" "${agents_root}/kagi-summarize/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-search/SKILL.md" "${claude_root}/kagi-search/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-maps/SKILL.md" "${claude_root}/kagi-maps/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-summarize/SKILL.md" "${claude_root}/kagi-summarize/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-search/SKILL.md" "${gemini_root}/kagi-search/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-maps/SKILL.md" "${gemini_root}/kagi-maps/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-summarize/SKILL.md" "${gemini_root}/kagi-summarize/SKILL.md"

echo "installed kagi-search, kagi-maps and kagi-summarize to ${bin_dir}"
echo "installed skills to ${agents_root}, ${claude_root} and ${gemini_root}"
