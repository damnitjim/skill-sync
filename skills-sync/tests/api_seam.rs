//! Integration tests at the skills-sync **application API** seam.

mod support;

use std::fs;

use skills_sync::{
    AdoptReport, Harness, PlacementKind, PlacementScope, WantedSkill,
};

use support::{SkillSpec, TestEnv};

#[test]
fn resolve_uses_lock_file_versions_when_no_overrides() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0"), ("story-writing", "2.1.0")]);
    env.write_personal(
        &["jira", "story-writing"],
        &[Harness::ClaudeCode, Harness::Cursor],
        PlacementScope::User,
    );

    let app = env.app();
    let wanted = app.resolve_wanted().expect("resolve");
    assert_eq!(
        wanted,
        vec![
            WantedSkill {
                name: "jira".into(),
                version: "1.0.0".into(),
                from_override: false,
            },
            WantedSkill {
                name: "story-writing".into(),
                version: "2.1.0".into(),
                from_override: false,
            },
        ]
    );
}

#[test]
fn resolve_applies_loud_overrides_over_lock_file() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0"), ("story-writing", "2.1.0")]);
    env.write_overrides(&[("jira", "2.0.0-rc.1")]);
    env.write_personal(&["jira", "story-writing"], &[Harness::ClaudeCode], PlacementScope::User);

    let wanted = env.app().resolve_wanted().expect("resolve");
    assert_eq!(wanted[0].name, "jira");
    assert_eq!(wanted[0].version, "2.0.0-rc.1");
    assert!(wanted[0].from_override);
    assert_eq!(wanted[1].version, "2.1.0");
    assert!(!wanted[1].from_override);
}

#[test]
fn resolve_only_includes_personally_enabled_skills() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0"), ("confluence", "0.1.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);

    let wanted = env.app().resolve_wanted().expect("resolve");
    assert_eq!(wanted.len(), 1);
    assert_eq!(wanted[0].name, "jira");
}

#[test]
fn list_reports_installed_skills_with_versions_and_harness_paths() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(
        &["jira"],
        &[Harness::ClaudeCode, Harness::Cursor],
        PlacementScope::User,
    );
    env.app().sync().expect("sync");

    let listed = env.app().list().expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].name, "jira");
    assert_eq!(listed[0].version, "1.0.0");
    assert!(
        listed[0]
            .placements
            .iter()
            .any(|p| p.harness == "claude-code" && p.path.exists()),
        "claude placement missing: {:?}",
        listed[0].placements
    );
    assert!(
        listed[0]
            .placements
            .iter()
            .any(|p| p.harness == "cursor" && p.path.exists()),
        "cursor placement missing: {:?}",
        listed[0].placements
    );
}

#[test]
fn remove_unlinks_placements_bins_and_disables_skill() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    let bin_dir = env.library_src().join("skills/jira/bin");
    fs::create_dir_all(&bin_dir).unwrap();
    fs::write(bin_dir.join("jira-mine"), "#!/usr/bin/env bash\necho mine\n").unwrap();
    env.commit_all("add bin");
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(
        &["jira"],
        &[Harness::ClaudeCode, Harness::Cursor],
        PlacementScope::User,
    );
    env.app().sync().expect("sync");
    assert!(env.claude_user_skill("jira").exists());
    assert!(env.codex_user_skill("jira").exists());
    assert!(env.cursor_user_skill("jira").exists());
    assert!(env.paths().home_dir.join(".local/bin/jira-mine").exists());

    env.app().remove("jira").expect("remove");

    assert!(
        !env.claude_user_skill("jira").exists(),
        "claude placement should be gone"
    );
    assert!(
        !env.cursor_user_skill("jira").exists(),
        "cursor placement should be gone"
    );
    assert!(
        !env.codex_user_skill("jira").exists(),
        "canonical placement should be gone"
    );
    assert!(
        !env.paths().home_dir.join(".local/bin/jira-mine").exists(),
        "bin link should be gone"
    );
    let wanted = env.app().resolve_wanted().expect("resolve after remove");
    assert!(
        wanted.iter().all(|w| w.name != "jira"),
        "jira should be removed from personal selection: {wanted:?}"
    );
}

