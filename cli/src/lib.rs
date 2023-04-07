mod commands;
mod config;

use std::{path::PathBuf, str::FromStr};

use anyhow::{anyhow, Result};
use clap::Parser;

use commands::{CliArgs, CliCommand, CliCommandResult, Commands};
use config::{merge, save};

pub async fn run() -> Result<()> {
    let cli: CliArgs = CliArgs::parse();
    let (cmd, config) = match &cli.command {
        Some(Commands::Start(start_args)) => start_args.parse(&cli),
        Some(Commands::Setup(setup_args)) => setup_args.parse(&cli),
        None => return Err(anyhow!("please enter a valid command")),
    };
    let filename = match cli.config_file {
        Some(f) => f,
        None => PathBuf::from_str(config.config_location.as_str())?,
    };
    let config = if cli.update {
        save(&filename, &merge(&filename, config)?)?
    } else {
        merge(&filename, config)?
    };

    match cmd.run(&config)? {
        CliCommandResult::Future(future) => {
            future.await;
            Ok(())
        }
        CliCommandResult::Result(_) => Ok(()),
    }
}
