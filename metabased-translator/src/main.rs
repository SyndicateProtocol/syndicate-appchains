use eyre::Result;
use metabased_translator::config::cli;
use metabased_translator::connectors::anvil;

#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber();
    anvil::run().await
}
