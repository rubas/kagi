## Install

Installs binaries to `~/.local/bin` and skills to both `~/.agents/skills` and `~/.claude/skills`.

```bash
tmp="$(mktemp -d)" && gh release download "__VERSION__" --repo "rubas/kagi" --pattern 'install.sh' --dir "$tmp" && sh "$tmp/install.sh" "__VERSION__" && rm -rf "$tmp"
```
