use crate::server;

use super::{
    config::{HttpConfig, ServerConfig, DEFAULT_HTTP_ADDRESS, DEFAULT_HTTP_PORT},
    validators::{is_port_in_range, is_valid_ip_address},
    CLiArguments, CliCommand, CliCommandResult,
};
use anyhow::Result;
use axum::{routing::get, Router};
use clap::Args;
use core::pin;
use std::{
    future::Future,
    net::{IpAddr, SocketAddr},
    pin::Pin,
    str::FromStr,
};

/// start a server instance
#[derive(Args)]
pub(crate) struct StartCommandArgs {
    /// Bind the server to specified IP address
    #[arg(value_parser = is_valid_ip_address, default_value_t = IpAddr::from_str(DEFAULT_HTTP_ADDRESS).unwrap())]
    address: IpAddr,
    /// Bind the server on the specified port
    #[arg(value_parser = is_port_in_range, default_value_t = DEFAULT_HTTP_PORT)]
    port: u16,
}

impl CliCommand for StartCommandArgs {
    fn parse(&self, cli: &CLiArguments) -> (&dyn CliCommand, ServerConfig) {
        (
            self,
            ServerConfig {
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

    fn run(&self, config: &ServerConfig) -> Result<CliCommandResult> {
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
