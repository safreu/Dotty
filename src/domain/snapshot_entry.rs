use std::path::PathBuf;

use crate::filesystem::PathKind;

#[derive(Debug, Clone)]
pub struct SnapshotEntry {
    kind: PathKind,
    origin: Option<PathBuf>,
}

impl SnapshotEntry {
    pub fn new(kind: PathKind, origin: Option<PathBuf>) -> Self {
        Self { kind, origin }
    }

    pub fn kind(&self) -> PathKind {
        self.kind
    }

    pub fn origin(&self) -> Option<&PathBuf> {
        self.origin.as_ref()
    }
}
