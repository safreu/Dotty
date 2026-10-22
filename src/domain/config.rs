use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// Chosen by user when calling init and
    /// will be used to store the managed files and directories
    dotfiles_dir: PathBuf,
}

impl Configuration {
    pub fn new(dotfiles_dir: PathBuf) -> Self {
        Self { dotfiles_dir }
    }

    pub fn dotfiles_dir(&self) -> &PathBuf {
        &self.dotfiles_dir
    }
}
