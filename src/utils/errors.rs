use std::path::{Path, PathBuf};

use crate::domain::FsOperation;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to {operation} `{path}`")]
    IoSingle {
        operation: FsOperation,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to {operation} `{from}` to `{to}`")]
    IoDouble {
        operation: FsOperation,
        from: PathBuf,
        to: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("cannot write to unsupported filesystem entry: `{path}`")]
    InvalidWriteTarget { path: PathBuf },

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
}

impl Error {
    pub fn io_single(
        operation: FsOperation,
        path: &Path,
    ) -> impl FnOnce(std::io::Error) -> Self + '_ {
        move |source| Self::IoSingle {
            operation,
            path: path.to_path_buf(),
            source,
        }
    }

    pub fn io_double<'a>(
        operation: FsOperation,
        from: &'a Path,
        to: &'a Path,
    ) -> impl FnOnce(std::io::Error) -> Self + 'a {
        move |source| Self::IoDouble {
            operation,
            from: from.to_path_buf(),
            to: to.to_path_buf(),
            source,
        }
    }
}
