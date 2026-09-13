# Building and Distributing a Cross-Harness "Skills" Repository for Claude Code, Codex, Cursor, and Pi Agent

## TL;DR
- **Use a hybrid model.** Adopt the portable **Agent Skills** standard (`SKILL.md` = YAML frontmatter + markdown, optional `scripts/`/`references/`/`assets/`) as your single source of truth in one private Git repo, then serve Claude Code users through its **native plugin marketplace** (`.claude-plugin/marketplace.json`) and serve Codex, Cursor, and Pi Agent users through a thin **custom sync/install CLI** — because all four harnesses now natively read the same `SKILL.md` format, the CLI mostly places/symlinks folders rather than transforming them.
- **The format war is largely over in your favor.** Anthropic's Agent Skills was published as an open standard at agentskills.io on Dec 18, 2025 (the feature first launched Oct 16, 2025), and by March 2026 had been adopted by 32 tools including Claude Code, Codex, Cursor, and Pi — so one authored skill runs nearly everywhere. The differences that remain are *where* each tool looks for skills and optional per-tool metadata files, not the core format.
- **Recommended plumbing:** one repo with per-skill folders under a canonical `skills/` tree + a `.claude-plugin/marketplace.json` overlay for Claude Code; distribute to developers via a `git subtree`/plain-clone or a small sync CLI that symlinks selected skills into each tool's directory. Prefer symlinks + a sync CLI over git submodules for individual-machine distribution.

## Key Findings

### 1. Claude Code native private marketplace (as of 2026)
Claude Code has a first-class, documented plugin/marketplace system. The architecture has three nested concepts:
- **Skill** — a folder with `SKILL.md` (YAML frontmatter + markdown). Lives in `~/.claude/skills/` (personal), `.claude/skills/` (project, committed to the repo), a plugin's `skills/` directory, or an enterprise-managed location. Priority when names collide: enterprise > personal > project.
- **Plugin** — a bundle described by `.claude-plugin/plugin.json` that can contain skills, agents/subagents, slash commands, hooks, MCP servers, and LSP servers.
- **Marketplace** — a catalog described by `.claude-plugin/marketplace.json` at a repo root that lists plugins and where to fetch each one.

Users add a marketplace with `/plugin marketplace add owner/repo` (or a full git URL, or a local path) and install with `/plugin install <plugin>@<marketplace>`. Equivalent non-interactive CLI commands exist: `claude plugin marketplace add`, `claude plugin install`, `claude plugin marketplace update`, `claude plugin validate`. Skills from a plugin are namespaced (`/plugin-name:skill-name`); manually-installed skills in `.claude/skills/` coexist and keep their bare name.

**Private hosting works** on any git host (GitHub, GitLab, Bitbucket, self-hosted). Two routes:
1. **Git-credential route** — `/plugin marketplace add` uses your existing git credential helpers for private repos. There is a known limitation: background auto-updates disable credential helpers over HTTPS, so private-marketplace auto-updates can fail intermittently and fall back to a re-clone; teams mitigate with SSH remotes, a scoped git URL rewrite embedding a read-only token, or `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`. (A GitHub issue documents `claude plugin marketplace add` failing on private repos despite configured credentials — flag this as an active rough edge.)
2. **Organization settings route (Team/Enterprise)** — Admins add the marketplace under **Organization settings > Plugins**; org sync reads it via the Claude GitHub App or a GitHub Enterprise App, so individual git credentials aren't involved. Private plugin sources must either share the marketplace repo's owner or live on your GHE host; otherwise vendor them into the marketplace repo via relative-path sources (`./plugins/...`).

**Versioning:** For git sources, omitting `version` means every new commit is an update (simplest for internal repos); setting `version` in `plugin.json` pins users until you bump it. Source types include relative path, `github`, `url`, `git-subdir` (sparse clone of a monorepo subdir), `npm`, `archive` (zip over HTTPS with optional `sha256`), and `command`. Release channels (stable/latest) are done with two marketplaces pointing at different refs/SHAs.

**Auto-provisioning for teams:** Put the marketplace in `.claude/settings.json` under `extraKnownMarketplaces` and pre-enable plugins with `enabledPlugins`, so team members get it after trusting the folder. Admins can lock down sources with `strictKnownMarketplaces` in managed settings.

### 2. How the other harnesses handle skills/instructions

