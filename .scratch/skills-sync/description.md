---
status: closed
labels: [ready-for-agent]
created: 2026-09-13
---

# skills-sync: cross-harness Skill distribution

## Problem Statement

Developers on a ~15-person team use Claude Code, Codex, Cursor, and Pi, but Skills live in one private repo and are hard to discover, install consistently, and keep on the same Versions. People ask where a Skill is and whether theirs is current; installs are manual and drift across machines. There is no Lock File, no Sync tool, no Release tagging, and no Catalog beyond browsing `skills/` in git.

## Solution

Ship **skills-sync**: a Rust CLI + full TUI that Syncs Skills from this library into each Harness’s directories at Lock File Versions (with loud Overrides and explicit Adopt). Pair it with tag-backed Releases (`name@semver` via CI on `version:` bump), a team `skills.lock`, bootstrap onboarding (binary + clone + TUI), warn-only Dependencies, Deprecation in Catalog, and a generated static Catalog site. Distribution is Sync-only (no Claude Marketplace). Behavior is built and proven under strict TDD at an agreed application-API seam, with line coverage kept above 85%.

## User Stories

1. As a new hire, I want a bootstrap script that downloads skills-sync and clones the library, so that I can get Skills in under five minutes without installing a Rust toolchain.
2. As a new hire, I want the first launch to open a TUI setup wizard, so that I can choose Skills and Harnesses without reading docs first.
3. As a developer, I want Sync to place selected Skills into my chosen Placement Scope for each Harness, so that my agents actually discover them.
4. As a Claude Code user, I want user-scope placement by default (`~/.claude/skills`), so that Skills work across all my projects.
5. As a developer, I want to optionally choose project-scope placement for a Harness instead of user-scope, so that I can keep Skills local to one repo when needed.
6. As a developer, I want Sync to never write both user and project scopes for the same Harness, so that I do not get confusing duplicate Skill resolution.
7. As a Codex user, I want Skills linked under `.agents/skills` (or the documented Codex discovery path), so that Codex loads the same Skill bodies.
8. As a Cursor user, I want Skills linked under `.cursor/skills` (or `~/.cursor/skills` per scope), so that Cursor loads them.
9. As a Pi user, I want Skills linked under `~/.pi/agent/skills` (or project `.pi/skills` per scope), so that Pi loads them.
10. As a developer, I want placement to prefer symlinks, so that edits in the version cache propagate to every Harness path.
11. As a developer on a machine that cannot symlink, I want copy fallback with a clear warning, so that Sync still works and I know edits will not propagate.
12. As a team member, I want a shared `skills.lock` that pins every Skill to a Version, so that we do not silently drift.
13. As a Skill author, I want Lock File changes to go through PR review, so that Version bumps are deliberate team decisions.
14. As a developer, I want an explicit Adopt action when the Lock File changes, so that upgrades are loud and intentional rather than silent.
15. As a developer with a personal Override, I want Adopt to skip that Skill and tell me, so that my test Version is not clobbered.
16. As a developer, I want Overrides stored in `~/.config/skills-sync/overrides.toml`, so that personal pins are durable and discoverable.
17. As a developer, I want every TUI launch to surface active Overrides, so that I never forget I am off the Lock File.
18. As a developer, I want a clear way to remove an Override, so that I can return to Lock File Versions.
19. As a developer, I want personal config for which Skills are on/off, so that I do not install Skills I never use.
20. As a developer, I want personal config for which Harnesses are enabled, so that unused tools stay clean.
21. As a developer, I want Sync to resolve each Skill from git tag `name@semver` via path extract of `skills/<name>/`, so that different Skills can be on different Versions.
22. As a developer, I want the library clone and extracted Version trees under `~/.cache/skills-sync/`, so that config and cache stay separated (XDG).
23. As a developer, I want config under `~/.config/skills-sync/`, so that Overrides and personal selection have a stable home.
24. As a Skill author, I want `version:` in Skill frontmatter, so that Catalog and CI can see the Version without parsing git history.
25. As a Skill author, I want CI to create `name@semver` when my `version:` bump merges to `main`, so that I do not forget to tag Releases.
26. As a Skill author, I want untagged work to live only on branches, so that `main` always has a tag for bumped Versions.
27. As a team lead, I want the Lock File to lag behind latest tags when we are not ready to roll out, so that Release to git is not the same as Adopt by the team.
28. As a developer, I want Sync to fail clearly when a Lock File Version has no matching tag, so that broken pins are obvious.
29. As a developer, I want Sync to fail clearly when frontmatter `version:` at a tag does not match the tag semver, so that corrupt Releases are caught.
30. As a developer, I want optional `dependencies:` in frontmatter, so that paired Skills are documented machine-readably.
31. As a developer, I want Sync to warn (not auto-install) when a Dependency is missing from my selection, so that I can choose deliberately.
32. As a Skill author, I want to mark a Skill deprecated with optional `successor:`, so that Catalog and TUI can warn users.
33. As a team member, I want deprecated Skills to remain installable until a Lock File PR removes them, so that we control rollout without deleting history.
34. As a developer, I want tags for deprecated Skills kept forever, so that old Lock pins still resolve.
35. As a developer, I want the TUI to list Skills with description, Version, Harness targets, and sync preview, so that I can discover and confirm placement before Syncing.
36. As a developer, I want CLI commands that perform the same operations as the TUI (init, sync, adopt, list/status, override management), so that automation and scripting work without a terminal UI.
37. As a developer, I want a generated static Catalog from Skills on `main`, so that people can browse Skills without running the TUI.
38. As a developer, I want the README plus Lock File plus Catalog to answer “what exists and what Version is current,” so that Slack “where is X?” questions drop.
39. As a Skill author, I want Migration Docs required for breaking major Versions, so that adopters know how to adapt.
40. As a developer running Sync, I want Migration Docs surfaced when Adopt moves me across a major Version, so that breaking changes are visible at Adopt time.
41. As a CI system, I want Lock File Versions validated against existing tags, so that bad pins cannot merge unnoticed.
42. As a release manager, I want every Skill listed in `skills.lock`, so that there is one pin table rather than a partial recommended set.
43. As a Windows developer, I want Sync to detect symlink failure and use copy fallback, so that I am not blocked on Developer Mode.
44. As a developer, I want Sync status to show linked vs copied targets, so that I know which Skills will not pick up cache edits.
45. As a maintainer, I want skills-sync behavior covered by tests at the application API seam under TDD, so that refactors do not silently break Adopt/Sync/Overrides.
46. As a maintainer, I want measured line coverage above 85% on skills-sync (and Release tagging helper code), so that untested paths cannot dominate the tool.
47. As a maintainer, I want tests to assert filesystem and status outcomes, not Ratatui internals or private helpers, so that UI churn does not break the suite.
48. As a developer, I want bootstrap to refresh/update the cached clone, so that later Syncs see new tags and Lock File commits.
49. As a developer, I want `skills sync` to re-link after Adopt without re-running full setup, so that day-two use stays fast.
50. As a developer, I want clear errors when the cache clone is missing or corrupt, so that I know to re-run bootstrap.
51. As a Skill author, I want Codex-only sidecars (e.g. `agents/openai.yaml`) left intact inside the Skill folder, so that portable bodies stay shared without Sync rewriting them.
52. As a team member, I want no Claude Marketplace requirement, so that all four Harnesses share one distribution path.

