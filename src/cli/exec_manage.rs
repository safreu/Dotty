use std::path::PathBuf;

use super::Result;
use crate::{
    cli::ManageArgs,
    domain::{
        self, EntryKind, ManagedEntry, ManagedSource, ManagedTarget, Manifest, StaticDottyPaths,
    },
    persistence::{read_toml, serialize_toml},
    services::{PlanBuilder, PlanExecutor},
    utils::RealFileSystem,
};

pub fn execute(args: ManageArgs) -> Result<()> {
    let paths = StaticDottyPaths::new()?.load()?;

    let mut manifest: Manifest = read_toml(&paths.static_dirs().manifest_toml())?;

    manifest.validate()?;

    let home = PathBuf::from(std::env::var("HOME")?);

    let target = if args.target.is_absolute() {
        args.target.strip_prefix(&home)?.to_path_buf()
    } else {
        args.target
    };

    let source = match args.source {
        Some(source) => source,
        None => {
            let name = target
                .file_name()
                .ok_or("target has no filename")?
                .to_string_lossy();
            PathBuf::from(name.strip_prefix(".").unwrap_or(&name))
        }
    };

    let absolute_target = home.join(&target);

    if absolute_target.is_symlink() {
        Err(domain::Error::UnsupportedFiletype(target.to_path_buf()))?;
    }

    let kind = if absolute_target.is_dir() {
        EntryKind::Dir
    } else if absolute_target.is_file() {
        EntryKind::File
    } else {
        return Err(domain::Error::UnsupportedFiletype(target.to_path_buf()))?;
    };
    let entry = ManagedEntry::new(
        ManagedSource::new(source)?,
        ManagedTarget::home_relative(target)?,
        kind,
    );

    manifest.add(entry.clone())?;

    let (source, target) = entry.resolve_paths(paths.dynamic_dirs().dotfiles_dir(), &home);

    let fs = RealFileSystem;

    let plan = PlanBuilder::new()
        .move_to_dotfiles(&fs, &home, &source, &target)?
        .create_managed_symlink(&source, &target)
        .write_file(
            &paths.static_dirs().manifest_toml(),
            serialize_toml(&manifest)?,
        )
        .build();

    PlanExecutor::execute(&plan)?;

    Ok(())
}
