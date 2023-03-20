use std::fs;

use rcgen::Certificate;

use crate::certs::{CertificateStoreType, CertificateType};

pub(super) fn certificate(cert_type: CertificateType, path: String, cert: &Certificate, signer: Option<&Certificate>) {
    let filename = match cert_type
    {
        CertificateType::RootCA => "root",
        CertificateType::IntermediateCA => "intermediate",
    };
    write_pem(path.to_owned(), filename.to_owned(), CertificateStoreType::Public, match signer {
        None => cert.serialize_pem().unwrap(),
        Some(s) => cert.serialize_pem_with_signer(s).unwrap()
    });
    write_pem(path, filename.to_owned(), CertificateStoreType::Private, cert.serialize_private_key_pem());
}

fn write_pem(mut path: String, mut file: String, location: CertificateStoreType, contents: String) {
    match location {
        CertificateStoreType::Public => {
            path.push_str("/public");
            file.push_str(".pub");
        }
        CertificateStoreType::Private => {
            path.push_str("/private");
            file.push_str(".key");
        }
    }
    fs::create_dir_all(path.clone()).unwrap_or_else(|_| panic!("unable to create path {path}"));
    path.push('/');
    path.push_str(file.as_str());
    fs::write(path.clone(), contents).unwrap_or_else(|_| panic!("unable to write file to {path}"));
}
