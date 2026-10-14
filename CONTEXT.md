# Cross-Harness Skills Repository

Canonical language for the skill-library: a shared library of Agent Skills placed into Claude Code, Codex, Cursor, and Pi via Sync.

## Language

**Skill**:
A reusable, independently versioned procedure at `skills/<name>/` with `SKILL.md` (`name`, `description`, `version`, optional `dependencies`, optional deprecation metadata) plus optional `scripts/`, `references/`, `assets/`, `bin/`. On `main`, only the latest tree for that Skill lives there.
_Avoid_: plugin, rule, prompt; parallel version folders; Claude Marketplace (out of scope)

**Harness**:
An agent tool that discovers and runs Skills. Supported: Claude Code, Codex, Cursor, Pi.
_Avoid_: agent, IDE, tool (too vague)

**Version**:
Semver in `SKILL.md` frontmatter, immutably referenced by git tag `name@semver`. Sync path-extracts `skills/<name>/` at that tag into a local cache, then places into the canonical store and Harness dirs.
_Avoid_: file-level git tags; repo-wide Version tags for one Skill

**Release**:
CI creates tag `name@semver` on merge to `main` when that Skill's `version:` bumped. Untagged work lives on branches only. **Released to git** ≠ **Adopted by the team** (Lock File may lag). Separate `skills-sync-v*` tags publish bootstrap binaries.
_Avoid_: humans tagging by hand as the normal path; “unreleased on main” (not a thing under CI tagging)

**Lock File** (`skills.lock`):
PR-gated pin of **every** Skill → Version. Binds team Versions. Personal config turns install on/off per Skill/Harness; optional loud Version Overrides for testing. CI validates each pin against an existing tag or matching HEAD frontmatter (`skills-sync validate-lock`).
_Avoid_: advisory-only lock; lock as a partial catalog

**Adopt**:
Explicit skills-sync action to align installed Versions with the Lock File. Skips overridden Skills and prints them; clear overrides separately. On a major Version bump, surfaces Migration Docs before Sync replaces the placement.
_Avoid_: silent auto-upgrade

**Override**:
Loud personal Version pin in `~/.config/skills-sync/overrides.toml`. Visible every TUI launch; Adopt does not overwrite it.
_Avoid_: silent local pins

**Placement Scope**:
Exactly one of user or project for a Sync run (global in `personal.toml`). Claude default: user. Canonical store is `~/.agents/skills` (user) or `./.agents/skills` (project).
_Avoid_: writing both scopes in one Sync; documenting per-Harness scopes that the config does not support

**Sync**:
Place from version cache into a canonical `.agents/skills/<name>/` store (symlink by default, copy fallback when symlinks fail), then link Claude Code / Cursor / Pi discovery dirs to that store. Codex discovers the canonical path directly. Skill `bin/` scripts are linked into `~/.local/bin`. Sole distribution path for all four Harnesses.
_Avoid_: Marketplace install; format transformation of Skill bodies; writing only into a single Harness dir without the canonical store

**Dependency**:
Optional Skill names in frontmatter. Sync warns if missing; does not auto-install.
_Avoid_: hard auto-install of dependents

**Deprecation**:
Frontmatter mark (optional `successor:`); Catalog/TUI warn. Lock removal is a normal PR when the team is ready (no enforced clock). Tags kept forever.
_Avoid_: deleting Skill history

**Catalog**:
TUI/CLI + README + Lock File + static site generated from Skills on `main` (includes per-Skill install snippets).
_Avoid_: hosted CMS in v1

**skills-sync**:
Custom Rust CLI + full TUI. Bootstrap script downloads a Release binary (or builds from checkout), clones this repo into the cache, writes default personal config if needed, then runs `sync` (TUI optional via `--tui` / `SKILLS_SYNC_TUI=1`). Also: `list`, `remove`, `validate-lock`, `adopt`, `catalog`. Config: `~/.config/skills-sync/`. Cache: `~/.cache/skills-sync/` (clone + extracted `name@semver` trees).
_Avoid_: requiring `cargo install` for normal onboarding; launching TUI as the only onboarding path

**Migration Docs**:
Required for breaking major Versions. Lives in the Skill folder (`## Migration` in `SKILL.md` or `MIGRATION.md`). Adopt prints a summary when crossing a major Version.
_Avoid_: changelog-only notes with no adaptation guidance
