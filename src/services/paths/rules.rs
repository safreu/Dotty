use crate::filesystem::{Filesystem, PathKind};

use crate::services::{Error, Result};
use std::path::Path;

/// Contains various path validation methods.
pub struct PathRules;

impl PathRules {
    pub fn requires_home_relative(path: &Path) -> Result<()> {
        let home = Path::new(&std::env::var("HOME")?).to_path_buf();

        if !path.starts_with(home) {
            return Err(Error::NotHomeRelative(path.to_path_buf()));
        }

        Ok(())
    }

    pub fn requires_parent_dir(fs: &impl Filesystem, path: &Path) -> Result<()> {
        let parent = path
            .parent()
            .ok_or_else(|| Error::MissingParent(path.to_path_buf()))?;

        Self::require_kind(fs, parent, PathKind::Directory)
    }

    pub fn requires_absent(fs: &impl Filesystem, path: &Path) -> Result<()> {
        if fs.kind(path)?.is_some() {
            return Err(Error::PathExists(path.to_path_buf()));
        }

        Ok(())
    }

    pub fn requires_existence(fs: &impl Filesystem, path: &Path) -> Result<()> {
        if fs.kind(path)?.is_none() {
            return Err(Error::PathNotExists(path.to_path_buf()));
        }

        Ok(())
    }

    pub fn require_kind(fs: &impl Filesystem, path: &Path, kind: PathKind) -> Result<()> {
        match fs.kind(path)? {
            Some(actual) if actual == kind => Ok(()),
            Some(_) => Err(Error::UnsupportedFiletype(path.to_path_buf())),
            None => Err(Error::PathNotExists(path.to_path_buf())),
        }
    }

    pub fn require_not_kind(fs: &impl Filesystem, path: &Path, kind: PathKind) -> Result<()> {
        match fs.kind(path)? {
            Some(actual) if actual == kind => Err(Error::UnsupportedFiletype(path.to_path_buf())),
            _ => Ok(()),
        }
    }

    pub fn require_empty_dir(fs: &impl Filesystem, path: &Path) -> Result<()> {
        match fs.is_empty_dir(path)? {
            true => Ok(()),
            false => Err(Error::DirNotEmpty(path.to_path_buf())),
        }
    }

    pub fn require_outside_source(from: &Path, to: &Path) -> Result<()> {
        if to.starts_with(from) {
            return Err(Error::PathInsideSource {
                from: from.to_path_buf(),
                to: to.to_path_buf(),
            });
        }

        Ok(())
    }
}
