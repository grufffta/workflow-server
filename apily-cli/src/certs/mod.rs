mod params;
mod write;

use self::params::get_ca_parameters;
use anyhow::{Context, Result};
use rcgen::Certificate;

enum CertificateType {
    /// Root Certificate Authority
    RootCA,
    /// Intermediate Certificate Authority
    IntermediateCA,
}

enum CertificateStoreType {
    /// Store in public key store
    Public,
    /// Store in private key store
    Private,
}

/// Creates a certificate authority with an intermediate signed by the root
pub(crate) fn create_ca(
    name: &Option<String>,
    alt_names: &[String],
    root_expiry_days: i64,
    intermediate_expiry_days: i64,
    store_location: &String,
) -> Result<()> {
    let root_cert = create_certificate(
        CertificateType::RootCA,
        name,
        alt_names.to_vec(),
        root_expiry_days,
        store_location,
        None,
    )
    .with_context(|| "unable to create root certificate")?;

    let intermediate_cert = create_certificate(
        CertificateType::IntermediateCA,
        name,
        alt_names.to_vec(),
        intermediate_expiry_days,
        store_location,
        Some(root_cert),
    )
    .with_context(|| "unable to create root certificate")?;

    Ok(())
}

fn create_certificate(
    certificate_type: CertificateType,
    name: &Option<String>,
    alt_names: Vec<String>,
    expiry_days: i64,
    store_location: &String,
    signer: Option<Certificate>,
) -> Result<Certificate, rcgen::RcgenError> {
    let certificate = Certificate::from_params(get_ca_parameters(
        ca_name(name.clone(), &certificate_type),
        alt_names,
        expiry_days,
    ))?;

    write::certificate(
        certificate_type,
        store_location.to_owned(),
        &certificate,
        signer,
    );
    Ok(certificate)
}

fn ca_name(name: Option<String>, cert_type: &CertificateType) -> String {
    match cert_type {
        CertificateType::RootCA => match &name {
            Some(s) => format!("{s} - Root CA"),
            None => "Root Certificate Authority".to_string(),
        },
        CertificateType::IntermediateCA => match &name {
            Some(s) => format!("{s} - Intermediate CA"),
            None => "Intermediate Certificate Authority".to_string(),
        },
    }
}
