use crate::filesystem::{Filesystem, PathKind};

use crate::services::{Error, Result};
use std::path::{Component, Path};

/// Contains various path validation methods.
pub struct SnapshotRules;

impl SnapshotRules {
    pub fn require_no_symlink_ancestors(fs: &impl Filesystem, path: &Path) -> Result<()> {
        for ancestor in path.ancestors().skip(1) {
            if fs.kind(ancestor)? == Some(PathKind::Symlink) {
                return Err(Error::Symlink(ancestor.to_path_buf()));
            }
        }

        Ok(())
    }

    pub fn require_normalized(path: &Path) -> Result<()> {
        if path
            .components()
            .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
        {
            return Err(Error::NotNormalized(path.to_path_buf()));
        }

        Ok(())
    }
}
