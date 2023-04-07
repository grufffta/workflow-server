use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Conductor: automation workflow engine!");
    cli::run().await
}
