---
name: confluence
description: Read, search, and publish Confluence wiki pages for Sophos using curl+jq shell scripts. Use this when working with Confluence pages, wiki documentation, spaces, or knowledge base content, or when publishing a report/page to Confluence.
version: 1.0.0
---

# Confluence Integration

Shell scripts for reading Confluence wiki pages via the REST API. They live in
this skill's `bin/` folder and are symlinked into `~/.local/bin/` by
`skills-sync sync`, so they resolve by bare name on your `$PATH`.

**Endpoint:** `https://sophos.atlassian.net/wiki`
**Auth:** Basic auth using `JIRA_API_TOKEN` env var (same Atlassian token as Jira)

## Available Commands

### Search Pages

```bash
# Search for pages by text
confluence-search "deployment guide"

# Search within a specific space
confluence-search "runbook" --space DE

# Limit results
confluence-search "architecture" --limit 10
```

Returns: ID, title, space, and last updated date.

### Read a Page

```bash
# Read by page ID
confluence-page 123456789

# Read by exact title (with optional space filter)
confluence-page "Deployment Guide" --space DE
```

Shows page metadata (title, space, version, author, URL) followed by the page content as plain text.

### List Child Pages

```bash
confluence-children 123456789
```

Lists all child pages under a given parent page ID.

### List Spaces

```bash
# List all global spaces
confluence-spaces

# Filter spaces by name or key
confluence-spaces "MDR"
```

### Open in Browser

```bash
confluence-browse 123456789
```

### Publish a Page (create or update)

Publishes a page from a storage-format (XHTML) body file — this is what the
agile-ceremony skills use to push sprint reviews, retros, and sprint goals to
Confluence. Uses the Confluence Cloud v2 API and handles version increments on
update automatically.

```bash
# Create a new page under a parent
confluence-publish create --space-key DE --parent 123456789 \
  --title "Sprint 42 Retro" --body-file /path/to/body.html

# Update an existing page (version bump handled for you)
confluence-publish update --page-id 987654321 \
  --title "Sprint 42 Retro" --body-file /path/to/body.html
```

Prints the resulting page id, version, and URL. Override the author with
`CONFLUENCE_USER` if needed. The body file must be Confluence **storage
format** (XHTML), not Markdown — render to storage format first.

## Ad-hoc API Queries

For queries not covered by the scripts above, use curl directly:

```bash
curl -s -u "jim.salinas@sophos.com:$JIRA_API_TOKEN" \
  -H "Accept: application/json" \
  -G "https://sophos.atlassian.net/wiki/rest/api/content/search" \
  --data-urlencode "cql=type = page AND text ~ \"your search\"" \
  --data-urlencode "limit=10" | jq .
```

### Useful CQL Examples

```
# Pages in a specific space
type = page AND space = "DE"

# Pages updated recently
type = page AND lastModified > now("-7d")

# Pages by a specific author
type = page AND creator = "jim.salinas"

# Pages with a specific label
type = page AND label = "runbook"
```

## Notes

- Uses the same `JIRA_API_TOKEN` as the Jira skill (Atlassian tokens are tenant-wide).
- Page content is returned as Confluence storage format (XHTML) and converted to plain text by stripping HTML tags.
- The `confluence-page` command accepts either a numeric page ID or an exact page title.
- CQL (Confluence Query Language) is used for search queries, similar to JQL for Jira.
- `confluence-publish` sends **storage format** (XHTML), not Markdown. The ceremony skills render HTML → storage format before publishing.
