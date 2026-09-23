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
Linux/x86_64) root="kagi-linux-x86_64" ;;
Darwin/arm64) root="kagi-macos-aarch64" ;;
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

# KAGI_INSTALL_BASE_URL points at local archives (file:// URL): the release
# smoke test, task install, and a host that verified the archives itself.
base_url="${KAGI_INSTALL_BASE_URL:-https://github.com/${repo}/releases/download/${version}}"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
trap 'exit 1' INT TERM

curl -fsSL "${base_url}/${root}.tar.gz" -o "$tmp/${root}.tar.gz"

if [ "$verify" != "skip" ]; then
  if command -v gh >/dev/null 2>&1; then
    gh attestation verify "$tmp/${root}.tar.gz" --repo "$repo"
  elif [ "$verify" = "require" ]; then
    echo "KAGI_INSTALL_VERIFY=require but the gh CLI is not available to verify attestations" >&2
    exit 1
  else
    echo "warning: gh CLI not found; skipping release attestation verification" >&2
    echo "warning: install the GitHub CLI or set KAGI_INSTALL_VERIFY=require to fail instead" >&2
  fi
fi

tar -xzf "$tmp/${root}.tar.gz" -C "$tmp"

# Fail before touching any installed files if the archive layout drifted.
for file in bin/kagi-search bin/kagi-maps bin/kagi-summarize skills/kagi/SKILL.md; do
  if [ ! -f "$tmp/$root/$file" ]; then
    echo "unexpected archive layout: missing $root/$file" >&2
    exit 1
  fi
done

# The per-binary skill dirs of v0.4 and earlier are legacy; remove them.
for dir in .agents/skills .claude/skills .codex/skills .gemini/antigravity-cli/skills .pi/agent/skills; do
  rm -rf "$HOME/$dir/kagi-search" "$HOME/$dir/kagi-maps" "$HOME/$dir/kagi-summarize"
done

# The skill has one source, ~/.agents/skills/kagi.
skill="$HOME/.agents/skills/kagi"
rm -rf "$skill"
install -d "$skill"
install -m 644 "$tmp/$root/skills/kagi/SKILL.md" "$skill/SKILL.md"
skill="$(cd "$skill" && pwd -P)"

# Each agent whose config root exists gets a relative link to the skill. Like
# the realpath -m --relative-to of a dotfiles fan-out, the target runs between
# the physical paths, so it resolves through a symlinked config root and the
# installer and the fan-out never replace each other's link.
link() { # <agent config root>
  [ -d "$HOME/$1" ] || return 0
  install -d "$HOME/$1/skills"
  dir="$(cd "$HOME/$1/skills" && pwd -P)"
  # A skills dir that is the source dir already holds the skill.
  [ "$dir/kagi" != "$skill" ] || return 0
  base="$dir"
  up=""
  while :; do
    case "$skill" in "$base"/*) break ;; esac
    base="${base%/*}"
    up="../$up"
  done
  rm -rf "$dir/kagi"
  ln -s "$up${skill#"$base"/}" "$dir/kagi"
  echo "linked $HOME/$1/skills/kagi -> $up${skill#"$base"/}"
}
link .claude
link .codex
link .gemini/antigravity-cli
link .pi/agent

# kagi-search goes last: its --version marks the install as current, so an
# install that fails before it runs again in full.
install -d "$bin_dir"
for bin in kagi-maps kagi-summarize kagi-search; do
  install -m 755 "$tmp/$root/bin/$bin" "$bin_dir/$bin"
done
echo "installed kagi ${target}: kagi-search, kagi-maps, and kagi-summarize in ${bin_dir}, the skill in ${skill}"
