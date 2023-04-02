#![allow(unused)]

use anyhow::Result;

mod certs;
mod cli;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Apily Workflow Server!");
    cli::run().await
}
