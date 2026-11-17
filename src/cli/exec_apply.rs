use crate::{
    cli::Result,
    domain::{Manifest, StaticDottyPaths},
    persistence::read_toml,
};

pub fn execute() -> Result<()> {
    let paths = StaticDottyPaths::new()?.load()?;

    let manifest: Manifest = read_toml(&paths.static_dirs().manifest_toml())?;

    manifest.validate()?;

    todo!()
}
