use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
    pub version: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub deprecated: bool,
    pub successor: Option<String>,
}

pub fn parse_skill_md(path: &Path) -> Result<SkillFrontmatter, Error> {
    let text = fs::read_to_string(path)?;
    parse_skill_md_str(&text)
}

pub fn parse_skill_md_str(text: &str) -> Result<SkillFrontmatter, Error> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("---") {
        return Err(Error::msg("SKILL.md missing YAML frontmatter"));
    }
    let rest = &trimmed[3..];
    let end = rest
        .find("\n---")
        .ok_or_else(|| Error::msg("SKILL.md frontmatter not closed"))?;
    let yaml = &rest[..end];
    Ok(serde_yaml::from_str(yaml)?)
}
