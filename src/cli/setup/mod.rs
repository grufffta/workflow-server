use std::fs;

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
struct ShowCommandArgs {
    #[arg(long, short = 'c', default_value_t = false)]
    host_cert: bool,
}

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
        match self.host_cert {
            true => {
                let root_cert = fs::read_to_string(format!(
                    "{}/public/root.pub",
                    config.certificates.location
                ))?;
                let host_cert = fs::read_to_string(format!(
                    "{}/public/root.pub",
                    config.certificates.location
                ))?;
                println!(
                    "\n----- ROOT CA CERTIFICATE -----\n\n{}\n----- HOST CERTIFICATE -----\n\n{}\n",
                    root_cert, host_cert
                );
            }
            false => println!("{:#?}", config),
        }
        Ok(CliCommandResult::Result(()))
    }
}
