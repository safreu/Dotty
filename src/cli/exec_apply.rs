use crate::{cli::Result, domain::Manifest, persistence::read_toml, services::PathsLoader};

pub fn execute() -> Result<()> {
    let paths = PathsLoader::load()?;

    let manifest: Manifest = read_toml(&paths.static_dirs().manifest_toml())?;

    manifest.validate()?;

    todo!()
}
