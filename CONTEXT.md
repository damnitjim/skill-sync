# Cross-Harness Skills Repository

Canonical language for the skill-library: a shared library of Agent Skills placed into Claude Code, Codex, Cursor, and Pi via Sync.

## Language

**Skill**:
A reusable, independently versioned procedure at `skills/<name>/` with `SKILL.md` (`name`, `description`, `version`, optional `dependencies`, optional deprecation metadata) plus optional `scripts/`, `references/`, `assets/`. On `main`, only the latest tree for that Skill lives there.
_Avoid_: plugin, rule, prompt; parallel version folders; Claude Marketplace (out of scope)

**Harness**:
An agent tool that discovers and runs Skills. Supported: Claude Code, Codex, Cursor, Pi.
_Avoid_: agent, IDE, tool (too vague)

**Version**:
Semver in `SKILL.md` frontmatter, immutably referenced by git tag `name@semver`. Sync path-extracts `skills/<name>/` at that tag into a local cache, then places into Harness dirs.
_Avoid_: file-level git tags; repo-wide Version tags for one Skill

**Release**:
CI creates tag `name@semver` on merge to `main` when that Skill's `version:` bumped. Untagged work lives on branches only. **Released to git** ≠ **Adopted by the team** (Lock File may lag).
_Avoid_: humans tagging by hand as the normal path; “unreleased on main” (not a thing under CI tagging)

**Lock File** (`skills.lock`):
PR-gated pin of **every** Skill → Version. Binds team Versions. Personal config turns install on/off per Skill/Harness; optional loud Version Overrides for testing.
_Avoid_: advisory-only lock; lock as a partial catalog

**Adopt**:
Explicit skills-sync action to align installed Versions with the Lock File. Skips overridden Skills and prints them; clear overrides separately.
_Avoid_: silent auto-upgrade

**Override**:
Loud personal Version pin in `~/.config/skills-sync/overrides.toml`. Visible every TUI launch; Adopt does not overwrite it.
_Avoid_: silent local pins

**Placement Scope**:
Per Harness, exactly one of user or project. Claude default: user (`~/.claude/skills`).
_Avoid_: writing both scopes for one Harness

**Sync**:
Place from version cache via symlink by default, copy fallback (with warning) when symlinks fail. Sole distribution path for all four Harnesses.
_Avoid_: Marketplace install; format transformation of Skill bodies

**Dependency**:
Optional Skill names in frontmatter. Sync warns if missing; does not auto-install.
_Avoid_: hard auto-install of dependents

**Deprecation**:
Frontmatter mark (optional `successor:`); Catalog/TUI warn. Lock removal is a normal PR when the team is ready (no enforced clock). Tags kept forever.
_Avoid_: deleting Skill history

**Catalog**:
TUI/CLI + README + Lock File + static site generated from Skills on `main`.
_Avoid_: hosted CMS in v1

**skills-sync**:
Custom Rust CLI + full TUI. Bootstrap script downloads a Release binary, clones this repo into the cache, launches TUI. Config: `~/.config/skills-sync/`. Cache: `~/.cache/skills-sync/` (clone + extracted `name@semver` trees).
_Avoid_: requiring `cargo install` for normal onboarding

**Migration Docs**:
Required for breaking major Versions. Lives in the Skill folder.
_Avoid_: changelog-only notes with no adaptation guidance
