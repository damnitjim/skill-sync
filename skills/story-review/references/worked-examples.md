# Worked Examples

Loaded on demand by the `story-review` skill. These show the full **scorecard → teaching
notes → rewrite** flow end to end. Model your reviews on them.

## Example 1 — a weak story, scored and taught

**Submitted:**

> As a user I want to sort cases so that I can sort them. Should work well.

**Scorecard:** S1 2 · S2 1 · S3 1 · S4 3 · S5 3 · S6 2 · S7 1

= (2/4·15)+(1/4·15)+(1/4·20)+(3/4·15)+(3/4·10)+(2/4·15)+(1/4·10)
= 7.5 + 3.75 + 5 + 11.25 + 7.5 + 7.5 + 2.5 = **45 → F**.

**Fastest win:** give the "so that" a real benefit and add testable acceptance criteria.

**Teaching notes (excerpt):**

- **S1 (2):** "so that I can sort them" restates the action. Name the payoff.
  *([Connextra template](https://www.mountaingoatsoftware.com/agile/user-stories) — the
  "so that" is the value, not the mechanics.)*
- **S2 (1):** no genuine business value stated, so a dev can't make trade-offs.
- **S3 (1):** "Should work well" isn't testable. Here's a real criterion:
  > Given a case list with mixed priorities, When I sort by priority descending, Then
  > cases appear High → Low and the sort indicator shows on the Priority column.
- **S7 (1):** no link to a parent epic or requirement, and "As a user" isn't a real
  persona.

**Rewritten version:**

> **As a** case manager **I want** to sort my case list by priority **so that** I can
> work the most critical cases first.
>
> - **AC1** — Given a list of cases, When I sort by priority descending, Then they order
>   High → Medium → Low and the Priority column shows the sort indicator.
> - **AC2** — Given an empty list, When I apply a sort, Then I see the empty state and no
>   error.
>
> Links: parent Epic DE-xx / requirement F3.1.

## Example 2 — a defect showing severity vs. priority

**Submitted:**

> Login broken, fix asap!!

**Scorecard:** D1 1 · D2 1 · D3 1 · D4 1 · D5 1 · D6 1 · D7 3 → **~20 → F**.

**Fastest win:** add numbered repro steps and state expected vs. actual explicitly — a
bug that can't be reproduced can't be fixed.

**Teaching notes (excerpt):**

- **D1 (1):** "Login broken" isn't searchable. Say what fails, where, and under what
  condition.
- **D2/D3 (1/1):** no repro steps and no expected-vs-actual, so the gap is undefined.
- **D5 (1):** "asap!!" conflates urgency with impact. Set **Severity** (impact) and
  **Priority** (urgency) separately.
  *([ISTQB glossary](https://glossary.istqb.org/en/search/) — severity vs. priority are
  distinct axes.)*

**Rewritten version:**

> **Title:** Login fails with "500" for SSO users on Safari 17 (v4.8.0)
> **Steps:** 1) Safari 17.x  2) go to `/login`  3) click "Sign in with SSO"  4) enter
> valid creds  5) submit.
> **Expected:** redirected to dashboard. **Actual:** "500 Server Error"; not logged in.
> **Env:** macOS 14.5, Safari 17.4, prod build v4.8.0, SSO accounts only.
> **Severity:** High (blocks all SSO login). **Priority:** High (affects most users).
> **Evidence:** screenshot + request ID 8f2c… **Frequency:** always (5/5).
