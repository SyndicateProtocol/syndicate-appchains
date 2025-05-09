//! e2e tests for the metabased stack
use crate::components::{ConfigurationOptions, TestComponents};
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, utils::parse_ether, Address, U256},
    providers::{ext::AnvilApi, fillers::WalletFiller, Provider, ProviderBuilder, WalletProvider},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use contract_bindings::arbitrum::rollup::Rollup;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use test_utils::wait_until;
use tracing::info;

mod components;

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

            let chain_id = components.chain_id;
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

            let tx_hash = components.send_maestro_tx_successful(&tx).await?;

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

// Duplicate txn spam is rejected
#[tokio::test]
async fn e2e_maestro_duplicate_rejected() -> Result<(), eyre::Error> {
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

            let chain_id = components.chain_id;
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

            let tx2 = tx.clone();

            let tx_hash = components.send_maestro_tx_successful(&tx).await?;

            // TODO - Commented because this errors
            // let tx_hash2 = components.send_maestro_tx(&tx2).await;
            let tx_hash2 = components.send_maestro_tx_should_be_unsuccessful(&tx2).await?;

            // 2nd txn fails
            assert!(tx_hash2.get("result").is_none());
            assert!(tx_hash2.get("error").is_some());
            assert_eq!(
                tx_hash2.get("error").unwrap().get("message").unwrap(),
                "transaction rejected: transaction nonce too low: expected 1, got 0"
            );

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

// Duplicate txn spam is rejected, dispatched concurrently
#[tokio::test]
async fn e2e_maestro_spam_rejected() -> Result<(), eyre::Error> {
    // Ok(())
    TestComponents::run(
        &ConfigurationOptions { pre_loaded: None, use_write_loop: true, ..Default::default() },
        |components| async move {
            components.sequencing_provider.anvil_set_block_timestamp_interval(0).await?;
            components.sequencing_provider.anvil_set_auto_mine(true).await?;
            // Send a deposit to the appchain to make sure the from address has funds
            let wallet_address = components.sequencing_provider.default_signer_address();
            let value = parse_ether("0.1")?;
            let inbox = Rollup::new(components.inbox_address, &components.settlement_provider);
            let _ = inbox.depositEth(wallet_address, wallet_address, value).send().await?;
            components.mine_both(0).await?;
            components.mine_both(1).await?; // Close slot

            // Wait for deposit to be processed
            wait_until!(
                components.appchain_provider.get_balance(wallet_address).await? > U256::from(0),
                Duration::from_secs(10)
            );

            // Create a second wallet
            let second_wallet = PrivateKeySigner::random();
            let second_wallet_address = second_wallet.address();
            let _ = inbox.depositEth(wallet_address, second_wallet_address, value).send().await?;
            components.mine_both(0).await?; // Closes second slot
            components.mine_both(1).await?; // Closes third slot

            // Wait for deposit to be processed for the second wallet
            wait_until!(
                components.appchain_provider.get_balance(second_wallet_address).await? >
                    U256::from(0),
                Duration::from_secs(10)
            );

            let chain_id = components.chain_id;
            let nonce = components.appchain_provider.get_transaction_count(wallet_address).await?;
            let tx1 = TransactionRequest::default()
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

            // How to get from second walletAddress to &EthereumWallet?
            // let wallet2 =
            // let provider = components.sequencing_provider.clone();
            // let provider =
            // ProviderBuilder::new().on_http(components.sequencing_rpc_url.parse().unwrap());
            // let wallet2 = WalletProvider::new(provider, second_wallet);
            let wallet2 = &EthereumWallet::from(second_wallet);

            let tx2 = TransactionRequest::default()
                .from(second_wallet_address)
                .with_to(TEST_ADDR)
                .with_value(U256::from(0))
                .with_nonce(nonce)
                .with_gas_limit(100_000)
                .with_chain_id(chain_id)
                .with_max_fee_per_gas(100000000)
                .with_max_priority_fee_per_gas(0)
                .build(wallet2)
                .await?;

            // Create 5x2 duplicate transactions (same nonce)
            let mut duplicate_txs = Vec::new();
            for _ in 0..5 {
                duplicate_txs.push(tx1.clone());
                duplicate_txs.push(tx2.clone());
            }

            // Use Arc to share components across tasks
            use std::sync::Arc;
            let components_arc = Arc::new(components);

            // Spawn tasks to send all transactions concurrently
            let mut handles = Vec::new();

            let test_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

            for (index, tx) in duplicate_txs.into_iter().enumerate() {
                let components_clone = Arc::clone(&components_arc);

                // Spawn a new task to send the transaction
                let handle = tokio::spawn(async move {
                    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                    // Using try_send method to capture the error responses as well
                    println!(
                        "TX {}: Starting submission at {} ms ({}ms from test start)",
                        index,
                        start_time.as_nanos(),
                        start_time.as_nanos() - test_start
                    );

                    // THEN
                    let json_resp =
                        components_clone.send_maestro_tx_should_be_unsuccessful(&tx).await.unwrap();

                    // Get time after receiving the response
                    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

                    let elapsed = end_time.as_nanos() - start_time.as_nanos();

                    println!(
                        "TX {}: Completed at {} ms (took {} ms). Success: {}",
                        index,
                        end_time.as_nanos(),
                        elapsed,
                        json_resp.get("result").is_some()
                    );

                    if json_resp.get("result").is_some() {
                        // Clone the value to avoid reference issues
                        Ok::<_, eyre::Error>(("success", json_resp))
                    } else {
                        // This is an error response (no "result" field)
                        // Clone the value to avoid reference issues
                        Ok::<_, eyre::Error>(("error", json_resp))
                    }
                });

                handles.push(handle);
            }

            // Collect all results
            let mut results = Vec::new();
            for handle in handles {
                // Wait for the task to complete and get the result
                match handle.await {
                    Ok(result) => results.push(result?),
                    Err(e) => println!("Task panicked: {:?}", e),
                }
            }

            // Check results - we expect one success and rest failures
            let success_count = results.iter().filter(|(status, _)| *status == "success").count();

            // TODO this needs to be 2 not 10
            // We expect only one transaction to succeed (the first one that gets processed)
            assert_eq!(success_count, 2, "Only two transactions should succeed");

            // Wait for the transaction to be processed
            wait_until!(
                components_arc.appchain_provider.get_transaction_count(wallet_address).await? ==
                    nonce + 1,
                Duration::from_secs(10)
            );

            // Verify that only one transaction was processed
            let current_nonce =
                components_arc.appchain_provider.get_transaction_count(wallet_address).await?;
            assert_eq!(current_nonce, nonce + 1, "Only one transaction should have been processed");

            let current_nonce_second_wallet = components_arc
                .appchain_provider
                .get_transaction_count(second_wallet_address)
                .await?;
            assert_eq!(
                current_nonce_second_wallet,
                nonce + 1,
                "Only one
            transaction should have been processed"
            );

            Ok(())
        },
    )
    .await
}

// Txn with higher nonce is accepted but doesn't make it onchain
#[tokio::test]
async fn e2e_maestro_higher_nonce_accepted() -> Result<(), eyre::Error> {
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

            let chain_id = components.chain_id;
            let nonce = components.appchain_provider.get_transaction_count(wallet_address).await?;
            let tx0 = TransactionRequest::default()
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

            let tx2 = TransactionRequest::default()
                .from(wallet_address)
                .with_to(TEST_ADDR)
                .with_value(U256::from(0))
                .with_nonce(nonce + 2) // NOTE: create a nonce gap
                .with_gas_limit(100_000)
                .with_chain_id(chain_id)
                .with_max_fee_per_gas(100000000)
                .with_max_priority_fee_per_gas(0)
                .build(components.sequencing_provider.wallet())
                .await?;

            let tx_hash0 = components.send_maestro_tx_successful(&tx0).await?;

            let tx_hash2 = components.send_maestro_tx_successful(&tx2).await?;

            wait_until!(
                components.appchain_provider.get_transaction_count(wallet_address).await? ==
                    nonce + 1,
                Duration::from_secs(5)
            );

            // Verify that the transaction was processed
            let receipt = components.appchain_provider.get_transaction_receipt(tx_hash0).await?;
            assert!(receipt.is_some());
            assert!(receipt.clone().unwrap().status());
            assert_eq!(receipt.unwrap().from, wallet_address);

            // Waiting txn still waiting
            let receipt2 = components.appchain_provider.get_transaction_receipt(tx_hash2).await?;
            assert!(receipt2.is_none());

            Ok(())
        },
    )
    .await
}