#[test]
fn sync_links_skill_bin_scripts_onto_user_path() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira helpers",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# Jira\n",
    }]);
    let bin_dir = env.library_src().join("skills/jira/bin");
    fs::create_dir_all(&bin_dir).unwrap();
    let script = bin_dir.join("jira-mine");
    fs::write(&script, "#!/usr/bin/env bash\necho mine\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    }
    env.commit_all("add jira-mine bin");
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);

    env.app().sync().expect("sync");

    let on_path = env.paths().home_dir.join(".local/bin/jira-mine");
    assert!(
        on_path.exists(),
        "expected bin link at {}",
        on_path.display()
    );
    let meta = fs::symlink_metadata(&on_path).unwrap();
    assert!(
        meta.file_type().is_symlink(),
        "bin should be symlinked onto PATH"
    );
    let body = fs::read_to_string(&on_path).unwrap();
    assert!(body.contains("echo mine"), "{body}");
}

#[test]
fn sync_places_canonical_under_agents_skills_then_links_claude() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira helpers",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# Jira v1\n",
    }]);
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);

    let report = env.app().sync().expect("sync");
    assert_eq!(report.placements.len(), 1);
    assert_eq!(report.placements[0].harness, "claude-code");
    assert!(report.warnings.is_empty());

    let canonical = env.codex_user_skill("jira");
    assert!(
        canonical.exists(),
        "canonical store missing at {}",
        canonical.display()
    );
    let skill_md = fs::read_to_string(canonical.join("SKILL.md")).expect("canonical SKILL.md");
    assert!(skill_md.contains("Jira v1"));

    let claude = env.claude_user_skill("jira");
    assert!(claude.exists(), "claude link missing at {}", claude.display());
    let meta = fs::symlink_metadata(&claude).unwrap();
    assert!(
        meta.file_type().is_symlink(),
        "claude placement should symlink to canonical"
    );
    let resolved = fs::canonicalize(&claude).unwrap();
    let canonical_resolved = fs::canonicalize(&canonical).unwrap();
    assert_eq!(resolved, canonical_resolved);
}

#[test]
fn sync_extracts_tagged_skill_and_symlinks_into_claude_user_scope() {
    let env = TestEnv::new();
    env.init_library_repo(&[
        SkillSpec {
            name: "jira",
            version: "1.0.0",
            description: "Jira helpers",
            dependencies: &[],
            deprecated: false,
            successor: None,
            body: "# Jira v1\n",
        },
    ]);
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);

    let report = env.app().sync().expect("sync");
    assert_eq!(report.placements.len(), 1);
    assert_eq!(report.placements[0].skill, "jira");
    assert_eq!(report.placements[0].kind, PlacementKind::Symlink);
    assert!(report.warnings.is_empty());

    let target = env.claude_user_skill("jira");
    assert!(target.exists(), "expected placed skill at {}", target.display());
    assert!(target.is_symlink() || fs::symlink_metadata(&target).unwrap().file_type().is_symlink());

    let skill_md = fs::read_to_string(target.join("SKILL.md")).expect("read SKILL.md");
    assert!(skill_md.contains("Jira v1"));
    assert!(skill_md.contains("version: 1.0.0") || skill_md.contains("version: \"1.0.0\""));
}

