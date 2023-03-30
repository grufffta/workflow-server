use std::path::PathBuf;

use clap::{
    ArgAction,
    Parser,
    Subcommand,
};

pub(crate) mod start_command;
pub(crate) mod test_command;
pub(crate) mod validators;
pub(crate) mod setup_command;

#[derive(Parser)]
/// Workflow engine command line interface.
///
/// You can use this interface to test the functionality and
/// start and stop the server
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub(crate) struct CliArguments {
    /// environment configuration file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Add additional logging, repeat flag for more detail
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Starts the workflow server
    Start(start_command::StartCommandArgs),
    /// Perform server setup tasks and configuration
    Setup(setup_command::SetupCommandArgs),
    /// Tests the server is up and network connections are available
    Test(test_command::TestCommandArgs),
}
