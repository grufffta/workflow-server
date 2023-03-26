use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::Args;

#[derive(Args)]
pub(crate) struct CertificateCommandArgs {
    /// directory name
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
    #[arg(long, default_value_t = String::from(".config/certs"))]
    store_location: String,
    /// force overwriting the certificate store
    #[arg(long, default_value_t = false)]
    force: bool,
}
impl CertificateCommandArgs {
    pub(crate) fn create(&self, mut config: crate::config::server::ServerConfig) -> Result<()> {
        if Path::new(format!("{}/private/intermediate.key", self.store_location).as_str()).exists()
            && !self.force
        {
            return Err(anyhow!(
                "certificate store already exists, use force to overwrite."
            ));
        }
        Ok(())
    }
}
