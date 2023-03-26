#![allow(unused)]

use anyhow::Result;

mod cli;
mod config;

fn main() -> Result<()> {
    println!("Apily CLI");
    cli::run()
}
