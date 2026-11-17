use crate::{
    filesystem::PathKind,
    services::{Error, Result},
};
use std::path::Path;

pub struct ManageRules;

impl ManageRules {
    pub fn require_manageable(path: &Path, kind: Option<PathKind>) -> Result<()> {
        match kind {
            Some(PathKind::Directory | PathKind::File) => Ok(()),
            _ => Err(Error::UnsupportedFiletype(path.to_path_buf())),
        }
    }
}
