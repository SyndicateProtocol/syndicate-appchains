use block_builder::config::cli;
use block_builder::connectors::anvil;
use eyre::Result;

// PoC deploying a contract using `anvil_set_code`, then interacting with it
#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber();
    anvil::run().await
}
