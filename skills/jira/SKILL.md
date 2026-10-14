---
name: jira
description: Manage Jira issues for Sophos using curl+jq shell scripts. Use this when working with Jira issues, sprints, tickets, assignments, transitions, filters, or queries for the DE project.
version: 1.0.0
---

# Jira Integration

Shell scripts for managing Jira issues via the REST API v3. They live in this
skill's `bin/` folder and are symlinked into `~/.local/bin/` by `skills-sync sync`,
so they resolve by bare name on your `$PATH`.

**Endpoint:** `https://sophos.atlassian.net`
**Default project:** DE
**Auth:** Basic auth using `JIRA_API_TOKEN` env var

## Available Commands

### List Issues

```bash
# Active sprint issues for project DE
jira-sprint

# My unresolved issues
jira-mine
```

### Run Saved Filter

```bash
# Run a saved Jira filter by name (partial match)
jira-filter "EAP Scope"
```

Searches for saved filters matching the given name. If exactly one filter matches, executes its JQL and displays results. If multiple filters match, lists them so the user can refine.

### View Issue Details

```bash
jira-view DE-123
```

Shows: key, summary, status, priority, type, assignee, reporter, story points, description, and last 3 comments.

### Add Comment

```bash
jira-comment DE-123 "This is my comment"
```

### Transition Issue Status

```bash
jira-transition "In Progress" DE-123
jira-transition "Done" DE-123
```

Shows available transitions if the requested status doesn't match.

### Assign Issue

```bash
jira-assign DE-123 user@sophos.com
```

Resolves email to Jira accountId automatically.

### Add / Remove Labels

```bash
jira-label DE-123 --add needs-elaboration,needs-ux
jira-label DE-123 --remove stale-label
jira-label DE-123 --add needs-pm-feedback --remove needs-elaboration
```

Adds and/or removes labels in a single PUT to the issue. At least one of `--add` or `--remove` is required; both accept a comma-separated list.

### Create Issue

```bash
jira-create --summary "Fix login bug" --type Bug --description "Users can't log in"
jira-create --summary "New feature" --project DE --type Story
```

Options: `--summary` (required), `--project` (default: DE), `--type` (default: Task), `--description` (optional).

### Create a DE Bug With Structured Fields

For DE Bugs, prefer the REST API over `jira-create` when the description needs headings,
lists, acceptance criteria, or other structure. `jira-create --description` is useful for
simple text, but complex descriptions can flatten or lose useful formatting.

Before creating a Bug, check the create metadata and any related bug you are mirroring:

```bash
curl -s -u "jim.salinas@sophos.com:$JIRA_API_TOKEN" \
  -H "Accept: application/json" \
  "https://sophos.atlassian.net/rest/api/3/issue/createmeta?projectKeys=DE&issuetypeNames=Bug&expand=projects.issuetypes.fields" \
  | jq -r '.projects[0].issuetypes[0].fields
    | to_entries[]
    | select(.value.required == true or (.value.name | test("Severity|Visibility"; "i")))
    | [.key, .value.name, (.value.allowedValues // [] | map(.name // .value // .id) | join(", "))]
    | @tsv'
```

DE validators may require fields that `createmeta` does not mark as required. Known DE Bug
fields:

- `customfield_10316` — Severity. Options observed: `Very High`, `High`, `Medium`, `Low`.
- `customfield_10317` — Visibility. Options observed: `High`, `Medium`, `Low`.

If filing a follow-up to an existing bug, read that issue's values and mirror them when the
impact is similar:

```bash
curl -s -u "jim.salinas@sophos.com:$JIRA_API_TOKEN" \
  -H "Accept: application/json" \
  "https://sophos.atlassian.net/rest/api/3/issue/DE-12345?fields=priority,customfield_10316,customfield_10317" \
  | jq '.fields | {priority:.priority.name, severity:.customfield_10316, visibility:.customfield_10317}'
```

Build the create payload in `/private/tmp` and POST it with `--data @file`; this avoids
fragile inline JSON and shell quoting:

```bash
curl -s -w "\nHTTP:%{http_code}\n" \
  -u "jim.salinas@sophos.com:$JIRA_API_TOKEN" \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -X POST \
  --data @/private/tmp/de-bug-create.json \
  "https://sophos.atlassian.net/rest/api/3/issue"
```

After creating a bug, verify it instead of assuming the request body landed correctly:

