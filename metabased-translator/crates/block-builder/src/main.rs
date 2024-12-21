use block_builder::config::cli;
use block_builder::connectors::anvil::MetaChainProvider;
use block_builder::rollups::arb::translator::ArbTranslator;
use block_builder::rollups::optimism::translator::OPTranslator;
use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber();

    let mut provider = MetaChainProvider::default();
    provider.start()?;
    let mchain = Arc::new(provider);

    // TEMP: Mine an OP batcher transaction
    let op_translator = OPTranslator::new(mchain.clone());
    op_translator.send_batcher_transaction().await?;

    // TEMP: Mine an ARB batcher transaction
    let arb_translator = ArbTranslator::new(mchain.clone());
    arb_translator.init_rollup().await?;
    arb_translator.send_batcher_transaction().await?;

    mchain.mine_block().await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
