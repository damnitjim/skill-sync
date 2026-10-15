# skill-library

Cross-harness Agent Skills for Claude Code, Codex, Cursor, and Pi.

## Demo

![skills-sync CLI and TUI](docs/demo/skills-sync.gif)

Regenerate with `./scripts/record-demo.sh` (requires [VHS](https://github.com/charmbracelet/vhs)).

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
skills-sync validate-skills # CI-friendly Skill frontmatter / lock completeness
skills-sync tui             # Interactive UI
```

Optional TUI on bootstrap: `SKILLS_SYNC_TUI=1 ./scripts/bootstrap-skills-sync.sh` or `./scripts/bootstrap-skills-sync.sh --tui`.

Env overrides: `SKILL_LIBRARY_REPO`, `SKILLS_SYNC_RELEASE_BASE` (defaults to this repo’s GitHub Releases when `origin` is a GitHub remote).

Details: [skills-sync/README.md](skills-sync/README.md). Domain language: [CONTEXT.md](CONTEXT.md). Decisions: [docs/adr/](docs/adr/).

## How it works

```
git tag name@semver  →  skills.lock (team pin)  →  personal.toml (what you install)
                              ↑                         ↓
                         Override can win          skills-sync sync
```

### Versions and tags

Each Skill carries semver in its `SKILL.md` frontmatter. On merge to `main`, CI creates an immutable git tag `name@semver` for every Skill whose `version:` changed in that commit (multiple Skills → multiple tags).

`skills-sync sync` installs from those tags (via a cached clone + path extract), not from whatever happens to sit on `main` in your working tree.

Authors: see [CONTRIBUTING.md](CONTRIBUTING.md) and [ADR-0002](docs/adr/0002-tag-backed-per-skill-versions.md).

### Lock File (`skills.lock`)

[`skills.lock`](skills.lock) is the team pin of **every** Skill → Version. Drift from the lock is not OK for normal installs.

**Released ≠ Adopted:** a tag can exist on git before the lock moves. Lock lag is intentional so the team can roll out Versions deliberately. A Release is the tag; Adopt is updating (or syncing to) the lock pin.

### personal.toml

Your machine chooses *what* to install. Config lives at `~/.config/skills-sync/personal.toml`. Bootstrap writes a default if missing.

```toml
skills = ["confluence", "jira", "story-review", "story-writing"]
harnesses = ["claude-code", "cursor"]
placement_scope = "user"   # or "project" — one scope for the whole Sync
```

| Field | Meaning |
|-------|---------|
| `skills` | Skills Sync will place (each must appear in the Lock File; Overrides can pin a different Version) |
| `harnesses` | Which tools get discovery links: `claude-code`, `codex`, `cursor`, `pi` |
| `placement_scope` | `user` (`~/…`) or `project` (`./…`) for **all** enabled harnesses in that Sync |

Edit with `skills-sync init`, by hand, or via `skills-sync remove <skill>` (drops a Skill from selection and unlinks placements). Placement paths: [skills-sync/README.md](skills-sync/README.md#placement).

### Sync vs Adopt

| Command | Behavior |
|---------|----------|
| `skills-sync sync` | Resolve effective Version (Override if set, else lock) → extract tag → place canonical `.agents/skills` + harness links + `bin/` onto `~/.local/bin` |
| `skills-sync adopt` | Align to the Lock File for Skills **without** Overrides, then sync; print skipped Overrides; surface Migration Docs on major bumps |
| `skills-sync status` / `resolve` | Show lock Version vs effective Version |

Sync does **not** silently upgrade you past an Override. Adopt does **not** clear Overrides — run `skills-sync clear-override <skill>` first if you want the lock Version.

### Overrides

Loud personal Version pins for testing, stored in `~/.config/skills-sync/overrides.toml`:

```toml
[overrides]
jira = "2.0.0-rc.1"
```

Or via CLI: `skills-sync override <skill> <version>` / `skills-sync clear-override <skill>`.

Effective Version for Sync = Override if present, else Lock File. Overrides show in `skills-sync status` and the TUI every launch.

### Day-two flow

1. Edit `personal.toml` for the Skills and harnesses you want.
2. Run `skills-sync sync` to install/update placements.
3. When the team bumps [`skills.lock`](skills.lock), run `skills-sync adopt` (clear Overrides first if you want the new lock Versions).

## Contributing

Adding or versioning Skills: [CONTRIBUTING.md](CONTRIBUTING.md).
