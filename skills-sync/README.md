# skills-sync

Sync Agent Skills from this library into Claude Code, Codex, Cursor, and Pi.

Demo (CLI + TUI): see the GIF in the [repo README](../README.md#demo). Regenerate with `../scripts/record-demo.sh`.

## Quick start

```bash
./scripts/bootstrap-skills-sync.sh
# or from a checkout:
cd skills-sync && cargo run -- sync
```

Bootstrap detects `SKILL_LIBRARY_REPO` (or `git remote origin`, or the local checkout), downloads `skills-sync-<os>-<arch>` from `SKILLS_SYNC_RELEASE_BASE` / GitHub Releases when possible, otherwise builds from source. It then clones the library into cache, seeds `personal.toml` if needed, and runs `sync`. Pass `--tui` or set `SKILLS_SYNC_TUI=1` to open the TUI afterward.

Config: `~/.config/skills-sync/` (`personal.toml`, `overrides.toml`)  
Cache: `~/.cache/skills-sync/` (library clone + version extracts)

## Placement

Sync writes a **canonical** skill tree under `.agents/skills/<name>/` (user: `~/.agents/skills`, project: `./.agents/skills`), then symlinks Harness discovery dirs to it:

| Harness | User scope | Project scope |
|---------|------------|---------------|
| Canonical / Codex | `~/.agents/skills` | `./.agents/skills` |
| Claude Code | `~/.claude/skills` → canonical | `./.claude/skills` → canonical |
| Cursor | `~/.cursor/skills` → canonical | `./.cursor/skills` → canonical |
| Pi | `~/.pi/agent/skills` → canonical | `./.pi/skills` → canonical |

`placement_scope` in `personal.toml` is one of `user` or `project` for the whole Sync run (not per Harness). Skill `bin/` scripts are linked into `~/.local/bin` so they resolve on `$PATH`.

## Commands

- `skills-sync init --skills jira,story-writing --harnesses claude-code,cursor`
- `skills-sync sync` — place Lock File / Override Versions into canonical + Harness links
- `skills-sync adopt` — adopt Lock File (skips Overrides) then sync; surfaces Migration Docs on major bumps
- `skills-sync status` / `resolve` / `list` / `tui`
- `skills-sync remove <skill>` — unlink placements, bins, and personal selection
- `skills-sync override <skill> <version>` / `clear-override <skill>`
- `skills-sync validate-lock [--library <path>]` — Lock File pins vs tags (or HEAD frontmatter)
- `skills-sync catalog --out docs/catalog`
- `skills-release --parent <sha> --create` — CI Release tagging helper

## CI

- `.github/workflows/skills-sync.yml` — tests (≥85% line coverage), `validate-lock`, Release tags on `main`
- `.github/workflows/skills-sync-binaries.yml` — publish `skills-sync-<os>-<arch>` on `skills-sync-v*` tags

## Development

```bash
cargo test
cargo llvm-cov --workspace --fail-under-lines 85
```

See `CONTEXT.md` and `docs/adr/` for domain decisions. Spec: `.scratch/skills-sync/description.md`.
