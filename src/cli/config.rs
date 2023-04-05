use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    net::IpAddr,
    path::{self, Path, PathBuf},
    str::FromStr,
};

pub(crate) const CONFIG_LOCATION: &str = ".config/app.toml";
pub(crate) const DEFAULT_CERT_STORE_LOCATION: &str = ".config/certs";
pub(crate) const DEFAULT_ROOTCA_EXPIRY: i64 = 7300;
pub(crate) const DEFAULT_INTCA_EXPIRY: i64 = 3650;
pub(crate) const DEFAULT_HTTP_ADDRESS: &str = "0.0.0.0";
pub(crate) const DEFAULT_HTTP_PORT: u16 = 5080;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ServerConfig {
    /// server friendly name
    pub name: Option<String>,
    /// certificate authority and store configuration
    pub certificates: CertificateAuthorityConfig,
    /// http server configuration
    pub http: HttpConfig,
    /// application logging level
    /// * 0 : No Logging
    /// * 1 : Normal Logging
    /// * 2 : Detailed Logging
    pub log_level: u8,
    /// config file
    #[serde(skip)]
    pub config_location: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct HttpConfig {
    /// http listener address
    pub address: IpAddr,
    /// http listener port
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub(crate) struct CertificateAuthorityConfig {
    /// location of certificate store
    pub location: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: None,
            certificates: CertificateAuthorityConfig {
                location: DEFAULT_CERT_STORE_LOCATION.to_string(),
            },
            http: HttpConfig {
                address: IpAddr::from_str(DEFAULT_HTTP_ADDRESS).unwrap(),
                port: DEFAULT_HTTP_PORT,
            },
            log_level: 0,
            config_location: CONFIG_LOCATION.to_string(),
        }
    }
}

pub(super) fn open(filename: &Path) -> Result<ServerConfig> {
    match Path::exists(filename) {
        true => {
            let toml = fs::read_to_string(filename)
                .with_context(|| format!("unable to read file {}", filename.display()))?;
            let config: ServerConfig = toml::from_str(toml.as_str())
                .with_context(|| "unable to deserialize configuration")?;
            Ok(config)
        }
        false => save(&ServerConfig::default(), filename),
    }
}

pub(super) fn save(config: &ServerConfig, filename: &Path) -> Result<ServerConfig> {
    let toml = toml::to_string(config)
        .with_context(|| format!("unable to serialize config: {:?}", config))?;
    fs::create_dir_all(filename.parent().unwrap())
        .with_context(|| "unable to create configuration directory")?;
    fs::write(filename, toml)
        .with_context(|| format!("unable to write file {}", filename.display()))?;
    Ok(config.clone())
}

#[macro_export]
macro_rules! update_if_changed {
    ($source:expr, $param:expr) => {{
        if $source != $param {
            $source = $param
        }
    }};
}

#[macro_export]
macro_rules! merge_config {
    ($config:expr, $params:expr) => {{
        let mut config: ServerConfig = $config;
        update_if_changed![config.name, $params.name];
        update_if_changed![config.certificates.location, $params.certificates.location];
        update_if_changed![config.http.address, $params.http.address];
        update_if_changed![config.http.port, $params.http.port];
        config
    }};
}
#[macro_export]
macro_rules! save_config {
    ($config:expr, $should_save:expr, $file:expr) => {{
        let config: ServerConfig = $config;
        if $should_save {
            save(&config, $file)?;
        }
        config
    }};
}
