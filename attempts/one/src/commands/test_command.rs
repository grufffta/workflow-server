use clap::Args;

#[derive(Args)]
pub(crate) struct TestCommandArgs {
    /// Test database connections
    #[arg(short, long)]
    database: bool,
}

pub(crate) fn test_server(args: &TestCommandArgs) {
    println!("Testing setup\n\tTest Database: {}", args.database)
}