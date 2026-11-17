use std::path::PathBuf;

use crate::{
    cli::{InitArgs, Result},
    domain::{Configuration, DottyPaths, Manifest, StaticDottyPaths},
    persistence::serialize_toml,
    services::{PlanBuilder, PlanExecutor},
    utils::{PathValidator, RealFileSystem},
};
/// Init will not overwrite an existing directory
/// Must be relative to home
/// NOT ALLOWED:
/// - existing Directories
/// - symlinks
pub fn execute(args: InitArgs) -> Result<()> {
    if !args.path.is_absolute() {
        Err(format!(
            "dotfiles directory must be an absolute path: {}",
            args.path.display()
        ))?;
    }

    let static_paths = StaticDottyPaths::new()?;
    let paths = DottyPaths::initialize(static_paths, args.path);

    let fs = RealFileSystem;
    let mut builder = PlanBuilder::new();
    let home = PathBuf::from(std::env::var("HOME")?);

    for directory in paths.dirs() {
        builder = builder.ensure_directories(&fs, &home, &directory)?;
    }

    let config = Configuration::new(paths.dynamic_dirs().dotfiles_dir().clone());

    for path in paths.files() {
        PathValidator::requires_absent(&fs, &path)?;
    }

    let plan = builder
        .write_file(&paths.static_dirs().config_toml(), serialize_toml(&config)?)
        .write_file(&paths.static_dirs().backups_toml(), Vec::new())
        .write_file(
            &paths.static_dirs().manifest_toml(),
            serialize_toml(&Manifest::new())?,
        )
        .build();

    PlanExecutor::execute(&plan)?;

    Ok(())
}
