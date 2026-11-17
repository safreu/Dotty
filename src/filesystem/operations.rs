use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsOperation {
    CopyFile,
    CopyDir,

    RemoveFile,
    RemoveDir,

    Rename,

    CreateDir,

    CreateSymlink,
    ReadSymlink,
    RemoveSymlink,

    ReadFile,
    WriteFile,

    Inspect,
}

impl fmt::Display for FsOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FsOperation::CopyFile => "copy file",
            FsOperation::CopyDir => "copy directory",
            FsOperation::RemoveFile => "remove file",
            FsOperation::RemoveDir => "remove directory",
            FsOperation::Rename => "rename",
            FsOperation::CreateDir => "create directory",
            FsOperation::CreateSymlink => "create symlink",
            FsOperation::ReadSymlink => "read symlink",
            FsOperation::RemoveSymlink => "remove symlink",
            FsOperation::ReadFile => "read file",
            FsOperation::WriteFile => "write file",
            FsOperation::Inspect => "inspect path",
        })
    }
}
