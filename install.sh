#!/usr/bin/env sh
set -eu

repo="${KAGI_INSTALL_REPO:-rubas/kagi}"
version="${1:-${KAGI_INSTALL_VERSION:-}}"

if [ -z "$version" ]; then
  echo "usage: install.sh <version-tag>" >&2
  echo "example: install.sh v0.1.3" >&2
  exit 1
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "gh is required to install from the private GitHub release" >&2
  exit 1
fi

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

gh release download "$version" --repo "$repo" --pattern "$archive" --pattern "kagi-skills.tar.gz" --dir "$tmp"

tar -xzf "$tmp/$archive" -C "$tmp"
tar -xzf "$tmp/kagi-skills.tar.gz" -C "$tmp"

bin_dir="${HOME}/.local/bin"
agents_root="${HOME}/.agents/skills"
claude_root="${HOME}/.claude/skills"

install -d "$bin_dir" "$agents_root" "$claude_root"

rm -rf \
  "${agents_root}/kagi-search" \
  "${agents_root}/kagi-summarize" \
  "${claude_root}/kagi-search" \
  "${claude_root}/kagi-summarize"

install -d \
  "${agents_root}/kagi-search" \
  "${agents_root}/kagi-summarize" \
  "${claude_root}/kagi-search" \
  "${claude_root}/kagi-summarize"

install -m 755 "$tmp/$root/bin/kagi-search" "${bin_dir}/kagi-search"
install -m 755 "$tmp/$root/bin/kagi-summarize" "${bin_dir}/kagi-summarize"

install -m 644 "$tmp/kagi-skills/kagi-search/SKILL.md" "${agents_root}/kagi-search/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-summarize/SKILL.md" "${agents_root}/kagi-summarize/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-search/SKILL.md" "${claude_root}/kagi-search/SKILL.md"
install -m 644 "$tmp/kagi-skills/kagi-summarize/SKILL.md" "${claude_root}/kagi-summarize/SKILL.md"

echo "installed kagi-search and kagi-summarize to ${bin_dir}"
echo "installed skills to ${agents_root} and ${claude_root}"
