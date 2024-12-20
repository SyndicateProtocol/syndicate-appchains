use block_builder::config::cli;
use eyre::Result;
use block_builder::connectors::anvil::MetaChainProvider;

// PoC deploying a contract using `anvil_set_code`, then interacting with it
#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber();
    let mut mchain = MetaChainProvider::default();
    mchain.start()?;

    mchain.mine_block().await?;

    //loop
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
