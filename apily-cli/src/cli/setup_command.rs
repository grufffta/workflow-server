use super::certificates::CertificateCommandArgs;
use super::config::server::ServerConfig;
use anyhow::{anyhow, Context, Result};
use clap::{Args, Subcommand};
use std::error::Error;

/// server setup
#[derive(Args)]
pub(crate) struct SetupCommandArgs {
    #[command(subcommand)]
    pub command: Option<SetupCommands>,
}

/// commands related to server setup
#[derive(Subcommand)]
pub(crate) enum SetupCommands {
    /// setup host server certificate
    HostCert(CertificateCommandArgs),
    /// show application configuration
    Show(ShowCommandArgs),
}

#[derive(Args)]
pub(crate) struct ShowCommandArgs {}

pub(crate) fn run(args: &SetupCommandArgs, config: ServerConfig) -> Result<()> {
    match &args.command {
        Some(SetupCommands::HostCert(certificate_args)) => {
            certificate_args.create_host_ca(config);
            Ok(())
        }
        Some(SetupCommands::Show(_)) => {
            println!("{:#?}", config);
            Ok(())
        }
        None => Err(anyhow!("setup requires a sub command")),
    }
}
