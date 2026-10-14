---
name: story-writing
description: >
  Write or review user stories, epics, defects/bugs, and tasks (Jira, DE project)
  following the team's story-writing guidelines — Connextra "As a / I want / So that"
  format, Given/When/Then + rule-oriented acceptance criteria, INVEST, Definition of
  Ready/Done, story splitting (SPIDR + Humanizing Work), and a full defect-reporting
  standard (repro steps, severity vs priority, bug-vs-story). Use whenever creating,
  refining, or reviewing Jira issues/requirements, writing a story/epic/task/bug,
  splitting an oversized story, or filing a defect. Pairs with the `jira` skill for
  ticket mechanics.
version: 1.1.0
dependencies:
  - jira
allowed-tools: Read
---

# Story Writing Guidelines (DE)

How we write and refine work in the DE Jira project: epics, feature stories,
technical/enabler work, and defects. Framework-agnostic (widely-accepted agile/Scrum
practice), optimized for small, vertically-sliced, testable increments traceable to
product requirements (F-numbers). Use the `jira` skill for the actual tickets.

**What's new in 1.1.0:** worked “good vs weak *so that*” examples under the feature
story format (§2).

Two deep references load on demand — read them when the task calls for them:

- **`references/defect-reporting.md`** — the full bug standard: report template,
  severity vs priority, bug-vs-story decision, common mistakes.
- **`references/story-splitting.md`** — the splitting pattern library: vertical vs
  horizontal slicing, SPIDR, the Humanizing Work nine patterns.

## When to use this

- Writing or reviewing a DE story, epic, task, or bug/defect.
- Refining a backlog item, checking it against INVEST / Ready / Done.
- An oversized story needs to become an epic + splits (→ `references/story-splitting.md`).
- Filing or reviewing a defect (→ `references/defect-reporting.md`).

## 1. Core philosophy

A **story is a promise for a conversation, not a specification** (Cockburn; Ron
Jeffries' **3 C's**):

- **Card** — a short placeholder, deliberately small. Not a mini-requirements doc.
- **Conversation** — where shared understanding actually happens (in refinement).
- **Confirmation** — the acceptance criteria/tests that prove it's done.

The most common failure mode is treating the card as a frozen spec. Don't.

We **slice work vertically** — every story delivers an observable change in behavior
for a user, cutting through all layers (UI → logic → data) as needed.

## 2. Feature story format

Connextra template (origin: Rachel Davies / Connextra ~2001; popularized by Mike Cohn):

```
As a [role / persona]
I want [capability / goal]
So that [benefit / value]
```

- Answer **who / what / why**. The "so that" is where value lives — never skip it.
  Prefer concrete value ("so that I can quickly spot high-priority cases") over
  restating the action ("so that I can sort cases").
- Write from the **user's** perspective, not the system's. Name the real persona
  (admin, analyst, case manager, on-call) — avoid "As a user…" as a lazy default.
- **Don't prescribe the solution** ("…via an autocomplete dropdown"). State the goal;
  leave the how to the team.

### Good vs weak “so that”

Weak *so that* lines restate the capability or name a UI widget. Strong ones name an
observable benefit for the persona.

| Weak | Better |
|------|--------|
| As a case manager / I want to sort cases by priority / **so that I can sort cases** | … / **so that I can triage the highest-risk cases first during my morning review** |
| As an admin / I want an autocomplete dropdown for assignees / **so that I can pick an assignee from a dropdown** | As an admin / I want to find the right assignee quickly / **so that tickets stop sitting unowned while I scroll a long list** |
| As an analyst / I want export to CSV / **so that I can export data** | As an analyst / I want to take a filtered case list offline / **so that I can finish a compliance review without staying logged into the tool** |

If the *so that* only repeats the *I want*, rewrite it before calling the story Ready.

## 3. INVEST — every story should be

- **Independent** — developable on its own; avoid dependencies between stories in the
  same sprint.
- **Negotiable** — a conversation starter, not a frozen spec.
- **Valuable** — delivers clear user/business value. If you can't state the value, it's
  probably a task or a component, not a story.
- **Estimable** — the team can size it.
- **Small** — fits comfortably in a sprint.
- **Testable** — acceptance criteria make pass/fail clear.

Two nuances (Bill Wake, 2003; Humanizing Work): there's a natural tension — **I-N-V**
are easiest when a story is big, **E-S-T** easiest when small. So **near the top of the
backlog prioritize Small / Estimable / Testable**; further out, Independent /
Negotiable / Valuable matter more. Rule of thumb for "small": aim for stories where
**6–10 could fit in a sprint** (tune to DE's sprint length, team size, and velocity).

## 4. Acceptance criteria

The **Confirmation** of the story. Pick the format that fits per story:

**Scenario-oriented (Given/When/Then)** — for behavior with a clear trigger (Dan North
/ BDD; maps to Precondition → Trigger → Outcome):

```
Given [precondition / context]
When  [single trigger / action]
Then  [observable outcome]
```

- **One trigger per scenario.** Don't chain multiple `When … And` statements — it blurs
  precondition and trigger.
- Don't reflexively start with "Given I am logged in" unless login is the subject.
- Use scenario outlines / tables for data variations.

**Rule-oriented (checklist)** — for simple rules, system-level behavior, or design/UX
constraints where Given/When/Then is awkward.

- Aim for **~1–3 criteria** (a small handful at most). If you need many more, the story
  is too big — split it (see §7).
- Each criterion must be **pass/fail testable without interpretation**. Focus on the
  outcome, not the implementation.

## 5. Definition of Ready & Definition of Done

- **Definition of Ready (guideline, not a hard gate).** Our refinement checklist: clear
  who/what/why; acceptance criteria drafted; dependencies known; sized; small enough for
  one sprint. INVEST doubles as our DoR. (Scrum doesn't mandate a DoR — this is a
  helpful convention.)
- **Definition of Done (mandatory commitment, applies to every item).** e.g. code
  reviewed; unit/automated tests passing; meets acceptance criteria; recurring NFRs
  (logging, security, accessibility) satisfied; docs updated; merged/deployable. Work
  that doesn't meet DoD is **not done** and cannot be released or demoed. (The 2020
  Scrum Guide formally defines only the DoD.)