// Higher nonce txns submitted, then the nonce gap txn is submitted and all succeed
#[tokio::test]
async fn e2e_maestro_waiting_txns_get_unstuck() -> Result<(), eyre::Error> {
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

            let chain_id = components.chain_id;
            let nonce = components.appchain_provider.get_transaction_count(wallet_address).await?;
            let tx0 = TransactionRequest::default()
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

            let tx1 = TransactionRequest::default()
                .from(wallet_address)
                .with_to(TEST_ADDR)
                .with_value(U256::from(0))
                .with_nonce(nonce + 1)
                .with_gas_limit(100_000)
                .with_chain_id(chain_id)
                .with_max_fee_per_gas(100000000)
                .with_max_priority_fee_per_gas(0)
                .build(components.sequencing_provider.wallet())
                .await?;

            let tx2 = TransactionRequest::default()
                .from(wallet_address)
                .with_to(TEST_ADDR)
                .with_value(U256::from(0))
                .with_nonce(nonce + 2)
                .with_gas_limit(100_000)
                .with_chain_id(chain_id)
                .with_max_fee_per_gas(100000000)
                .with_max_priority_fee_per_gas(0)
                .build(components.sequencing_provider.wallet())
                .await?;

            // Send higher nonce txns first. These won't fail
            let tx_hash1 = components.send_maestro_tx_successful(&tx1).await?;
            let tx_hash2 = components.send_maestro_tx_successful(&tx2).await?;

            // Assert that nonce didn't increase
            tokio::time::sleep(Duration::from_millis(500)).await;
            assert_eq!(
                components.appchain_provider.get_transaction_count(wallet_address).await?,
                nonce
            );

            // Send txn to "unstick" cache
            let tx_hash0 = components.send_maestro_tx_successful(&tx0).await?;

            wait_until!(
                components.appchain_provider.get_transaction_count(wallet_address).await? ==
                    nonce + 3,
                Duration::from_secs(5)
            );

            // Verify that the transaction was processed
            let receipt0 = components.appchain_provider.get_transaction_receipt(tx_hash0).await?;
            assert!(receipt0.is_some());
            let receipt1 = components.appchain_provider.get_transaction_receipt(tx_hash1).await?;
            assert!(receipt1.is_some());
            let receipt2 = components.appchain_provider.get_transaction_receipt(tx_hash2).await?;
            assert!(receipt2.is_some());

            // Waiting txn still waiting
            assert_eq!(
                components.appchain_provider.get_transaction_count(wallet_address).await?,
                nonce + 3
            );

            Ok(())
        },
    )
    .await
}
