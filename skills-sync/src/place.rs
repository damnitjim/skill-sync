use std::fs;
use std::path::{Path, PathBuf};

use crate::sync_ops::{Harness, PlacementKind, PlacementScope};
use crate::Error;

/// Canonical Agent Skills store (`.agents/skills`), shared across harnesses.
pub fn canonical_skills_dir(home: &Path, project: &Path, scope: PlacementScope) -> PathBuf {
    match scope {
        PlacementScope::User => home.join(".agents").join("skills"),
        PlacementScope::Project => project.join(".agents").join("skills"),
    }
}

pub fn harness_skills_dir(
    home: &Path,
    project: &Path,
    harness: Harness,
    scope: PlacementScope,
) -> PathBuf {
    match (harness, scope) {
        (Harness::ClaudeCode, PlacementScope::User) => home.join(".claude").join("skills"),
        (Harness::ClaudeCode, PlacementScope::Project) => project.join(".claude").join("skills"),
        (Harness::Codex, PlacementScope::User) => home.join(".agents").join("skills"),
        (Harness::Codex, PlacementScope::Project) => project.join(".agents").join("skills"),
        (Harness::Cursor, PlacementScope::User) => home.join(".cursor").join("skills"),
        (Harness::Cursor, PlacementScope::Project) => project.join(".cursor").join("skills"),
        (Harness::Pi, PlacementScope::User) => home.join(".pi").join("agent").join("skills"),
        (Harness::Pi, PlacementScope::Project) => project.join(".pi").join("skills"),
    }
}

/// True when this harness discovers from the canonical `.agents/skills` store.
pub fn harness_is_canonical(harness: Harness) -> bool {
    matches!(harness, Harness::Codex)
}

pub fn place_skill(
    cached_skill_dir: &Path,
    dest_parent: &Path,
    skill_name: &str,
    force_copy: bool,
) -> Result<(PathBuf, PlacementKind, Option<String>), Error> {
    fs::create_dir_all(dest_parent)?;
    let dest = dest_parent.join(skill_name);
    remove_path(&dest)?;

    if force_copy {
        copy_dir(cached_skill_dir, &dest)?;
        return Ok((
            dest,
            PlacementKind::Copy,
            Some("placed via copy (symlink disabled)".into()),
        ));
    }

    match symlink_dir(cached_skill_dir, &dest) {
        Ok(()) => Ok((dest, PlacementKind::Symlink, None)),
        Err(e) => {
            copy_dir(cached_skill_dir, &dest)?;
            Ok((
                dest,
                PlacementKind::Copy,
                Some(format!(
                    "symlink failed ({e}); placed via copy — edits will not propagate"
                )),
            ))
        }
    }
}

/// Remove `bin_dir` symlinks that resolve into this skill's `bin/` directory.
pub fn unlink_skill_bins(skill_dir: &Path, bin_dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let skill_bin = skill_dir.join("bin");
    if !bin_dir.is_dir() {
        return Ok(Vec::new());
    }
    let skill_bin_canon = fs::canonicalize(&skill_bin).unwrap_or_else(|_| skill_bin.clone());
    let mut removed = Vec::new();
    for entry in fs::read_dir(bin_dir)? {
        let entry = entry?;
        let dest = entry.path();
        let meta = match fs::symlink_metadata(&dest) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if !meta.file_type().is_symlink() {
            continue;
        }
        let target = match fs::read_link(&dest) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let resolved = if target.is_absolute() {
            target
        } else {
            dest.parent().unwrap_or(bin_dir).join(target)
        };
        let resolved = fs::canonicalize(&resolved).unwrap_or(resolved);
        if resolved.starts_with(&skill_bin_canon) {
            fs::remove_file(&dest)?;
            removed.push(dest);
        }
    }
    Ok(removed)
}

/// Symlink each file under `skill_dir/bin/` into `bin_dir` (typically `~/.local/bin`).
pub fn link_skill_bins(skill_dir: &Path, bin_dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let skill_bin = skill_dir.join("bin");
    if !skill_bin.is_dir() {
        return Ok(Vec::new());
    }
    fs::create_dir_all(bin_dir)?;
    let mut linked = Vec::new();
    for entry in fs::read_dir(&skill_bin)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let dest = bin_dir.join(&name);
        remove_path(&dest)?;
        let src = entry.path();
        match symlink_file(&src, &dest) {
            Ok(()) => linked.push(dest),
            Err(e) => {
                fs::copy(&src, &dest).map_err(|copy_err| {
                    Error::msg(format!(
                        "failed to place bin {}: symlink ({e}), copy ({copy_err})",
                        name.to_string_lossy()
                    ))
                })?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mode = fs::metadata(&src)?.permissions().mode();
                    fs::set_permissions(&dest, fs::Permissions::from_mode(mode))?;
                }
                linked.push(dest);
            }
        }
    }
    Ok(linked)
}

pub fn remove_path(path: &Path) -> Result<(), Error> {
    match path.symlink_metadata() {
        Ok(meta) if meta.file_type().is_symlink() || meta.is_file() => {
            fs::remove_file(path)?;
        }
        Ok(_) => {
            fs::remove_dir_all(path)?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }
    Ok(())
}

fn symlink_dir(src: &Path, dest: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dest)
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_dir(src, dest)
    }
}

fn symlink_file(src: &Path, dest: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dest)
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_file(src, dest)
    }
}

fn copy_dir(src: &Path, dest: &Path) -> Result<(), Error> {
    // Recreate dest as a directory and copy contents
    fs::create_dir_all(dest)?;
    for entry in walkdir::WalkDir::new(src).min_depth(1) {
        let entry = entry.map_err(|e| Error::msg(format!("walk: {e}")))?;
        let rel = entry.path().strip_prefix(src).unwrap();
        let target = dest.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}
