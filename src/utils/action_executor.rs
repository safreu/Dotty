use super::Result;
use crate::{domain::Action, utils::Filesystem};

pub struct ActionExecutor;

impl ActionExecutor {
    pub fn execute(fs: &mut impl Filesystem, action: &Action) -> Result<()> {
        match action {
            Action::CreateDirectory(path) => fs.create_directory(path),
            Action::RemoveDirectory(path) => fs.remove_directory(path),
            Action::WriteFile { path, content } => fs.write_file(path, content),
            Action::RemoveFile(path) => fs.remove_file(path),
            Action::Rename { from, to } => fs.rename(from, to),
            Action::CreateSymlink { target, link } => fs.create_symlink(target, link),
            Action::RemoveSymlink(path) => fs.remove_symlink(path),
            Action::CopyFile { from, to } => fs.copy_file(from, to),
            Action::CopyDirectory { from, to } => fs.copy_directory(from, to),
        }
    }
}
