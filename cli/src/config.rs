use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use utils::update_if_changed;

use certs::config::CertificateAuthorityConfig;
use server::config::HttpConfig;

const CONFIG_LOCATION: &str = ".config/app.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
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

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: None,
            certificates: CertificateAuthorityConfig::default(),
            http: HttpConfig::default(),
            log_level: 0,
            config_location: CONFIG_LOCATION.to_string(),
        }
    }
}

/// merge the configuration arguments with the file stored on disk
pub(super) fn merge(filename: &Path, config: AppConfig) -> Result<AppConfig> {
    let mut stored = open(filename)?;
    update_if_changed!(stored.name, config.name);
    update_if_changed!(stored.http.address, config.http.address);
    update_if_changed!(stored.http.port, config.http.port);
    update_if_changed!(stored.certificates.location, config.certificates.location);
    Ok(stored)
}

/// open the config file from disk, or save the default configuration and return
pub(super) fn open(filename: &Path) -> Result<AppConfig> {
    match Path::exists(filename) {
        true => open_config_file(filename),
        false => save(filename, &AppConfig::default()),
    }
}

/// save the application configuration to `app.toml`
pub(super) fn save(filename: &Path, config: &AppConfig) -> Result<AppConfig> {
    let toml = toml::to_string(config)
        .with_context(|| format!("unable to serialize config: {:?}", config))?;
    fs::create_dir_all(filename.parent().unwrap())
        .with_context(|| "unable to create configuration directory")?;
    fs::write(filename, toml)
        .with_context(|| format!("unable to write file {}", filename.display()))?;
    Ok(config.clone())
}

fn open_config_file(filename: &Path) -> Result<AppConfig> {
    let toml = fs::read_to_string(filename)
        .with_context(|| format!("unable to read file {}", filename.display()))?;
    let config: AppConfig =
        toml::from_str(toml.as_str()).with_context(|| "unable to deserialize configuration")?;
    Ok(config)
}
