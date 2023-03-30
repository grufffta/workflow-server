mod certificates;
mod setup_command;
mod start_command;
mod validators;

use crate::config::{self, server::ServerConfig};
use anyhow::{anyhow, Context, Result};
use clap::{ArgAction, Parser, Subcommand};
use std::{
    error::Error,
    path::{Path, PathBuf},
};

use self::start_command::StartCommandArgs;

const CONFIG_FILE_LOCATION: &str = ".config/app.toml";

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

    /// update configuration with passed arguments
    #[arg(short, long, default_value_t = false)]
    update: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Start(start_command::StartCommandArgs),
    Setup(setup_command::SetupCommandArgs),
}

pub(crate) async fn run() -> Result<()> {
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
        Some(Commands::Start(start_args)) => {
            start_command::run(merge(start_args, config, cli.update)).await
        }
        Some(Commands::Setup(setup_args)) => setup_command::run(setup_args, config),
        None => Err(anyhow!("please enter a valid command")),
    }
}

fn merge(start_args: &StartCommandArgs, mut config: ServerConfig, save: bool) -> ServerConfig {
    config.http.address = start_args.address;
    config.http.port = start_args.port;
    if save {
        config::save(config, &PathBuf::from(CONFIG_FILE_LOCATION));
    }
    config
}
