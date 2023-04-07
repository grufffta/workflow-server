use rcgen::{BasicConstraints, CertificateParams, IsCa};

use crate::params::get_cert_parameters;

/// Creates the parameters used to generate a certificate
pub(super) fn get_ca_parameters(
    name: String,
    alt_names: Vec<String>,
    expiry_days: i64,
) -> CertificateParams {
    let mut params = get_cert_parameters(name, alt_names, expiry_days);
    {
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    }
    params
}
