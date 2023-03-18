use clap::Parser;

use crate::commands::{CliArguments, Commands};

mod commands;

fn main() {
    println!("Apily Workflow Server!");

    let cli = CliArguments::parse();
    match &cli.command {
        Some(Commands::Start(start)) => commands::start_command::start_server(start),
        Some(Commands::Test(test)) => commands::test_command::test_server(test),
        None => { println!("Please enter a valid command") }
    }
}