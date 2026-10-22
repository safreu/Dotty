use super::{Error, Result};
use serde::{Serialize, de::DeserializeOwned};
use std::{fs, path::Path};

pub fn read_toml<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    let content = fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_path_buf(),
        source,
    })?;

    let value: T = toml::from_str(&content).map_err(|source| Error::Deserialize {
        path: path.to_path_buf(),
        source,
    })?;

    Ok(value)
}

pub fn serialize_toml<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let content = toml::to_string_pretty(value)?;

    Ok(content.into_bytes())
}

#[allow(unused)]
pub fn write_toml<T>(to_write: &T, path: &Path) -> Result<()>
where
    T: Serialize,
{
    let content = toml::to_string_pretty(to_write)?;

    fs::write(path, content.into_bytes())?;

    Ok(())
}