- **DoD ≠ acceptance criteria.** DoD = universal quality standards; AC = per-story
  functional conditions. **Both must be met.**

## 6. Hierarchy (DE convention)

**Epic → Story → Sub-task**, with **Defects linked to their originating Story/Epic**.
(Note: Scrum itself recognizes only "Product Backlog Items"; the epic/story/bug/task
taxonomy is a Jira convention layered on top — useful, not prescribed.)

- **Epic** — large body of work spanning multiple sprints; estimated in weeks+. Split
  into stories. (Rule of thumb: anything estimated in *weeks* rather than hours/days is
  an epic and should be split.)
- **Story** — finishable in one sprint; a vertical slice of value.
- **Sub-task** — an implementation step of a story, including technical work.
- Every story/defect should link to a **parent epic** and its **F-number** requirement
  (traceability). Use the originating Jira/requirement when no F-number exists.
  Housekeep orphaned items regularly.

## 7. Story types & technical work

- **Feature story** — default; user-facing value.
  > As a case manager / I want to filter cases by priority / so that I can focus on the
  > most critical issues first.
- **Technical work** — minimize standalone "technical stories." Prefer capturing it as
  **sub-tasks under a user-valuable story**, or (if unavoidable) as an explicitly
  **labeled enabler** with a clear reason. Don't dress a task as a story
  ("As a developer, I want a database…" is a task masquerading as a story).
- **Non-functional requirements** — capture as **acceptance criteria** on the affected
  story; put recurring qualities in the **DoD**; for system-wide NFRs (performance,
  security) use a **constraint story** (narrative + testable criteria). Always frame as
  observable behavior ("search responds within 3 seconds").
- **Spike** — time-boxed research (commonly 1–3 days of effort), **one clear question**,
  output is **knowledge, not shippable code**. Report findings even if the question
  isn't fully answered. Use sparingly — a spike is not "design everything up front."

## 8. When a story is too big → split it

If a story is estimated in weeks, or needs more than a handful of acceptance criteria,
**convert it to an Epic and split into thin vertical slices**.

**Slice vertically, never horizontally.** Horizontal slices ("build the DB", "build the
API", "build the UI" as separate stories) are the **#1 splitting anti-pattern** — they
fail INVEST's Independent and Valuable and demo nothing. Think of eating a slice through
all layers of a cake, not one layer at a time.

For the full pattern library — **SPIDR** (Spike, Paths, Interfaces, Data, Rules) and the
**Humanizing Work nine patterns** with worked examples — see
**`references/story-splitting.md`**.

## 9. Defects / bugs

A defect is a **discrepancy between actual and expected behavior**. Frame the fix as the
correct behavior the user expects, and (for anything non-trivial) give it acceptance
criteria.

The full DE bug standard — report template (title, environment, numbered repro steps,
expected vs actual, evidence), **severity vs priority** (independent axes), and the
**bug-vs-story decision** (fix in-story vs new ticket vs reopen) — lives in
**`references/defect-reporting.md`**. Read it before writing or reviewing a defect.

## 10. Common mistakes to avoid

- Writing technical tasks as user stories.
- Over-detailed specs up front (detail emerges just-in-time in refinement).
- Stories too large or too small; dependencies between stories in the same sprint.
- **Horizontal slices** (DB/API/UI as separate stories).
- **Multiple bugs in one ticket** — one bug per ticket.
- Missing repro steps or environment on a defect; stating guessed root causes as fact.

## Review checklist (before "Ready")

- [ ] `As a / I want / So that` present, user-centric, with a real "so that" that states
      value (not a restatement of the capability — see §2 examples).
- [ ] Persona is a user/role, not a system component (unless a deliberate enabler/NFR).
- [ ] Passes INVEST — especially **Small** (fits comfortably in a sprint) and **Independent**.
- [ ] **Vertical slice** — not a horizontal DB/API/UI fragment or a disguised task.
- [ ] **~1–3 testable acceptance criteria**, outcome-focused (GWT or rule-oriented).
- [ ] NFRs captured where needed (AC / DoD / constraint story).
- [ ] Linked to **parent epic** and **F-number** (or related ticket).
- [ ] Too big → Epic + vertical splits + supporting sub-tasks.
- [ ] **For defects:** full template complete; **severity and priority both set**;
      evidence attached; one bug only. (→ `references/defect-reporting.md`)

## Creating the ticket

Use the `jira` skill. Note `jira-create --description` flattens to a single plain
paragraph (ADF). For a structured description (headings, the story block, Given/When/Then
bullets), POST a proper ADF `description` via `curl .../rest/api/3/issue` instead, or
create with a short summary and add the structured body in a follow-up. Default project:
`DE`.

## Changelog

### 1.1.0

- Added **Good vs weak “so that”** worked examples under the feature story format.
- Review checklist now points authors at those examples when validating value statements.

### 1.0.0

- Initial Release of the DE story-writing guidelines.
