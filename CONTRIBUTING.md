# Contributing Skills

This repository is a **Sync-only** library of Agent Skills for Claude Code, Codex, Cursor, and Pi. Use the vocabulary in [CONTEXT.md](CONTEXT.md) (Skill, Version, Lock File, Adopt, Override, Release, Migration Docs). Product decisions live in [docs/adr/](docs/adr/).

End users install via [README.md](README.md) / [skills-sync/README.md](skills-sync/README.md). This guide is for **authors** adding or changing Skills.

## Skill layout

Each Skill is one folder under `skills/`:

```
skills/<name>/
├── SKILL.md                 # required — frontmatter + body
├── references/              # optional — deep docs loaded on demand
├── scripts/                 # optional — helper scripts
├── assets/                  # optional — templates, images
├── bin/                     # optional — executables; Sync links into ~/.local/bin
├── agents/openai.yaml       # optional — Codex sidecar
└── MIGRATION.md             # optional — required for breaking majors (or use ## Migration in SKILL.md)
```

Rules:

- Folder name **must** match frontmatter `name:`.
- On `main`, only the **latest** tree for that Skill lives under `skills/<name>/` (no parallel version folders).
- Historical Versions are immutable git tags `name@semver`, not folders.

## Frontmatter

Required:

| Field | Meaning |
|-------|---------|
| `name` | Skill id; matches directory name |
| `description` | Agent-facing trigger text (when to use this Skill) |
| `version` | Semver string (e.g. `1.2.0`) |

Optional:

| Field | Meaning |
|-------|---------|
| `dependencies` | List of other Skill names; Sync **warns** if missing, does not auto-install |
| `deprecated` | `true` when the Skill should not be used for new work |
| `successor` | Recommended Skill name when deprecated |

Keep the body **portable** Agent Skills markdown. Tool-specific extras belong in sidecars (e.g. `agents/openai.yaml`), not forked bodies.

Example:

```yaml
---
name: my-skill
description: >
  One or two sentences that tell an agent when to invoke this Skill.
  Include concrete triggers and what outcome it produces.
version: 1.0.0
dependencies:
  - jira
---
```

## Authoring guidance

1. **Write the description for agents**, not marketers. Include verbs and contexts (“use when…”, “pairs with…”).
2. **Keep `SKILL.md` the procedure.** Push long examples into `references/` and link them.
3. **If you ship CLIs**, put them in `bin/` and document that `skills-sync sync` links them onto `~/.local/bin`.
4. **Name consistently** with existing Skills (`jira`, `confluence`, `story-writing`, `story-review`).
5. **Test with a real harness** after Sync (Claude `/skill-name`, Codex `$skill-name`, etc.).

## Versioning (semver)

Bump `version:` in `SKILL.md` whenever you change Skill content that consumers should pin:

| Bump | When |
|------|------|
| **patch** (`1.0.1`) | Fixes, typos, clarifications that do not change procedure |
| **minor** (`1.1.0`) | Additive guidance, new optional sections, new references |
| **major** (`2.0.0`) | Breaking changes to procedure, required fields, or expected outputs |

**Majors require Migration Docs** — either a `## Migration` section in `SKILL.md` or a `MIGRATION.md` file in the Skill folder. Adopt surfaces a summary when a user crosses a major Version.

Do **not** create `name@semver` tags by hand in normal flow. CI runs `skills-release` on push to `main` when `version:` changes and creates the tag.

## Release ≠ Adopt

```
version bump on main  →  CI tag name@semver   (Released to git)
skills.lock pin bump  →  skills-sync adopt    (Adopted by the team)
```

- **Released** means the Version exists as a tag and can be installed.
- **Adopted** means [`skills.lock`](skills.lock) pins the team to that Version.
- Lock may **lag** behind latest tags on purpose (rollout control).
- Personal **Overrides** (`~/.config/skills-sync/overrides.toml`) let one developer test a newer Version; Adopt skips overridden Skills until cleared.

Every Skill in the library must appear in `skills.lock` (complete pin table, not a partial catalog).

## PR checklist

