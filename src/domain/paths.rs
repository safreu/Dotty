//! Manages the paths dotty uses internally.
use crate::{domain::config::Configuration, persistence::read_toml};

use super::{Error, Result};
use std::path::PathBuf;
pub struct StaticDottyPaths {
    /// Data directory (e.g `~/.local/share/dotty` on Linux).
    /// Holds backups.toml and `backups_dir`.
    share_dir: PathBuf,

    /// Backups directory (e.g `~/.local/share/dotty/backups` on Linux).
    /// Holds backups of managed entries.
    backups_dir: PathBuf,

    /// State directory (e.g `~/.local/state/dotty` on Linux).
    /// Holds config.toml.
    state_dir: PathBuf,

    /// Config directory (e.g `~/.config/dotty` on Linux).
    /// Holds manifest.toml.
    config_dir: PathBuf,
}

impl StaticDottyPaths {
    pub fn share_dir(&self) -> &PathBuf {
        &self.share_dir
    }

    pub fn backups_dir(&self) -> &PathBuf {
        &self.backups_dir
    }

    pub fn backups_toml(&self) -> PathBuf {
        self.share_dir.join("backups.toml")
    }

    pub fn state_dir(&self) -> &PathBuf {
        &self.state_dir
    }

    pub fn config_toml(&self) -> PathBuf {
        self.state_dir.join("config.toml").to_path_buf()
    }

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn manifest_toml(&self) -> PathBuf {
        self.config_dir.join("manifest.toml")
    }

    /// Constructs the internally used paths from XDG variables.
    ///
    /// # Errors
    ///
    /// Returns [`Error::BaseDirectories`] if XDG variables are not set.
    pub fn new() -> Result<Self> {
        let project_dirs =
            directories::ProjectDirs::from("", "", "dotty").ok_or(Error::BaseDirectories)?;

        let share_dir = project_dirs.data_dir();
        let state_dir = project_dirs.state_dir().ok_or(Error::BaseDirectories)?;
        let config_dir = project_dirs.config_dir();

        Ok(Self {
            share_dir: share_dir.to_path_buf(),
            state_dir: state_dir.to_path_buf(),
            backups_dir: share_dir.join("backups"),
            config_dir: config_dir.to_path_buf(),
        })
    }

    /// Resolves the path to `dotfiles_dir` from the on-disk `config.toml`.
    ///
    /// # Errors
    ///
    /// Return [`Error::Persistence`] if `config.toml` does not exist yet.
    pub fn load(self) -> Result<DottyPaths> {
        let config: Configuration = read_toml(&self.config_toml())?;
        let dotfiles_dir = config.dotfiles_dir();

        Ok(DottyPaths {
            static_dirs: self,
            dynamic_dirs: DynamicDottyPaths {
                dotfiles_dir: dotfiles_dir.to_path_buf(),
            },
        })
    }
}

pub struct DynamicDottyPaths {
    /// Dotfiles directory, chosen by user when calling init.
    /// Holds the directories and files managed by dotty.
    dotfiles_dir: PathBuf,
}

impl DynamicDottyPaths {
    pub fn dotfiles_dir(&self) -> &PathBuf {
        &self.dotfiles_dir
    }
}

pub struct DottyPaths {
    static_dirs: StaticDottyPaths,
    dynamic_dirs: DynamicDottyPaths,
}

impl DottyPaths {
    pub fn static_dirs(&self) -> &StaticDottyPaths {
        &self.static_dirs
    }

    pub fn dynamic_dirs(&self) -> &DynamicDottyPaths {
        &self.dynamic_dirs
    }

    pub fn dirs(&self) -> impl Iterator<Item = PathBuf> {
        [
            self.static_dirs().share_dir().to_path_buf(),
            self.static_dirs().backups_dir().to_path_buf(),
            self.static_dirs().state_dir().to_path_buf(),
            self.static_dirs().config_dir().to_path_buf(),
            self.dynamic_dirs().dotfiles_dir().to_path_buf(),
        ]
        .into_iter()
    }

    pub fn files(&self) -> impl Iterator<Item = PathBuf> {
        [
            self.static_dirs().backups_toml(),
            self.static_dirs().config_toml(),
            self.static_dirs().manifest_toml(),
        ]
        .into_iter()
    }
}
