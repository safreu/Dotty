use crate::services::Result;
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use crate::{
    domain::{
        Action::{self, CreateDirectory},
        Plan,
    },
    filesystem::{Filesystem, PathKind},
    services::Error,
};

pub struct PlanBuilder {
    plan: Plan,
    directories: BTreeSet<PathBuf>,
}

impl PlanBuilder {
    pub fn new() -> Self {
        Self {
            plan: Plan::new(),
            directories: BTreeSet::new(),
        }
    }

    pub fn push(mut self, action: Action) -> Self {
        self.plan.push(action);
        self
    }

    pub fn build(self) -> Plan {
        let mut plan = Plan::new();

        for directory in self.directories {
            plan.push(CreateDirectory(directory));
        }

        plan.extend(self.plan);
        plan
    }

    pub fn ensure_directories(
        mut self,
        fs: &impl Filesystem,
        root: &Path,
        target: &Path,
    ) -> Result<Self> {
        for directory in Self::missing_directories(fs, root, target)? {
            self.directories.insert(directory);
        }

        Ok(self)
    }

    pub fn move_to_dotfiles(
        self,
        fs: &impl Filesystem,
        home: &Path,
        source: &Path,
        target: &Path,
    ) -> Result<Self> {
        let builder = self.ensure_directories(
            fs,
            home,
            source
                .parent()
                .expect("parent must have a parent directory"),
        )?;

        Ok(builder.push(Action::Rename {
            from: target.to_path_buf(),
            to: source.to_path_buf(),
        }))
    }

    pub fn create_managed_symlink(self, source: &Path, target: &Path) -> Self {
        self.push(Action::CreateSymlink {
            target: source.to_path_buf(),
            link: target.to_path_buf(),
        })
    }

    pub fn write_file(self, path: &Path, content: Vec<u8>) -> Self {
        self.push(Action::WriteFile {
            path: path.to_path_buf(),
            content,
        })
    }

    fn missing_directories(
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
}
