//! Responsible for validating existence of the Paths from DottyPaths.
use crate::{domain::DottyPaths, services::errors::Error};

use super::Result;

pub struct PathValidator;

impl PathValidator {}
pub struct InitValidator;

impl InitValidator {
    /// Validates if init was already called, by checking the existing of the paths.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Symlink`] if one of the paths points to a symlink.
    /// Returns [`Error::NotInitialized`] if one of the paths doesn' exist yet.
    pub fn validate(paths: DottyPaths) -> Result<()> {
        for path in paths.dirs() {
            if path.is_symlink() {
                return Err(Error::Symlink(path));
            }

            if !path.exists() {
                return Err(Error::NotInitialized);
            }
        }

        Ok(())
    }

    pub fn requires_init(paths: DottyPaths) -> Result<()> {
        InitValidator::validate(paths)
    }
}
