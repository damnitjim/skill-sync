//! skills-sync application API (primary test seam).

mod catalog;
mod error;
mod git_cache;
mod lock;
mod personal;
mod place;
pub mod release;
mod skill_meta;
mod sync_ops;

pub use error::Error;
pub use sync_ops::{
    AdoptReport, Harness, OverridePin, PlacementKind, PlacementRecord, PlacementScope, SkillStatus,
    StatusReport, SyncPaths, SyncReport, WantedSkill,
};

use std::path::Path;

use sync_ops::SkillsSyncInner;

/// Facade used by CLI, TUI, and integration tests.
pub struct SkillsSync {
    inner: SkillsSyncInner,
}

impl SkillsSync {
    pub fn new(paths: SyncPaths) -> Self {
        Self {
            inner: SkillsSyncInner::new(paths),
        }
    }

    pub fn with_xdg() -> Result<Self, Error> {
        Ok(Self::new(SyncPaths::xdg()?))
    }

    pub fn resolve_wanted(&self) -> Result<Vec<WantedSkill>, Error> {
        self.inner.resolve_wanted()
    }

    pub fn sync(&self) -> Result<SyncReport, Error> {
        self.inner.sync()
    }

    pub fn adopt(&self) -> Result<AdoptReport, Error> {
        self.inner.adopt()
    }

    pub fn status(&self) -> Result<StatusReport, Error> {
        self.inner.status()
    }

    pub fn clear_override(&self, skill: &str) -> Result<(), Error> {
        self.inner.clear_override(skill)
    }

    pub fn set_override(&self, skill: &str, version: &str) -> Result<(), Error> {
        self.inner.set_override(skill, version)
    }

    pub fn generate_catalog(&self, out_dir: &Path) -> Result<(), Error> {
        catalog::generate(&self.inner.paths.library_clone, out_dir)
    }

    pub fn paths(&self) -> &SyncPaths {
        &self.inner.paths
    }
}
