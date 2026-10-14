# Lock File binds Versions; Adopt is explicit; Overrides are loud

**Status:** accepted

`skills.lock` PR-gates a pin for every Skill → Version and binds what the team should run (drift is not OK). Personal config selects which Skills and Harnesses are installed. Applying a lock change requires an explicit Adopt action in skills-sync. Adopt surfaces Migration Docs when a selected Skill crosses a major Version. Optional personal Version Overrides live in `~/.config/skills-sync/overrides.toml`; they are shown every TUI launch; Adopt skips them until cleared. CI runs `skills-sync validate-lock` so bad pins cannot merge unnoticed.

**Considered options:**
- Lock as recommendation only — abandons drift prevention
- Silent auto-upgrade on sync — too easy to miss breaking changes
- No personal Overrides — blocks testing majors; deferred pain isn’t worth it if Overrides stay loud

Released to git (CI tag on merge) is not the same as Adopted by the team (lock bump + Adopt).
