//! Block builder service for processing and building L3 blocks.

use block_builder::config::{cli, Configuration};
use block_builder::connectors::anvil::MetaChainProvider;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber();
    let config = Configuration::parse();

    let mchain = MetaChainProvider::start(config).await?;

    mchain.mine_block().await?;

    //loop
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
