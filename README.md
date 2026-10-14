# skill-library

Cross-harness Agent Skills for Claude Code, Codex, Cursor, and Pi.

## Catalog

See [docs/catalog/index.md](docs/catalog/index.md) for Skills and Versions on `main`.  
Team pins: [`skills.lock`](skills.lock).

## Install / Sync

From a checkout:

```bash
./scripts/bootstrap-skills-sync.sh
```

That installs `skills-sync` (Release binary or local `cargo build`), clones/updates the library cache, writes a default `personal.toml` if missing, and runs `skills-sync sync`.

Or after the binary is on your `PATH`:

```bash
skills-sync sync
```

Day-two commands:

```bash
skills-sync status          # Lock vs effective Versions, Overrides, deprecation
skills-sync list            # Installed Skills and Harness paths
skills-sync adopt           # Align to Lock File (skips Overrides; surfaces Migrations)
skills-sync remove jira     # Unlink placements, bins, and personal selection
skills-sync validate-lock   # CI-friendly Lock File ↔ tag check
skills-sync tui             # Interactive UI
```

Optional TUI on bootstrap: `SKILLS_SYNC_TUI=1 ./scripts/bootstrap-skills-sync.sh` or `./scripts/bootstrap-skills-sync.sh --tui`.

Env overrides: `SKILL_LIBRARY_REPO`, `SKILLS_SYNC_RELEASE_BASE` (defaults to this repo’s GitHub Releases when `origin` is a GitHub remote).

Details: [skills-sync/README.md](skills-sync/README.md). Domain language: [CONTEXT.md](CONTEXT.md). Decisions: [docs/adr/](docs/adr/).
