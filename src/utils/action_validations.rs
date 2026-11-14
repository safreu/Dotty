use super::Result;
use crate::{
    domain::Action,
    utils::{Filesystem, PathKind, PathValidator},
};

pub struct ActionValidator;

impl ActionValidator {
    pub fn validate(fs: &impl Filesystem, action: &Action) -> Result<()> {
        match action {
            Action::CreateDirectory(path) => {
                PathValidator::requires_home_relative(path)?;
                PathValidator::requires_absent(fs, path)?;
                PathValidator::requires_parent_dir(fs, path)
            }

            Action::RemoveDirectory(path) => {
                PathValidator::requires_home_relative(path)?;
                PathValidator::requires_existence(fs, path)?;
                PathValidator::require_kind(fs, path, PathKind::Directory)?;
                PathValidator::require_empty_dir(fs, path)
            }
            Action::WriteFile { path, .. } => {
                PathValidator::requires_home_relative(path)?;
                PathValidator::requires_parent_dir(fs, path)?;
                PathValidator::require_not_kind(fs, path, PathKind::Symlink)
            }
            Action::RemoveFile(path) => {
                PathValidator::requires_home_relative(path)?;
                PathValidator::requires_existence(fs, path)?;
                PathValidator::require_kind(fs, path, PathKind::File)
            }
            Action::Rename { from, to } => {
                PathValidator::requires_home_relative(from)?;
                PathValidator::requires_home_relative(to)?;
                PathValidator::requires_existence(fs, from)?;
                PathValidator::requires_absent(fs, to)
            }
            Action::CreateSymlink { target, link } => {
                PathValidator::requires_home_relative(target)?;
                PathValidator::requires_home_relative(link)?;
                PathValidator::requires_existence(fs, target)?;
                PathValidator::requires_absent(fs, link)
            }
            Action::RemoveSymlink(path) => {
                PathValidator::requires_home_relative(path)?;
                PathValidator::requires_existence(fs, path)?;
                PathValidator::require_kind(fs, path, PathKind::Symlink)
            }
            Action::CopyFile { from, to } => {
                PathValidator::requires_home_relative(from)?;
                PathValidator::requires_home_relative(to)?;
                PathValidator::requires_existence(fs, from)?;
                PathValidator::requires_absent(fs, to)
            }
            Action::CopyDirectory { from, to } => {
                PathValidator::requires_home_relative(from)?;
                PathValidator::requires_home_relative(to)?;
                PathValidator::require_kind(fs, from, PathKind::Directory)?;
                PathValidator::requires_existence(fs, from)?;

                PathValidator::requires_absent(fs, to)
            }
        }
    }
}
