use clap::Parser;

use crate::commands::{
    CliArguments,
    Commands,
    start_command::start_server,
    test_command::test_server,
};

mod commands;
mod threading;

fn main() {
    println!("Workflow Engine: Server!");

    let cli = CliArguments::parse();
    match &cli.command {
        Some(Commands::Start(start_args)) => start_server(start_args, cli.verbose),
        Some(Commands::Test(test_args)) => test_server(test_args),
        None => println!("Please enter a valid command"),
    }
}