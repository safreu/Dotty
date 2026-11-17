use crate::persistence::Error as PersistenceError;
use crate::utils::Error as UtilsError;
use std::env;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("required environment variable is missing or invalid")]
    Environment(#[from] env::VarError),
    #[error(transparent)]
    Persistence(#[from] PersistenceError),
    #[error("Unable to determine base directories")]
    BaseDirectories,
    #[error(transparent)]
    Utils(#[from] UtilsError),
    #[error("expected directory at `{path}`")]
    InvalidDirectory { path: PathBuf },
    #[error("path `{target}` is outside root `{root}`")]
    PathOutsideRoot { root: PathBuf, target: PathBuf },
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
    #[error("unsupported filetype: {0}")]
    UnsupportedFiletype(PathBuf),
}
