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
}
