use super::Result;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    domain::SnapshotEntry,
    filesystem::{Filesystem, PathKind},
};

pub struct SimulatedFilesystem {
    entries: HashMap<PathBuf, SnapshotEntry>,
}

impl SimulatedFilesystem {
    pub fn new(entries: HashMap<PathBuf, SnapshotEntry>) -> Self {
        Self { entries }
    }

    pub fn origin(&self, path: &Path) -> Option<&Path> {
        self.entries
            .get(path)
            .and_then(|entry| entry.origin().map(PathBuf::as_path))
    }
}

impl Filesystem for SimulatedFilesystem {
    fn kind(&self, path: &Path) -> Result<Option<PathKind>> {
        Ok(self.entries.get(path).map(SnapshotEntry::kind))
    }

    fn is_empty_dir(&self, path: &Path) -> Result<bool> {
        Ok(!self
            .entries
            .keys()
            .any(|entry| entry.parent() == Some(path)))
    }

    fn create_directory(&mut self, path: &Path) -> Result<()> {
        self.entries.insert(
            path.to_path_buf(),
            SnapshotEntry::new(PathKind::Directory, None),
        );
        Ok(())
    }

    fn remove_directory(&mut self, path: &Path) -> Result<()> {
        self.entries.remove(path);
        Ok(())
    }

    fn remove_directory_all(&mut self, path: &Path) -> Result<()> {
        self.entries.retain(|entry, _| !entry.starts_with(path));
        Ok(())
    }

    fn write_file(&mut self, path: &Path, _content: &[u8]) -> Result<()> {
        self.entries
            .insert(path.to_path_buf(), SnapshotEntry::new(PathKind::File, None));
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

            if let Some(entry) = self.entries.remove(&old) {
                self.entries.insert(new, entry);
            };
        }

        Ok(())
    }

    fn create_symlink(&mut self, _target: &Path, link: &Path) -> Result<()> {
        self.entries.insert(
            link.to_path_buf(),
            SnapshotEntry::new(PathKind::Symlink, None),
        );

        Ok(())
    }

    fn remove_symlink(&mut self, path: &Path) -> Result<()> {
        self.entries.remove(path);
        Ok(())
    }

    fn copy_file(&mut self, from: &Path, to: &Path) -> Result<()> {
        if let Some(entry) = self.entries.get(from) {
            self.entries
                .insert(to.to_path_buf(), SnapshotEntry::new(entry.kind(), None));
        }

        Ok(())
    }

    fn copy_directory(&mut self, from: &Path, to: &Path) -> Result<()> {
        let affected: Vec<_> = self
            .entries
            .iter()
            .filter(|(path, _)| path.starts_with(from))
            .map(|(path, entry)| (path.clone(), entry.kind()))
            .collect();

        for (old, kind) in affected {
            let relative = old.strip_prefix(from).expect("path must be within source");
            let new = to.join(relative);

            self.entries.insert(new, SnapshotEntry::new(kind, None));
        }

        Ok(())
    }

    fn copy_symlink(&mut self, _from: &Path, to: &Path) -> Result<()> {
        self.entries.insert(
            to.to_path_buf(),
            SnapshotEntry::new(PathKind::Symlink, None),
        );

        Ok(())
    }

    fn read_file(&mut self, _path: &Path) -> Result<Vec<u8>> {
        todo!()
    }

    fn read_symlink(&mut self, _path: &Path) -> Result<PathBuf> {
        todo!()
    }
}
