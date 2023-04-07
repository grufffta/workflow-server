use clap::Args;
use server::config::{HttpConfig, DEFAULT_HTTP_ADDRESS, DEFAULT_HTTP_PORT};
use std::{net::IpAddr, str::FromStr};
use utils::validators::{is_port_in_range, is_valid_ip_address};

use crate::{commands::CliCommandResult, config::AppConfig};

use super::CliCommand;

#[derive(Args)]
pub(crate) struct StartCommandArgs {
    /// Bind the server to specified IP address
    #[arg(value_parser = is_valid_ip_address, default_value_t = IpAddr::from_str(DEFAULT_HTTP_ADDRESS).unwrap())]
    pub address: IpAddr,
    /// Bind the server on the specified port
    #[arg(value_parser = is_port_in_range, default_value_t = DEFAULT_HTTP_PORT)]
    pub port: u16,
}

impl CliCommand for StartCommandArgs {
    fn parse(&self, cli: &super::CliArgs) -> (&dyn CliCommand, crate::config::AppConfig) {
        (
            self,
            AppConfig {
                name: cli.name.clone(),
                log_level: cli.verbose,
                http: HttpConfig {
                    address: self.address,
                    port: self.port,
                },
                ..Default::default()
            },
        )
    }

    fn run(&self, config: &AppConfig) -> anyhow::Result<super::CliCommandResult> {
        println!(
            "\n  Starting server on {}:{} verbosity level: {}",
            config.http.address, config.http.port, config.log_level
        );
        Ok(CliCommandResult::Future(Box::pin(server::launch(
            config.http.address,
            config.http.port,
        ))))
    }
}
