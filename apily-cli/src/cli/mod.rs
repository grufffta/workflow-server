use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{ArgAction, Parser};

use crate::config;

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
    pub verbose: u8,
}

pub(crate) fn run() -> Result<()> {
    let cli: CliArguments = CliArguments::parse();
    let config =
        config::load(cli.name, cli.verbose, match cli.config_file {
            Some(f) => f,
            None => PathBuf::from(".config/app.toml"),
        })?;
    Ok(())
}
