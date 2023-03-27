use crate::config::server::ServerConfig;
use anyhow::{Context, Result};
use clap::builder::TypedValueParser;
use std::{
    error::Error,
    fmt::format,
    fs,
    path::{Path, PathBuf},
    result,
};

pub(crate) mod server;

/// Loads the server configuration from the app config file
pub(super) fn load(name: Option<String>, verbosity: u8, path: PathBuf) -> Result<ServerConfig> {
    match Path::exists(path.as_path()) {
        true => open(&path),
        false => save(
            ServerConfig {
                name,
                log_level: verbosity,
                ..Default::default()
            },
            &path,
        ),
    }
}

fn save(config: ServerConfig, path: &PathBuf) -> Result<ServerConfig> {
    let toml = toml::to_string(&config)
        .with_context(|| format!("unable to serialize config: {:?}", config))?;

    fs::create_dir_all(path.parent().unwrap())?;

    fs::write(path, toml).with_context(|| format!("Unable to write {}", path.display()))?;

    Ok(config)
}

fn open(path: &PathBuf) -> Result<ServerConfig> {
    let toml = fs::read_to_string(path)
        .with_context(|| format!("unable to read file {}", path.display()))?;

    let config: ServerConfig = toml::from_str(toml.as_str())
        .with_context(|| format!("unable to deserialize {}", path.display()))?;

    Ok(config)
}
