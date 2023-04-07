use serde::{Deserialize, Serialize};

pub const DEFAULT_CA_LOCATION: &str = ".config/certs";

/// represents the configuration of a certificate authority for
/// issuing public/private key pairs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CertificateAuthorityConfig {
    /// location of the certificate store
    pub location: String,
}

impl Default for CertificateAuthorityConfig {
    fn default() -> Self {
        Self {
            location: DEFAULT_CA_LOCATION.to_string(),
        }
    }
}
