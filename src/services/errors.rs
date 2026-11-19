use crate::{domain::BackupId, persistence::Error as PersistenceError};
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] std::env::VarError),

    #[error("expected directory at `{path}`")]
    InvalidDirectory { path: PathBuf },
    #[error("path `{target}` is outside root `{root}`")]
    PathOutsideRoot { root: PathBuf, target: PathBuf },

    #[error(transparent)]
    Domain(#[from] crate::domain::Error),
    #[error(transparent)]
    Filesystem(#[from] crate::filesystem::Error),

    #[error(transparent)]
    Persistence(#[from] PersistenceError),

    #[error("Path has to be relative to home: `{0}")]
    NotHomeRelative(PathBuf),
    #[error("Path is not allowed as symlink: `{0}")]
    Symlink(PathBuf),
    #[error("Path needs a parent directory: `{0}")]
    MissingParent(PathBuf),
    #[error("Path is not allowed to exist yet: `{0}`")]
    PathExists(PathBuf),
    #[error("Path does not exist: `{0}`")]
    PathNotExists(PathBuf),
    #[error("Unsupported filetype: `{0}`")]
    UnsupportedFiletype(PathBuf),
    #[error("Directory must be empty: `{0}`")]
    DirNotEmpty(PathBuf),
    #[error("destination `{to}` is inside source `{from}`")]
    PathInsideSource { from: PathBuf, to: PathBuf },
    #[error("path is not normalized: `{0}`")]
    NotNormalized(PathBuf),

    #[error("Unable to determine base directories")]
    BaseDirectories,

    #[error("dotfiles directory must be an absolute path: `{0}`")]
    InvalidDotfilesPath(PathBuf),

    #[error("missing backup for `{0}`")]
    MissingBackup(PathBuf),

    #[error("plan execution failed: {original}; rollback failed: {rollback}")]
    RollbackFailed {
        original: Box<Error>,
        rollback: Box<Error>,
    },

    #[error("rollback-only action cannot appear in a regular plan")]
    RollbackOnlyAction,

    #[error("backup with ID {0} not found")]
    BackupNotFound(BackupId),

    #[error("plan execution failed: {original}; updating backup status failed: {status}")]
    BackupStatusUpdateFailed {
        original: Box<Error>,
        status: Box<Error>,
    },
}
