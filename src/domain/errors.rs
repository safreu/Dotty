use crate::persistence::Error as PersistenceError;
use std::env;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("required environment variable is missing or invalid")]
    Environment(#[from] env::VarError),
    #[error(transparent)]
    Persistence(#[from] PersistenceError),
    #[error("Unable to determine base directories")]
    BaseDirectories,
}
