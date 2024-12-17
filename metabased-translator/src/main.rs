use eyre::Result;
use metabased_translator::config::cli;
use metabased_translator::connectors::anvil;

// PoC deploying a contract using `anvil_set_code`, then interacting with it
#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber();
    anvil::run().await
}
