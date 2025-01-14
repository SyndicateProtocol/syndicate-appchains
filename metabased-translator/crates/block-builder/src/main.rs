//! Block builder service for processing and building L3 blocks.

use block_builder::config::{cli, Configuration};
use block_builder::connectors::anvil::MetaChainProvider;
use block_builder::rollups::arbitrum::arbitrum_builder::ArbitrumBlockBuilder;
use block_builder::rollups::utils::sequencing_chain_blocks_to_mbtxs;
use block_builder::rollups::utils::BlockBuilder;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Block Builder
    cli::init_tracing_subscriber();
    let config = Configuration::parse();

    // Initialize MChain
    let mchain = MetaChainProvider::start(config).await?;
    // TODO: Setup MChain if we are starting from scratch
    //       - Deploy contracts
    //       - Fund accounts

    // Initialize block builder
    let builder = ArbitrumBlockBuilder::new();

    //loop
    loop {
        // TODO: Consume Blob of blocks from ingestor
        let blocks = vec![];

        // Process sequencing chain blocks into mB transactions
        let mbtxs = sequencing_chain_blocks_to_mbtxs(blocks);

        // TODO: [OP / ARB] Process deposit transactions

        // [OP / ARB] Build batch
        let batch_txn = builder.build_batch_txn(mbtxs);

        // Submit batch transaction to mchain
        mchain.submit_txn(batch_txn).await?;

        // Mine mchain block
        mchain.mine_block().await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
