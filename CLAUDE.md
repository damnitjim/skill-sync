# CLAUDE.md

This file documents how Claude Code agents should work with this project.

## Agent skills

### Issue tracker

Issues are tracked as markdown files in `.scratch/`. See `docs/agents/issue-tracker.md`.

### Domain docs

Single-context layout: one `CONTEXT.md` + `docs/adr/` at repo root. See `docs/agents/domain.md`.

Use `CONTEXT.md` vocabulary (Skill, Harness, Version, Lock File, Adopt, Override, Sync, Placement Scope) when changing distribution or `skills-sync`. Product decisions live in `docs/adr/`; `cross-harness-skill-repository.md` is historical research (Marketplace hybrid superseded by ADR-0001).

### Install / Sync tooling

End-user and maintainer flows for placing Skills: [README.md](README.md) and [skills-sync/README.md](skills-sync/README.md). Bootstrap: `scripts/bootstrap-skills-sync.sh`. Behavior is covered at the application API seam under TDD (`skills-sync/tests/`).

### Contributing Skills

Authoring, semver, Lock File, and PR checklist: [CONTRIBUTING.md](CONTRIBUTING.md).
