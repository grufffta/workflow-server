use std::path::{Path, PathBuf};

use crate::{certs::create_ca, config::server::ServerConfig};

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
    /// Create Host Certificate Authority
    ///
    /// Creates a CA that will be used to issue certificates for this host.
    /// Alternaticvely you can add a intermediate key and certificate to the
    /// config store.
    ///
    /// ** Errors **
    /// Will return an error if a intermediate key exists in the target directory.
    pub(crate) fn create_host_ca(&self, mut config: ServerConfig) -> Result<()> {
        if Path::new(format!("{}/private/intermediate.key", self.store_location).as_str()).exists()
            && !self.force
        {
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
        create_ca(
            &self.name,
            &subject_alt_names,
            self.root_expiry,
            self.intermediate_expiry,
            &self.store_location,
        )
    }
}
