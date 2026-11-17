use crate::{
    domain::{Configuration, DottyPaths, StaticDottyPaths},
    persistence::read_toml,
};

use crate::services::{Error, Result};

pub struct PathsLoader;

impl PathsLoader {
    /// Resolves the path to `dotfiles_dir` from the on-disk `config.toml`.
    ///
    /// # Errors
    ///
    /// Return [`Error::Persistence`] if `config.toml` does not exist yet.
    pub fn load() -> Result<DottyPaths> {
        let static_paths = Self::static_paths()?;
        let config: Configuration = read_toml(&static_paths.config_toml())?;

        Ok(DottyPaths::initialize(
            static_paths,
            config.dotfiles_dir().to_path_buf(),
        ))
    }

    /// Constructs the internally used paths from XDG variables.
    ///
    /// # Errors
    ///
    /// Returns [`Error::BaseDirectories`] if XDG variables are not set.
    pub fn static_paths() -> Result<StaticDottyPaths> {
        let project_dirs =
            directories::ProjectDirs::from("", "", "dotty").ok_or(Error::BaseDirectories)?;

        let state_dir = project_dirs.state_dir().ok_or(Error::BaseDirectories)?;

        Ok(StaticDottyPaths::new(
            project_dirs.data_dir().to_path_buf(),
            state_dir.to_path_buf(),
            project_dirs.config_dir().to_path_buf(),
        ))
    }
}
