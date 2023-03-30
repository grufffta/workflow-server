use serde::{Deserialize, Serialize};
use std::{net::IpAddr, path::PathBuf, str::FromStr};

/// default location for application configuration
pub(crate) const DEFAULT_CERT_LOCATION: &str = ".config/certs";
pub(crate) const DEFAULT_HTTP_PORT: u16 = 5080;
pub(crate) const DEFAULT_HTTP_ADDRESS: &str = "0.0.0.0";

/// Represents the server configuration
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ServerConfig {
    /// server friendly name
    pub name: Option<String>,

    /// certificate authority and store configuration
    pub certificates: CertificateAuthorityConfig,
    /// http configuration
    pub http: HttpConfig,
    /// application logging level
    /// * 0 : No Logging
    /// * 1 : Normal Logging
    /// * 2 : Detailed Logging
    pub log_level: u8,
}

/// Represents the server http configuration
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct HttpConfig {
    /// http listening address, defaults to all IPs
    pub address: IpAddr,
    /// http listening port
    pub port: u16,
}

/// Represent the server certificate configuration
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CertificateAuthorityConfig {
    /// location of certificate store
    location: PathBuf,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: None,
            certificates: CertificateAuthorityConfig {
                location: PathBuf::from(DEFAULT_CERT_LOCATION),
            },
            http: HttpConfig {
                address: IpAddr::from_str(DEFAULT_HTTP_ADDRESS).unwrap(),
                port: DEFAULT_HTTP_PORT,
            },
            log_level: 0,
        }
    }
}
