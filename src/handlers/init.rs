use crate::{
    domain::{Configuration, DottyPaths, Manifest},
    filesystem::RealFileSystem,
    persistence::serialize_toml,
    services::{InitValidator, PathsLoader, PlanBuilder, PlanService},
};

use super::Result;
use std::path::PathBuf;

pub struct InitHandler;

impl InitHandler {
    pub fn execute(path: PathBuf) -> Result<()> {
        let paths = DottyPaths::initialize(PathsLoader::static_paths()?, path);

        let fs = RealFileSystem;

        InitValidator::validate(&fs, &paths)?;

        let mut builder = PlanBuilder::new();
        let home = PathBuf::from(std::env::var("HOME")?);

        for directory in paths.dirs() {
            builder = builder.ensure_directories(&fs, &home, &directory)?;
        }

        let config = Configuration::new(paths.dynamic_dirs().dotfiles_dir().clone());

        let plan = builder
            .write_file(&paths.static_dirs().config_toml(), serialize_toml(&config)?)
            .write_file(&paths.static_dirs().backups_toml(), Vec::new())
            .write_file(
                &paths.static_dirs().manifest_toml(),
                serialize_toml(&Manifest::new())?,
            )
            .build();

        PlanService::run(&plan)?;

        Ok(())
    }
}
