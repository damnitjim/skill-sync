# story-writing

**Version:** 1.0.0

Write or review user stories, epics, defects/bugs, and tasks (Jira, DE project) following the team's story-writing guidelines — Connextra "As a / I want / So that" format, Given/When/Then + rule-oriented acceptance criteria, INVEST, Definition of Ready/Done, story splitting (SPIDR + Humanizing Work), and a full defect-reporting standard (repro steps, severity vs priority, bug-vs-story). Use whenever creating, refining, or reviewing Jira issues/requirements, writing a story/epic/task/bug, splitting an oversized story, or filing a defect. Pairs with the `jira` skill for ticket mechanics.


**Dependencies:** jira

## Install

```bash
skills-sync init --skills story-writing --harnesses claude-code,cursor && skills-sync sync
```
