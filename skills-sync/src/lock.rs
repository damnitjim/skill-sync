use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LockFile {
    #[serde(default)]
    pub skills: BTreeMap<String, String>,
}

impl LockFile {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let text = fs::read_to_string(path).map_err(|e| {
            Error::msg(format!(
                "failed to read Lock File at {}: {e}",
                path.display()
            ))
        })?;
        Ok(toml::from_str(&text)?)
    }
}
