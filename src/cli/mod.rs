mod cmd;
mod dispatcher;
mod exec_apply;
mod exec_completions;
mod exec_exile;
mod exec_list;
mod exec_test;

use cmd::*;

pub use dispatcher::dispatch;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
