---
name: story-review
description: >
  Score and give teaching feedback on user stories, epics, and defects (Jira, DE
  project) using a weighted per-dimension rubric that rolls up to a 0–100 score and
  letter grade. Use whenever asked to review, score, grade, critique, or give feedback
  on a story/epic/bug, or when a user pastes a ticket and wants to know how to improve
  it. Pairs with the `story-writing` (v2) skill for the authoring standards this rubric
  enforces.
version: 1.0.0
dependencies:
  - story-writing
allowed-tools: Read
---

# Story Review & Scoring Rubric (DE)

Apply this whenever a user asks you to review, score, grade, or improve a user story,
epic, or defect — or pastes a ticket and asks "is this good?" This skill is the
assessment counterpart to the `story-writing` (v2) skill, which holds the authoring
standards; scores here map directly to those standards.

Your job is not just to grade — it's to **teach**. Every dimension score comes with a
short "why," and every dimension below full marks comes with a concrete rewrite of the
user's own text plus a one-line reference to the principle behind it, hyperlinked to its
source (see `references/calibration.md` §Sources).

**One rule above all: quote the user's words, then improve them.** Generic advice ("add
acceptance criteria") teaches far less than showing their story with the criteria
written in.

Two deep references load on demand:

- **`references/worked-examples.md`** — fully scored example reviews (weak story, weak
  defect) showing the scorecard → teaching notes → rewrite flow end to end.
- **`references/calibration.md`** — how to score consistently: anchoring, drift traps,
  the tie-breaker, and the citable source list. Read it when scores feel subjective, when
  reviewing a batch, or whenever you're about to write Full-mode teaching notes — every
  docked dimension needs a source link, and this is where the links live.

## When to use this

- Reviewing/scoring/grading a DE story, epic, or defect.
- A user pastes a ticket and asks "is this good / ready?" or "how do I improve this?"
- Triaging a batch of tickets (→ Compact mode, §8).

## 0. Workflow

Pick a **mode** first. Default to **Full mode** (scorecard + teaching notes + rewrite).
Use **Compact mode** (§8) when the user asks for a quick score, says "just the score,"
is triaging several tickets, or pastes a batch. When in doubt, give the compact line and
offer the full review. Both modes use the identical dimensions, weights, and formula —
compact simply omits teaching notes and rewrite.

1. **Detect the artifact type** — Story, Epic, or Defect. If ambiguous, infer from
   content (has "As a…"? → story; spans multiple sprints / lists child stories? → epic;
   describes something broken / has repro steps? → defect). State which rubric you apply.
2. **Score each dimension** for that type on the 1–4 scale (§1).
3. **Multiply by weight and sum** to a 0–100 total (§2).
4. **Convert to a letter grade** (§3).
5. **Render feedback** via the output template (§7) — scorecard first, then per-dimension
   teaching notes, then a fully rewritten version of their ticket.
6. **Be encouraging and specific.** Praise what scored well before fixing what didn't.
   Never invent facts to fill gaps — if the story omits something, that *is* the finding;
   show the placeholder to fill in.

## 1. The 1–4 scale (every dimension)

| Score | Label | Meaning |
|:-:|---|---|
| **4** | Exemplary | Meets the standard fully; could be a team example. |
| **3** | Solid | Meets the standard; minor polish possible. |
| **2** | Developing | Partially meets it; a real gap to fix before pulling into a sprint. |
| **1** | Inadequate | Missing or wrong; blocks the ticket from being "ready." |

No half-points — force a decision. **When torn between two scores, pick the lower and
state what would raise it.** That keeps the rubric honest and the feedback actionable.

## 2. Dimensions & weights by artifact type

Weights sum to 100. See §2.4 for the formula.

### 2.1 STORY (7 dimensions)

