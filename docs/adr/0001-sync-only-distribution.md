# Sync-only distribution (no Claude Marketplace)

**Status:** accepted

We distribute Skills to Claude Code, Codex, Cursor, and Pi solely via Sync (`skills-sync` placing into a canonical `.agents/skills` store, then linking Harness discovery dirs). Claude Code’s native Marketplace is out of scope for now.

Marketplace would give Claude better org-managed updates and allowlists, but dual channels (Marketplace + Sync) create duplicate installs, name collisions, and two support paths. One Sync path keeps onboarding and Versioning identical across all four Harnesses until Marketplace pain is real enough to revisit.
