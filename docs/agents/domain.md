# Domain docs

Your project uses a **single-context** domain docs layout: one unified knowledge base for the entire project.

## Structure

```
.
├── CONTEXT.md           # Project overview, architecture, key concepts
└── docs/adr/            # Architecture Decision Records
    ├── 0001-*.md
    ├── 0002-*.md
    └── ...
```

## CONTEXT.md

The root `CONTEXT.md` is the single source of truth for project knowledge. It should include:

- **Overview**: what this project does, who uses it, key goals
- **Architecture**: system design, major components, data flow
- **Tech stack**: languages, frameworks, major dependencies
- **Conventions**: naming, patterns, code style
- **How to run**: setup, dev environment, common commands
- **Key files**: brief map of important directories and modules

Format: plain markdown, no special syntax required.

## Architecture Decision Records (ADRs)

Store ADRs in `docs/adr/` using the naming convention `NNNN-short-title.md`:

```markdown
# NNNN. Decision title

**Date**: 2026-09-12  
**Status**: Accepted | Pending | Rejected | Superseded

## Context

Why we needed to decide this.

## Decision

What we decided and why.

## Consequences

What changes as a result.
```

Number ADRs sequentially; they form a decision journal.

## Using with agent skills

Agent skills read from these files to understand your project:

- `mattpocock-skills:domain-modeling` — extract domain concepts
- `mattpocock-skills:codebase-design` — design new features
- `mattpocock-skills:diagnosing-bugs` — debug issues with context

## Creating CONTEXT.md

Start with a template or existing documentation. If you have a README, you can often repurpose its content as a starting point for CONTEXT.md.

To create one now, run:

```
touch CONTEXT.md
# Then edit with your project overview
```

## Switching layouts

If your project grows into a monorepo, you can switch to **multi-context** mode with a root `CONTEXT-MAP.md` pointing to per-context `CONTEXT.md` files. Update `docs/agents/domain.md` when you do.
