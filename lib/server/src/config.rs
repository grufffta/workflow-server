use serde::{Deserialize, Serialize};
use std::{net::IpAddr, str::FromStr};

pub const DEFAULT_HTTP_ADDRESS: &str = "0.0.0.0";
pub const DEFAULT_HTTP_PORT: u16 = 5080;

/// Represents the configuration of the HTTP server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpConfig {
    /// http listening address
    pub address: IpAddr,
    /// http listener port
    pub port: u16,
}

impl Default for HttpConfig {
    fn default() -> Self {
        HttpConfig {
            address: IpAddr::from_str(DEFAULT_HTTP_ADDRESS).unwrap(),
            port: DEFAULT_HTTP_PORT,
        }
    }
}
