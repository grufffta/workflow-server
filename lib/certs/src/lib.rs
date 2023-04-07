pub mod ca;
pub mod config;
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
