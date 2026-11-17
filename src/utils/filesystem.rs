use atomic_write_file::{AtomicWriteFile, unix::OpenOptionsExt};

use super::{Error, Result};
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use crate::{
    domain::{Action, FsOperation},
    utils::{ActionExecutor, PathKind},
};

pub trait Filesystem {
    fn kind(&self, path: &Path) -> Result<Option<PathKind>>;
    fn is_empty_dir(&self, path: &Path) -> Result<bool>;

    fn create_directory(&mut self, path: &Path) -> Result<()>;
    fn remove_directory(&mut self, path: &Path) -> Result<()>;
    fn write_file(&mut self, path: &Path, content: &[u8]) -> Result<()>;
    fn read_file(&mut self, path: &Path) -> Result<Vec<u8>>;
    fn remove_file(&mut self, path: &Path) -> Result<()>;
    fn rename(&mut self, from: &Path, to: &Path) -> Result<()>;
    fn create_symlink(&mut self, target: &Path, link: &Path) -> Result<()>;
    fn read_symlink(&mut self, path: &Path) -> Result<PathBuf>;
    fn remove_symlink(&mut self, path: &Path) -> Result<()>;
    fn copy_file(&mut self, from: &Path, to: &Path) -> Result<()>;
    fn copy_directory(&mut self, from: &Path, to: &Path) -> Result<()>;
}

pub struct RealFileSystem;

impl RealFileSystem {
    pub fn apply(&mut self, action: &Action) -> Result<()> {
        ActionExecutor::execute(self, action)
    }

    fn copy_dir_impl(&mut self, from: &Path, to: &Path) -> std::io::Result<()> {
        let parent = to.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "destination has no parent directory",
            )
        })?;

        let staging = tempfile::Builder::new().tempdir_in(parent)?;

        let staged_copy = staging.path().join("contents");

        self.copy_dir_inner(from, &staged_copy)?;

        self.rename_noreplace(&staged_copy, to)?;

        staging.close()?;

        Ok(())
    }

    fn copy_dir_inner(
        &mut self,
        from: &Path,
        to: &Path,
    ) -> std::result::Result<(), std::io::Error> {
        fs::create_dir(to)?;

        for entry in fs::read_dir(from)? {
            let entry = entry?;

            let source = entry.path();
            let destination = to.join(entry.file_name());

            let metadata = fs::symlink_metadata(&source)?;
            let file_type = metadata.file_type();

            if file_type.is_symlink() {
                let target = fs::read_link(&source)?;
                symlink(target, destination)?;
            } else if file_type.is_dir() {
                self.copy_dir_inner(&source, &destination)?;
            } else if file_type.is_file() {
                fs::copy(&source, &destination)?;
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("Unsupported filesystem entry: {}", source.display()),
                ));
            }
        }

        Ok(())
    }

    fn copy_file_impl(&mut self, from: &Path, to: &Path) -> std::io::Result<()> {
        let parent = to.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "destination has no parent directory",
            )
        })?;

        let mut src = fs::File::open(from)?;

        let mut staging = tempfile::Builder::new()
            .prefix(".dotty-copy-")
            .tempfile_in(parent)?;

        io::copy(&mut src, staging.as_file_mut())?;

        staging
            .as_file()
            .set_permissions(src.metadata()?.permissions())?;

        staging.as_file().sync_all()?;

        staging.persist_noclobber(to).map_err(|e| e.error)?;

        Ok(())
    }

    fn rename_noreplace(&mut self, from: &Path, to: &Path) -> std::io::Result<()> {
        rustix::fs::renameat_with(
            rustix::fs::CWD,
            from,
            rustix::fs::CWD,
            to,
            rustix::fs::RenameFlags::NOREPLACE,
        )
        .map_err(Into::into)
    }

    fn write_file_impl(&mut self, path: &Path, content: &[u8]) -> std::io::Result<()> {
        let mut file = AtomicWriteFile::options().preserve_mode(true).open(path)?;

        file.write_all(content)?;
        file.commit()
    }
}

impl Filesystem for RealFileSystem {
    fn kind(&self, path: &Path) -> Result<Option<PathKind>> {
        let metadata = match fs::symlink_metadata(path) {
            Ok(metadata) => metadata,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(source) => {
                return Err(Error::IoSingle {
                    operation: FsOperation::Inspect,
                    path: path.to_path_buf(),
                    source,
                });
            }
        };

        let file_type = metadata.file_type();

        let kind = if file_type.is_symlink() {
            PathKind::Symlink
        } else if file_type.is_file() {
            PathKind::File
        } else if file_type.is_dir() {
            PathKind::Directory
        } else {
            PathKind::Other
        };

        Ok(Some(kind))
    }

    fn is_empty_dir(&self, path: &Path) -> Result<bool> {
        let mut entries =
            fs::read_dir(path).map_err(Error::io_single(FsOperation::Inspect, path))?;

        match entries.next() {
            None => Ok(true),
            Some(Ok(_)) => Ok(false),
            Some(Err(source)) => Err(Error::IoSingle {
                operation: FsOperation::Inspect,
                path: path.to_path_buf(),
                source,
            }),
        }
    }

