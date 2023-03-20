extern crate rcgen;

use clap::{Args, Subcommand};

use crate::certs::create_ca;

#[derive(Args)]
pub(crate) struct SetupCommandArgs {
    #[command(subcommand)]
    pub command: Option<SetupCommands>,
}

#[derive(Subcommand)]
pub(crate) enum SetupCommands {
    /// setup server host certificate
    HostCert(CertificateCommandArgs),
    /// setup partner
    Partner(PartnerCommandArgs),
}

#[derive(Args)]
pub(crate) struct CertificateCommandArgs {
    /// directory name
    name: Option<String>,

    /// alternative subject names
    alt_names: Option<String>,
    /// root certificate expiry, in days
    #[arg(short = 'r', long, default_value_t = 3650, value_name = "DAYS")]
    root_expiry: i64,
    /// intermediate certificate expiry, in days
    #[arg(short = 'i', long, default_value_t = 1825, value_name = "DAYS")]
    intermediate_expiry: i64,

    /// include local host in subject names list
    #[arg(short = 'l', long, default_value_t = false)]
    include_localhost: bool,
    /// path to store certificate too
    #[arg(long, default_value_t = String::from(".config/certs"))]
    store_location: String,
}

#[derive(Args)]
pub(crate) struct PartnerCommandArgs {

}

pub(crate) fn configure(args: &SetupCommandArgs) {
    match &args.command {
        Some(SetupCommands::HostCert(certificate_args)) => create_certificate(certificate_args),
        Some(SetupCommands::Partner(partner_args)) => create_partner(partner_args),
        None => println!("Setup requires a valid subcommand"),
    }
}

fn create_certificate(args: &CertificateCommandArgs) {
    let mut subject_alt_names = vec![];

    match &args.alt_names {
        None => {}
        Some(name) => subject_alt_names.push(name.to_string()),
    };

    if args.include_localhost {
        subject_alt_names.push("localhost".to_string());
    }
    create_ca(&args.name, &subject_alt_names, args.root_expiry, args.intermediate_expiry, &args.store_location);
}

fn create_partner(args: &PartnerCommandArgs) {
    todo!()
}
