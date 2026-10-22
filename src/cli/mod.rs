mod cmd;
mod exec_apply;
mod exec_completions;
mod exec_exile;
mod exec_init;
mod exec_list;
mod exec_manage;
mod exec_test;
mod executor;

use cmd::*;

pub use executor::execute;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