When changing an existing Skill:

- [ ] Bump `version:` appropriately (patch / minor / major).
- [ ] For majors: add Migration Docs (`## Migration` or `MIGRATION.md`).
- [ ] Update [`skills.lock`](skills.lock) to the new Version **or** deliberately leave it lagging (call that out in the PR).
- [ ] Regenerate the catalog:  
  `cd skills-sync && cargo run -- catalog --out ../docs/catalog --library ..`
- [ ] Run `skills-sync validate-skills --library .` (frontmatter, name/folder, semver, deps, Migration Docs, lock completeness).
- [ ] Run `skills-sync validate-lock --library .` from the repo root (or via a built binary). Pending tags are OK if HEAD frontmatter matches the lock pin.
- [ ] Smoke-test: `skills-sync sync` (or `adopt`) in a disposable `HOME` / XDG fixture if you want isolation.
- [ ] PR description mentions Version bump and whether the Lock File moved.

When adding a **new** Skill: same list, plus add a new lock pin and ensure the Skill is discoverable from the catalog.

## Validation

`skills-sync validate-skills --library .` hard-fails when any Skill folder violates:

1. Required frontmatter: `name`, `description`, `version`
2. `name` equals the folder name under `skills/`
3. `version` is valid semver
4. `description` is non-empty (after trim)
5. Major ≥ 2 ⇒ Migration Docs (`## Migration` in `SKILL.md` or `MIGRATION.md`)
6. Each `dependencies` entry names an existing `skills/<dep>/` directory
7. Every Skill folder is pinned in `skills.lock`, and every lock pin names an existing Skill with parseable `SKILL.md`
8. No orphan directories under `skills/` (must contain `SKILL.md`)

## New Skill walkthrough

1. Create `skills/<name>/SKILL.md` with frontmatter (`name`, `description`, `version: 1.0.0`) and body.
2. Add optional `references/`, `bin/`, etc. as needed.
3. Add `<name> = "1.0.0"` under `[skills]` in [`skills.lock`](skills.lock).
4. Regenerate `docs/catalog/`.
5. Open a PR. After merge to `main`, CI tags `name@1.0.0`.
6. Teammates pull/refresh the library cache and run `skills-sync adopt` (or `sync`) to install.

Minimal scaffold:

```bash
mkdir -p skills/my-skill
cat > skills/my-skill/SKILL.md <<'EOF'
---
name: my-skill
description: >
  Describe when an agent should use this Skill and what it accomplishes.
version: 1.0.0
---

# My Skill

## When to use this

- …

## Steps

1. …
EOF
```

Then edit `skills.lock`, regen catalog, and open the PR.

## Deprecating a Skill

1. Set `deprecated: true` and usually `successor: other-skill` in frontmatter.
2. Bump Version (typically minor or major depending on urgency).
3. Catalog and status/TUI will warn.
4. When the team is ready, remove the pin from `skills.lock` in a normal PR.
5. **Keep git tags forever** so old pins and history still resolve.

## Local tooling

```bash
# From a checkout
./scripts/bootstrap-skills-sync.sh

skills-sync status
skills-sync sync
skills-sync adopt
skills-sync list
skills-sync validate-skills --library .
skills-sync validate-lock --library .
skills-sync catalog --out docs/catalog --library .
skills-sync tui
```

Colors/emoji honor TTY; disable with `--no-color`, `NO_COLOR`, or `SKILLS_SYNC_NO_COLOR=1`.

Release tagging helper (CI uses this on `main`):

```bash
cargo run -p skills-sync --bin skills-release -- --repo . --parent <sha> --head HEAD
# add --create to write tags locally (normally CI’s job)
```

## Related docs

| Doc | Role |
|-----|------|
| [CONTEXT.md](CONTEXT.md) | Domain language |
| [docs/adr/](docs/adr/) | Architecture decisions |
| [docs/catalog/index.md](docs/catalog/index.md) | Generated Skill catalog |
| [skills-sync/README.md](skills-sync/README.md) | CLI placement map and commands |
| [README.md](README.md) | Install / Sync for consumers |