#[test]
fn sync_can_pin_different_skills_to_different_tag_versions() {
    let env = TestEnv::new();
    env.init_library_repo(&[
        SkillSpec {
            name: "jira",
            version: "1.0.0",
            description: "Jira",
            dependencies: &[],
            deprecated: false,
            successor: None,
            body: "# jira 1\n",
        },
        SkillSpec {
            name: "story-writing",
            version: "1.0.0",
            description: "Stories",
            dependencies: &[],
            deprecated: false,
            successor: None,
            body: "# stories 1\n",
        },
    ]);
    env.tag_skill("jira", "1.0.0");
    env.tag_skill("story-writing", "1.0.0");

    // Bump jira only on a new commit/tag; story-writing stays at 1.0.0
    env.update_skill(SkillSpec {
        name: "jira",
        version: "1.1.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira 1.1\n",
    });
    env.tag_skill("jira", "1.1.0");

    env.write_lock(&[("jira", "1.0.0"), ("story-writing", "1.0.0")]);
    env.write_personal(
        &["jira", "story-writing"],
        &[Harness::ClaudeCode],
        PlacementScope::User,
    );

    env.app().sync().expect("sync");

    let jira = fs::read_to_string(env.claude_user_skill("jira").join("SKILL.md")).unwrap();
    let stories =
        fs::read_to_string(env.claude_user_skill("story-writing").join("SKILL.md")).unwrap();
    assert!(jira.contains("jira 1"), "lock pinned jira@1.0.0: {jira}");
    assert!(!jira.contains("jira 1.1"));
    assert!(stories.contains("stories 1"));
}

#[test]
fn sync_fails_when_lock_version_has_no_tag() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    // no tag
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);

    let err = env.app().sync().expect_err("should fail");
    assert!(
        err.to_string().contains("jira@1.0.0") || err.to_string().contains("tag"),
        "error should mention missing tag: {err}"
    );
}

#[test]
fn sync_fails_when_frontmatter_version_mismatches_tag() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "9.9.9",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    // Tag claims 1.0.0 but frontmatter says 9.9.9
    env.tag_skill_raw("jira@1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);

    let err = env.app().sync().expect_err("mismatch");
    assert!(
        err.to_string().contains("mismatch") || err.to_string().contains("9.9.9"),
        "{err}"
    );
}

#[test]
fn sync_warns_when_dependency_not_selected() {
    let env = TestEnv::new();
    env.init_library_repo(&[
        SkillSpec {
            name: "story-review",
            version: "1.0.0",
            description: "Review",
            dependencies: &["story-writing"],
            deprecated: false,
            successor: None,
            body: "# review\n",
        },
        SkillSpec {
            name: "story-writing",
            version: "1.0.0",
            description: "Write",
            dependencies: &[],
            deprecated: false,
            successor: None,
            body: "# write\n",
        },
    ]);
    env.tag_skill("story-review", "1.0.0");
    env.tag_skill("story-writing", "1.0.0");
    env.write_lock(&[("story-review", "1.0.0"), ("story-writing", "1.0.0")]);
    env.write_personal(&["story-review"], &[Harness::ClaudeCode], PlacementScope::User);

    let report = env.app().sync().expect("sync");
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.contains("story-writing") && w.contains("depend")),
        "warnings: {:?}",
        report.warnings
    );
    assert!(env.claude_user_skill("story-review").exists());
    assert!(!env.claude_user_skill("story-writing").exists());
}

#[test]
fn status_flags_deprecated_skills() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "legacy",
        version: "1.0.0",
        description: "Old",
        dependencies: &[],
        deprecated: true,
        successor: Some("modern"),
        body: "# legacy\n",
    }]);
    env.tag_skill("legacy", "1.0.0");
    env.write_lock(&[("legacy", "1.0.0")]);
    env.write_personal(&["legacy"], &[Harness::ClaudeCode], PlacementScope::User);
    env.app().sync().expect("sync");

    let status = env.app().status().expect("status");
    let skill = status.skills.iter().find(|s| s.name == "legacy").unwrap();
    assert!(skill.deprecated);
    assert_eq!(skill.successor.as_deref(), Some("modern"));
    assert!(status.overrides.is_empty());
}

