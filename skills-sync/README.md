# skills-sync

Sync Agent Skills from this library into Claude Code, Codex, Cursor, and Pi.

## Quick start

```bash
./scripts/bootstrap-skills-sync.sh
# or from a checkout:
cd skills-sync && cargo run -- sync
```

Config: `~/.config/skills-sync/` (`personal.toml`, `overrides.toml`)  
Cache: `~/.cache/skills-sync/` (library clone + version extracts)

## Commands

- `skills-sync init --skills jira,story-writing --harnesses claude-code,cursor`
- `skills-sync sync` — place Lock File / Override Versions
- `skills-sync adopt` — adopt Lock File (skips Overrides) then sync
- `skills-sync status` / `resolve` / `tui`
- `skills-sync override <skill> <version>` / `clear-override <skill>`
- `skills-sync catalog --out docs/catalog`
- `skills-release --parent <sha> --create` — CI Release tagging helper

## Development

```bash
cargo test
cargo llvm-cov --workspace --fail-under-lines 85
```

See `CONTEXT.md` and `docs/adr/` for domain decisions. Spec: `.scratch/skills-sync/description.md`.
