#![allow(unused)]
mod certs;
mod cli;
mod config;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Apily CLI");
    cli::run().await
}
