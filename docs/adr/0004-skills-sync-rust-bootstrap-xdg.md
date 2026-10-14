# Custom Rust skills-sync, bootstrap binary, XDG paths

**Status:** accepted

We build a custom Rust CLI + full TUI (`skills-sync`) rather than adopting rulesync/AgentsMesh. Onboarding uses a bootstrap script: download a GitHub Releases binary (`skills-sync-<os>-<arch>` on `skills-sync-v*` tags), clone this repo into cache, seed personal config, then `sync` (TUI optional). Config lives under `~/.config/skills-sync/`; clone and extracted `name@semver` trees under `~/.cache/skills-sync/`. Placement writes a canonical `.agents/skills` store first (symlink by default, copy fallback), then links other Harness dirs to it; Skill `bin/` scripts link into `~/.local/bin`. Per Sync run, exactly one Placement Scope (user or project, not both). Lifecycle commands include `list`, `remove`, and `validate-lock`.

**Considered options:**
- Adopt rulesync/AgentsMesh — fine for multi-feature fan-out; we need lock/Adopt/tag extraction and a product TUI, so a thin wrapper would still grow into a custom tool
- `cargo install` for everyone — fails &lt;5 min onboarding for a mixed team
- Marketplace-primary Claude path — rejected in ADR-0001; Sync must serve all four Harnesses
- Fan-out only into each Harness dir (no canonical store) — harder updates and diverges from the ecosystem `.agents/skills` convention
- Content-hash install locks (`npx skills`) — unfit for team semver Adopt; we keep tag-backed Versions
