use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read TOML file `{path}`")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to deserialize TOML file `{path}`")]
    Deserialize {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("failed to serialize TOML")]
    Serialize(#[from] toml::ser::Error),

    #[error(transparent)]
    InvalidManifest(#[from] std::io::Error),
}
