use super::config::server::{ServerConfig, DEFAULT_HTTP_ADDRESS, DEFAULT_HTTP_PORT};
use super::validators::{is_valid_ip_address, port_in_range};
use anyhow::{anyhow, Context, Result};
use clap::Args;
use std::{net::IpAddr, str::FromStr};

#[derive(Args)]
pub(crate) struct StartCommandArgs {
    /// Bind to specified IP address
    #[arg(default_value_t = IpAddr::from_str(DEFAULT_HTTP_ADDRESS).unwrap(), value_parser = is_valid_ip_address)]
    address: IpAddr,
    /// Start server on specified port
    #[arg(default_value_t = DEFAULT_HTTP_PORT, value_parser = port_in_range)]
    port: u16,
}

pub(crate) fn run(args: &StartCommandArgs, config: ServerConfig) -> Result<()> {
    Ok(())
}
