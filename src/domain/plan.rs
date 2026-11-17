use super::{Error, Result};
use std::path::{Path, PathBuf};

use crate::{
    domain::Action,
    utils::{Filesystem, PathKind},
};

pub struct Plan {
    actions: Vec<Action>,
}

impl Plan {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn into_actions(self) -> Vec<Action> {
        self.actions
    }

    pub fn extend(&mut self, other: Plan) {
        self.actions.extend(other.actions);
    }

    pub fn missing_directories(
        fs: &impl Filesystem,
        root: &Path,
        target: &Path,
    ) -> Result<Vec<PathBuf>> {
        let mut current = root.to_path_buf();
        let mut missing = Vec::new();

        match fs.kind(root)? {
            Some(PathKind::Directory) => {}
            _ => {
                return Err(Error::InvalidDirectory {
                    path: root.to_path_buf(),
                });
            }
        };

        let relative = target
            .strip_prefix(root)
            .map_err(|_| Error::PathOutsideRoot {
                root: root.to_path_buf(),
                target: target.to_path_buf(),
            })?;

        for component in relative.components() {
            current.push(component);

            match fs.kind(&current)? {
                Some(PathKind::Directory) => {}
                None => missing.push(current.clone()),
                Some(_) => return Err(Error::InvalidDirectory { path: current }),
            }
        }

        Ok(missing)
    }

    pub fn actions(&self) -> &[Action] {
        &self.actions
    }
}
