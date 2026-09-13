//! Release tagging helper (secondary test seam).

use std::path::Path;
use std::process::Command;

use crate::skill_meta;
use crate::Error;

/// Compare two commits and return (skill_name, new_version) for Skills whose `version:` changed.
pub fn detect_version_bumps(
    repo: &Path,
    parent: &str,
    head: &str,
) -> Result<Vec<(String, String)>, Error> {
    let diff = git_output(
        repo,
        &["diff", "--name-only", &format!("{parent}..{head}"), "--", "skills/"],
    )?;
    let mut bumps = Vec::new();
    for line in diff.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // skills/<name>/SKILL.md or other files under skills/<name>/
        let parts: Vec<_> = line.split('/').collect();
        if parts.len() < 2 || parts[0] != "skills" {
            continue;
        }
        let name = parts[1];
        let old_ver = version_at(repo, parent, name).ok();
        let new_ver = version_at(repo, head, name)?;
        if old_ver.as_ref() != Some(&new_ver) {
            // Only record once per skill
            if !bumps.iter().any(|(n, _)| n == name) {
                bumps.push((name.to_string(), new_ver));
            }
        }
    }
    bumps.sort();
    Ok(bumps)
}

pub fn proposed_tags(bumps: &[(String, String)]) -> Vec<String> {
    bumps
        .iter()
        .map(|(n, v)| format!("{n}@{v}"))
        .collect()
}

pub fn validate_tag_frontmatter(repo: &Path, name: &str, version: &str) -> Result<(), Error> {
    let ver = version_at(repo, "HEAD", name)?;
    if ver != version {
        return Err(Error::msg(format!(
            "version mismatch: frontmatter has {ver}, expected {version}"
        )));
    }
    Ok(())
}

fn version_at(repo: &Path, rev: &str, name: &str) -> Result<String, Error> {
    let path = format!("skills/{name}/SKILL.md");
    let text = git_output(repo, &["show", &format!("{rev}:{path}")])?;
    let meta = skill_meta::parse_skill_md_str(&text)?;
    Ok(meta.version)
}

fn git_output(repo: &Path, args: &[&str]) -> Result<String, Error> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|e| Error::msg(format!("git failed: {e}")))?;
    if !out.status.success() {
        return Err(Error::msg(format!(
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}
