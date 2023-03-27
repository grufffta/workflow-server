#![allow(unused)]
mod certs;
mod cli;
mod config;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Apily CLI");
    cli::run()
}
