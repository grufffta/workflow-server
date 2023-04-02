use self::params::get_ca_parameters;
use anyhow::{Context, Result};
use rcgen::Certificate;

mod params;
mod write;

enum CertificateType {
    /// Root Certification Authority
    RootCA,
    /// Intermediate Certification Authority
    IntermediateCA,
}

enum CertificateStoreType {
    /// Represents a public certificate
    Public,
    /// Represents a private certificate
    Private,
}

pub(crate) fn create_cert_authority(
    name: &Option<String>,
    alt_names: &[String],
    root_expiry_days: i64,
    int_expiry_days: i64,
    store_location: &str,
) -> Result<()> {
    let root = create_certificate(
        CertificateType::RootCA,
        name,
        alt_names.to_vec(),
        root_expiry_days,
        store_location,
        None,
    )
    .with_context(|| "unable to create root ca certificate")?;

    create_certificate(
        CertificateType::IntermediateCA,
        name,
        alt_names.to_vec(),
        int_expiry_days,
        store_location,
        Some(root),
    )
    .with_context(|| "unable to create intermediate ca certificate")?;
    Ok(())
}

fn create_certificate(
    certificate_type: CertificateType,
    name: &Option<String>,
    alt_names: Vec<String>,
    expiry_days: i64,
    store_location: &str,
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
