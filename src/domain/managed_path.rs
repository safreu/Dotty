use super::{Error, Result};
use std::{
    os::unix::ffi::OsStrExt,
    path::{Component, Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone)]
#[serde(transparent)]
pub struct ManagedPath(PathBuf);

impl ManagedPath {
    pub fn new(path: PathBuf) -> Result<Self> {
        if path.is_empty() || path == Path::new(".") {
            return Err(Error::EmptyManagedPath);
        }
        if path.is_absolute() {
            return Err(Error::AbsoluteManagedPath(path));
        }
        if path
            .components()
            .any(|c| matches!(c, Component::CurDir | Component::ParentDir))
        {
            return Err(Error::ManagedPathTraversal(path));
        }
        if path
            .as_os_str()
            .as_bytes()
            .split(|&byte| byte == b'/')
            .any(|segment| segment == b"." || segment == b"..")
        {
            return Err(Error::ManagedPathTraversal(path));
        }
        Ok(Self(path))
    }

    pub fn from_target(path: PathBuf, home: &Path) -> Result<Self> {
        let path = if path.is_absolute() {
            path.strip_prefix(home)?.to_path_buf()
        } else {
            path
        };

        Self::new(path)
    }

    pub fn from_source(source: Option<PathBuf>, target: &ManagedPath) -> Result<Self> {
        match source {
            Some(source) => Self::new(source),
            None => {
                let name = target
                    .path()
                    .file_name()
                    .ok_or_else(|| Error::InvalidManagedPath(target.path().to_path_buf()))?
                    .to_string_lossy();

                Self::new(PathBuf::from(name.strip_prefix(".").unwrap_or(&name)))
            }
        }
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn resolve(&self, root: &Path) -> PathBuf {
        root.join(&self.0)
    }
}

impl<'de> Deserialize<'de> for ManagedPath {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = PathBuf::deserialize(deserializer)?;
        Self::new(path).map_err(serde::de::Error::custom)
    }
}
