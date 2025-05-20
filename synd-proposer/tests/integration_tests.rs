//! Integration tests for the Proposer
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
use prometheus_client::registry::Registry;
use std::{str::FromStr, time::Duration};
use synd_proposer::{config::Config, metrics::ProposerMetrics, proposer, types::NitroBlock};
use test_utils::{port_manager::PortManager, wait_until};
use tokio::time::sleep;
use url::Url;

#[ctor::ctor]
fn init() {
    shared::logger::set_global_default_subscriber();
}

#[tokio::test]
async fn e2e_proposer_test() -> Result<()> {
    let set_port = PortManager::instance().next_port().await;
    let app_port = PortManager::instance().next_port().await;
    let proposer_port = PortManager::instance().next_port().await;

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
        port: proposer_port,
        metrics_port: 9090,
    };

    let mut registry = Registry::default();
    let metrics = ProposerMetrics::new(&mut registry);

    let _proposer_handler = tokio::spawn(proposer::run(config, metrics));
    sleep(Duration::from_secs(1)).await;

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

    // Trigger the /post endpoint
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://localhost:{}/post", proposer_port))
        .send()
        .await
        .expect("Failed to send POST to /post");

    assert!(response.status().is_success(), "Expected 200 OK, got {}", response.status());
    let body = response.text().await?;
    assert!(body.contains("Assertion posted successfully"), "Unexpected response body: {}", body);

    wait_until!(
        {
            let block_option = set_provider.get_block_by_number(BlockNumberOrTag::Latest).await?;
            block_option.is_some() && block_option.unwrap().header.number == 2
        },
        Duration::from_secs(10)
    );

    let block_option = set_provider.get_block_by_number(BlockNumberOrTag::Latest).full().await?;
    assert!(block_option.is_some());

    let block = block_option.unwrap();
    assert_eq!(block.header.number, 2);
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
