//! Integration tests for the poster
pub mod utils;
use alloy::{
    consensus::Transaction,
    eips::BlockNumberOrTag,
    primitives::{address, B256, U256},
    providers::Provider,
    rpc::types::BlockTransactions,
    sol_types::SolCall,
};
use contract_bindings::arbitrum::assertionposter::AssertionPoster;
use core::panic;
use eyre::Result;
use jsonrpsee::types::ErrorObjectOwned;
use metabased_poster::{config::Config, metrics::PosterMetrics, poster, types::NitroBlock};
use prometheus_client::registry::Registry;
use std::{str::FromStr, time::Duration};
use url::Url;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    shared::logger::set_global_default_subscriber();
}

#[tokio::test]
async fn e2e_poster_test() -> Result<()> {
    let set_port = 8080;
    let app_port = 8081;

    let (_set_anvil, set_provider) = utils::start_anvil(1, set_port).await?;

    let appchain_url = format!("0.0.0.0:{}", app_port);
    let server = jsonrpsee::server::Server::builder().build(appchain_url).await?;
    let mut module = jsonrpsee::RpcModule::new(());
    let nitro_block = NitroBlock {
        number: U256::from(1),
        l1_block_number: U256::from(1),
        timestamp: U256::from(1),
        hash: B256::from([1u8; 32]),
        send_root: B256::from([2u8; 32]),
    };

    module.register_method(
        "eth_getBlockByNumber",
        move |_, _, _| -> Result<NitroBlock, ErrorObjectOwned> { Ok(nitro_block) },
    )?;
    let _handler = server.start(module);

    let assertion_poster_contract_address = address!("0x32a725c440Ab3e855048C4620862754B7c51828C");
    let config = Config {
        settlement_rpc_url: Url::from_str(&format!("http://localhost:{}", set_port))?,
        appchain_rpc_url: Url::from_str(&format!("http://localhost:{}", app_port))?,
        assertion_poster_contract_address,
        private_key: utils::DEFAULT_PRIVATE_KEY_SIGNER.to_string(),
        polling_interval: Duration::from_secs(60),
        metrics_port: 9090,
    };

    let mut registry = Registry::default();
    let metrics = PosterMetrics::new(&mut registry);

    let _poster_handler = tokio::spawn(poster::run(config, metrics));
    tokio::time::sleep(Duration::from_secs(1)).await;

    let block_option = set_provider.get_block_by_number(BlockNumberOrTag::Latest).full().await?;

    assert!(block_option.is_some());

    let block = block_option.unwrap();
    assert_eq!(block.header.number, 1);
    assert_eq!(block.transactions.len(), 1);

    let transactions = match block.transactions {
        BlockTransactions::Full(tx) => tx,
        _ => panic!("Empty block"),
    };
    let tx = transactions[0].clone();
    assert_eq!(tx.inner.to(), Some(assertion_poster_contract_address));
    let decode = AssertionPoster::postAssertionCall::abi_decode(tx.inner.input(), true)?;
    assert_eq!(nitro_block.hash, decode.blockHash);
    assert_eq!(nitro_block.send_root, decode.sendRoot);

    Ok(())
}
