use clap::Parser as _;

use crate::cli::{
    CliCmd, CliSubCmd, Result, exec_apply, exec_completions, exec_exile, exec_init, exec_list,
    exec_manage, exec_test,
};

pub fn execute() -> Result<()> {
    let cli_cmd = CliCmd::parse();

    let Some(sub_cmd) = cli_cmd.command else {
        println!("Welcome to dotty.");
        println!("Dotty is a tool for managing your dotfiles.");
        return Ok(());
    };

    match sub_cmd {
        CliSubCmd::Init(args) => exec_init::execute(args),
        CliSubCmd::Completions(args) => exec_completions::execute(args),
        CliSubCmd::Manage(args) => exec_manage::execute(args),
        CliSubCmd::Exile(args) => exec_exile::execute(args),
        CliSubCmd::Test(args) => exec_test::execute(args),
        CliSubCmd::List => exec_list::execute(),
        CliSubCmd::Apply => exec_apply::execute(),
    }
}
