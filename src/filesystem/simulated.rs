use super::Result;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::filesystem::{Filesystem, PathKind};

pub struct SimulatedFilesystem {
    entries: HashMap<PathBuf, PathKind>,
}

impl SimulatedFilesystem {
    pub fn new(entries: HashMap<PathBuf, PathKind>) -> Self {
        Self { entries }
    }
}

impl Filesystem for SimulatedFilesystem {
    fn kind(&self, path: &Path) -> Result<Option<PathKind>> {
        Ok(self.entries.get(path).copied())
    }

    fn is_empty_dir(&self, path: &Path) -> Result<bool> {
        Ok(!self
            .entries
            .keys()
            .any(|entry| entry.parent() == Some(path)))
    }

    fn create_directory(&mut self, path: &Path) -> Result<()> {
        self.entries.insert(path.to_path_buf(), PathKind::Directory);
        Ok(())
    }

    fn remove_directory(&mut self, path: &Path) -> Result<()> {
        self.entries.remove(path);
        Ok(())
    }

    fn write_file(&mut self, path: &Path, _content: &[u8]) -> Result<()> {
        self.entries.insert(path.to_path_buf(), PathKind::File);
        Ok(())
    }

    fn remove_file(&mut self, path: &Path) -> Result<()> {
        self.entries.remove(path);
        Ok(())
    }

    fn rename(&mut self, from: &Path, to: &Path) -> Result<()> {
        let affected: Vec<_> = self
            .entries
            .keys()
            .filter(|path| path.starts_with(from))
            .cloned()
            .collect();

        for old in affected {
            let relative = old.strip_prefix(from).expect("path must be within source");

            let new = to.join(relative);

            if let Some(kind) = self.entries.remove(&old) {
                self.entries.insert(new, kind);
            };
        }

        Ok(())
    }

    fn create_symlink(&mut self, _target: &Path, link: &Path) -> Result<()> {
        self.entries.insert(link.to_path_buf(), PathKind::Symlink);

        Ok(())
    }

    fn remove_symlink(&mut self, path: &Path) -> Result<()> {
        self.entries.remove(path);
        Ok(())
    }

    fn copy_file(&mut self, from: &Path, to: &Path) -> Result<()> {
        if let Some(kind) = self.entries.get(from).copied() {
            self.entries.insert(to.to_path_buf(), kind);
        }

        Ok(())
    }

    fn copy_directory(&mut self, from: &Path, to: &Path) -> Result<()> {
        let affected: Vec<_> = self
            .entries
            .iter()
            .filter(|(path, _)| path.starts_with(from))
            .map(|(path, kind)| (path.clone(), *kind))
            .collect();

        for (old, kind) in affected {
            let relative = old.strip_prefix(from).expect("path must be within source");

            let new = to.join(relative);

            self.entries.insert(new, kind);
        }

        Ok(())
    }

    fn read_file(&mut self, _path: &Path) -> Result<Vec<u8>> {
        todo!()
    }

    fn read_symlink(&mut self, _path: &Path) -> Result<PathBuf> {
        todo!()
    }
}
