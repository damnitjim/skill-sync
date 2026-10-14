# Skill Catalog

Skills on `main` (latest frontmatter Versions). Team pins may lag — see [`skills.lock`](../../skills.lock).

## Install

```bash
./scripts/bootstrap-skills-sync.sh
# or per Skill (see each page):
skills-sync init --skills <name> --harnesses claude-code,cursor && skills-sync sync
```

Details: [skills-sync/README.md](../../skills-sync/README.md).

| Skill | Version | Description | Status |
|-------|---------|-------------|--------|
| [confluence](confluence/) | 1.0.0 | Read, search, and publish Confluence wiki pages for Sophos using curl+jq shell scripts. Use this when working with Confluence pages, wiki documentation, spaces, or knowledge base content, or when publishing a report/page to Confluence. | active |
| [jira](jira/) | 1.0.0 | Manage Jira issues for Sophos using curl+jq shell scripts. Use this when working with Jira issues, sprints, tickets, assignments, transitions, filters, or queries for the DE project. | active |
| [story-review](story-review/) | 1.0.0 | Score and give teaching feedback on user stories, epics, and defects (Jira, DE project) using a weighted per-dimension rubric that rolls up to a 0–100 score and letter grade. Use whenever asked to review, score, grade, critique, or give feedback on a story/epic/bug, or when a user pastes a ticket and wants to know how to improve it. Pairs with the `story-writing` (v2) skill for the authoring standards this rubric enforces. | active |
| [story-writing](story-writing/) | 1.0.0 | Write or review user stories, epics, defects/bugs, and tasks (Jira, DE project) following the team's story-writing guidelines — Connextra "As a / I want / So that" format, Given/When/Then + rule-oriented acceptance criteria, INVEST, Definition of Ready/Done, story splitting (SPIDR + Humanizing Work), and a full defect-reporting standard (repro steps, severity vs priority, bug-vs-story). Use whenever creating, refining, or reviewing Jira issues/requirements, writing a story/epic/task/bug, splitting an oversized story, or filing a defect. Pairs with the `jira` skill for ticket mechanics. | active |
