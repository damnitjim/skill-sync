use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::sync_ops::{Harness, PlacementScope};
use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalConfig {
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub harnesses: Vec<String>,
    #[serde(default = "default_scope")]
    pub placement_scope: String,
}

fn default_scope() -> String {
    "user".into()
}

impl PersonalConfig {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let text = fs::read_to_string(path).map_err(|e| {
            Error::msg(format!(
                "failed to read personal config at {}: {e} (run init)",
                path.display()
            ))
        })?;
        Ok(toml::from_str(&text)?)
    }

    pub fn harnesses(&self) -> Result<Vec<Harness>, Error> {
        self.harnesses
            .iter()
            .map(|h| Harness::parse(h))
            .collect()
    }

    pub fn placement_scope(&self) -> Result<PlacementScope, Error> {
        match self.placement_scope.as_str() {
            "user" => Ok(PlacementScope::User),
            "project" => Ok(PlacementScope::Project),
            other => Err(Error::msg(format!("invalid placement_scope: {other}"))),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OverridesFile {
    #[serde(default)]
    pub overrides: BTreeMap<String, String>,
}

impl OverridesFile {
    pub fn load(path: &Path) -> Result<Self, Error> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}
