# story-review

**Version:** 1.0.0

Score and give teaching feedback on user stories, epics, and defects (Jira, DE project) using a weighted per-dimension rubric that rolls up to a 0–100 score and letter grade. Use whenever asked to review, score, grade, critique, or give feedback on a story/epic/bug, or when a user pastes a ticket and wants to know how to improve it. Pairs with the `story-writing` (v2) skill for the authoring standards this rubric enforces.


**Dependencies:** story-writing

## Install

```bash
skills-sync init --skills story-review --harnesses claude-code,cursor && skills-sync sync
```