## Implementation Decisions

- Respect ADRs 0001–0004: Sync-only distribution; tag-backed per-Skill Versions; Lock + Adopt + loud Overrides; custom Rust skills-sync with bootstrap binary and XDG paths.
- Use domain vocabulary from `CONTEXT.md` (Skill, Harness, Version, Lock File, Adopt, Override, Placement Scope, Sync, Dependency, Deprecation, Catalog, Release, Migration Docs).
- Build skills-sync as a Rust workspace/crate with:
  - An **application API** module used by both CLI and TUI (the sole primary test seam).
  - CLI adapter (clap or equivalent) that only parses args and calls the API.
  - TUI adapter (Ratatui) that only renders API state and sends API commands — full mockup-scope UX in v1 (skill list, details, harness toggles, sync preview, status).
  - Adapters for git (clone/fetch, path extract at `name@semver`), filesystem (symlink/copy placement), and path resolution for Harness dirs — injected/replaceable in tests via the application API’s configuration (temp roots).
- Lock File format: TOML mapping every Skill name → semver Version string; lives at repo root as `skills.lock`.
- Personal selection + Harness enablement: TOML under `~/.config/skills-sync/` (exact filename chosen in implementation; not Overrides).
- Overrides: `~/.config/skills-sync/overrides.toml` mapping Skill → Version; always printed in status/TUI.
- Cache layout under `~/.cache/skills-sync/`: git clone of the library + extracted trees keyed by `name@version` (or equivalent), staged as clean `<name>/` folders for placement.
- Placement targets (scope-aware): Claude Code, Codex, Cursor, Pi discovery directories as documented in CONTEXT/research; exactly one Placement Scope per enabled Harness.
- Symlink by default; on failure, copy and record a warning in the Sync result.
- Skill frontmatter: require `name`, `description`, `version`; optional `dependencies` (list of Skill names); optional deprecation fields (`deprecated`, `successor`).
- Release helper: CI (or a small script invoked by CI) detects Skill `version:` bumps on merge to `main` and creates annotated/lightweight tags `name@semver`; validates tag semver matches frontmatter at that path.
- Catalog generator: static pages/markdown under something like `docs/catalog/` generated from `skills/*/SKILL.md` on `main` (name, description, version, deprecation, migration links).
- Bootstrap script: detect OS/arch, download skills-sync from GitHub Releases, ensure cache clone exists/updates, launch TUI init.
- Existing Skills (`confluence`, `jira`, `story-review`, `story-writing`) must gain `version:` (and Lock File pins) as part of bringing the library online; no Marketplace overlay files required.
- Dependencies: warn-only; never auto-install.
- Deprecation: warn in Catalog/API status; Lock removal is a normal human PR (no enforced grace timer).

