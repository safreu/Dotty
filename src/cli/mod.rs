mod cmd;
mod dispatcher;

use cmd::*;

pub use dispatcher::dispatch;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
