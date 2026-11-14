use std::path::PathBuf;

pub enum Action {
    CreateDirectory(PathBuf),
    RemoveDirectory(PathBuf),

    WriteFile { path: PathBuf, contents: Vec<u8> },
    RemoveFile(PathBuf),

    Rename { from: PathBuf, to: PathBuf },

    CreateSymlink { target: PathBuf, link: PathBuf },
    RemoveSymlink(PathBuf),

    CopyFile { from: PathBuf, to: PathBuf },
    CopyDirectory { from: PathBuf, to: PathBuf },
}
