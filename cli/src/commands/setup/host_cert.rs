use anyhow::{anyhow, Result};
use certs::{
    ca::create_cert_authority,
    config::{CertificateAuthorityConfig, DEFAULT_CA_LOCATION},
};
use clap::Args;
use std::{io, path::Path};

use crate::{
    commands::{CliArgs, CliCommand, CliCommandResult},
    config::AppConfig,
};

#[derive(Args)]
pub(super) struct CertificateCommandArgs {
    /// certificate authority directory name
    name: Option<String>,
    /// alternative subject names
    alt_names: Option<String>,
    /// include local host in subject names list
    #[arg(short = 'l', long, default_value_t = false)]
    include_localhost: bool,
    /// root certificate expiry, in days
    #[arg(short = 'r', long, default_value_t = 7300, value_name = "DAYS")]
    root_expiry: i64,
    /// intermediate certificate expiry, in days
    #[arg(short = 'i', long, default_value_t = 3650, value_name = "DAYS")]
    intermediate_expiry: i64,
    /// path to store certificate too
    #[arg(long, default_value_t = DEFAULT_CA_LOCATION.to_string())]
    store_location: String,
    /// force overwriting the certificate store
    #[arg(long, default_value_t = false)]
    force: bool,
}

impl CliCommand for CertificateCommandArgs {
    fn parse(&self, cli: &CliArgs) -> (&dyn CliCommand, AppConfig) {
        (
            self,
            AppConfig {
                name: cli.name.clone(),
                log_level: cli.verbose,
                certificates: CertificateAuthorityConfig {
                    location: self.store_location.clone(),
                },
                ..Default::default()
            },
        )
    }

    fn run(&self, config: &AppConfig) -> Result<CliCommandResult> {
        if cert_store_exists(&config.certificates.location)? && !self.force {
            return Err(anyhow!(
                "certificate store already exists, use force to overwrite."
            ));
        }
        let mut subject_alt_names = match &self.alt_names {
            Some(name) => vec![name.clone()],
            None => vec![],
        };
        if self.include_localhost {
            subject_alt_names.push("localhost".to_string());
        }
        Ok(CliCommandResult::Result(create_cert_authority(
            &self.name,
            &subject_alt_names,
            self.root_expiry,
            self.intermediate_expiry,
            &self.store_location,
        )?))
    }
}

fn cert_store_exists(location: &String) -> io::Result<bool> {
    Path::new(&format!("{location}/private/intermediate.key")).try_exists()
}
