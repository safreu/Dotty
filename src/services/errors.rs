use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] std::env::VarError),
    #[error("Path is not allowed as symlink: `{0}")]
    Symlink(PathBuf),
    #[error("Dotty is not initialized yet. Run `dotty init <repos>`")]
    NotInitialized,

    #[error(transparent)]
    Utils(#[from] crate::utils::Error),
    #[error(transparent)]
    Domain(#[from] crate::domain::Error),
}