**The unifying fact:** All four target harnesses now natively support the **Agent Skills** `SKILL.md` standard. So does the ecosystem broadly — 32 tools by March 2026 per Paperclipped's interoperability analysis (Google's Gemini CLI, JetBrains' Junie, AWS's Kiro, Block's Goose, and others all read the same `SKILL.md` files), and agentskills.io's official client showcase lists on the order of 40 skills-compatible products by mid-2026. This means your existing skill format is portable with minimal transformation.

- **OpenAI Codex CLI** — Has native Agent Skills support. Official docs (developers.openai.com/codex/skills) document discovery from `.agents/skills` directories (repo CWD up to repo root), `$HOME/.agents/skills` (user), `/etc/codex/skills` (admin), plus OpenAI-bundled system skills. Skills use standard `SKILL.md` with `name`+`description` frontmatter. An optional per-skill `agents/openai.yaml` configures Codex app UI metadata, invocation policy (`allow_implicit_invocation`, default `true`), and tool/MCP dependencies. Install via the built-in `$skill-creator`, `$skill-installer <name>`, manual placement, or (recommended for team distribution) packaging as a **skill-only plugin** (the `openai/skills` catalog repo is now deprecated in favor of the OpenAI Plugins repo + Build plugins guide). Skills launched experimentally in December 2025 behind `codex --enable skills` / `[features].skills = true`; by mid-2026 the current docs present them as generally available with no flag, and `~/.codex/config.toml` is used only to enable/disable individual skills via `[[skills.config]]`. Codex also reads `AGENTS.md` for project instructions (global `~/.codex/AGENTS.md` + a hierarchy of repo/nested files, with an `AGENTS.override.md` escape hatch).
- **Cursor** — Native Agent Skills support shipped in Cursor 2.4 (a January 2026 release from Anysphere), whose changelog states: "We're excited to add Agent Skills to Cursor, an open standard for extending agents with domain-specific knowledge and workflows." Skills live in `.cursor/skills/<name>/SKILL.md` (project) or `~/.cursor/skills/` (personal), discovered recursively (nested `.cursor/skills/` or `.agents/skills/` folders scope to their subtree). Separately, Cursor **Rules** live in `.cursor/rules/*.mdc` (Markdown-with-frontmatter: `description`, `globs`, `alwaysApply`) — a *plain `.md` file there is silently ignored*. The legacy single-file `.cursorrules` still works but is deprecated. Cursor also has a team **marketplace** for publishing skills (personal skills in `~/.cursor/skills/` can be published to a team Default marketplace on Teams/Enterprise). Use Skills for procedures, Rules for always-on policy.
- **Pi Agent** — This is **pi** (pi.dev), a minimal, extensible terminal coding-agent harness by Mario Zechner (npm `@mariozechner/pi-coding-agent` / `@earendil-works/pi-coding-agent`; repo earendil-works/pi). It follows the Agent Skills `SKILL.md` format. Skills are discovered from `~/.pi/agent/skills/` (user), `.pi/skills/` (project), `.agents/skills/` (cwd up to repo root), and from installed **Pi packages** (npm/git packages declaring a `pi.skills` entry or a conventional `skills/` dir). Pi can also read skills from other tools' directories via a settings array (e.g. `"skills": ["~/.claude/skills", "~/.codex/skills"]`). Pi notably relaxes the standard's requirement that the skill name match the parent directory (deliberately, for shared skill dirs). Distribution: `pi install git:github.com/OWNER/repo`, `pi install npm:pkg`, or plain `git clone` into the skills dir; extras include TypeScript extensions, prompt templates, themes. Config root is `~/.pi/agent/` (mirrors Claude's `~/.claude`).
- **Cross-tool standards:** **AGENTS.md** (plain markdown, no schema) is the cross-tool instruction-file standard. It was released by OpenAI in August 2025 and is now stewarded by the Linux Foundation's Agentic AI Foundation (AAIF), formed Dec 9, 2025; per the Linux Foundation, "AGENTS.md has already been adopted by more than 60,000 open source projects and agent frameworks including Amp, Codex, Cursor, Devin, Factory, Gemini CLI, GitHub Copilot, Jules and VS Code among others." **Agent Skills / SKILL.md** is the cross-tool *skill* standard (agentskills.io); the reference `anthropics/skills` repo crossed 100K GitHub stars and Vercel's skills.sh marketplace listed 89,753 skills by March 2026. Together these cover "always-on instructions" (AGENTS.md) and "on-demand procedures" (SKILL.md).

### 3. Prior art: cross-harness sync/distribution tooling
A mature ecosystem of open-source "author once, project to every tool" CLIs exists:
- **Ruler** (`@intellectronica/ruler`) — single source of truth in `.ruler/`, `ruler apply` distributes concatenated rules to each agent's native config file; per-agent output configured in `ruler.toml`; propagates MCP config; automates `.gitignore`. Rules-focused (all agents get the same concatenated rules).
- **rulesync** (`rulesync` on npm) — generates/synchronizes rule, MCP, command, subagent, and skill files across 20+ tools from `.rulesync/`; supports `import`, `generate --targets "*" --features "*"`, and direct `convert --from cursor --to copilot,claudecode`. Has an explicit per-tool/per-feature support matrix (including Codex CLI global-only quirks).
- **agentsync** (spxrogers/agentsync), **agent_sync** (yelmuratoff), **agent-rules-sync** (dhruv-anand-aintech, a real-time daemon that syncs rules/skills/MCP), **AgentsMesh** (`agentsmesh generate`, explicitly lists Pi Agent as a target), **ai-rules-sync** (symlink-based, supports Cursor/Claude/Copilot/OpenCode/Codex/Gemini/Warp) — all solve the same fan-out with different tradeoffs (copy vs symlink, rules vs skills, one-shot vs daemon).
- **skill.sh / skills CLI** — a universal skill installer targeting Pi, Claude Code, Codex, Goose, Windsurf (`skill.sh install <agent> <url>` places the skill in the right per-tool dir).

**Design pattern for a custom CLI** (if you build your own rather than adopt one): a root manifest/index (e.g. `skills.toml`/`skills.json`) listing each skill and metadata; per-harness **adapter** functions that know each tool's target dir and optional metadata file (Claude → plugin/marketplace or `.claude/skills/`; Codex → `.agents/skills/` + optional `agents/openai.yaml`; Cursor → `.cursor/skills/`; Pi → `~/.pi/agent/skills/` or a Pi package); a `select` step letting a developer pick skills; and a `place` step that symlinks (or copies) the canonical folder into each target. Because the body format is shared, adapters mostly manage *location and optional sidecar metadata*, not frontmatter rewriting. Keep only the portable frontmatter fields (`name`, `description`) as the common denominator; agents ignore fields they don't understand.

**Distribution mechanics (submodule vs subtree vs sparse vs sync CLI):**
- **git submodule** — repo is a pointer to a pinned commit; minimal size impact; explicit version control; but requires `--recurse-submodules`, extra CI config, and is famously error-prone for casual contributors. Good when you want strict pinning and to edit/push upstream.
- **git subtree** — vendored copy embedded in a subdirectory; contributors need to learn nothing, works out-of-the-box in CI, `--squash` keeps history small; harder to push changes back upstream. Generally the easier choice for consuming a shared repo.
- **sparse checkout / `git-subdir`** — fetch only the needed subdirectories of a large monorepo; Claude Code's `git-subdir` plugin source and `--sparse` flag do exactly this.
- **custom sync CLI + symlinks** — most flexible for per-developer, per-tool selection and updates (`git pull` + re-link); symlinks give a single source of truth so edits propagate, but can break with tools that replace files rather than edit in place, and need care on Windows.

## Details

**Why the hybrid (not one tool for everything):** Claude Code's marketplace is the only harness with a rich, org-managed *distribution and versioning* system (auto-update, managed allowlists, org sync, release channels). The other three primarily just *discover* skills from directories. So the highest-leverage move is: (a) let Claude Code users consume via the native marketplace, and (b) give everyone else a lightweight installer that drops the same skill folders into the right place.

**Single source of truth vs generators:** Because SKILL.md is shared, you do NOT need heavy per-tool generation of the skill bodies. You need: (1) the canonical skill folders, (2) a Claude Code marketplace overlay, and (3) a placement mechanism for the other tools. This is much simpler than the rules-file world (CLAUDE.md vs `.cursor/rules/*.mdc` vs AGENTS.md), where tools like Ruler/rulesync genuinely transform formats.

**Handling frontmatter differences:** The safe common denominator is `name` + `description` + markdown body. Claude Code accepts the full six-field spec. Codex adds an optional `agents/openai.yaml` sidecar. Cursor supports `paths` scoping. Pi relaxes the name-matches-directory rule and adds fields like `disable-model-invocation`. Keep tool-specific extras as *additive sidecar files or optional frontmatter keys* that other tools ignore, rather than forking the body.

## Recommendations

**Stage 1 — Structure the repo (single source of truth).**
```
skills-repo/
├── .claude-plugin/
│   └── marketplace.json        # Claude Code catalog (overlay)
├── plugins/                    # optional: group skills into Claude plugins
│   └── <plugin>/.claude-plugin/plugin.json
├── skills/                     # CANONICAL: one folder per skill
│   └── <skill-name>/
│       ├── SKILL.md            # name + description + markdown (portable)
│       ├── scripts/            # optional
│       ├── references/         # optional
│       ├── assets/             # optional
│       └── agents/openai.yaml  # optional Codex sidecar
├── skills.toml                 # root manifest/index for your sync CLI
└── install.sh / bin/skills     # custom sync CLI
```
Point Claude plugin entries at the shared `skills/` folder using per-subdirectory `skills` paths so each plugin loads only its own skills. Keep `version` omitted for internal fast iteration (commit = update) or bump per release for pinning.

**Stage 2 — Claude Code users:** Publish the marketplace. For a small team, `/plugin marketplace add your-org/skills-repo` + `/plugin install`. For an org on Team/Enterprise, distribute via **Organization settings > Plugins** (avoids per-user git-credential issues) and/or ship `extraKnownMarketplaces` + `enabledPlugins` in a project `.claude/settings.json`. Validate with `claude plugin validate .` in CI.

**Stage 3 — Codex / Cursor / Pi users:** Don't hand-roll transformation you don't need. First **evaluate adopting rulesync or AgentsMesh** (both already target these tools and handle skills). If you need finer control (developer picks a subset), build a small CLI that: reads `skills.toml`, prompts for selection, and symlinks selected `skills/<name>/` into the appropriate `.agents/skills/` location for Codex, `.cursor/skills/` for Cursor, and `~/.pi/agent/skills/` for Pi (or generates a Pi package / `pi install git:` target). Add `agents/openai.yaml` only where a skill needs Codex UI metadata or MCP deps.

**Stage 4 — Updates:** For consumers, prefer **plain clone or `git subtree` + a `skills update` command** (git pull then re-link) over submodules — less friction, no CI special-casing. Reserve submodules for cases needing strict commit pinning. Claude Code users get updates via `/plugin marketplace update` (or auto-update where private-repo auth allows).

**Thresholds that change the recommendation:**
- If you're on **Claude Enterprise**, lean harder on org-managed marketplace distribution and `strictKnownMarketplaces` — it removes the private-repo auth friction entirely.
- If **>2 non-Claude tools** are in heavy use and you also need rules/MCP/subagents synced (not just skills), adopt **rulesync** as the backbone instead of a bespoke CLI.
- If skills grow large or binary-heavy, switch Claude sources to `git-subdir`/`archive` and use sparse checkout for consumers.

## TUI Mockup — Ratatui-style skill selector

The custom sync CLI from Stage 3 is a natural fit for a [Ratatui](https://ratatui.rs) terminal interface (Rust, immediate-mode rendering, bordered widgets, constraint-based layout). Below is a mockup of what `skills-tui` could look like, using Ratatui's typical visual language: bordered `Block` panels with titles, a `List` with checkbox-style selection, a side panel for skill details, and a bottom status/gauge bar.

```
┌ skills-tui — org/skills-repo ────────────────────────────────────────────────┐
│ ┌ Skills (12) ───────────────────┐ ┌ Details ───────────────────────────────┐│
│ │ [x] docx                       │ │ docx                                   ││
│ │ [x] pptx                       │ │ ───────────────────────────────────    ││
│ │ [ ] xlsx                       │ │ Create, read, and edit Word documents. ││
│ │ [x] pdf                        │ │ Handles headers, tables, tracked       ││
│ │ >>[x] frontend-design<<        │ │ changes, and templates (.dotx).        ││
│ │ [ ] product-self-knowledge     │ │                                         ││
│ │ [x] file-reading                │ │ Path:    skills/docx/SKILL.md          ││
│ │ [ ] pdf-reading                 │ │ Version: 1.4.0  (unpinned → tracks HEAD)││
│ │ [x] data-analysis               │ │ Targets: ✔ claude ✔ codex ✔ cursor ✔ pi ││
│ │ [ ] skill-creator               │ │                                         ││
│ │ [x] morning                     │ │ ┌ Sync preview ──────────────────────┐ ││
│ │ [ ] import-memory                │ │ │ → .agents/skills/docx (symlink)    │ ││
│ │                                  │ │ │ → .cursor/skills/docx (symlink)    │ ││
│ │                                  │ │ │ → .claude/skills/docx (symlink)    │ ││
│ │                                  │ │ │ → ~/.pi/agent/skills/docx (symlink)│ ││
│ │                                  │ │ └─────────────────────────────────────┘││
│ └──────────────────────────────────┘ └─────────────────────────────────────┘│
│ ┌ Harnesses ─────────────────────────────────────────────────────────────────┐│
│ │ [x] claude   [x] codex   [x] cursor   [x] pi                               ││
│ └──────────────────────────────────────────────────────────────────────────┘│
│ Sync status ████████████████████████████████████████░░░░░░░░  7/12 linked   │
│ ↑↓ move · space toggle skill · tab switch pane · h toggle harness · s sync · q quit│
└────────────────────────────────────────────────────────────────────────────┘
```

**Widget mapping to Ratatui primitives**, if this were implemented for real:
- Outer frame + each labeled panel → `Block::bordered().title(...)`, arranged with `Layout` using a vertical split (main area / harness row / status bar) and a horizontal split within the main area (skills list / details).
- `[x]`/`[ ]` skill list → `List` + `ListItem`s with a `Stylize` highlight (`.on_blue()` or `.reversed()`) for the selected row, and a custom `HighlightSymbol` (`>> <<` above, or Ratatui's default `> `).
- Harness toggle row → a small `Tabs` or a manually rendered row of `Span`s, each `.green()` when enabled and `.dim()` when disabled.
- Sync status bar → `Gauge::default().block(Block::bordered().title("Sync status")).ratio(7.0/12.0)`.
- Details pane → `Paragraph` with `Wrap { trim: true }`, plus a nested bordered `Block` for the "Sync preview" sub-panel.
- Footer key-hints → a plain `Line`/`Paragraph` styled `.dim()`, the idiomatic Ratatui footer pattern seen across the showcase apps linked from ratatui.rs (e.g. `csvlens`, `oxker`, `binsider`).

This is presented as a static mockup for the document; wiring it up would mean a small `ratatui` + `crossterm` binary that reads `skills.toml`, renders this layout, and on `s` invokes the same placement/symlink logic described in Stage 3.

## Caveats
- **Rapidly evolving / beta flags.** Codex Skills graduated from an experimental flag (`codex --enable skills`, Dec 2025) to GA during 2026; Cursor Skills are recent (2.4, Jan 2026); Pi is a young, fast-moving minimal harness. Re-verify exact directory names and flags against each tool's live docs before standardizing.
- **Claude Code private-marketplace auth is a known rough edge** — background auto-updates over HTTPS can fail on private repos, and a GitHub issue reports `plugin marketplace add` failing despite valid credentials. Prefer SSH remotes, org-settings distribution, or a scoped URL-rewrite token.
- **Directory-name conflicts across sources.** Codex's officially-documented discovery path is `.agents/skills` (not `.codex/skills`); some third-party repos claim a `.codex/skills` precedence that is not in official docs. `~/.codex/config.toml` is used for enable/disable, not discovery.
- **Third-party sync tools vary in maturity** (agentsync is beta v0.1.0; several are one-maintainer projects). Vet before depending on them in CI.
- **Treat third-party skills as executable code.** A Snyk audit of 3,984 public skills (Feb 2026, cited by Articsledge) found 36.82% contained at least one security flaw; skills can run arbitrary scripts, so review before ingesting external skills into your shared repo.
- **Windows/symlink caveats** and tools that rewrite-rather-than-edit files can silently break symlink-based sync; test your placement mechanism on every OS your team uses.
- Some adoption/star counts come from secondary or marketing sources and should be treated as directional rather than exact.