## Testing Decisions

- **Strict TDD (red → green, vertical slices):** For each behavior, write one failing test at an agreed seam first; write only enough production code to pass; repeat. No bulk “all tests then all code.” Do not test implementation details or private helpers. Refactors that preserve behavior should not require test rewrites.
- **Coverage gate:** Maintain **line coverage &gt; 85%** for skills-sync application code and the Release tagging helper. Enforce in CI (fail the build under the threshold). Prefer covering behavior through the application API rather than padding with low-value tests.
- **Primary seam — skills-sync application API:** Tests drive the API with temporary config/cache/home directories and a fixture git repository containing Skills at multiple `name@semver` tags plus a Lock File. Assert external behavior only: cache contents, Placement Scope filesystem results (symlink vs copy semantics observable by the user), Adopt/Override skip rules, Dependency warnings, Deprecation flags in status, error cases (missing tag, version mismatch), and catalog/status DTOs the TUI would display.
- **Secondary seam — Release tagging helper:** Tests use fixture commits/trees where `version:` bumps (or not) and assert which tags would be created / which validations fail. Do not require a live GitHub Actions run for unit/integration proof; the workflow wires the helper.
- **Non-seams (do not test directly):** Ratatui widgets, clap parsing beyond smoke that CLI dispatches to API, exact git CLI argv strings, internal cache key formatting — only via observable outcomes.
- **Good tests:** Named in domain language (“Adopt skips Override and reports it”); expected values from the spec/fixtures (known semver, known tag), never tautological recomputation of production logic; no mocking of internal collaborators — replace only outer adapters (fs root, git fixture) at the API boundary.
- **Prior art:** None in-repo yet (greenfield). Establish the first tests as the pattern for later slices.
- **Harness/OS note:** Symlink-vs-copy behavior is asserted through API results and filesystem probes in temp dirs; Windows-specific fallback can be simulated by injecting a failing symlink operation in tests.

## Out of Scope

- Claude Code Marketplace / `.claude-plugin/marketplace.json` publishing
- Hosted Catalog CMS or web app (static generation only)
- Auto-installing Dependencies
- Personal Version Overrides that are silent or default-on for the whole team
- Parallel version folders on `main` (e.g. `skills/jira/1.2.0/`)
- Repo-wide Version tags for individual Skills
- Adopting rulesync/AgentsMesh/Ruler as the Sync backbone
- Requiring `cargo install` as the normal onboarding path
- Enforced deprecation grace-period timers
- Transforming Skill bodies per Harness (sidecars may exist; Sync does not rewrite SKILL.md)
- Multi-context domain split / CONTEXT-MAP
- Guaranteeing 100% coverage or testing UI pixel/layout details

## Further Notes

- Glossary: `CONTEXT.md`. Architecture decisions: `docs/adr/0001`–`0004`. Background research: `cross-harness-skill-repository.md` (Marketplace sections superseded by ADR-0001).
- Suggested implementation order under TDD: (1) Lock + resolve desired Versions with Overrides, (2) git path-extract into cache, (3) placement symlink/copy + one Placement Scope, (4) Adopt, (5) Dependency/Deprecation warnings, (6) CLI, (7) TUI, (8) Release helper + CI, (9) Catalog generator, (10) bootstrap script + Release binaries.
- Issue tracker path: `.scratch/skills-sync/`.