#[test]
fn adopt_surfaces_migration_docs_on_major_version_bump() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# v1\n",
    }]);
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);
    env.app().sync().expect("sync v1");

    env.update_skill(SkillSpec {
        name: "jira",
        version: "2.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# v2\n\n## Migration\n\nRename jira-old to jira-new.\n",
    });
    env.tag_skill("jira", "2.0.0");
    env.write_lock(&[("jira", "2.0.0")]);

    let report = env.app().adopt().expect("adopt");
    assert!(
        report
            .migrations
            .iter()
            .any(|m| m.skill == "jira"
                && m.from_version == "1.0.0"
                && m.to_version == "2.0.0"
                && m.summary.contains("Rename jira-old")),
        "expected migration notice, got: {:?}",
        report.migrations
    );
}

#[test]
fn adopt_updates_installed_to_lock_but_skips_overrides() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# v1\n",
    }]);
    env.tag_skill("jira", "1.0.0");
    env.update_skill(SkillSpec {
        name: "jira",
        version: "1.1.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# v1.1\n",
    });
    env.tag_skill("jira", "1.1.0");
    env.update_skill(SkillSpec {
        name: "jira",
        version: "2.0.0-rc.1",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# rc\n",
    });
    env.tag_skill("jira", "2.0.0-rc.1");

    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);
    env.app().sync().expect("sync v1");

    // Team bumps lock to 1.1.0; Alice overrides to RC
    env.write_lock(&[("jira", "1.1.0")]);
    env.write_overrides(&[("jira", "2.0.0-rc.1")]);

    let report: AdoptReport = env.app().adopt().expect("adopt");
    assert!(
        report
            .skipped_overrides
            .iter()
            .any(|s| s == "jira"),
        "skipped: {:?}",
        report.skipped_overrides
    );

    let body = fs::read_to_string(env.claude_user_skill("jira").join("SKILL.md")).unwrap();
    assert!(body.contains("rc"), "override should remain after adopt: {body}");

    // Clear override and adopt again → lock 1.1.0
    env.write_overrides(&[]);
    let report2 = env.app().adopt().expect("adopt after clear");
    assert!(report2.skipped_overrides.is_empty());
    assert!(report2.updated.iter().any(|s| s == "jira"));
    let body = fs::read_to_string(env.claude_user_skill("jira").join("SKILL.md")).unwrap();
    assert!(body.contains("v1.1"), "{body}");
}

#[test]
fn sync_uses_project_scope_when_configured_not_both() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::Project);

    env.app().sync().expect("sync");
    assert!(env.claude_project_skill("jira").exists());
    assert!(!env.claude_user_skill("jira").exists());
}

#[test]
fn sync_places_into_all_enabled_harnesses() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(
        &["jira"],
        &[
            Harness::ClaudeCode,
            Harness::Codex,
            Harness::Cursor,
            Harness::Pi,
        ],
        PlacementScope::User,
    );

    let report = env.app().sync().expect("sync");
    assert_eq!(report.placements.len(), 4);
    assert!(env.claude_user_skill("jira").exists());
    assert!(env.codex_user_skill("jira").exists());
    assert!(env.cursor_user_skill("jira").exists());
    assert!(env.pi_user_skill("jira").exists());
}

#[test]
fn sync_falls_back_to_copy_when_symlink_fails() {
    let env = TestEnv::new().with_force_copy();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);

    let report = env.app().sync().expect("sync");
    assert_eq!(report.placements[0].kind, PlacementKind::Copy);
    assert!(report.warnings.iter().any(|w| w.to_lowercase().contains("copy")));
    assert!(env.claude_user_skill("jira").join("SKILL.md").exists());
    let meta = fs::symlink_metadata(env.claude_user_skill("jira")).unwrap();
    assert!(meta.file_type().is_dir() && !meta.file_type().is_symlink());
}

