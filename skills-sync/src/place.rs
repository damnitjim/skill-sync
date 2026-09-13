use std::fs;
use std::path::{Path, PathBuf};

use crate::sync_ops::{Harness, PlacementKind, PlacementScope};
use crate::Error;

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

fn remove_path(path: &Path) -> Result<(), Error> {
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
