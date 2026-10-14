# Tag-backed per-skill Versions

**Status:** accepted

Each Skill Version is semver in `SKILL.md` frontmatter plus an immutable git tag `name@semver`. `main` holds only the latest tree at `skills/<name>/`. Sync path-extracts `skills/<name>/` at the tagged commit into a local cache, then places that tree into the canonical `.agents/skills` store (and Harness links). CI creates the tag on merge to `main` when that Skill’s `version:` bumps. Lock File pins are validated in CI against existing tags or matching HEAD frontmatter.

**Considered options:**
- Repo-wide tags — cannot pin Skills to different Versions independently
- Parallel version folders on `main` — fights harness discovery (`skills/<name>/SKILL.md`)
- Frontmatter-only / main-only — cannot install historical Versions
- Tagging individual files — Git cannot do this; tags point at commits; path extract gives per-Skill history

Different Skills can sit on different Versions because Sync ignores sibling paths at a tagged commit. Untagged work lives on branches only; Lock File lag (not missing tags) paces team rollout.
