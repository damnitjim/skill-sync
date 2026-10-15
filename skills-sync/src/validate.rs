//! Authoring validation for Skill folders under a library root.

use std::fs;
use std::path::Path;

use semver::Version;

use crate::lock::LockFile;
use crate::skill_meta::parse_skill_md;
use crate::Error;

/// Scan `library/skills/*/SKILL.md` and `skills.lock`; fail with aggregated errors.
pub fn validate_skills(library: &Path) -> Result<(), Error> {
    let skills_dir = library.join("skills");
    if !skills_dir.is_dir() {
        return Err(Error::msg(format!(
            "skills directory missing at {}",
            skills_dir.display()
        )));
    }

    let lock = LockFile::load(&library.join("skills.lock"))?;
    let mut errors: Vec<String> = Vec::new();

    let mut entries: Vec<_> = fs::read_dir(&skills_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let folder = entry.file_name().to_string_lossy().into_owned();
        let skill_path = entry.path();
        let skill_md = skill_path.join("SKILL.md");

        if !skill_md.is_file() {
            errors.push(format!(
                "skills/{folder}: directory has no SKILL.md (orphan folder)"
            ));
            continue;
        }

        let meta = match parse_skill_md(&skill_md) {
            Ok(m) => m,
            Err(e) => {
                errors.push(format!("skills/{folder}: {e}"));
                continue;
            }
        };

        if meta.name != folder {
            errors.push(format!(
                "skills/{folder}: frontmatter name {:?} must match folder name",
                meta.name
            ));
        }

        if meta.description.trim().is_empty() {
            errors.push(format!("skills/{folder}: description must be non-empty"));
        }

        let version = match Version::parse(&meta.version) {
            Ok(v) => v,
            Err(_) => {
                errors.push(format!(
                    "skills/{folder}: version {:?} is not valid semver",
                    meta.version
                ));
                continue;
            }
        };

        if version.major >= 2 {
            let text = fs::read_to_string(&skill_md).unwrap_or_default();
            let has_section = text.lines().any(|l| {
                let t = l.trim();
                t == "## Migration" || t.starts_with("## Migration ")
            });
            let has_file = skill_path.join("MIGRATION.md").is_file();
            if !has_section && !has_file {
                errors.push(format!(
                    "skills/{folder}: version {} requires Migration Docs (## Migration in SKILL.md or MIGRATION.md)",
                    meta.version
                ));
            }
        }

        for dep in &meta.dependencies {
            if !skills_dir.join(dep).is_dir() {
                errors.push(format!(
                    "skills/{folder}: dependency {dep:?} does not exist under skills/"
                ));
            }
        }

        if !lock.skills.contains_key(&folder) {
            errors.push(format!(
                "skills/{folder}: missing from skills.lock (complete pin table required)"
            ));
        }
    }

    for name in lock.skills.keys() {
        let skill_md = skills_dir.join(name).join("SKILL.md");
        if !skill_md.is_file() {
            errors.push(format!(
                "skills.lock: pin {name:?} has no skills/{name}/SKILL.md"
            ));
            continue;
        }
        if let Err(e) = parse_skill_md(&skill_md) {
            errors.push(format!(
                "skills.lock: pin {name:?} has unparseable SKILL.md: {e}"
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::msg(errors.join("\n")))
    }
}
