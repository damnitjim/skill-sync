# Calibration & Sources

Loaded on demand by the `story-review` skill. Read this when scores feel subjective, when
reviewing a batch, or when you need to cite the principle behind a docked point.

The rubric is only useful if the **same ticket earns roughly the same score every time**,
regardless of reviewer or day. Follow these to stay calibrated.

## 1. Anchor to evidence, not impression

Score each dimension against the "what 4 looks like" column in the rubric — and **quote
the line of the ticket that justifies the score**. If you can't point to text that earns
a 4, it isn't a 4. This single habit removes most drift.

## 2. Score independently, in order, then total

Score S1→S7 (or the E/D dimensions) **before** looking at the total, and don't let an
overall gut feeling pull individual scores toward it (halo effect). **The total is an
output, never an input.** Resist "this feels like a B, let me back into the numbers."

## 3. Use the tie-breaker rule

When torn between two scores, **pick the lower and state what would raise it.** This is
the biggest single source of reviewer-to-reviewer variance, so applying it consistently
matters more than which way any one call goes.

## 4. Common drift traps (and the correction)

- **Rewarding length.** A long story isn't a better story. Score clarity and scope (S6),
  not word count — verbose tickets often score lower.
- **Rewarding fluent format over real value.** A perfect "As a / I want / So that" with a
  hollow "so that" is S1 ≤ 2 and S2 ≤ 2, not a free 4. Check that the *why* is real.
- **Domain-expert blind spot.** If you can infer the missing repro step or acceptance
  criterion from your own knowledge, you'll over-score. Grade **what's written**, as a
  new team member would read it — the absence is the finding.
- **Severity/priority conflation (defects).** If both aren't set and distinguished, D5 is
  ≤ 2 even when the bug is well described.
- **Recency/contrast effect (batches).** A mediocre ticket right after a terrible one
  looks great. Re-anchor each ticket to the rubric, not to the previous one.

## 5. Self-consistency check

For any score, be ready to answer in one sentence: *"What specific change would move this
dimension up one point?"* If you can't, re-read the anchor and re-score — the score isn't
yet grounded.

## 6. Team calibration ritual (optional, for humans using this rubric)

To align multiple reviewers: have several people score the same 3–5 tickets
independently, then compare. Discuss only the dimensions that differ by **2+ points** —
those reveal genuine interpretation gaps. Capture the resolution as a one-line "house
rule" example. Re-run quarterly or when the team's notion of "ready" visibly drifts. A
handful of shared reference tickets (one clear A, one borderline C, one F) is the fastest
way to keep everyone anchored.

## Sources

Cite the specific principle **by name, with a link**, when you dock points, so the user
learns the rule — and can go read it — not just your verdict.

- **Role-goal-benefit / Connextra template & story basics** — Mike Cohn, *User Stories
  Applied*; Mountain Goat Software.
  [mountaingoatsoftware.com/agile/user-stories](https://www.mountaingoatsoftware.com/agile/user-stories)
- **3 C's (Card, Conversation, Confirmation)** — Ron Jeffries.
  [ronjeffries.com — Card, Conversation, Confirmation](https://ronjeffries.com/xprog/articles/expcardconversationconfirmation/)
- **INVEST** — Bill Wake, "INVEST in Good Stories, and SMART Tasks."
  [xp123.com/articles/invest-in-good-stories-and-smart-tasks](https://xp123.com/articles/invest-in-good-stories-and-smart-tasks/)
- **Given/When/Then (BDD)** — Dan North; Gherkin.
  [dannorth.net/introducing-bdd](https://dannorth.net/introducing-bdd/) ·
  [cucumber.io/docs/gherkin/reference](https://cucumber.io/docs/gherkin/reference/)
- **Story splitting — SPIDR** — Mike Cohn. Splitting patterns & flowchart — Richard
  Lawrence / Humanizing Work.
  [mountaingoatsoftware.com/agile/user-stories/splitting-user-stories](https://www.mountaingoatsoftware.com/agile/user-stories/splitting-user-stories) ·
  [humanizingwork.com — splitting user stories guide](https://www.humanizingwork.com/the-humanizing-work-guide-to-splitting-user-stories/)
- **Definition of Ready / Done & sprint sizing** — Scrum Guide (2020) and common practice
  (DoR is a widely used convention, not mandated by the Guide).
  [scrumguides.org/scrum-guide.html](https://scrumguides.org/scrum-guide.html)
- **Defect reporting / severity vs. priority** — established software-testing references
  (e.g., ISTQB glossary).
  [glossary.istqb.org](https://glossary.istqb.org/en/search/)
