use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] std::env::VarError),

    #[error("Path has to be relative: `{0}")]
    AbsolutePath(PathBuf),

    #[error("Path has to be relative to home: `{0}")]
    NotHomeRelative(PathBuf),

    #[error("Path is not allowed as symlink: `{0}")]
    Symlink(PathBuf),

    #[error("Dotty is not initialized yet. Run `dotty init <repos>`")]
    NotInitialized,
}
