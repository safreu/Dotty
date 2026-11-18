use clap::Parser as _;

use crate::{
    cli::{CliCmd, CliSubCmd, Result},
    handlers::{self, ManageHandler},
};

pub fn dispatch() -> Result<()> {
    let cli_cmd = CliCmd::parse();

    let Some(sub_cmd) = cli_cmd.command else {
        println!("Welcome to dotty.");
        println!("Dotty is a tool for managing your dotfiles.");
        return Ok(());
    };

    match sub_cmd {
        CliSubCmd::Init(args) => handlers::InitHandler::execute(args.path),
        CliSubCmd::Completions(args) => handlers::CompletionsHandler::execute(args.shell),
        CliSubCmd::Manage(args) => ManageHandler::execute(args.target, args.source),
        CliSubCmd::Exile(args) => handlers::ExileHandler::execute(args.path),
        CliSubCmd::Test(args) => handlers::TestHandler::execute(args.path),
        CliSubCmd::List => handlers::ListHandler::execute(),
        CliSubCmd::Apply => handlers::ApplyHandler::execute(),
    }
}
