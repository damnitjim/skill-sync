# Issue tracker

Issues are tracked as markdown files in the `.scratch/` directory. This supports solo projects and local-first workflows without requiring a remote repository.

## How it works

Each issue is a markdown file stored under `.scratch/<feature>/`. The filename serves as the issue ID.

### Creating an issue

Use the `to-tickets` skill or create a file manually:

```
.scratch/<feature-name>/
├── description.md       # Issue description and context
├── discussion.md        # Comments and discussion (optional)
└── metadata.json        # Labels, assignee, status (optional)
```

### Issue properties

Within `description.md`, use frontmatter for metadata:

```markdown
---
status: open
labels: [bug, urgent]
created: 2026-09-12
---

# Issue title

Issue description and acceptance criteria.
```

### Supported status values

- `open` — work not started
- `in-progress` — actively being worked on
- `in-review` — awaiting review
- `closed` — completed or rejected

## Using with agent skills

The following agent skills read from this issue tracker:

- `mattpocock-skills:to-tickets` — convert ideas into issues
- `mattpocock-skills:to-spec` — expand issues into specifications

## Switching trackers

To switch to GitHub Issues or another tracker, update `docs/agents/issue-tracker.md` and `CLAUDE.md`.
