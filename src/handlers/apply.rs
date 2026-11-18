use super::Result;
use crate::{domain::Manifest, persistence::read_toml, services::PathsLoader};

pub struct ApplyHandler;

impl ApplyHandler {
    pub fn execute() -> Result<()> {
        let paths = PathsLoader::load()?;

        let manifest: Manifest = read_toml(&paths.static_dirs().manifest_toml())?;

        manifest.validate()?;

        todo!()
    }
}
