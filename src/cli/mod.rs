use crate::{merge_config, save_config, update_if_changed};

use self::{
    config::{open, save, ServerConfig},
    setup::SetupCommandArgs,
    start::StartCommandArgs,
    validators::{is_port_in_range, is_valid_ip_address},
};
use anyhow::{anyhow, Result};
use clap::{command, ArgAction, Args, Parser, Subcommand};
use std::{
    future::{ready, Future, IntoFuture, Ready},
    net::IpAddr,
    path::{Path, PathBuf},
    pin::Pin,
    str::FromStr,
};

mod config;
mod setup;
mod start;
mod validators;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct CLiArguments {
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
enum Commands {
    Start(StartCommandArgs),
    Setup(SetupCommandArgs),
}

/// Represents a CLI command
trait CliCommand: Send {
    /// parse the cli arguments and return `ServerConfig`
    fn parse(&self, cli: &CLiArguments) -> (&dyn CliCommand, ServerConfig);
    /// run the command with the provided config
    fn run(&self, config: &ServerConfig) -> Result<CliCommandResult>;
}

type AsyncCliCommand = Pin<Box<dyn Future<Output = ()>>>;
type SyncResult = ();

enum CliCommandResult {
    Future(AsyncCliCommand),
    Result(SyncResult),
}

pub(crate) async fn run() -> Result<()> {
    let cli: CLiArguments = CLiArguments::parse();
    let (cmd, config) = match &cli.command {
        Some(Commands::Start(start_args)) => start_args.parse(&cli),
        Some(Commands::Setup(setup_args)) => setup_args.parse(&cli),
        None => return Err(anyhow!("please enter a valid command")),
    };
    let filename = match cli.config_file {
        Some(f) => f,
        None => PathBuf::from_str(config.config_location.as_str())?,
    };
    let config = save_config![
        merge_config![open(&filename)?, config],
        cli.update,
        filename.as_path()
    ];
    match cmd.run(&config)? {
        CliCommandResult::Future(future) => {
            future.await;
            Ok(())
        }
        CliCommandResult::Result(_) => Ok(()),
    }
}
