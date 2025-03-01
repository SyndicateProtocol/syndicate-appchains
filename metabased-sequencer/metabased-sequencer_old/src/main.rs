use eyre::Result;
use metabased_sequencer::presentation::{cli, configuration::Configuration, server};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber()?;

    let args = Configuration::parse()?;
    let (addr, handle) = server::run(&args).await?;

    info!(
        addr = %addr,
        "Metabased Sequencer server running"
    );

    // Keep the server running
    handle.stopped().await;

    Ok(())
}