#[test]
fn clear_override_removes_personal_pin() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_overrides(&[("jira", "2.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);

    env.app().clear_override("jira").expect("clear");
    let status = env.app().status().expect("status");
    assert!(status.overrides.is_empty());
    let wanted = env.app().resolve_wanted().unwrap();
    assert_eq!(wanted[0].version, "1.0.0");
    assert!(!wanted[0].from_override);
}

#[test]
fn generate_catalog_includes_per_skill_install_snippet() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira helpers",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    env.ensure_clone();
    let out = env.paths().cache_dir.join("catalog-install");
    env.app().generate_catalog(&out).expect("catalog");

    let page = fs::read_to_string(out.join("jira/index.md")).expect("jira page");
    assert!(
        page.contains("skills-sync init") && page.contains("--skills jira"),
        "expected install snippet, got:\n{page}"
    );
    assert!(
        page.contains("skills-sync sync"),
        "expected sync step in install snippet:\n{page}"
    );
}

#[test]
fn generate_catalog_lists_skills_from_library_main_tree() {
    let env = TestEnv::new();
    env.init_library_repo(&[
        SkillSpec {
            name: "jira",
            version: "1.2.0",
            description: "Jira helpers",
            dependencies: &[],
            deprecated: false,
            successor: None,
            body: "# jira\n",
        },
        SkillSpec {
            name: "legacy",
            version: "0.1.0",
            description: "Old skill",
            dependencies: &[],
            deprecated: true,
            successor: Some("jira"),
            body: "# legacy\n## Migration\nUse jira.\n",
        },
    ]);
    // Catalog reads working tree on main (clone), not tags
    env.ensure_clone();

    let out = env.paths().cache_dir.join("catalog-out");
    fs::create_dir_all(&out).unwrap();
    env.app().generate_catalog(&out).expect("catalog");

    let index = fs::read_to_string(out.join("index.md")).expect("index.md");
    assert!(index.contains("jira"));
    assert!(index.contains("1.2.0"));
    assert!(index.contains("legacy"));
    assert!(index.contains("deprecated") || index.contains("Deprecated"));
}

#[test]
fn status_lists_active_overrides() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_overrides(&[("jira", "2.0.0-rc.1")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);

    let status = env.app().status().expect("status");
    assert_eq!(status.overrides.len(), 1);
    assert_eq!(status.overrides[0].name, "jira");
    assert_eq!(status.overrides[0].version, "2.0.0-rc.1");
}

#[test]
fn resolve_errors_when_enabled_skill_missing_from_lock() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira", "ghost"], &[Harness::ClaudeCode], PlacementScope::User);
    let err = env.app().resolve_wanted().expect_err("missing lock pin");
    assert!(err.to_string().contains("ghost"), "{err}");
}

#[test]
fn sync_errors_when_library_clone_missing() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);
    // Remove clone git + leave empty-ish; app checks .git or lock — remove both parents content
    let clone = env.paths().library_clone;
    fs::remove_dir_all(&clone).ok();
    let err = env.app().sync().expect_err("missing clone");
    assert!(
        err.to_string().contains("bootstrap") || err.to_string().contains("clone"),
        "{err}"
    );
}

#[test]
fn set_override_persists_and_shows_in_status() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);
    env.app().set_override("jira", "3.0.0").expect("set");
    let status = env.app().status().expect("status");
    assert_eq!(status.overrides[0].version, "3.0.0");
    assert_eq!(env.app().resolve_wanted().unwrap()[0].version, "3.0.0");
}

#[test]
fn sync_errors_on_invalid_harness_in_personal_config() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0")]);
    fs::create_dir_all(env.paths().library_clone.join(".git")).unwrap();
    fs::create_dir_all(env.paths().config_dir).unwrap();
    fs::write(
        env.paths().config_dir.join("personal.toml"),
        r#"skills = ["jira"]
harnesses = ["not-a-harness"]
placement_scope = "user"
"#,
    )
    .unwrap();
    let err = env.app().sync().expect_err("bad harness");
    assert!(err.to_string().contains("harness"), "{err}");
}