| # | Dimension | Weight | What "4" looks like |
|---|---|:-:|---|
| S1 | Format & role–goal–benefit | 15 | Clean `As a [real persona] / I want [goal] / So that [value]`; the "so that" states genuine value, not a restatement of the action. |
| S2 | Value & the "why" | 15 | Benefit is concrete and user/business-meaningful; a dev could make trade-off decisions from it. |
| S3 | Acceptance criteria | 20 | Testable criteria (Given/When/Then or clear rule list); covers happy path + key edge/error cases; unambiguous pass/fail. |
| S4 | INVEST — Small & Independent | 15 | Fits in a sprint; no hidden dependency on another in-sprint story. |
| S5 | INVEST — Valuable & vertical slice | 10 | Thin end-to-end slice of user function, not a technical-layer fragment. |
| S6 | Clarity & correct scope | 15 | One piece of user value; unambiguous; not over-specified; jargon kept out of the story sentence. |
| S7 | Traceability & metadata | 10 | Links to parent/requirement (F-number or related ticket); right type; sensible title. |

### 2.2 EPIC (6 dimensions)

| # | Dimension | Weight | What "4" looks like |
|---|---|:-:|---|
| E1 | Goal & scope clarity | 20 | Coherent, sizable outcome and its business value; clear boundaries of what's in/out. |
| E2 | Decomposition into stories | 25 | Split into sprint-sized stories, each an independent vertical slice of value (not layers/components). |
| E3 | Slice quality of children | 15 | Uses recognized splitting patterns (workflow steps, rules, data, paths, interface, spike — SPIDR); no "design/build/test" layer splits. |
| E4 | Value sequencing | 15 | Earliest stories deliver usable value first; progressive enhancement is deliberate. |
| E5 | Technical work placed correctly | 15 | Supporting technical work captured as Tasks/enablers, not inflating story count or masquerading as user stories. |
| E6 | Traceability & metadata | 10 | Linked to initiative/requirement; children linked to the epic; sensible title. |

### 2.3 DEFECT (7 dimensions)

| # | Dimension | Weight | What "4" looks like |
|---|---|:-:|---|
| D1 | Title / summary | 10 | Specific and searchable: what's broken + where + under what condition. |
| D2 | Steps to reproduce | 20 | Numbered, minimal, deterministic; someone else could follow them cold. |
| D3 | Expected vs. actual | 20 | Both stated explicitly and separately; the gap is unmistakable. |
| D4 | Environment / build / version | 10 | OS/browser/device, app version or build, relevant config/account/data. |
| D5 | Severity & priority | 15 | Both set and distinguished (impact vs. urgency); justified by the described effect. |
| D6 | Evidence & reproducibility | 15 | Screenshot/log/video attached where useful; frequency stated (always / intermittent / once). |
| D7 | Scope & objectivity | 10 | One defect per ticket; facts not opinions; no bundled feature requests. |

### 2.4 Scoring formula

```
dimension_contribution = (score_1to4 / 4) * weight
total = sum(dimension_contributions)          # 0–100, round to nearest integer
```

Example (a story): S1=4, S2=3, S3=2, S4=3, S5=3, S6=3, S7=4
= (4/4·15)+(3/4·15)+(2/4·20)+(3/4·15)+(3/4·10)+(3/4·15)+(4/4·10)
= 15+11.25+10+11.25+7.5+11.25+10 = **76 → C**.

## 3. Letter grades

| Score | Grade | Verdict |
|:-:|:-:|---|
| 90–100 | **A** | Ready. Sprint-ready; use as a team example. |
| 80–89 | **B** | Nearly ready. One or two small fixes. |
| 70–79 | **C** | Needs work. Real gaps; refine before committing. |
| 60–69 | **D** | Not ready. Multiple core gaps. |
| < 60 | **F** | Rework. Rewrite against the guide before review. |

Always pair the grade with the **single highest-leverage next action** ("The fastest way
to move this from C to B: write Given/When/Then for the error case").

## 4. Scoring guidance per dimension (the "why" + how to teach)

Apply the anchor, then teach with the user's own text.

**Story**

- **S1 Format & role–goal–benefit** (Connextra template, popularized by Mike Cohn). Dock
  to 2 if the "so that" restates the action ("so that I can sort" on a sorting story).
  Dock to 1 if the persona is a system component when it should be a user. Teach: rewrite
  their "so that" to name a real benefit.
- **S2 Value & the "why"** — the benefit is what lets devs make good calls in refinement.
  1–2 if value is missing, circular, or purely technical on a user-facing story.
- **S3 Acceptance criteria** (highest weight — it defines "done"). 4 needs testable
  criteria covering happy path + at least one edge/error case, in Given/When/Then or a
  clear rule list. 1 if absent; 2 if present but vague ("works correctly") or
  happy-path-only. Teach: convert one of their vague lines into a proper Given/When/Then.
- **S4 INVEST — Small & Independent** — 1–2 if it can't fit a sprint (→ suggest it become
  an Epic and show 2–3 candidate splits) or depends on another in-sprint story.
