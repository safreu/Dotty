use crate::domain::Action;
use crate::domain::BackupEntry;
use crate::services::{Error, Result};

use std::path::Path;

use crate::filesystem::PathKind;

pub struct BackupBuilder;

impl BackupBuilder {
    pub fn build(
        original_path: &Path,
        destination: &Path,
        kind: PathKind,
    ) -> Result<(BackupEntry, Action)> {
        let action = match kind {
            PathKind::File => Action::CopyFile {
                from: original_path.to_path_buf(),
                to: destination.to_path_buf(),
            },
            PathKind::Directory => Action::CopyDirectory {
                from: original_path.to_path_buf(),
                to: destination.to_path_buf(),
            },
            PathKind::Symlink => Action::CopySymlink {
                from: original_path.to_path_buf(),
                to: destination.to_path_buf(),
            },
            _ => return Err(Error::UnsupportedFiletype(original_path.to_path_buf())),
        };

        let entry = BackupEntry::new(original_path.to_path_buf(), destination.to_path_buf());

        Ok((entry, action))
    }
}
