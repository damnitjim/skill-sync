//! Seam: validate_skills authoring rules on a library tree.

mod support;

use std::fs;

use skills_sync::validate_skills;
use support::{SkillSpec, TestEnv};

fn write_skill_md(lib: &std::path::Path, name: &str, body: &str) {
    let dir = lib.join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), body).unwrap();
}

fn lock(lib: &std::path::Path, pins: &[(&str, &str)]) {
    let mut body = String::from("[skills]\n");
    for (n, v) in pins {
        body.push_str(&format!("{n} = \"{v}\"\n"));
    }
    fs::write(lib.join("skills.lock"), body).unwrap();
}

#[test]
fn validate_skills_accepts_valid_tree() {
    let env = TestEnv::new();
    env.init_library_repo(&[SkillSpec {
        name: "jira",
        version: "1.0.0",
        description: "Jira helpers for DE",
        dependencies: &[],
        deprecated: false,
        successor: None,
        body: "# jira\n",
    }]);
    env.write_lock(&[("jira", "1.0.0")]);
    // validate against library-src (working tree with skills + lock)
    let lib = env.library_src();
    fs::write(
        lib.join("skills.lock"),
        "[skills]\njira = \"1.0.0\"\n",
    )
    .unwrap();

    validate_skills(&lib).expect("valid tree");
}

#[test]
fn validate_skills_rejects_missing_version_field() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills")).unwrap();
    write_skill_md(
        &lib,
        "jira",
        "---\nname: jira\ndescription: Jira helpers\n---\n\n# jira\n",
    );
    lock(&lib, &[("jira", "1.0.0")]);

    let err = validate_skills(&lib).expect_err("missing version");
    assert!(
        err.to_string().contains("version") || err.to_string().contains("jira"),
        "{err}"
    );
}

#[test]
fn validate_skills_rejects_name_folder_mismatch() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills")).unwrap();
    write_skill_md(
        &lib,
        "jira",
        "---\nname: not-jira\ndescription: Jira helpers\nversion: 1.0.0\n---\n\n# x\n",
    );
    lock(&lib, &[("jira", "1.0.0")]);

    let err = validate_skills(&lib).expect_err("name mismatch");
    assert!(
        err.to_string().contains("name") || err.to_string().contains("not-jira"),
        "{err}"
    );
}

#[test]
fn validate_skills_rejects_invalid_semver() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills")).unwrap();
    write_skill_md(
        &lib,
        "jira",
        "---\nname: jira\ndescription: Jira helpers\nversion: not-a-version\n---\n\n# jira\n",
    );
    lock(&lib, &[("jira", "not-a-version")]);

    let err = validate_skills(&lib).expect_err("bad semver");
    assert!(
        err.to_string().contains("semver")
            || err.to_string().contains("version")
            || err.to_string().contains("not-a-version"),
        "{err}"
    );
}

#[test]
fn validate_skills_rejects_empty_description() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills")).unwrap();
    write_skill_md(
        &lib,
        "jira",
        "---\nname: jira\ndescription: \"   \"\nversion: 1.0.0\n---\n\n# jira\n",
    );
    lock(&lib, &[("jira", "1.0.0")]);

    let err = validate_skills(&lib).expect_err("empty description");
    assert!(err.to_string().to_lowercase().contains("description"), "{err}");
}

#[test]
fn validate_skills_rejects_major_without_migration_docs() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills")).unwrap();
    write_skill_md(
        &lib,
        "jira",
        "---\nname: jira\ndescription: Jira helpers\nversion: 2.0.0\n---\n\n# jira\n",
    );
    lock(&lib, &[("jira", "2.0.0")]);

    let err = validate_skills(&lib).expect_err("missing migration");
    assert!(
        err.to_string().to_lowercase().contains("migration"),
        "{err}"
    );
}

#[test]
fn validate_skills_accepts_major_with_migration_section() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills")).unwrap();
    write_skill_md(
        &lib,
        "jira",
        "---\nname: jira\ndescription: Jira helpers\nversion: 2.0.0\n---\n\n# jira\n\n## Migration\n\nUpdate scripts.\n",
    );
    lock(&lib, &[("jira", "2.0.0")]);

    validate_skills(&lib).expect("major with migration ok");
}

#[test]
fn validate_skills_rejects_unknown_dependency() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills")).unwrap();
    write_skill_md(
        &lib,
        "jira",
        "---\nname: jira\ndescription: Jira helpers\nversion: 1.0.0\ndependencies:\n  - ghost\n---\n\n# jira\n",
    );
    lock(&lib, &[("jira", "1.0.0")]);

    let err = validate_skills(&lib).expect_err("unknown dep");
    assert!(err.to_string().contains("ghost"), "{err}");
}

#[test]
fn validate_skills_rejects_skill_missing_from_lock() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills")).unwrap();
    write_skill_md(
        &lib,
        "jira",
        "---\nname: jira\ndescription: Jira helpers\nversion: 1.0.0\n---\n\n# jira\n",
    );
    lock(&lib, &[]);

    let err = validate_skills(&lib).expect_err("not in lock");
    assert!(
        err.to_string().contains("lock") || err.to_string().contains("jira"),
        "{err}"
    );
}

#[test]
fn validate_skills_rejects_lock_pin_for_missing_skill() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills")).unwrap();
    write_skill_md(
        &lib,
        "jira",
        "---\nname: jira\ndescription: Jira helpers\nversion: 1.0.0\n---\n\n# jira\n",
    );
    lock(&lib, &[("jira", "1.0.0"), ("ghost", "1.0.0")]);

    let err = validate_skills(&lib).expect_err("ghost in lock");
    assert!(err.to_string().contains("ghost"), "{err}");
}

#[test]
fn validate_skills_rejects_orphan_skill_directory() {
    let env = TestEnv::new();
    let lib = env.library_src();
    fs::create_dir_all(lib.join("skills/jira")).unwrap();
    // no SKILL.md
    lock(&lib, &[]);

    let err = validate_skills(&lib).expect_err("orphan dir");
    assert!(
        err.to_string().contains("SKILL.md") || err.to_string().contains("jira"),
        "{err}"
    );
}
