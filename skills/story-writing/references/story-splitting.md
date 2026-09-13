# Story Splitting Pattern Library (DE)

Loaded on demand by the `story-writing` skill. Read this when a story is estimated in
weeks, needs more than a handful of acceptance criteria, or otherwise fails INVEST's
**Small** — i.e. it should become an Epic and be split.

## Vertical vs. horizontal slicing

**Always slice vertically.** A vertical slice cuts through all architectural layers
(UI + logic + data) to deliver an observable change in behavior for a user.

**Horizontal slices — "build the DB", "build the API", "build the UI" as separate
stories — are the #1 splitting anti-pattern.** They fail INVEST's **Independent** and
**Valuable**, and demo nothing on their own. The teaching device: you eat a slice
through all layers of a cake, not one layer at a time. Splitting by technical component,
or by role only, are the classic decomposition anti-patterns.

**"API-only" isn't always a layer split — it depends on who the real consumer is.**
Contrast: (a) a story to ship a public API endpoint for an external/automation client
that only ever calls the API directly — that's a complete, demoable slice for its real
consumer, not a layer fragment. (b) a story to "build the API" for a human-facing
feature, where the only consumer of that API is a UI story that hasn't been built yet —
neither story is independently demoable, so this is the horizontal-slice anti-pattern
wearing an Interfaces costume. The test: **would this story's real consumer get full,
demoable value from it alone, with no sibling story required to exist first?** If the
only "consumer" is a sibling story, it's a layer split, not a vertical slice.

## The split workflow (Richard Lawrence / Humanizing Work flowchart)

1. Make sure the input story satisfies INVEST **except Small**.
2. Apply the first splitting pattern below that fits.
3. Evaluate the split by two rules of thumb:
   - Prefer the split that lets you **deprioritize / throw away** the low-value stories.
   - Prefer **more, equally-sized small stories**.

**Meta-pattern:** *focus on the complexity and reduce the variations through it* — find
the core complexity, identify its variations, and reduce them to a single case first.

## SPIDR (Mike Cohn)

> "Almost every story can be divided using one of the five techniques."

- **Spike** — split off time-boxed research to remove uncertainty (last resort).
- **Paths** — split the distinct paths through the story (e.g. each branch of a workflow).
- **Interfaces** — split by interface/client, where **each interface has its own real,
  distinct consumer** who gets complete value from that interface alone (e.g. ship the
  public API first because an external/automation client only ever calls it directly;
  ship the web UI first, mobile later). **Not valid** when "API" and "UI" are really
  just two halves of one single human-facing interaction — if the API-only story's only
  consumer is the UI story that hasn't been built yet, neither is independently
  demoable, and this collapses into the horizontal-slice anti-pattern above rather than
  a legitimate Interfaces split.
- **Data** — split by the data handled (one type/category/locale first).
- **Rules** — split by business rule (implement the simplest rule set first).

## Humanizing Work — nine patterns (apply the first that fits; spike is last resort)

1. **Workflow Steps** — do the simple end-to-end case first, add middle steps later.
   *e.g.* Ship "submit order → confirmation" before adding coupons, gift-wrap, split
   shipping.
2. **Operations (CRUD)** — the word **"manage"** signals multiple operations; split
   create / read / update / delete.
   *e.g.* "Manage saved filters" → create a filter; then list; then edit; then delete.
3. **Business Rule Variations** — each rule becomes its own story.
   *e.g.* "Flexible dates" search → exact dates first, then ±3 days, then whole month.
4. **Variations in Data** — split by data type/category/locale.
   *e.g.* Support US addresses first, then international address formats.
5. **Data Entry Methods** — simplest UI first, fancier UI later.
   *e.g.* Plain text field first, then autocomplete, then map picker.
6. **Major Effort** — isolate the big first piece from later variations.
   *e.g.* Build the payment integration for one card type; other cards ride on it after.
7. **Simple / Complex** — capture the simplest version; move every variation to its own
   story.
   *e.g.* "Estimate shipping" → flat rate first; weight/zone/carrier rules later.
8. **Defer Performance** — "make it work" then "make it fast" (works for any NFR).
   *e.g.* Return correct search results first; the sub-second-response story comes later.
9. **Break Out a Spike** — time-box research to remove uncertainty. **Last resort** —
   prefer splitting the real work over researching it.

## After splitting

Each split story must still be **INVEST** — it need not deliver the whole feature, but it
must deliver something usable and demonstrable. Defer pure technical work (backend
services, OpenAPI, DB indexing, schedules/Lambdas, infra) to **sub-tasks** under the
story/epic, not standalone stories. Keep every split story linked to its parent epic and
F-number.
