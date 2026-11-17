use std::path::{Path, PathBuf};

use crate::filesystem::FsOperation;

pub type Result<T> = std::result::Result<T, crate::filesystem::Error>;

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
