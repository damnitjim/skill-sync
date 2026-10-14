use std::path::PathBuf;

use crate::git_cache;
use crate::lock::LockFile;
use crate::personal::{OverridesFile, PersonalConfig};
use crate::place;
use crate::skill_meta;
use crate::Error;

#[derive(Debug, Clone)]
pub struct SyncPaths {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub home_dir: PathBuf,
    pub project_dir: PathBuf,
    pub library_clone: PathBuf,
    pub force_copy: bool,
}

impl SyncPaths {
    pub fn xdg() -> Result<Self, Error> {
        let home = dirs::home_dir().ok_or_else(|| Error::msg("no home directory"))?;
        let config_dir = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"))
            .join("skills-sync");
        let cache_dir = std::env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".cache"))
            .join("skills-sync");
        let project = std::env::current_dir()?;
        Ok(Self {
            config_dir,
            cache_dir: cache_dir.clone(),
            home_dir: home,
            project_dir: project,
            library_clone: cache_dir.join("library"),
            force_copy: false,
        })
    }

    pub fn lock_path(&self) -> PathBuf {
        self.library_clone.join("skills.lock")
    }

    pub fn personal_path(&self) -> PathBuf {
        self.config_dir.join("personal.toml")
    }

    pub fn overrides_path(&self) -> PathBuf {
        self.config_dir.join("overrides.toml")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Harness {
    ClaudeCode,
    Codex,
    Cursor,
    Pi,
}

impl Harness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ClaudeCode => "claude-code",
            Self::Codex => "codex",
            Self::Cursor => "cursor",
            Self::Pi => "pi",
        }
    }

    pub fn parse(s: &str) -> Result<Self, Error> {
        match s {
            "claude-code" | "claude" => Ok(Self::ClaudeCode),
            "codex" => Ok(Self::Codex),
            "cursor" => Ok(Self::Cursor),
            "pi" => Ok(Self::Pi),
            other => Err(Error::msg(format!("unknown harness: {other}"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementScope {
    User,
    Project,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WantedSkill {
    pub name: String,
    pub version: String,
    pub from_override: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementKind {
    Symlink,
    Copy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementRecord {
    pub skill: String,
    pub harness: String,
    pub path: PathBuf,
    pub kind: PlacementKind,
}

#[derive(Debug, Clone, Default)]
pub struct SyncReport {
    pub placements: Vec<PlacementRecord>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationNotice {
    pub skill: String,
    pub from_version: String,
    pub to_version: String,
    pub summary: String,
}

#[derive(Debug, Clone, Default)]
pub struct AdoptReport {
    pub updated: Vec<String>,
    pub skipped_overrides: Vec<String>,
    pub migrations: Vec<MigrationNotice>,
    pub sync: SyncReport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverridePin {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct SkillStatus {
    pub name: String,
    pub lock_version: String,
    pub effective_version: String,
    pub from_override: bool,
    pub deprecated: bool,
    pub successor: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Default)]
pub struct StatusReport {
    pub skills: Vec<SkillStatus>,
    pub overrides: Vec<OverridePin>,
    pub harnesses: Vec<String>,
    pub placement_scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedPlacement {
    pub harness: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedSkill {
    pub name: String,
    pub version: String,
    pub placements: Vec<ListedPlacement>,
}

pub struct SkillsSyncInner {
    pub paths: SyncPaths,
}

impl SkillsSyncInner {
    pub fn new(paths: SyncPaths) -> Self {
        Self { paths }
    }

    pub fn resolve_wanted(&self) -> Result<Vec<WantedSkill>, Error> {
        let lock = LockFile::load(&self.paths.lock_path())?;
        let personal = PersonalConfig::load(&self.paths.personal_path())?;
        let overrides = OverridesFile::load(&self.paths.overrides_path())?;

        let mut wanted = Vec::new();
        for name in &personal.skills {
            let (version, from_override) = if let Some(v) = overrides.overrides.get(name) {
                (v.clone(), true)
            } else if let Some(v) = lock.skills.get(name) {
                (v.clone(), false)
            } else {
                return Err(Error::msg(format!(
                    "skill '{name}' enabled in personal config but missing from Lock File"
                )));
            };
            wanted.push(WantedSkill {
                name: name.clone(),
                version,
                from_override,
            });
        }
        wanted.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(wanted)
    }

    pub fn sync(&self) -> Result<SyncReport, Error> {
        //! Check `.git` presence; Lock File alone is not enough without a clone.
        if !self.paths.library_clone.join(".git").exists() {
            return Err(Error::msg(
                "library clone missing or corrupt; re-run bootstrap",
            ));
        }
        let personal = PersonalConfig::load(&self.paths.personal_path())?;
        let harnesses = personal.harnesses()?;
        let scope = personal.placement_scope()?;
        let wanted = self.resolve_wanted()?;
        let selected: std::collections::BTreeSet<_> =
            wanted.iter().map(|w| w.name.clone()).collect();

        let mut report = SyncReport::default();

        for skill in &wanted {
            let cached = git_cache::extract_skill_at_tag(
                &self.paths.library_clone,
                &self.paths.cache_dir,
                &skill.name,
                &skill.version,
            )?;
            let meta = skill_meta::parse_skill_md(&cached.join("SKILL.md"))?;
            for dep in &meta.dependencies {
                if !selected.contains(dep) {
                    report.warnings.push(format!(
                        "skill '{}' depends on '{}' which is not selected — install it or Sync will warn each run",
                        skill.name, dep
                    ));
                }
            }

            // Canonical store first; non-Codex harnesses link to it.
            let canonical_parent = place::canonical_skills_dir(
                &self.paths.home_dir,
                &self.paths.project_dir,
                scope,
            );
            let (canonical_path, _canonical_kind, canonical_warn) = place::place_skill(
                &cached,
                &canonical_parent,
                &skill.name,
                self.paths.force_copy,
            )?;
            if let Some(w) = canonical_warn {
                report.warnings.push(format!("{w} (canonical)"));
            }

            let local_bin = self.paths.home_dir.join(".local").join("bin");
            place::link_skill_bins(&canonical_path, &local_bin)?;

            for harness in &harnesses {
                let parent = place::harness_skills_dir(
                    &self.paths.home_dir,
                    &self.paths.project_dir,
                    *harness,
                    scope,
                );
                let (path, kind, warn) = if place::harness_is_canonical(*harness) {
                    (
                        canonical_path.clone(),
                        if self.paths.force_copy {
                            PlacementKind::Copy
                        } else {
                            PlacementKind::Symlink
                        },
                        None,
                    )
                } else {
                    place::place_skill(
                        &canonical_path,
                        &parent,
                        &skill.name,
                        self.paths.force_copy,
                    )?
                };
                if let Some(w) = warn {
                    report.warnings.push(format!("{} ({})", w, harness.as_str()));
                }
                report.placements.push(PlacementRecord {
                    skill: skill.name.clone(),
                    harness: harness.as_str().to_string(),
                    path,
                    kind,
                });
            }
        }
        Ok(report)
    }

    pub fn adopt(&self) -> Result<AdoptReport, Error> {
        let lock = LockFile::load(&self.paths.lock_path())?;
        let personal = PersonalConfig::load(&self.paths.personal_path())?;
        let overrides = OverridesFile::load(&self.paths.overrides_path())?;
        let scope = personal.placement_scope()?;

        let mut skipped = Vec::new();
        let mut updated = Vec::new();
        let mut migrations = Vec::new();
        for name in &personal.skills {
            if overrides.overrides.contains_key(name) {
                skipped.push(name.clone());
                continue;
            }
            let Some(to_version) = lock.skills.get(name) else {
                continue;
            };
            updated.push(name.clone());

            let installed_md = place::canonical_skills_dir(
                &self.paths.home_dir,
                &self.paths.project_dir,
                scope,
            )
            .join(name)
            .join("SKILL.md");
            let from_version = if installed_md.exists() {
                skill_meta::parse_skill_md(&installed_md)
                    .ok()
                    .map(|m| m.version)
            } else {
                None
            };
            if let Some(from) = from_version {
                if is_major_bump(&from, to_version) {
                    // Peek at the target Version's migration docs before Sync replaces placement.
                    if let Ok(cached) = git_cache::extract_skill_at_tag(
                        &self.paths.library_clone,
                        &self.paths.cache_dir,
                        name,
                        to_version,
                    ) {
                        if let Some(summary) = migration_summary(&cached) {
                            migrations.push(MigrationNotice {
                                skill: name.clone(),
                                from_version: from,
                                to_version: to_version.clone(),
                                summary,
                            });
                        }
                    }
                }
            }
        }

        let sync = self.sync()?;
        Ok(AdoptReport {
            updated,
            skipped_overrides: skipped,
            migrations,
            sync,
        })
    }

    pub fn status(&self) -> Result<StatusReport, Error> {
        let lock = LockFile::load(&self.paths.lock_path())?;
        let personal = PersonalConfig::load(&self.paths.personal_path())?;
        let overrides = OverridesFile::load(&self.paths.overrides_path())?;
        let wanted = self.resolve_wanted()?;

        let mut skills = Vec::new();
        for w in &wanted {
            let mut deprecated = false;
            let mut successor = None;
            let mut description = String::new();
            let cached = self
                .paths
                .cache_dir
                .join("versions")
                .join(&w.name)
                .join(&w.version)
                .join(&w.name)
                .join("SKILL.md");
            if cached.exists() {
                if let Ok(meta) = skill_meta::parse_skill_md(&cached) {
                    deprecated = meta.deprecated;
                    successor = meta.successor;
                    description = meta.description;
                }
            } else {
                // Try library working tree
                let main_skill = self
                    .paths
                    .library_clone
                    .join("skills")
                    .join(&w.name)
                    .join("SKILL.md");
                if main_skill.exists() {
                    if let Ok(meta) = skill_meta::parse_skill_md(&main_skill) {
                        deprecated = meta.deprecated;
                        successor = meta.successor;
                        description = meta.description;
                    }
                }
            }
            let lock_version = lock
                .skills
                .get(&w.name)
                .cloned()
                .unwrap_or_default();
            skills.push(SkillStatus {
                name: w.name.clone(),
                lock_version,
                effective_version: w.version.clone(),
                from_override: w.from_override,
                deprecated,
                successor,
                description,
            });
        }

        let ovr: Vec<_> = overrides
            .overrides
            .iter()
            .map(|(n, v)| OverridePin {
                name: n.clone(),
                version: v.clone(),
            })
            .collect();

        Ok(StatusReport {
            skills,
            overrides: ovr,
            harnesses: personal.harnesses.clone(),
            placement_scope: personal.placement_scope.clone(),
        })
    }

    pub fn clear_override(&self, skill: &str) -> Result<(), Error> {
        let mut o = OverridesFile::load(&self.paths.overrides_path())?;
        o.overrides.remove(skill);
        o.save(&self.paths.overrides_path())?;
        Ok(())
    }

    pub fn set_override(&self, skill: &str, version: &str) -> Result<(), Error> {
        let mut o = OverridesFile::load(&self.paths.overrides_path())?;
        o.overrides.insert(skill.to_string(), version.to_string());
        o.save(&self.paths.overrides_path())?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<ListedSkill>, Error> {
        let personal = PersonalConfig::load(&self.paths.personal_path())?;
        let scope = personal.placement_scope()?;
        let harnesses = personal.harnesses()?;
        let wanted = self.resolve_wanted()?;
        let mut listed = Vec::new();

        for skill in &wanted {
            let mut placements = Vec::new();
            for harness in &harnesses {
                let path = place::harness_skills_dir(
                    &self.paths.home_dir,
                    &self.paths.project_dir,
                    *harness,
                    scope,
                )
                .join(&skill.name);
                if path.exists() || path.symlink_metadata().is_ok() {
                    placements.push(ListedPlacement {
                        harness: harness.as_str().to_string(),
                        path,
                    });
                }
            }

            let version = {
                let skill_md = place::canonical_skills_dir(
                    &self.paths.home_dir,
                    &self.paths.project_dir,
                    scope,
                )
                .join(&skill.name)
                .join("SKILL.md");
                if skill_md.exists() {
                    skill_meta::parse_skill_md(&skill_md)
                        .map(|m| m.version)
                        .unwrap_or_else(|_| skill.version.clone())
                } else {
                    skill.version.clone()
                }
            };

            if !placements.is_empty() {
                listed.push(ListedSkill {
                    name: skill.name.clone(),
                    version,
                    placements,
                });
            }
        }
        Ok(listed)
    }

    pub fn remove(&self, skill: &str) -> Result<(), Error> {
        let personal = PersonalConfig::load(&self.paths.personal_path())?;
        let scope = personal.placement_scope()?;
        let harnesses = personal.harnesses()?;
        let local_bin = self.paths.home_dir.join(".local").join("bin");

        let canonical = place::canonical_skills_dir(
            &self.paths.home_dir,
            &self.paths.project_dir,
            scope,
        )
        .join(skill);

        // Unlink bins while the canonical tree (or its target) still exists.
        if canonical.exists() || canonical.symlink_metadata().is_ok() {
            place::unlink_skill_bins(&canonical, &local_bin)?;
        }

        for harness in &harnesses {
            if place::harness_is_canonical(*harness) {
                continue;
            }
            let dest = place::harness_skills_dir(
                &self.paths.home_dir,
                &self.paths.project_dir,
                *harness,
                scope,
            )
            .join(skill);
            place::remove_path(&dest)?;
        }
        place::remove_path(&canonical)?;

        let mut personal = personal;
        personal.skills.retain(|s| s != skill);
        personal.save(&self.paths.personal_path())?;

        let mut overrides = OverridesFile::load(&self.paths.overrides_path())?;
        overrides.overrides.remove(skill);
        overrides.save(&self.paths.overrides_path())?;

        Ok(())
    }
}

fn is_major_bump(from: &str, to: &str) -> bool {
    let Ok(from_v) = semver::Version::parse(from) else {
        return false;
    };
    let Ok(to_v) = semver::Version::parse(to) else {
        return false;
    };
    to_v.major > from_v.major
}

fn migration_summary(skill_dir: &std::path::Path) -> Option<String> {
    let migration_md = skill_dir.join("MIGRATION.md");
    if migration_md.exists() {
        return std::fs::read_to_string(migration_md).ok().map(|s| {
            s.lines()
                .map(str::trim)
                .find(|l| !l.is_empty() && !l.starts_with('#'))
                .unwrap_or("")
                .to_string()
        }).filter(|s| !s.is_empty());
    }
    let skill_md = std::fs::read_to_string(skill_dir.join("SKILL.md")).ok()?;
    let idx = skill_md.find("## Migration")?;
    let after = &skill_md[idx + "## Migration".len()..];
    let section = after.split("\n## ").next().unwrap_or(after);
    section
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .map(|s| s.to_string())
}
