mod init;
mod manage;

pub use init::*;
pub use manage::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