#[test]
fn sync_errors_on_invalid_placement_scope() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0")]);
    fs::create_dir_all(env.paths().library_clone.join(".git")).unwrap();
    fs::create_dir_all(env.paths().config_dir).unwrap();
    fs::write(
        env.paths().config_dir.join("personal.toml"),
        r#"skills = ["jira"]
harnesses = ["claude-code"]
placement_scope = "galaxy"
"#,
    )
    .unwrap();
    let err = env.app().sync().expect_err("bad scope");
    assert!(err.to_string().contains("placement_scope"), "{err}");
}

#[test]
fn sync_errors_when_skill_frontmatter_name_mismatches_folder() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    env.delete_tag("jira@1.0.0");
    fs::write(
        env.library_src().join("skills/jira/SKILL.md"),
        "---\nname: not-jira\ndescription: x\nversion: 1.0.0\n---\n\n# x\n",
    )
    .unwrap();
    env.commit_all("wrong name");
    env.tag_skill_raw("jira@1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);
    let err = env.app().sync().expect_err("name mismatch");
    assert!(
        err.to_string().contains("name") || err.to_string().contains("not-jira"),
        "{err}"
    );
}

#[test]
fn resolve_errors_on_malformed_lock_file() {
    let env = TestEnv::new();
    fs::create_dir_all(env.paths().library_clone).unwrap();
    fs::write(env.paths().lock_path(), "not = [valid toml").unwrap();
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);
    let err = env.app().resolve_wanted().expect_err("bad toml");
    assert!(!err.to_string().is_empty());
}

#[test]
fn generate_catalog_includes_migration_marker() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "2.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n\n## Migration\n\nBreaking changes here.\n",
    }]);
    env.ensure_clone();
    let out = env.paths().cache_dir.join("catalog-mig");
    env.app().generate_catalog(&out).unwrap();
    let index = fs::read_to_string(out.join("index.md")).unwrap();
    assert!(index.contains("migration"), "{index}");
}

#[test]
fn with_xdg_constructs_app() {
    let app = skills_sync::SkillsSync::with_xdg().expect("xdg");
    assert!(
        app.paths()
            .config_dir
            .ends_with(std::path::Path::new("skills-sync"))
    );
    assert!(
        app.paths()
            .cache_dir
            .ends_with(std::path::Path::new("skills-sync"))
    );
    assert!(app.paths().library_clone.ends_with("library"));
}

#[test]
fn resolve_errors_when_lock_file_missing() {
    let env = TestEnv::new();
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);
    fs::create_dir_all(env.paths().library_clone).unwrap();
    let err = env.app().resolve_wanted().expect_err("no lock");
    assert!(err.to_string().contains("Lock File") || err.to_string().contains("skills.lock"), "{err}");
}

#[test]
fn resolve_errors_when_personal_config_missing() {
    let env = TestEnv::new();
    env.write_lock(&[("jira", "1.0.0")]);
    let err = env.app().resolve_wanted().expect_err("no personal");
    assert!(err.to_string().contains("personal"), "{err}");
}

#[test]
fn sync_replaces_existing_placement() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# v1\n",
    }]);
    env.tag_skill("jira", "1.0.0");
    env.write_lock(&[("jira", "1.0.0")]);
    env.write_personal(&["jira"], &[Harness::ClaudeCode], PlacementScope::User);
    env.app().sync().unwrap();
    env.app().sync().unwrap();
    assert!(env.claude_user_skill("jira").exists());
}

#[test]
fn catalog_handles_deprecated_without_successor() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "legacy",
        version: "0.1.0",
        description: "gone",
        dependencies: &[],
        deprecated: true,
        successor: None,
        body: "# x\n",
    }]);
    env.ensure_clone();
    let out = env.paths().cache_dir.join("cat2");
    env.app().generate_catalog(&out).unwrap();
    let index = fs::read_to_string(out.join("index.md")).unwrap();
    assert!(index.contains("Deprecated"), "{index}");
}
