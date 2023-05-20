/// Represents a trading partner that can receive or send files
struct TradingPartner {
    /// id representing this trading partner
    id: i32,
    /// name of this trading partner
    name: String,
    /// client reference used by third party
    client_reference: String,
}

/// Represents the certificate settings for this trading partner
struct TradingPartnerCertificateSettings {
    /// the certificate thumbprint
    partner_thumbprint: String,
}

/// Represents the settings used in standardised electronic document formats, eg: EDIFACT
struct TradingPartnerEdiSettings {}
