use crate::services::{Result, snapshot::rules::SnapshotRules};
use std::path::Path;

use crate::filesystem::Filesystem;

pub struct SnapshotValidator;

impl SnapshotValidator {
    pub fn validate(fs: &impl Filesystem, path: &Path) -> Result<()> {
        SnapshotRules::require_normalized(path)?;
        SnapshotRules::require_no_symlink_ancestors(fs, path)?;

        Ok(())
    }
}
