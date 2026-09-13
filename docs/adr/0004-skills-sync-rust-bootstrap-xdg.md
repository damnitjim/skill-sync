# Custom Rust skills-sync, bootstrap binary, XDG paths

**Status:** accepted

We build a custom Rust CLI + full TUI (`skills-sync`) rather than adopting rulesync/AgentsMesh. Onboarding uses a bootstrap script: download a GitHub Releases binary, clone this repo into cache, launch the TUI. Config lives under `~/.config/skills-sync/`; clone and extracted `name@semver` trees under `~/.cache/skills-sync/`. Placement is symlink by default with copy fallback when symlinks fail. Per Harness, Sync writes exactly one Placement Scope (user or project, not both).

**Considered options:**
- Adopt rulesync/AgentsMesh — fine for multi-feature fan-out; we need lock/Adopt/tag extraction and a product TUI, so a thin wrapper would still grow into a custom tool
- `cargo install` for everyone — fails &lt;5 min onboarding for a mixed team
- Marketplace-primary Claude path — rejected in ADR-0001; Sync must serve all four Harnesses
