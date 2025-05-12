//! e2e tests for the metabased stack
// mod components;

use alloy::{
    eips::Encodable2718,
    network::TransactionBuilder,
    primitives::{address, utils::parse_ether, Address, U256},
    providers::{ext::AnvilApi, Provider, WalletProvider},
    rpc::types::TransactionRequest,
};
use contract_bindings::arbitrum::rollup::Rollup;
use std::time::Duration;
use test_framework::components::{
    configuration::ConfigurationOptions, test_components::TestComponents,
};
use test_utils::wait_until;

// an arbitrary eoa address used for testing
const TEST_ADDR: Address = address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380");

#[ctor::ctor]
fn init() {
    shared::logger::set_global_default_subscriber();
}

// Happy path - 1 txn submitted, 1 txn received. Identical to
// `e2e_maestro_batch_sequencer_translator()`
#[tokio::test]
async fn e2e_maestro_happy_path() -> Result<(), eyre::Error> {
    TestComponents::run(
        &ConfigurationOptions { pre_loaded: None, use_write_loop: true, ..Default::default() },
        |components| async move {
            components.sequencing_provider.anvil_set_block_timestamp_interval(0).await?;
            components.sequencing_provider.anvil_set_auto_mine(true).await?;
            // Send a deposit to the appchain to make sure the from address has funds
            let wallet_address = components.sequencing_provider.default_signer_address();
            let value = parse_ether("0.01")?;
            let inbox = Rollup::new(components.inbox_address, &components.settlement_provider);
            let _ = inbox.depositEth(wallet_address, wallet_address, value).send().await?;
            components.mine_set_block(0).await?;
            components.mine_set_block(1).await?;

            // Wait for deposit to be processed
            wait_until!(
                components.appchain_provider.get_balance(wallet_address).await? > U256::from(0),
                Duration::from_secs(10)
            );

            let chain_id = components.appchain_provider.get_chain_id().await?;
            let nonce = components.appchain_provider.get_transaction_count(wallet_address).await?;
            let tx = TransactionRequest::default()
                .from(wallet_address)
                .with_to(TEST_ADDR)
                .with_value(U256::from(0))
                .with_nonce(nonce)
                .with_gas_limit(100_000)
                .with_chain_id(chain_id)
                .with_max_fee_per_gas(100000000)
                .with_max_priority_fee_per_gas(0)
                .build(components.sequencing_provider.wallet())
                .await?;

            let tx_hash = components.send_maestro_tx_successful(&tx.encoded_2718()).await?;

            wait_until!(
                components.appchain_provider.get_transaction_count(wallet_address).await? ==
                    nonce + 1,
                Duration::from_secs(5)
            );

            // Verify that the transaction was processed
            let receipt = components.appchain_provider.get_transaction_receipt(tx_hash).await?;
            assert!(receipt.is_some());
            assert!(receipt.clone().unwrap().status());
            assert_eq!(receipt.unwrap().from, wallet_address);
            Ok(())
        },
    )
    .await
}
