use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueHint};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(name = "dotty", about = "Dotfile management tool")]
#[command(version)]
pub struct CliCmd {
    #[command(subcommand)]
    pub command: Option<CliSubCmd>,
}

#[derive(Subcommand, Debug)]
pub enum CliSubCmd {
    /// This command will initialize dotty.
    Init(InitArgs),

    /// This command is used to manage a file/dir by dotty.
    Manage(ManageArgs),

    /// This command is used quit managing a file/dir by dotty.
    Exile(ExileArgs),

    /// This command is used to get completions in you shell for dotty.
    Completions(CompletionsArgs),

    /// This command can be used to test the entries managed by dotty,
    /// and verify they are still working as they should,
    /// in case you want to test a specific file/dir, you can pass a path as an argument.
    Test(TestArgs),

    /// List all entries managed by dotty.
    List,

    /// In case you have a manifest.toml, which contains your previous managed entries by dotty,
    /// you can call `apply` to let dotty iterate through the manifest and try to restore the managed state.
    Apply,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// This path will be used to store the dotfiles,
    /// in case it doesn't exist yet it will be created.
    #[arg(value_hint = ValueHint::DirPath)]
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub struct ManageArgs {
    /// The path to the file/dir you want to manage by dotty.
    /// This will create a file/dir in you dotfiles repository,
    /// stripping the leading dot in case its a hidden file/dir.
    #[arg(value_hint = ValueHint::AnyPath)]
    pub target: PathBuf,

    /// as can be used to rename the file/dir you will manage by dotty
    /// This can be used to avoid conflicts in case you want to manage multiple files with the same name,
    /// or if you just want a name which is more telling then just config for example.
    #[arg(long = "as", value_hint = ValueHint::AnyPath)]
    pub source: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct ExileArgs {
    /// The path leading to the file/dir you want dotty to forget.
    #[arg(value_hint = ValueHint::AnyPath)]
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    /// This will create a file containing completions for dotty.
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Args, Debug)]
pub struct TestArgs {
    /// In case you don't want to test all entries managed by dotty,
    /// you can use a path to a specific file/dir which is managed by dotty to test it.
    #[arg(value_hint = ValueHint::AnyPath)]
    pub path: Option<PathBuf>,
}
