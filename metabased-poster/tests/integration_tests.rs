//! Integration tests for the poster
mod common;
use alloy::{
    hex,
    primitives::Bytes,
    providers::{Provider, WalletProvider},
    rpc::types::TransactionRequest,
    sol,
};
use eyre::Result;
use metabased_poster::{config::Config, metrics::PosterMetrics, poster};
use prometheus_client::registry::Registry;
use std::{str::FromStr, time::Duration};
use url::Url;

sol! {
    contract AssertionPosterDummy {
        function postAssertion(bytes32 blockHash, bytes32 sendRoot) external;
    }
}

const DUMMY_BYTECODE: &str = "6080604052348015600f57600080fd5b5060bb8061001e6000396000f3fe6080604052600080fdfea264697066735822122058b1491ce7f48120ae711ce4f8f0d0378e07dce8f7770dcf1ce728101514093264736f6c63430008140033";

#[tokio::test]
async fn e2e_poster_test() -> Result<()> {
    let set_port = 8080;
    let app_port = 8081;

    let (_set_anvil, set_provider) = common::start_anvil(1, set_port).await?;
    let (_app_anvil, _app_provider) = common::start_anvil(10, app_port).await?;

    let bytecode = hex::decode(DUMMY_BYTECODE)?;
    let tx = TransactionRequest {
        from: Some(set_provider.default_signer_address()),
        input: Bytes::from(bytecode).into(),
        gas: Some(5_000_000u64),
        ..Default::default()
    };

    // Send the transaction and get the receipt
    let pending_tx = set_provider.send_transaction(tx).await?;
    let receipt = pending_tx.get_receipt().await?;
    let contract_address = receipt.contract_address.expect("No contract address");

    let config = &Config {
        settlement_rpc_url: Url::from_str(&format!("http://localhost:{}", set_port))?,
        appchain_rpc_url: Url::from_str(&format!("http://localhost:{}", app_port))?,
        assertion_poster_contract_address: contract_address,
        private_key: common::DEFAULT_PRIVATE_KEY_SIGNER.to_string(),
        polling_interval: Duration::from_secs(60),
        metrics_port: 9090,
    };

    let mut registry = Registry::default();
    let metrics = PosterMetrics::new(&mut registry);

    let _poster_handle = poster::run(config, metrics);

    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
