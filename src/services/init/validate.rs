use crate::services::Result;
use crate::services::init::rules::InitRules;
use crate::{domain::DottyPaths, filesystem::Filesystem};

pub struct InitValidator;

impl InitValidator {
    pub fn validate(fs: &impl Filesystem, paths: &DottyPaths) -> Result<()> {
        InitRules::require_absolute(paths.dynamic_dirs().dotfiles_dir())?;

        for path in paths.files() {
            InitRules::require_absent(fs, &path)?;
        }

        Ok(())
    }
}
