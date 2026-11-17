use crate::services::Result;
use crate::{
    domain::Action,
    filesystem::{Filesystem, PathKind},
    services::PathRules,
};

pub struct ActionValidator;

impl ActionValidator {
    pub fn validate(fs: &impl Filesystem, action: &Action) -> Result<()> {
        match action {
            Action::CreateDirectory(path) => {
                PathRules::requires_home_relative(path)?;
                PathRules::requires_absent(fs, path)?;
                PathRules::requires_parent_dir(fs, path)
            }

            Action::RemoveDirectory(path) => {
                PathRules::requires_home_relative(path)?;
                PathRules::requires_existence(fs, path)?;
                PathRules::require_kind(fs, path, PathKind::Directory)?;
                PathRules::require_empty_dir(fs, path)
            }
            Action::WriteFile { path, .. } => {
                PathRules::requires_home_relative(path)?;
                PathRules::requires_parent_dir(fs, path)?;
                PathRules::require_not_kind(fs, path, PathKind::Directory)?;
                PathRules::require_not_kind(fs, path, PathKind::Symlink)?;
                PathRules::require_not_kind(fs, path, PathKind::Other)
            }
            Action::RemoveFile(path) => {
                PathRules::requires_home_relative(path)?;
                PathRules::requires_existence(fs, path)?;
                PathRules::require_kind(fs, path, PathKind::File)
            }
            Action::Rename { from, to } => {
                PathRules::requires_home_relative(from)?;
                PathRules::requires_home_relative(to)?;
                PathRules::requires_existence(fs, from)?;
                if fs.kind(from)? == Some(PathKind::Directory) {
                    PathRules::require_outside_source(from, to)?;
                };
                PathRules::requires_absent(fs, to)?;
                PathRules::requires_parent_dir(fs, to)
            }
            Action::CreateSymlink { target, link } => {
                PathRules::requires_home_relative(target)?;
                PathRules::requires_home_relative(link)?;
                PathRules::requires_existence(fs, target)?;
                PathRules::requires_absent(fs, link)?;
                PathRules::requires_parent_dir(fs, link)
            }
            Action::RemoveSymlink(path) => {
                PathRules::requires_home_relative(path)?;
                PathRules::requires_existence(fs, path)?;
                PathRules::require_kind(fs, path, PathKind::Symlink)
            }
            Action::CopyFile { from, to } => {
                PathRules::requires_home_relative(from)?;
                PathRules::requires_home_relative(to)?;
                PathRules::require_kind(fs, from, PathKind::File)?;
                PathRules::requires_absent(fs, to)?;
                PathRules::requires_parent_dir(fs, to)
            }
            Action::CopyDirectory { from, to } => {
                PathRules::requires_home_relative(from)?;
                PathRules::requires_home_relative(to)?;
                PathRules::require_outside_source(from, to)?;
                PathRules::require_kind(fs, from, PathKind::Directory)?;
                PathRules::requires_absent(fs, to)?;
                PathRules::requires_parent_dir(fs, to)
            }
        }?;

        Ok(())
    }
}
