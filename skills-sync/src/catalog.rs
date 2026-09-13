use std::fs;
use std::path::Path;

use crate::skill_meta;
use crate::Error;

pub fn generate(library_root: &Path, out_dir: &Path) -> Result<(), Error> {
    fs::create_dir_all(out_dir)?;
    let skills_dir = library_root.join("skills");
    let mut entries = Vec::new();

    if skills_dir.exists() {
        for entry in fs::read_dir(&skills_dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let skill_md = entry.path().join("SKILL.md");
            if !skill_md.exists() {
                continue;
            }
            let meta = skill_meta::parse_skill_md(&skill_md)?;
            let has_migration = fs::read_to_string(&skill_md)?.contains("## Migration")
                || entry.path().join("MIGRATION.md").exists();
            entries.push((meta, has_migration));
        }
    }

    entries.sort_by(|a, b| a.0.name.cmp(&b.0.name));

    let mut index = String::from("# Skill Catalog\n\n");
    index.push_str("| Skill | Version | Description | Status |\n");
    index.push_str("|-------|---------|-------------|--------|\n");
    for (meta, has_migration) in &entries {
        let status = if meta.deprecated {
            match &meta.successor {
                Some(s) => format!("**Deprecated** → `{s}`"),
                None => "**Deprecated**".into(),
            }
        } else {
            "active".into()
        };
        let mig = if *has_migration { " _(migration)_" } else { "" };
        let desc = meta.description.replace('\n', " ").trim().to_string();
        index.push_str(&format!(
            "| [{}]({}/) | {} | {}{} | {} |\n",
            meta.name, meta.name, meta.version, desc, mig, status
        ));

        let skill_dir = out_dir.join(&meta.name);
        fs::create_dir_all(&skill_dir)?;
        let mut page = format!(
            "# {}\n\n**Version:** {}\n\n{}\n\n",
            meta.name, meta.version, meta.description
        );
        if meta.deprecated {
            page.push_str("**Deprecated.**");
            if let Some(s) = &meta.successor {
                page.push_str(&format!(" Successor: `{s}`.\n\n"));
            } else {
                page.push_str("\n\n");
            }
        }
        if !meta.dependencies.is_empty() {
            page.push_str(&format!(
                "**Dependencies:** {}\n\n",
                meta.dependencies.join(", ")
            ));
        }
        fs::write(skill_dir.join("index.md"), page)?;
    }

    fs::write(out_dir.join("index.md"), index)?;
    Ok(())
}
