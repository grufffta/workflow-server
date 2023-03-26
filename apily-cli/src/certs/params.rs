use std::ops::Add;

use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, IsCa, KeyPair,
    PKCS_ECDSA_P256_SHA256,
};
use time::OffsetDateTime;

/// Creates the parameters used to generate a certificate
pub(crate) fn get_ca_parameters(
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

fn get_cert_parameters(
    name: String,
    alt_names: Vec<String>,
    expiry_days: i64,
) -> CertificateParams {
    let now = OffsetDateTime::now_utc();
    let expiry = now.add(time::Duration::days(expiry_days));
    let mut params = CertificateParams::new(alt_names);
    {
        params.not_before = now;
        params.not_after = expiry;
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, name);
        params.key_pair = Option::from(
            KeyPair::generate(&PKCS_ECDSA_P256_SHA256).expect("unable to generate private key"),
        );
    }
    params
}
