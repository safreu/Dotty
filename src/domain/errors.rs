use std::path::PathBuf;

use crate::domain::BackupId;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("managed path must not be empty")]
    EmptyManagedPath,
    #[error("managed path must be relative: `{0}`")]
    AbsoluteManagedPath(PathBuf),
    #[error("managed path must not contain parent traversal: `{0}`")]
    ManagedPathTraversal(PathBuf),
    #[error("managed target `{new}` conflicts with existing target `{existing}`")]
    ManagedTargetConflict { existing: PathBuf, new: PathBuf },
    #[error("managed source `{new}` conflicts with existing source `{existing}`")]
    ManagedSourceConflict { existing: PathBuf, new: PathBuf },

    #[error("invalid managed path: {0}")]
    InvalidManagedPath(PathBuf),
    #[error("failed to strip path prefix: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),

    #[error("backup with ID {0} already exists")]
    DuplicateBackupId(BackupId),
}