- **S5 INVEST — Valuable & vertical slice** — 1–2 if it's a horizontal/technical fragment
  ("build the API endpoint") with no user-visible slice. Teach: reframe as a thin
  end-to-end slice.
- **S6 Clarity & correct scope** — 2 if two pieces of value are bundled (split them) or
  if it's over-specified to the point of freezing design.
- **S7 Traceability & metadata** — 1–2 if no link to a parent/requirement or the issue
  type is wrong.

**Epic**

- **E2 Decomposition** (highest weight): 4 = children are independent vertical slices;
  1–2 = split by technical layer/phase ("design", "backend", "testing") or one giant
  story. Reference SPIDR (Cohn) and the Humanizing Work splitting patterns.
- **E3 Slice quality**: name which pattern each split uses; penalize component splits.
- **E5 Technical work placement**: enablers/infra belong in Tasks, not user stories —
  reference the team standard.

**Defect**

- **D2 Steps to reproduce** & **D3 Expected vs. actual** carry the most weight — a bug
  that can't be reproduced, or whose "wrongness" isn't defined, can't be fixed.
- **D5 Severity vs. Priority** — different axes. Severity = impact if it occurs;
  Priority = urgency to fix. A homepage typo can be low severity / high priority; a crash
  in a rarely-used admin export can be high severity / low priority. Dock to 2 if
  conflated or only one is set.
- **D7 Scope & objectivity** — one bug per ticket; observed facts, not "the page is
  broken" or bundled "and while you're at it…" requests.

For fully worked, scored examples, see **`references/worked-examples.md`**. For citable
source references, see **`references/calibration.md` §Sources** — always pull the
matching link from there when writing a teaching note.

## 5. Output template (Full mode — use every time)

```
## Review: <ticket title>  —  <TOTAL>/100  (<GRADE>)
Artifact type: Story | Epic | Defect
Fastest win: <the single highest-leverage improvement>

### Scorecard
| Dimension | Score (1–4) | Weight | Contribution |
|-----------|:-----------:|:------:|:------------:|
| … one row per dimension … |
| **Total** | | **100** | **<TOTAL>** |

### What's working
- <praise the dimensions that scored 3–4, specifically>

### What to improve (teaching notes)
- **<Dimension> (<score>):** why it lost points + a rewrite of THEIR text + a markdown
  link to the principle/reference (from calibration.md §Sources).
- … one bullet per dimension scoring 1–2 (and any 3 worth polishing) …

### Rewritten version
<full improved ticket they can paste into Jira>
```

Keep the tone of a supportive coach: concrete, brief, always modeling the fix rather than
only naming it. When the ticket is oversized, don't just score S4 low — show the
candidate split. When a defect can't be reproduced from what's written, say exactly which
line is missing and provide the placeholder to fill.

## 6. Compact scoring mode (quick triage)

Same dimensions, weights, and formula as Full mode — only the presentation changes.
Output one block per ticket:

```
<ticket title> — <TOTAL>/100 (<GRADE>) · <artifact type>
Dims: S1 4 · S2 3 · S3 2 · S4 3 · S5 3 · S6 3 · S7 4
Fastest win: <the single highest-leverage fix, one line>
```

Rules:

- Still **score every dimension** (show the per-dimension 1–4 line) — the number is the
  teaching signal even without prose.
- Give **exactly one "fastest win"** — the lowest-scoring, highest-weight dimension.
- No rewrite, no per-dimension prose, no "what's working" section.
- For a **batch**, emit one block per ticket, then a one-line roll-up:
  `Batch: 5 tickets · avg 74 (C) · weakest dimension across set: Acceptance criteria (S3).`
  A shared weak dimension across many tickets is a coaching signal for the whole team.
- Always close with an offer: *"Say 'full review of #2' for the scorecard, teaching
  notes, and a rewrite."* Keep the ticket text in context so the follow-up works
  immediately — never make the user re-paste.
