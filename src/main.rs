extern crate core;

use clap::Parser;

use crate::commands::{
    CliArguments,
    Commands,
    setup_command::configure,
    start_command::start_server,
    test_command::test_server,
};

mod commands;
mod threading;
mod certs;

fn main() {
    println!("Workflow Engine: Server!");

    let cli = CliArguments::parse();
    match &cli.command {
        Some(Commands::Start(start_args)) => start_server(start_args, cli.verbose),
        Some(Commands::Setup(setup_args)) => configure(setup_args),
        Some(Commands::Test(test_args)) => test_server(test_args),
        None => println!("Please enter a valid command"),
    }
}