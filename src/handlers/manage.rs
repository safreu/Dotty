use std::path::PathBuf;

use crate::{
    domain::{EntryKind, ManagedEntry, ManagedPath, Manifest},
    filesystem::{Filesystem, PathKind, RealFileSystem},
    persistence::{read_toml, serialize_toml},
    services::{ManageValidator, PathsLoader, PlanBuilder, PlanService},
};

use super::Result;
pub struct ManageHandler;

impl ManageHandler {
    pub fn execute(target: PathBuf, source: Option<PathBuf>) -> Result<()> {
        let paths = PathsLoader::load()?;

        let mut manifest: Manifest = read_toml(&paths.static_dirs().manifest_toml())?;

        manifest.validate()?;

        let home = PathBuf::from(std::env::var("HOME")?);

        let target = ManagedPath::from_target(target, &home)?;
        let source = ManagedPath::from_source(source, &target)?;

        let absolute_target = target.resolve(&home);
        let fs = RealFileSystem;
        let kind = fs.kind(&absolute_target)?;

        ManageValidator::validate(&absolute_target, kind)?;

        let kind = match kind {
            Some(PathKind::File) => EntryKind::File,
            Some(PathKind::Directory) => EntryKind::Dir,
            _ => unreachable!("ManageValidator already checked the file type"),
        };

        let entry = ManagedEntry::new(source, target, kind);

        manifest.add(entry.clone())?;

        let (source, target) = entry.resolve_paths(paths.dynamic_dirs().dotfiles_dir(), &home);

        let plan = PlanBuilder::new()
            .move_to_dotfiles(&fs, &home, &source, &target)?
            .create_managed_symlink(&source, &target)
            .write_file(
                &paths.static_dirs().manifest_toml(),
                serialize_toml(&manifest)?,
            )
            .build();

        PlanService::run(
            &plan,
            paths.static_dirs().backups_dir(),
            &paths.static_dirs().backups_toml(),
        )?;

        Ok(())
    }
}
