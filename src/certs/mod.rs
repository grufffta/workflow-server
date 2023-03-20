use rcgen::Certificate;

use crate::certs::params::get_ca_parameters;

mod params;
mod write;

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

/// Creates a Root and Intermediate CA
pub(crate) fn create_ca(name: &Option<String>, alt_names: &[String], root_expiry_days: i64, intermediate_expiry_days: i64, store_location: &String) {
    let root_cert = get_root_certificate(name, alt_names, root_expiry_days);
    let intermediate_cert = get_intermediate_certificate(name, alt_names, intermediate_expiry_days);

    write::certificate(CertificateType::RootCA, store_location.to_owned(), &root_cert, None);
    println!("Certificate created for Root CA");

    write::certificate(CertificateType::IntermediateCA, store_location.to_owned(), &intermediate_cert, Some(&root_cert));
    println!("Certificate created for Intermediate CA");
}

fn get_root_certificate(name: &Option<String>, alt_names: &[String], expiry_days: i64) -> Certificate {
    let common_name = match &name
    {
        None => "Root Certificate Authority".to_string(),
        Some(s) => format!("{s} - Root CA"),
    };
    let params = get_ca_parameters(common_name, alt_names.to_vec(), expiry_days);
    Certificate::from_params(params).expect("unable to create root certificate")
}

fn get_intermediate_certificate(name: &Option<String>, alt_names: &[String], expiry_days: i64) -> Certificate {
    let common_name = match &name
    {
        None => "Intermediate Certificate Authority".to_string(),
        Some(s) => format!("{s} - Intermediate CA"),
    };
    let params = get_ca_parameters(common_name, alt_names.to_vec(), expiry_days);
    Certificate::from_params(params).expect("unable to create root certificate")
}
