mod apply;
mod completions;
mod exile;
mod init;
mod list;
mod manage;
mod test;

pub use apply::*;
pub use completions::*;
pub use exile::*;
pub use init::*;
pub use list::*;
pub use manage::*;
pub use test::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
