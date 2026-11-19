use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Action {
    CreateDirectory(PathBuf),
    RemoveDirectory(PathBuf),
    RemoveDirectoryAll(PathBuf),

    WriteFile { path: PathBuf, content: Vec<u8> },
    RemoveFile(PathBuf),

    Rename { from: PathBuf, to: PathBuf },

    CreateSymlink { target: PathBuf, link: PathBuf },
    CopySymlink { from: PathBuf, to: PathBuf },
    RemoveSymlink(PathBuf),

    CopyFile { from: PathBuf, to: PathBuf },
    CopyDirectory { from: PathBuf, to: PathBuf },
}
