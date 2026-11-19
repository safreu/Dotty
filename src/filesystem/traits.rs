use crate::filesystem::PathKind;

use super::Result;
use std::path::{Path, PathBuf};

pub trait Filesystem {
    fn kind(&self, path: &Path) -> Result<Option<PathKind>>;
    fn is_empty_dir(&self, path: &Path) -> Result<bool>;

    fn create_directory(&mut self, path: &Path) -> Result<()>;
    fn remove_directory(&mut self, path: &Path) -> Result<()>;
    fn remove_directory_all(&mut self, path: &Path) -> Result<()>;
    fn write_file(&mut self, path: &Path, content: &[u8]) -> Result<()>;
    fn read_file(&mut self, path: &Path) -> Result<Vec<u8>>;
    fn remove_file(&mut self, path: &Path) -> Result<()>;
    fn rename(&mut self, from: &Path, to: &Path) -> Result<()>;
    fn create_symlink(&mut self, target: &Path, link: &Path) -> Result<()>;
    fn read_symlink(&mut self, path: &Path) -> Result<PathBuf>;
    fn copy_symlink(&mut self, from: &Path, to: &Path) -> Result<()>;
    fn remove_symlink(&mut self, path: &Path) -> Result<()>;
    fn copy_file(&mut self, from: &Path, to: &Path) -> Result<()>;
    fn copy_directory(&mut self, from: &Path, to: &Path) -> Result<()>;
}
