//! Secondary seam: Release tagging helper.

mod support;

use std::fs;

use skills_sync::release::{
    detect_version_bumps, proposed_tags, validate_lock_tags, validate_tag_frontmatter,
};
use support::{SkillSpec, TestEnv};

#[test]
fn validate_lock_tags_accepts_existing_tag() {
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
    // Lock is written to library-src and clone; ensure clone has lock + tags
    env.ensure_clone();
    fs::write(
        env.paths().library_clone.join("skills.lock"),
        "[skills]\njira = \"1.0.0\"\n",
    )
    .unwrap();

    validate_lock_tags(&env.paths().library_clone).expect("lock tags valid");
}

#[test]
fn validate_lock_tags_accepts_head_frontmatter_when_tag_pending() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.1.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# v1.1\n",
    }]);
    // No tag yet; lock matches HEAD frontmatter (PR before release-tags runs)
    fs::write(
        env.paths().library_clone.join("skills.lock"),
        "[skills]\njira = \"1.1.0\"\n",
    )
    .unwrap();

    validate_lock_tags(&env.paths().library_clone).expect("pending tag ok when HEAD matches");
}

#[test]
fn validate_lock_tags_rejects_pin_with_neither_tag_nor_head_match() {
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
    fs::write(
        env.paths().library_clone.join("skills.lock"),
        "[skills]\njira = \"9.9.9\"\n",
    )
    .unwrap();

    let err = validate_lock_tags(&env.paths().library_clone).expect_err("bad pin");
    assert!(
        err.to_string().contains("jira@9.9.9") || err.to_string().contains("9.9.9"),
        "{err}"
    );
}

#[test]
fn detect_version_bump_proposes_name_at_semver_tag() {
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
    let lib = env.paths().library_clone;
    // Parent = initial; bump on new commit
    env.update_skill(SkillSpec {
        name: "jira",
        version: "1.1.0",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# v1.1\n",
    });

    let parent = git_rev(&lib, "HEAD~1");
    let head = git_rev(&lib, "HEAD");
    let bumps = detect_version_bumps(&lib, &parent, &head).expect("detect");
    assert_eq!(bumps, vec![("jira".into(), "1.1.0".into())]);
    let tags = proposed_tags(&bumps);
    assert_eq!(tags, vec!["jira@1.1.0".to_string()]);
}

#[test]
fn detect_ignores_skills_without_version_change() {
    let env = TestEnv::new();
    env.init_library_repo(&[
        SkillSpec {
            name: "jira",
            version: "1.0.0",
            description: "Jira",
            dependencies: &[],
            deprecated: false,
            successor: None,
            body: "# v1\n",
        },
        SkillSpec {
            name: "confluence",
            version: "0.1.0",
            description: "Conf",
            dependencies: &[],
            deprecated: false,
            successor: None,
            body: "# c\n",
        },
    ]);
    env.update_skill(SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira updated docs only",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# v1 docs\n",
    });
    let lib = env.paths().library_clone;
    let bumps = detect_version_bumps(&lib, "HEAD~1", "HEAD").expect("detect");
    assert!(bumps.is_empty(), "docs-only change must not tag: {bumps:?}");
}

#[test]
fn validate_tag_frontmatter_rejects_mismatch() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "9.9.9",
        description: "Jira",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# x\n",
    }]);
    let lib = env.paths().library_clone;
    let err = validate_tag_frontmatter(&lib, "jira", "1.0.0").expect_err("mismatch");
    assert!(err.to_string().contains("9.9.9") || err.to_string().contains("mismatch"));
}

fn git_rev(cwd: &std::path::Path, rev: &str) -> String {
    let out = std::process::Command::new("git")
        .args(["rev-parse", rev])
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}