```bash
jira-view DE-12345
curl -s -u "jim.salinas@sophos.com:$JIRA_API_TOKEN" \
  -H "Accept: application/json" \
  "https://sophos.atlassian.net/rest/api/3/issue/DE-12345?fields=summary,issuetype,priority,labels,issuelinks,customfield_10316,customfield_10317" \
  | jq '{summary:.fields.summary, type:.fields.issuetype.name, priority:.fields.priority.name, labels:.fields.labels, severity:.fields.customfield_10316.value, visibility:.fields.customfield_10317.value, links:.fields.issuelinks}'
```

To link a follow-up bug, use the issue-link API and verify the link after creation:

```bash
curl -s -w "\nHTTP:%{http_code}\n" \
  -u "jim.salinas@sophos.com:$JIRA_API_TOKEN" \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -X POST \
  --data @/private/tmp/de-bug-link.json \
  "https://sophos.atlassian.net/rest/api/3/issueLink"
```

### Open in Browser

```bash
jira-browse DE-123
```

## Agile / Scrum queries

These back the agile-ceremony skills (standup, sprint-planning, sprint-retro,
etc.). All accept `--json` for machine-readable output that the ceremony skills
consume; without it they print a table.

```bash
# Active-sprint issues as JSON (feeds standup / sprint-health / sprint-review)
jira-sprint --json

# Rolling velocity + say-do ratio for a board (feeds sprint-planning / retro)
jira-velocity <boardId> --sprints 3 --json

# Work added or removed after sprint start (feeds sprint-health / retro)
jira-scope-creep <boardId> --json

# Backlog items needing refinement: unestimated / unassigned / stale
jira-grooming --project DE --stale-days 30 --json

# Per-issue lead / cycle / review time from the changelog (feeds flow-metrics)
jira-cycle-time --sprint --json
jira-cycle-time --jql "project = DE AND resolved >= -14d" --json

# One person's recent activity (feeds one-on-one-prep)
jira-activity user@sophos.com --since -14d --json

# Ticket -> pull-request map from the dev panel (feeds flow-metrics)
jira-pr-activity --sprint --json                      # discovery only
jira-pr-activity --jql "project = DE AND resolved >= -14d" --timing --json
```

**Finding a board id** (needed by `jira-velocity` / `jira-scope-creep`):

```bash
curl -s -u "jim.salinas@sophos.com:$JIRA_API_TOKEN" \
  -H "Accept: application/json" \
  "https://sophos.atlassian.net/rest/agile/1.0/board?projectKeyOrId=DE" \
  | jq '.values[] | {id, name}'
```

> Velocity/scope-creep read committed-vs-completed points from the Greenhopper
> sprint-report endpoint (`/rest/greenhopper/1.0/rapid/charts/sprintreport`),
> falling back to summing `customfield_10016` over the Agile API if that
> endpoint is unavailable.
>
> `jira-pr-activity` reads the **development panel** (`/rest/dev-status/1.0`),
> which links PRs to tickets across every repo they touched. That data carries
> only a single `lastUpdate` per PR, so `--timing` enriches each PR via `gh`
> with real open/merge/first-review timestamps.

## Ad-hoc API Queries

For queries not covered by the scripts above, use curl directly:

```bash
curl -s -u "jim.salinas@sophos.com:$JIRA_API_TOKEN" \
  -H "Accept: application/json" \
  -G "https://sophos.atlassian.net/rest/api/3/search/jql" \
  --data-urlencode "jql=YOUR JQL HERE" \
  --data-urlencode "fields=summary,status" | jq .
```

### Useful JQL Examples

```
# Issues updated in the last day
project = DE AND updated >= -1d

# Unassigned bugs
project = DE AND type = Bug AND assignee is EMPTY

# Issues in code review
project = DE AND status = "Code Review"

# Sprint issues by status
project = DE AND sprint in openSprints() AND status = "In Progress"
```

## Setup

1. Generate an API token at https://id.atlassian.com/manage-profile/security/api-tokens
2. Add to your shell profile: `export JIRA_API_TOKEN="your-token-here"`
3. Ensure `~/.local/bin` is in your PATH

## Notes

- Story points use `customfield_10016` (Jira Cloud default). If your instance uses a different field, update the scripts.
- The v3 API uses Atlassian Document Format (ADF) for rich text fields. The scripts handle this automatically for comments and descriptions.
- All scripts exit with code 1 on error and print diagnostics to stderr.
