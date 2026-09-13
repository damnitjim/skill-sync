use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::skill_meta;
use crate::Error;

/// Extract `skills/<name>/` at tag `name@version` into `cache_dir/versions/<name>/<version>/`.
pub fn extract_skill_at_tag(
    library_clone: &Path,
    cache_dir: &Path,
    name: &str,
    version: &str,
) -> Result<PathBuf, Error> {
    let tag = format!("{name}@{version}");
    ensure_tag_exists(library_clone, &tag)?;

    let dest = cache_dir
        .join("versions")
        .join(name)
        .join(version)
        .join(name);
    if dest.exists() {
        fs::remove_dir_all(&dest)?;
    }
    fs::create_dir_all(dest.parent().unwrap())?;

    // git archive <tag>:skills/<name> | tar extract into dest parent, resulting in dest
    let staging = dest.parent().unwrap().join("_staging");
    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;

    let archive = Command::new("git")
        .args(["archive", &tag, &format!("skills/{name}")])
        .current_dir(library_clone)
        .output()
        .map_err(|e| Error::msg(format!("git archive failed to start: {e}")))?;
    if !archive.status.success() {
        return Err(Error::msg(format!(
            "missing or unreadable git tag {tag}: {}",
            String::from_utf8_lossy(&archive.stderr)
        )));
    }

    let mut child = Command::new("tar")
        .args(["-x", "-C"])
        .arg(&staging)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| Error::msg(format!("tar failed to start: {e}")))?;
    {
        use std::io::Write;
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(&archive.stdout)?;
    }
    let status = child.wait()?;
    if !status.success() {
        return Err(Error::msg(format!("tar extract failed for {tag}")));
    }

    // archive contains skills/<name>/... under staging
    let extracted = staging.join("skills").join(name);
    if !extracted.exists() {
        return Err(Error::msg(format!(
            "git archive for {tag} did not contain skills/{name}"
        )));
    }
    fs::rename(&extracted, &dest)?;
    let _ = fs::remove_dir_all(&staging);

    // Validate frontmatter version matches tag
    let meta = skill_meta::parse_skill_md(&dest.join("SKILL.md"))?;
    if meta.version != version {
        return Err(Error::msg(format!(
            "version mismatch for {tag}: frontmatter has {}, tag expects {version}",
            meta.version
        )));
    }
    if meta.name != name {
        return Err(Error::msg(format!(
            "name mismatch for {tag}: frontmatter has {}",
            meta.name
        )));
    }

    Ok(dest)
}

fn ensure_tag_exists(library_clone: &Path, tag: &str) -> Result<(), Error> {
    let out = Command::new("git")
        .args(["rev-parse", "--verify", &format!("refs/tags/{tag}")])
        .current_dir(library_clone)
        .output()
        .map_err(|e| Error::msg(format!("git rev-parse failed: {e}")))?;
    if !out.status.success() {
        return Err(Error::msg(format!(
            "git tag {tag} not found in library clone"
        )));
    }
    Ok(())
}
