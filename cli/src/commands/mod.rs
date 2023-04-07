mod setup;
mod start;

use anyhow::Result;
use clap::{command, ArgAction, Parser, Subcommand};
use std::path::PathBuf;
use std::{future::Future, pin::Pin};

use crate::config::AppConfig;

/// Represents a CLI command
pub(crate) trait CliCommand: Send {
    /// parse the cli arguments and return `ServerConfig`
    fn parse(&self, cli: &CliArgs) -> (&dyn CliCommand, AppConfig);
    /// run the command with the provided config
    fn run(&self, config: &AppConfig) -> Result<CliCommandResult>;
}

type AsyncCliCommand = Pin<Box<dyn Future<Output = ()>>>;
type SyncResult = ();

pub(crate) enum CliCommandResult {
    Future(AsyncCliCommand),
    Result(SyncResult),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub(crate) struct CliArgs {
    /// name for this server instance
    name: Option<String>,

    /// environment configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config_file: Option<PathBuf>,

    /// Add additional logging, repeat flag for more detail
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    /// update configuration with passed arguments
    #[arg(short, long, default_value_t = false)]
    pub update: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Start(start::StartCommandArgs),
    Setup(setup::SetupCommandArgs),
}
