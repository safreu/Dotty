use crate::{
    domain::SnapshotEntry,
    services::{Result, snapshot::validate::SnapshotValidator},
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    domain::{Action, Plan},
    filesystem::{self, Filesystem, FsOperation, PathKind, RealFileSystem, SimulatedFilesystem},
};

use filesystem::Error as FilesystemError;

pub struct SnapshotBuilder {
    entries: HashMap<PathBuf, SnapshotEntry>,
}

impl SnapshotBuilder {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn build(&mut self, plan: &Plan) -> Result<SimulatedFilesystem> {
        for action in plan.actions() {
            match action {
                Action::CreateDirectory(path)
                | Action::WriteFile { path, .. }
                | Action::RemoveFile(path)
                | Action::RemoveSymlink(path) => self.snapshot_path(path)?,

                Action::RemoveDirectory(path) => self.snapshot_children(path)?,
                Action::RemoveDirectoryAll(path) => self.snapshot_tree(path)?,

                Action::Rename { from, to } | Action::CopyDirectory { from, to } => {
                    self.snapshot_tree(from)?;
                    self.snapshot_path(to)?;
                }

                Action::CopyFile { from, to } | Action::CopySymlink { from, to } => {
                    self.snapshot_path(from)?;
                    self.snapshot_path(to)?;
                }

                Action::CreateSymlink { target, link } => {
                    self.snapshot_path(target)?;
                    self.snapshot_path(link)?;
                }
            }
        }
        Ok(SimulatedFilesystem::new(self.entries.clone()))
    }

    fn snapshot_path(&mut self, path: &Path) -> Result<()> {
        let fs = RealFileSystem;

        SnapshotValidator::validate(&fs, path)?;

        for ancestor in path.ancestors() {
            if self.entries.contains_key(ancestor) {
                continue;
            }

            if let Some(kind) = fs.kind(ancestor)? {
                self.entries.insert(
                    ancestor.to_path_buf(),
                    SnapshotEntry::new(kind, Some(ancestor.to_path_buf())),
                );
            }
        }

        Ok(())
    }

    fn snapshot_children(&mut self, path: &Path) -> Result<()> {
        self.snapshot_path(path)?;

        let fs = RealFileSystem;

        if fs.kind(path)? != Some(PathKind::Directory) {
            return Ok(());
        }

        for entry in
            fs::read_dir(path).map_err(FilesystemError::io_single(FsOperation::Inspect, path))?
        {
            let entry = entry.map_err(FilesystemError::io_single(FsOperation::Inspect, path))?;

            self.snapshot_path(&entry.path())?;
        }

        Ok(())
    }

    fn snapshot_tree(&mut self, path: &Path) -> Result<()> {
        self.snapshot_path(path)?;

        if self.entries.get(path).map(SnapshotEntry::kind) != Some(PathKind::Directory) {
            return Ok(());
        }

        for entry in
            fs::read_dir(path).map_err(FilesystemError::io_single(FsOperation::Inspect, path))?
        {
            let entry = entry.map_err(FilesystemError::io_single(FsOperation::Inspect, path))?;

            let child = entry.path();
            self.snapshot_tree(&child)?;
        }

        Ok(())
    }
}
