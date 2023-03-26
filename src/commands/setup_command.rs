extern crate rcgen;

use std::fs;
use std::path::{Path, PathBuf};

use clap::{Args, Subcommand};
use toml::Table;

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
    /// show application config
    Show(ShowCommandArgs),
}

#[derive(Args)]
pub(crate) struct CertificateCommandArgs {
    /// directory name
    name: Option<String>,

    /// alternative subject names
    alt_names: Option<String>,
    /// include local host in subject names list
    #[arg(short = 'l', long, default_value_t = false)]
    include_localhost: bool,
    /// root certificate expiry, in days
    #[arg(short = 'r', long, default_value_t = 7300, value_name = "DAYS")]
    root_expiry: i64,
    /// intermediate certificate expiry, in days
    #[arg(short = 'i', long, default_value_t = 3650, value_name = "DAYS")]
    intermediate_expiry: i64,
    /// path to store certificate too
    #[arg(long, default_value_t = String::from(".config/certs"))]
    store_location: String,
    /// force overwriting the certificate store
    #[arg(long, default_value_t = false)]
    force: bool,
}

#[derive(Args)]
pub(crate) struct PartnerCommandArgs {
    /// partner name
    name: Option<String>,
    /// partner id
    id: Option<String>,
    /// number of days until certificate is expired
    #[arg(short, long, default_value_t = 1825)]
    expiry: i64,
    /// import a public certificate, if this option is omitted a certificate and key will be generated
    #[arg(short, long, value_name = "FILE")]
    import_certificate: Option<PathBuf>,
}

#[derive(Args)]
pub(crate) struct ShowCommandArgs {}

pub(crate) fn configure(args: &SetupCommandArgs) {
    match &args.command {
        Some(SetupCommands::HostCert(certificate_args)) => create_certificate(certificate_args),
        Some(SetupCommands::Partner(partner_args)) => create_partner(partner_args),
        Some(SetupCommands::Show(_)) => show_config(),
        None => println!("Setup requires a valid subcommand"),
    }
}

fn show_config() {
    let config = fs::read_to_string(".config/app.toml")
        .unwrap_or_else(|_| "no configuration defined".to_string())
        .parse::<Table>().expect("invalid configuration file");
    println!("Server Configuration:\n\n{}", config)
}

fn create_certificate(args: &CertificateCommandArgs) {
    if Path::exists(format!("{}/private/intermediate.key", &args.store_location).as_ref()) && !args.force {
        println!("certificate store exists, use `--force` to overwrite.");
        return;
    }
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
