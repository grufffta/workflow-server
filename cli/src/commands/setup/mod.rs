mod host_cert;
mod show_config;

use self::{host_cert::CertificateCommandArgs, show_config::ShowCommandArgs};
use super::{CliArgs, CliCommand, CliCommandResult};
use crate::config::AppConfig;
use anyhow::Result;
use clap::{Args, Subcommand};

/// server setup
#[derive(Args)]
pub(crate) struct SetupCommandArgs {
    #[command(subcommand)]
    command: Option<SetupCommands>,
}

/// Setup command options
#[derive(Subcommand)]
enum SetupCommands {
    /// setup host server certificate authority
    HostCert(CertificateCommandArgs),
    /// show application configuration
    Show(ShowCommandArgs),
}

impl CliCommand for SetupCommandArgs {
    fn parse(&self, cli: &CliArgs) -> (&dyn CliCommand, AppConfig) {
        match &self.command {
            Some(SetupCommands::HostCert(certificate_args)) => certificate_args.parse(cli),
            Some(SetupCommands::Show(show_config_args)) => show_config_args.parse(cli),
            None => todo!(),
        }
    }

    fn run(&self, _config: &AppConfig) -> Result<CliCommandResult> {
        unreachable!("sub command run will be called.")
    }
}
