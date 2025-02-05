use eyre::Result;
use metabased_sequencer::presentation::cli;

#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber()?;
    cli::run().await
}
