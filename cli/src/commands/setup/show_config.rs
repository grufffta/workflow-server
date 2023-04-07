use anyhow::Result;
use clap::Args;
use std::fs;

use crate::{
    commands::{CliArgs, CliCommand, CliCommandResult},
    config::AppConfig,
};

#[derive(Args)]
pub(super) struct ShowCommandArgs {
    #[arg(long, short = 'c', default_value_t = false)]
    host_cert: bool,
}

impl CliCommand for ShowCommandArgs {
    fn parse(&self, cli: &CliArgs) -> (&dyn CliCommand, AppConfig) {
        (
            self,
            AppConfig {
                name: cli.name.clone(),
                log_level: cli.verbose,
                ..Default::default()
            },
        )
    }

    fn run(&self, config: &AppConfig) -> Result<CliCommandResult> {
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
