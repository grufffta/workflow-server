use super::config::server::{ServerConfig, DEFAULT_HTTP_ADDRESS, DEFAULT_HTTP_PORT};
use super::validators::{is_valid_ip_address, port_in_range};
use anyhow::{anyhow, Context, Result};
use axum::routing::get;
use axum::Router;
use clap::Args;
use std::{net::IpAddr, str::FromStr};

#[derive(Args)]
pub(crate) struct StartCommandArgs {
    /// Bind to specified IP address
    #[arg(default_value_t = IpAddr::from_str(DEFAULT_HTTP_ADDRESS).unwrap(), value_parser = is_valid_ip_address)]
    pub address: IpAddr,
    /// Start server on specified port
    #[arg(default_value_t = DEFAULT_HTTP_PORT, value_parser = port_in_range)]
    pub port: u16,
}

pub(crate) async fn run(config: ServerConfig) -> Result<()> {
    println!(
        "\n  Starting server on {}:{} verbosity level: {}",
        config.http.address, config.http.port, config.log_level
    );
    let app: Router = Router::new()
        .route("/", get(|| async { include_str!("../../assets/up.html") }))
        .fallback(|| async { include_str!("../../assets/404.html") });
    // axum::Server::bind(config.http.address);

    Ok(())
}
