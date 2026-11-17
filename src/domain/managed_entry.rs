use super::Error;
use super::Result;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct ManagedEntry {
    source: ManagedSource,
    target: ManagedTarget,
    kind: EntryKind,
}

impl ManagedEntry {
    pub fn new(source: ManagedSource, target: ManagedTarget, kind: EntryKind) -> Self {
        Self {
            source,
            target,
            kind,
        }
    }

    pub fn kind(&self) -> &EntryKind {
        &self.kind
    }

    pub fn target(&self) -> &ManagedTarget {
        &self.target
    }

    pub fn source(&self) -> &ManagedSource {
        &self.source
    }

    pub fn resolve_paths(&self, dotfiles_dir: &Path, home: &Path) -> (PathBuf, PathBuf) {
        let source = dotfiles_dir.join(self.source().path());
        let target = home.join(self.target().path());

        (source, target)
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub enum EntryKind {
    Dir,
    File,
}

#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone)]
pub enum ManagedTarget {
    HomeRelative(PathBuf),
}

impl ManagedTarget {
    pub fn home_relative(path: PathBuf) -> Result<Self> {
        validate_managed_path(&path)?;

        Ok(Self::HomeRelative(path))
    }

    pub fn path(&self) -> &Path {
        match self {
            Self::HomeRelative(path) => path,
        }
    }
}

impl<'de> Deserialize<'de> for ManagedTarget {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        enum Repr {
            HomeRelative(PathBuf),
        }

        match Repr::deserialize(deserializer)? {
            Repr::HomeRelative(path) => Self::home_relative(path).map_err(serde::de::Error::custom),
        }
    }
}

#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone)]
pub struct ManagedSource(PathBuf);

impl ManagedSource {
    pub fn new(path: PathBuf) -> Result<Self> {
        validate_managed_path(&path)?;

        Ok(Self(path))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ManagedSource {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = PathBuf::deserialize(deserializer)?;

        Self::new(path).map_err(serde::de::Error::custom)
    }
}

fn validate_managed_path(path: &Path) -> Result<()> {
    if path.is_empty() || path == Path::new(".") {
        return Err(Error::EmptyManagedPath);
    }
    if path.is_absolute() {
        return Err(Error::AbsoluteManagedPath(path.to_path_buf()));
    }
    if path
        .components()
        .any(|component| component == Component::ParentDir)
    {
        return Err(Error::ManagedPathTraversal(path.to_path_buf()));
    }

    Ok(())
}