    fn create_directory(&mut self, path: &Path) -> Result<()> {
        fs::create_dir(path).map_err(Error::io_single(FsOperation::CreateDir, path))
    }

    fn remove_directory(&mut self, path: &Path) -> Result<()> {
        fs::remove_dir(path).map_err(Error::io_single(FsOperation::RemoveDir, path))
    }

    fn write_file(&mut self, path: &Path, content: &[u8]) -> Result<()> {
        self.write_file_impl(path, content)
            .map_err(Error::io_single(FsOperation::WriteFile, path))
    }

    fn read_file(&mut self, path: &Path) -> Result<Vec<u8>> {
        fs::read(path).map_err(Error::io_single(FsOperation::ReadFile, path))
    }

    fn read_symlink(&mut self, path: &Path) -> Result<PathBuf> {
        fs::read_link(path).map_err(Error::io_single(FsOperation::ReadSymlink, path))
    }

    fn remove_file(&mut self, path: &Path) -> Result<()> {
        fs::remove_file(path).map_err(Error::io_single(FsOperation::RemoveFile, path))
    }

    fn rename(&mut self, from: &Path, to: &Path) -> Result<()> {
        self.rename_noreplace(from, to)
            .map_err(Error::io_double(FsOperation::Rename, from, to))
    }

    fn create_symlink(&mut self, target: &Path, link: &Path) -> Result<()> {
        symlink(target, link).map_err(Error::io_double(FsOperation::CreateSymlink, link, target))
    }

    fn remove_symlink(&mut self, path: &Path) -> Result<()> {
        fs::remove_file(path).map_err(Error::io_single(FsOperation::RemoveSymlink, path))
    }

    fn copy_file(&mut self, from: &Path, to: &Path) -> Result<()> {
        self.copy_file_impl(from, to)
            .map(|_| ())
            .map_err(Error::io_double(FsOperation::CopyFile, from, to))
    }

    fn copy_directory(&mut self, from: &Path, to: &Path) -> Result<()> {
        self.copy_dir_impl(from, to)
            .map_err(Error::io_double(FsOperation::CopyDir, from, to))
    }
}

pub struct SimulatedFilesystem {
    entries: HashMap<PathBuf, PathKind>,
}

impl SimulatedFilesystem {
    pub fn new(entries: HashMap<PathBuf, PathKind>) -> Self {
        Self { entries }
    }

    pub fn apply(&mut self, action: &Action) -> Result<()> {
        ActionExecutor::execute(self, action)
    }
}

impl Filesystem for SimulatedFilesystem {
    fn kind(&self, path: &Path) -> Result<Option<PathKind>> {
        Ok(self.entries.get(path).copied())
    }

    fn is_empty_dir(&self, path: &Path) -> Result<bool> {
        Ok(!self
            .entries
            .keys()
            .any(|entry| entry.parent() == Some(path)))
    }

    fn create_directory(&mut self, path: &Path) -> Result<()> {
        self.entries.insert(path.to_path_buf(), PathKind::Directory);
        Ok(())
    }

    fn remove_directory(&mut self, path: &Path) -> Result<()> {
        self.entries.remove(path);
        Ok(())
    }

    fn write_file(&mut self, path: &Path, _content: &[u8]) -> Result<()> {
        self.entries.insert(path.to_path_buf(), PathKind::File);
        Ok(())
    }

    fn remove_file(&mut self, path: &Path) -> Result<()> {
        self.entries.remove(path);
        Ok(())
    }

    fn rename(&mut self, from: &Path, to: &Path) -> Result<()> {
        let affected: Vec<_> = self
            .entries
            .keys()
            .filter(|path| path.starts_with(from))
            .cloned()
            .collect();

        for old in affected {
            let relative = old.strip_prefix(from).expect("path must be within source");

            let new = to.join(relative);

            if let Some(kind) = self.entries.remove(&old) {
                self.entries.insert(new, kind);
            };
        }

        Ok(())
    }

    fn create_symlink(&mut self, _target: &Path, link: &Path) -> Result<()> {
        self.entries.insert(link.to_path_buf(), PathKind::Symlink);

        Ok(())
    }

    fn remove_symlink(&mut self, path: &Path) -> Result<()> {
        self.entries.remove(path);
        Ok(())
    }

    fn copy_file(&mut self, from: &Path, to: &Path) -> Result<()> {
        if let Some(kind) = self.entries.get(from).copied() {
            self.entries.insert(to.to_path_buf(), kind);
        }

        Ok(())
    }

    fn copy_directory(&mut self, from: &Path, to: &Path) -> Result<()> {
        let affected: Vec<_> = self
            .entries
            .iter()
            .filter(|(path, _)| path.starts_with(from))
            .map(|(path, kind)| (path.clone(), *kind))
            .collect();

        for (old, kind) in affected {
            let relative = old.strip_prefix(from).expect("path must be within source");

            let new = to.join(relative);

            self.entries.insert(new, kind);
        }

        Ok(())
    }

    fn read_file(&mut self, _path: &Path) -> Result<Vec<u8>> {
        todo!()
    }

    fn read_symlink(&mut self, _path: &Path) -> Result<PathBuf> {
        todo!()
    }
}
