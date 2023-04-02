use crate::cli::CliCommandResult;

use self::certificates::CertificateCommandArgs;
use super::{config::ServerConfig, CLiArguments, CliCommand};
use anyhow::{anyhow, Result};
use clap::{command, ArgAction, Args, Parser, Subcommand};

mod certificates;

/// Setup command options
#[derive(Subcommand)]
enum SetupCommands {
    /// setup host server certificate authority
    HostCert(CertificateCommandArgs),
    /// show application configuration
    Show(ShowCommandArgs),
}

/// server setup
#[derive(Args)]
pub(crate) struct SetupCommandArgs {
    #[command(subcommand)]
    command: Option<SetupCommands>,
}

impl CliCommand for SetupCommandArgs {
    fn parse(&self, cli: &CLiArguments) -> (&dyn CliCommand, ServerConfig) {
        match &self.command {
            Some(SetupCommands::HostCert(certificate_args)) => certificate_args.parse(cli),
            Some(SetupCommands::Show(show_config_args)) => show_config_args.parse(cli),
            None => todo!(),
        }
    }

    fn run(&self, config: &ServerConfig) -> Result<CliCommandResult> {
        /// the parse method returns its subcommand which will have run called on it
        unreachable!()
    }
}

#[derive(Args)]
struct ShowCommandArgs {}

impl CliCommand for ShowCommandArgs {
    fn parse(&self, cli: &CLiArguments) -> (&dyn CliCommand, ServerConfig) {
        (
            self,
            ServerConfig {
                name: cli.name.clone(),
                log_level: cli.verbose,
                ..Default::default()
            },
        )
    }

    fn run(&self, config: &ServerConfig) -> Result<CliCommandResult> {
        println!("{:#?}", config);
        Ok(CliCommandResult::Result(()))
    }
}
