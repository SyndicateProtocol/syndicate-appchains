use block_builder::config::cli;
use block_builder::connectors::anvil::MetaChainProvider;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber();
    let config = cli::init_config();

    let mut mchain = MetaChainProvider::with_config(config);
    mchain.start()?;

    mchain.mine_block().await?;

    //loop
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
