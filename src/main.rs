use clap::Parser;

use crate::commands::{CliArguments, Commands};

mod commands;

fn main() {
    println!("Apily Workflow Server!");

    let cli = CliArguments::parse();
    match &cli.command {
        Some(Commands::Start(start_args)) => commands::start_command::start_server(start_args),
        Some(Commands::Test(test_args)) => commands::test_command::test_server(test_args),
        None => println!("Please enter a valid command"),
    }
}