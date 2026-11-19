use crate::filesystem::{Filesystem, PathKind};
use std::path::Path;
use std::vec;

use crate::services::Result;

use crate::domain::{Action, Backup};

pub struct RollbackBuilder;

impl RollbackBuilder {
    pub fn build(
        fs: &mut impl Filesystem,
        action: &Action,
        _backup: &Backup,
    ) -> Result<Vec<Action>> {
        let inverse = match action {
            Action::CreateDirectory(path) => vec![Action::RemoveDirectory(path.clone())],
            Action::RemoveDirectory(path) => vec![Action::CreateDirectory(path.clone())],
            Action::RemoveDirectoryAll(_) => vec![],
            Action::WriteFile { path, .. } => Self::rollback_write_file(fs, path)?,
            Action::RemoveFile(path) => Self::rollback_remove_file(fs, path)?,
            Action::Rename { from, to } => vec![Action::Rename {
                from: to.clone(),
                to: from.clone(),
            }],
            Action::CreateSymlink { link, .. } => vec![Action::RemoveSymlink(link.clone())],
            Action::RemoveSymlink(path) => Self::rollback_remove_symlink(fs, path)?,
            Action::CopyFile { to, .. } => vec![Action::RemoveFile(to.clone())],
            Action::CopyDirectory { to, .. } => vec![Action::RemoveDirectoryAll(to.clone())],
            Action::CopySymlink { to, .. } => vec![Action::RemoveSymlink(to.clone())],
        };

        Ok(inverse)
    }

    fn rollback_write_file(fs: &mut impl Filesystem, path: &Path) -> Result<Vec<Action>> {
        match fs.kind(path)? {
            None => Ok(vec![Action::RemoveFile(path.to_path_buf())]),
            Some(PathKind::File) => {
                let content = fs.read_file(path)?;

                Ok(vec![Action::WriteFile {
                    path: path.to_path_buf(),
                    content,
                }])
            }
            _ => unreachable!(),
        }
    }

    fn rollback_remove_file(fs: &mut impl Filesystem, path: &Path) -> Result<Vec<Action>> {
        let content = fs.read_file(path)?;

        Ok(vec![Action::WriteFile {
            path: path.to_path_buf(),
            content,
        }])
    }

    fn rollback_remove_symlink(fs: &mut impl Filesystem, path: &Path) -> Result<Vec<Action>> {
        let target = fs.read_symlink(path)?;

        Ok(vec![Action::CreateSymlink {
            target,
            link: path.to_path_buf(),
        }])
    }
}
