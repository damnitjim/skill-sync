//! Test harness for the application API seam.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use skills_sync::{Harness, PlacementScope, SkillsSync, SyncPaths};

pub struct SkillSpec<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub description: &'a str,
    pub dependencies: &'a [&'a str],
    pub deprecated: bool,
    pub successor: Option<&'a str>,
    pub body: &'a str,
}

pub struct TestEnv {
    _root: tempfile::TempDir,
    root: PathBuf,
    force_copy: bool,
}

impl TestEnv {
    pub fn new() -> Self {
        let _root = tempfile::tempdir().expect("tempdir");
        let root = _root.path().to_path_buf();
        for d in [
            "config",
            "cache",
            "home",
            "project",
            "library-src",
        ] {
            fs::create_dir_all(root.join(d)).unwrap();
        }
        Self {
            _root,
            root,
            force_copy: false,
        }
    }

    pub fn with_force_copy(mut self) -> Self {
        self.force_copy = true;
        self
    }

    pub fn paths(&self) -> SyncPaths {
        SyncPaths {
            config_dir: self.root.join("config"),
            cache_dir: self.root.join("cache"),
            home_dir: self.root.join("home"),
            project_dir: self.root.join("project"),
            library_clone: self.root.join("cache").join("library"),
            force_copy: self.force_copy,
        }
    }

    pub fn app(&self) -> SkillsSync {
        SkillsSync::new(self.paths())
    }

    pub fn write_lock(&self, pins: &[(&str, &str)]) {
        let mut body = String::from("[skills]\n");
        for (name, ver) in pins {
            body.push_str(&format!("{name} = \"{ver}\"\n"));
        }
        // Lock file lives in the library clone / source of truth; for resolve we also
        // keep a copy path the app reads from library_clone/skills.lock after clone.
        // Before clone exists, write to a staging lock that ensure_clone / init will use.
        let lib = self.root.join("library-src");
        fs::create_dir_all(&lib).unwrap();
        fs::write(lib.join("skills.lock"), &body).unwrap();
        // Also write into clone location if present
        let clone = self.paths().library_clone;
        if clone.exists() {
            fs::write(clone.join("skills.lock"), &body).unwrap();
        } else {
            // App reads lock from library_clone; for resolve-only tests without git,
            // create clone dir with just the lock.
            fs::create_dir_all(&clone).unwrap();
            fs::write(clone.join("skills.lock"), body).unwrap();
        }
    }

    pub fn write_overrides(&self, pins: &[(&str, &str)]) {
        let mut body = String::from("[overrides]\n");
        for (name, ver) in pins {
            body.push_str(&format!("{name} = \"{ver}\"\n"));
        }
        if pins.is_empty() {
            body = String::from("[overrides]\n");
        }
        fs::create_dir_all(self.paths().config_dir).unwrap();
        fs::write(self.paths().config_dir.join("overrides.toml"), body).unwrap();
    }

    pub fn write_personal(
        &self,
        skills: &[&str],
        harnesses: &[Harness],
        scope: PlacementScope,
    ) {
        let skills_list = skills
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let harness_list = harnesses
            .iter()
            .map(|h| format!("\"{}\"", h.as_str()))
            .collect::<Vec<_>>()
            .join(", ");
        let scope_str = match scope {
            PlacementScope::User => "user",
            PlacementScope::Project => "project",
        };
        let body = format!(
            r#"
skills = [{skills_list}]
harnesses = [{harness_list}]
placement_scope = "{scope_str}"
"#
        );
        fs::create_dir_all(self.paths().config_dir).unwrap();
        fs::write(self.paths().config_dir.join("personal.toml"), body).unwrap();
    }

