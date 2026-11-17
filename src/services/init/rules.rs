use crate::{
    filesystem::Filesystem,
    services::{Error, Result},
};
use std::path::Path;

pub struct InitRules;

impl InitRules {
    pub fn require_absent(fs: &impl Filesystem, path: &Path) -> Result<()> {
        if fs.kind(path)?.is_some() {
            return Err(Error::PathExists(path.to_path_buf()));
        }

        Ok(())
    }

    pub fn require_absolute(path: &Path) -> Result<()> {
        if !path.is_absolute() {
            return Err(Error::InvalidDotfilesPath(path.to_path_buf()));
        }

        Ok(())
    }
}
