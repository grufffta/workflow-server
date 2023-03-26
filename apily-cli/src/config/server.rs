use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// default location for application configuration
const DEFAULT_CONFIG_LOCATION: &str = ".config/";

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
    /// http listening port
    port: u16,
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
                location: PathBuf::from(format!("{}/certs", DEFAULT_CONFIG_LOCATION)),
            },
            http: HttpConfig { port: 5080 },
            log_level: 0,
        }
    }
}