    pub fn init_library_repo(&self, skills: &[SkillSpec<'_>]) {
        let lib = self.root.join("library-src");
        fs::create_dir_all(lib.join("skills")).unwrap();
        if !lib.join("skills.lock").exists() {
            fs::write(lib.join("skills.lock"), "[skills]\n").unwrap();
        }
        for skill in skills {
            self.write_skill_into(&lib, skill);
        }
        git(&lib, &["init", "-b", "main"]);
        git(&lib, &["config", "user.email", "test@example.com"]);
        git(&lib, &["config", "user.name", "Test"]);
        git(&lib, &["add", "."]);
        git(&lib, &["commit", "-m", "initial"]);
        self.ensure_clone();
    }

    pub fn update_skill(&self, skill: SkillSpec<'_>) {
        let lib = self.root.join("library-src");
        self.write_skill_into(&lib, &skill);
        git(&lib, &["add", "."]);
        git(
            &lib,
            &["commit", "-m", &format!("update {}", skill.name)],
        );
        self.mirror_to_clone();
    }

    pub fn tag_skill(&self, name: &str, version: &str) {
        self.tag_skill_raw(&format!("{name}@{version}"));
    }

    pub fn tag_skill_raw(&self, tag: &str) {
        let lib = self.root.join("library-src");
        git(&lib, &["tag", tag]);
        self.mirror_to_clone();
    }

    pub fn commit_all(&self, message: &str) {
        let lib = self.root.join("library-src");
        git(&lib, &["add", "."]);
        git(&lib, &["commit", "-m", message]);
        self.mirror_to_clone();
    }

    pub fn delete_tag(&self, tag: &str) {
        let lib = self.root.join("library-src");
        let _ = Command::new("git")
            .args(["tag", "-d", tag])
            .current_dir(&lib)
            .status();
        self.mirror_to_clone();
    }

    pub fn library_src(&self) -> PathBuf {
        self.root.join("library-src")
    }

    pub fn ensure_clone(&self) {
        let clone = self.paths().library_clone;
        let lib = self.root.join("library-src");
        if clone.exists() {
            fs::remove_dir_all(&clone).unwrap();
        }
        // Local clone via file URL / path
        let status = Command::new("git")
            .args(["clone", lib.to_str().unwrap(), clone.to_str().unwrap()])
            .status()
            .expect("git clone");
        assert!(status.success(), "git clone failed");
        // Fetch tags
        git(&clone, &["fetch", "--tags"]);
    }

    fn mirror_to_clone(&self) {
        let clone = self.paths().library_clone;
        if !clone.exists() {
            self.ensure_clone();
            return;
        }
        git(&clone, &["fetch", "origin", "--tags"]);
        git(&clone, &["reset", "--hard", "origin/main"]);
        git(&clone, &["fetch", "--tags", "--force"]);
    }

    fn write_skill_into(&self, lib: &Path, skill: &SkillSpec<'_>) {
        let dir = lib.join("skills").join(skill.name);
        fs::create_dir_all(&dir).unwrap();
        let mut fm = format!(
            "---\nname: {}\ndescription: {}\nversion: {}\n",
            skill.name, skill.description, skill.version
        );
        if !skill.dependencies.is_empty() {
            fm.push_str("dependencies:\n");
            for d in skill.dependencies {
                fm.push_str(&format!("  - {d}\n"));
            }
        }
        if skill.deprecated {
            fm.push_str("deprecated: true\n");
            if let Some(s) = skill.successor {
                fm.push_str(&format!("successor: {s}\n"));
            }
        }
        fm.push_str("---\n\n");
        fm.push_str(skill.body);
        fs::write(dir.join("SKILL.md"), fm).unwrap();
    }

    pub fn claude_user_skill(&self, name: &str) -> PathBuf {
        self.paths()
            .home_dir
            .join(".claude")
            .join("skills")
            .join(name)
    }

    pub fn claude_project_skill(&self, name: &str) -> PathBuf {
        self.paths()
            .project_dir
            .join(".claude")
            .join("skills")
            .join(name)
    }

    pub fn codex_user_skill(&self, name: &str) -> PathBuf {
        self.paths()
            .home_dir
            .join(".agents")
            .join("skills")
            .join(name)
    }

    pub fn cursor_user_skill(&self, name: &str) -> PathBuf {
        self.paths()
            .home_dir
            .join(".cursor")
            .join("skills")
            .join(name)
    }

    pub fn pi_user_skill(&self, name: &str) -> PathBuf {
        self.paths()
            .home_dir
            .join(".pi")
            .join("agent")
            .join("skills")
            .join(name)
    }
}

fn git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    if !out.status.success() {
        panic!(
            "git {args:?} failed: {} {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
}
