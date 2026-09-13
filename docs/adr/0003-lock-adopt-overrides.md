# Lock File binds Versions; Adopt is explicit; Overrides are loud

**Status:** accepted

`skills.lock` PR-gates a pin for every Skill → Version and binds what the team should run (drift is not OK). Personal config selects which Skills and Harnesses are installed. Applying a lock change requires an explicit Adopt action in skills-sync. Optional personal Version Overrides live in `~/.config/skills-sync/overrides.toml`; they are shown every TUI launch; Adopt skips them until cleared.

**Considered options:**
- Lock as recommendation only — abandons drift prevention
- Silent auto-upgrade on sync — too easy to miss breaking changes
- No personal Overrides — blocks testing majors; deferred pain isn’t worth it if Overrides stay loud

Released to git (CI tag on merge) is not the same as Adopted by the team (lock bump + Adopt).
