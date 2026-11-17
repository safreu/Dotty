use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::ManagedPath;

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct ManagedEntry {
    source: ManagedPath,
    target: ManagedPath,
    kind: EntryKind,
}

impl ManagedEntry {
    pub fn new(source: ManagedPath, target: ManagedPath, kind: EntryKind) -> Self {
        Self {
            source,
            target,
            kind,
        }
    }

    pub fn kind(&self) -> &EntryKind {
        &self.kind
    }

    pub fn resolve_paths(&self, dotfiles_dir: &Path, home: &Path) -> (PathBuf, PathBuf) {
        (
            self.source().resolve(dotfiles_dir),
            self.target().resolve(home),
        )
    }

    pub fn source(&self) -> &ManagedPath {
        &self.source
    }

    pub fn target(&self) -> &ManagedPath {
        &self.target
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub enum EntryKind {
    Dir,
    File,
}
