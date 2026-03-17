use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = aihelp::Cli::parse();
    aihelp::run(cli).await
}
