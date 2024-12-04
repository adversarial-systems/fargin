use anyhow::Result;
use clap::Parser;
use fargin::{cli::Cli, run};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    run(cli).await
}
