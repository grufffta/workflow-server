mod certificates;
mod setup_command;
mod start_command;
mod validators;

use crate::config;
use anyhow::{anyhow, Context, Result};
use clap::{ArgAction, Parser, Subcommand};
use std::{
    error::Error,
    path::{Path, PathBuf},
};

/// Workflow engine command line interface.
///
/// You can use this interface to test the functionality and
/// * start and stop the server
/// * configure server
/// * configure users
/// * test services
#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct CliArguments {
    /// name for this server instance
    name: Option<String>,
    /// environment configuration file
    #[arg(short, long, value_name = "FILE")]
    config_file: Option<PathBuf>,

    /// Add additional logging, repeat flag for more detail
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Setup(setup_command::SetupCommandArgs),
}

pub(crate) fn run() -> Result<()> {
    let cli: CliArguments = CliArguments::parse();
    let config = config::load(
        cli.name,
        cli.verbose,
        match cli.config_file {
            Some(f) => f,
            None => PathBuf::from(".config/app.toml"),
        },
    )?;
    match &cli.command {
        Some(Commands::Setup(setup_args)) => setup_command::run(setup_args, config),
        None => Err(anyhow!("please enter a valid command")),
    }
}
