# Defect / Bug Reporting Standard (DE)

Loaded on demand by the `story-writing` skill. Read this whenever writing or reviewing a
defect in the DE project.

A **defect is a discrepancy between actual and expected behavior.** A good report is the
most-read artifact in triage — write it so someone else can reproduce and fix without
asking you a single question.

## Bug report template

| Field | Guidance |
|---|---|
| **Title** | Specific and descriptive: *component + problem + context*. Not "search broken" → "Search button misaligned on Safari 16 results page". This is the most-read part. |
| **Environment / build** | OS, browser/device, version/build number, environment (staging/prod). |
| **Preconditions** | Any state/setup needed before the steps (account type, feature flag, seeded data). |
| **Steps to reproduce** | Numbered, minimal, exact. Start from a known entry point (first step = the URL/screen). |
| **Expected result** | What *should* happen — detailed, not a negation of "actual". |
| **Actual result** | What actually happened — detailed, observed facts; **not** a restatement of the title. |
| **Severity** | Technical impact on functionality (set by reporter/QA; objective). See scale below. |
| **Priority** | Business urgency of the fix (set with PO/manager; subjective). See scale below. |
| **Evidence** | Screenshot / video / logs — **at least one is mandatory**. |
| **Reproducibility** | Frequency (e.g. "12/12 reproduced"). Log intermittent bugs anyway and say so. |
| **Links** | Originating story/epic; F-number if applicable. |

## Severity vs. Priority — independent axes

These are **not** the same thing and must be set independently.

- **Severity** = technical impact on the system/functionality. Objective, set by the
  reporter/QA. Scale: **Critical / Major / Moderate / Minor / Cosmetic.**
- **Priority** = business urgency of the fix. Subjective, set with the PO/manager.
  Scale: **High / Medium / Low.**

Worked examples showing they move independently:

| Defect | Severity | Priority | Why |
|---|---|---|---|
| Misspelled company name on the homepage | Low (cosmetic) | **High** | Brand/legal impact; embarrassing, but nothing is functionally broken. |
| Crash in a rarely-used feature, no workaround | **High** | Low | Serious when hit, but almost nobody hits it. |
| Cannot complete checkout / login | **High** | **High** | Core flow blocked; fix now. |

> The scales are industry **conventions**, not an IEEE/ISO standard. The point is to
> pick one and apply it **consistently** in DE.

### DE Jira field names

Do not confuse the defect-writing concepts with the DE Jira field labels:

- **Jira Priority** is the business urgency field used by DE, with values such as
  `Major`, `Minor`, and `Critical`.
- **Jira Severity** is a DE custom field, currently `customfield_10316`, with values
  such as `Very High`, `High`, `Medium`, and `Low`.
- **Jira Visibility** is a DE custom field, currently `customfield_10317`, with values
  such as `High`, `Medium`, and `Low`.

When creating a DE Bug, use the Jira skill to query the current field metadata and, for
follow-up defects, mirror severity/visibility from the related bug when the impact is
similar.

## RBAC / permissions defects

For access-control bugs, include the permission delta explicitly. Reviewers should not
have to infer it from code or follow-up comments.

Include:

- The upstream source of truth, such as the access service or identity group mapping.
- The downstream permission gate, such as UI role validation, route guards, or sidebar
  visibility.
- The role or group name before the fix and the role or group name after the fix.
- Permissions the role already had before the fix.
- Permissions added by the fix.
- Permissions intentionally still denied by the fix.
- Tests or evidence that prove both the allowed and denied paths.

For role-name drift, state both names clearly. Example: "The UI accepted
`infosec-engineer`, but the access service returns `infosec-engineers`, so the UI could
filter the role out before applying view permissions."

## Bug vs. story decision

- **Found before the story is Done** and it fails (or should have had) acceptance
  criteria → **fix inside the story.** Don't spawn a ticket — it's just incomplete work.
  Update the AC and talk to the PO.
- **Create a separate defect ticket only when all three hold** (AgileAmbition's
  three-part test):
  1. it **won't block release** of the original item, **and**
  2. the effort to fix **exceeds** the effort to create the ticket, **and**
  3. it **needn't be fixed this sprint**.
- **Defects in released/production work** → always their own backlog item, **linked to
  the origin** story/epic for traceability.
- **Acceptance criteria for bugs:** for anything non-trivial, **yes** — express the fix
  as the correct expected behavior the user wants.
  > As a user / I want case search to return accurate results / so that I don't waste
  > time on irrelevant cases.

## Common mistakes (reject in review)

- Vague titles ("search broken", "doesn't work").
- Missing repro steps or missing environment/build.
- **Multiple bugs in one ticket** — one bug per ticket.
- Guessed root causes stated as fact instead of observed behavior.
- Opinions, blame, or emotional language.
- Expected and Actual being mirror-images of each other (both just restate the title).
- No evidence attached.
