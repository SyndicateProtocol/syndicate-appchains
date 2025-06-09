//! e2e tests for the `synd-appchains` stack
use alloy::{
    consensus::TxEnvelope,
    eips::Encodable2718,
    network::{Ethereum, EthereumWallet, TransactionBuilder, TransactionBuilderError},
    primitives::{address, utils::parse_ether, Address, U256},
    providers::{ext::AnvilApi, Provider, WalletProvider},
    rpc::types::TransactionRequest,
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
};
use contract_bindings::synd::rollup::{Rollup, Rollup::RollupInstance};
use serde_json::json;
use shared::types::FilledProvider;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use synd_maestro::valkey::models::waiting_transaction::WaitingGapTxnExt;
use test_framework::components::{
    configuration::ConfigurationOptions, test_components::TestComponents,
};
use test_utils::wait_until;

// an arbitrary eoa address used for testing
const TEST_ADDR: Address = address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380");

#[ctor::ctor]
fn init() {
    shared::tracing::setup_global_logging();
}

// Happy path - 1 txn submitted, 1 txn received. Identical to
// `e2e_maestro_batch_sequencer_translator()`
#[tokio::test]
async fn e2e_maestro_happy_path() -> Result<(), eyre::Error> {
    TestComponents::run(
        &ConfigurationOptions { use_write_loop: true, ..Default::default() },
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

// Duplicate txn spam is rejected
#[tokio::test]
async fn e2e_maestro_duplicate_rejected() -> Result<(), eyre::Error> {
    TestComponents::run(
        &ConfigurationOptions { use_write_loop: true, ..Default::default() },
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

            let tx2 = tx.clone();

            let tx_hash = components.send_maestro_tx_successful(&tx.encoded_2718()).await?;

            let tx_hash2 = components.send_maestro_tx_could_be_unsuccessful(&tx2).await?;

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

// TODO (SEQ-917): Fix flaky test
// Duplicate txn spam is rejected, dispatched concurrently
#[tokio::test]
async fn e2e_maestro_spam_rejected() -> Result<(), eyre::Error> {
    TestComponents::run(
        &ConfigurationOptions { use_write_loop: true, ..Default::default() },
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

            let chain_id = components.appchain_provider.get_chain_id().await?;
            let nonce = components.appchain_provider.get_transaction_count(wallet_address).await?;
            let tx1 = create_txn(
                chain_id,
                nonce,
                wallet_address,
                components.sequencing_provider.wallet(),
            )
            .await?;

            let mut funded_addresses = Vec::new();
            let mut unique_wallet_txns = Vec::new();
            funded_addresses.push(wallet_address);
            unique_wallet_txns.push(tx1.clone());

            for _ in 1..10 {
                let (funded_wallet_signer, funded_wallet_address) =
                    create_and_fund_wallet(&components, wallet_address, value, &inbox).await?;
                funded_addresses.push(funded_wallet_address);

                let funded_wallet = EthereumWallet::from(funded_wallet_signer);

                let tx = create_txn(chain_id, nonce, funded_wallet_address, &funded_wallet).await?;
                unique_wallet_txns.push(tx);
            }

            // IMP!: After all wallet funding, mine a set block with higher timestamp to indicate
            // sequencing slot is closed
            components.mine_set_block(1).await?;

            // Create 5x10 duplicate transactions (same nonce)
            let mut duplicate_txs = Vec::new();
            for _ in 0..5 {
                for tx in &unique_wallet_txns {
                    duplicate_txs.push(tx.clone());
                }
            }
            let size_duplicate_txns = duplicate_txs.len();

            // Use Arc to share components across tasks
            use std::sync::Arc;
            let components_arc = Arc::new(components);

            // Spawn tasks to send all transactions concurrently
            let mut handles = Vec::new();

            let test_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

            for (index, tx) in duplicate_txs.into_iter().enumerate() {
                let components_clone = components_arc.clone();

                // Spawn a new task to send the transaction
                let handle = tokio::spawn(async move {
                    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                    // Using try_send method to capture the error responses as well
                    println!(
                        "TX {}: Starting submission at {} ms ({}ms from test start)",
                        index,
                        start_time.as_millis(),
                        start_time.as_millis() - test_start.as_millis()
                    );

                    // THEN
                    let json_resp =
                        components_clone.send_maestro_tx_could_be_unsuccessful(&tx).await.unwrap();

                    // Get time after receiving the response
                    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

                    let elapsed = end_time.as_millis() - start_time.as_millis();

                    println!(
                        "TX {}: Completed at {} ms (took {} ms). Success: {}",
                        index,
                        end_time.as_millis(),
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

            let test_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let test_duration = test_end - test_start;
            println!(
                "Q_Q_Q: Full test for {} wallets {} txns completed at {} us (took {} us)",
                funded_addresses.len(),
                size_duplicate_txns,
                test_end.as_micros(),
                test_duration.as_micros(),
            );
            println!(
                "Q_Q_Q: Full test for {} wallets {} txns completed at {} ms (took {} ms)",
                funded_addresses.len(),
                size_duplicate_txns,
                test_end.as_millis(),
                test_duration.as_millis(),
            );

            // Check results - we expect one success and rest failures
            let success_count = results.iter().filter(|(status, _)| *status == "success").count();

            // We expect only one transaction per wallet to succeed (the first one that gets
            // processed)
            assert_eq!(
                success_count,
                10,
                "Only {} transactions should succeed not {}",
                funded_addresses.len(),
                size_duplicate_txns
            );

            // Wait for the transactions to be processed
            for address in &funded_addresses {
                wait_until!(
                    components_arc.appchain_provider.get_transaction_count(*address).await? ==
                        nonce + 1,
                    Duration::from_secs(3)
                );
            }

            // Verify that only one transaction per wallet was processed
            for address in funded_addresses {
                let current_nonce =
                    components_arc.appchain_provider.get_transaction_count(address).await?;
                assert_eq!(
                    current_nonce,
                    nonce + 1,
                    "Only one transaction should have been processed for address {}",
                    address
                );
            }
            Ok(())
        },
    )
    .await
}

// Test that distinct wallet txns are processed in parallel. Anecdotal upper limit is between
// 100-150 simultaneous txns on Daniil's machine
#[tokio::test]
async fn e2e_maestro_concurrency() -> Result<(), eyre::Error> {
    TestComponents::run(
        &ConfigurationOptions { use_write_loop: true, ..Default::default() },
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

            let chain_id = components.appchain_provider.get_chain_id().await?;
            let nonce = components.appchain_provider.get_transaction_count(wallet_address).await?;
            let tx1 = create_txn(
                chain_id,
                nonce,
                wallet_address,
                components.sequencing_provider.wallet(),
            )
            .await?;

            let mut funded_addresses = Vec::new();
            let mut unique_wallet_txns = Vec::new();
            funded_addresses.push(wallet_address);
            unique_wallet_txns.push(tx1.clone());

            let deposited_value = parse_ether("0.1")?;
            for _ in 1..100 {
                let (funded_wallet_signer, funded_wallet_address) =
                    create_and_fund_wallet(&components, wallet_address, deposited_value, &inbox)
                        .await?;
                funded_addresses.push(funded_wallet_address);

                let funded_wallet = EthereumWallet::from(funded_wallet_signer);

                let tx = create_txn(chain_id, nonce, funded_wallet_address, &funded_wallet).await?;
                unique_wallet_txns.push(tx);
            }

            // IMP!: After all wallet funding, mine a set block with higher timestamp to indicate
            // sequencing slot is closed
            components.mine_set_block(1).await?;

            // Create 1x100 transactions, unique wallets (same nonce)
            let mut duplicate_txs = Vec::new();
            for _ in 0..1 {
                for tx in &unique_wallet_txns {
                    duplicate_txs.push(tx.clone());
                }
            }
            let size_duplicate_txns = duplicate_txs.len();

            // Use Arc to share components across tasks
            use std::sync::Arc;
            let components_arc = Arc::new(components);

            // Spawn tasks to send all transactions concurrently
            let mut handles = Vec::new();

            let test_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

            for tx in duplicate_txs {
                let components_clone = components_arc.clone();

                // Spawn a new task to send the transaction
                let handle = tokio::spawn(async move {
                    // let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                    // Using try_send method to capture the error responses as well
                    // println!(
                    //     "TX {}: Starting submission at {} ms ({}ms from test start)",
                    //     index,
                    //     start_time.as_millis(),
                    //     start_time.as_millis() - test_start.as_millis()
                    // );

                    // THEN
                    let json_resp =
                        components_clone.send_maestro_tx_could_be_unsuccessful(&tx).await;

                    // Server error
                    if json_resp.is_err() {
                        println!("hit a request error, we're at JSON-RPC server limit");
                        return Ok::<_, eyre::Error>(("error", json!("")));
                    }

                    // Get time after receiving the response
                    // let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                    //
                    // let elapsed = end_time.as_millis() - start_time.as_millis();

                    // println!(
                    //     "TX {}: Completed at {} ms (took {} ms). Success: {}",
                    //     index,
                    //     end_time.as_millis(),
                    //     elapsed,
                    //     json_resp.get("result").is_some()
                    // );

                    let json_resp = json_resp.unwrap();
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

            let test_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let test_duration = test_end - test_start;
            println!(
                "Q_Q_Q: Full test for {} wallets {} txns completed at {} us (took {} us)",
                funded_addresses.len(),
                size_duplicate_txns,
                test_end.as_micros(),
                test_duration.as_micros(),
            );
            println!(
                "Q_Q_Q: Full test for {} wallets {} txns completed at {} ms (took {} ms)",
                funded_addresses.len(),
                size_duplicate_txns,
                test_end.as_millis(),
                test_duration.as_millis(),
            );

            // Check results - we expect one success and rest failures
            let success_count = results.iter().filter(|(status, _)| *status == "success").count();

            // We expect only one transaction per wallet to succeed (the first one that gets
            // processed)
            assert_eq!(
                success_count,
                100,
                "Only {} transactions should succeed not {}",
                funded_addresses.len(),
                size_duplicate_txns
            );
            Ok(())
        },
    )
    .await
}

async fn create_txn(
    chain_id: u64,
    nonce: u64,
    funded_wallet_address: Address,
    funded_wallet: &EthereumWallet,
) -> Result<TxEnvelope, TransactionBuilderError<Ethereum>> {
    TransactionRequest::default()
        .from(funded_wallet_address)
        .with_to(TEST_ADDR)
        .with_value(U256::from(0))
        .with_nonce(nonce)
        .with_gas_limit(100_000)
        .with_chain_id(chain_id)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(&funded_wallet)
        .await
}

async fn create_and_fund_wallet(
    components: &TestComponents,
    funding_wallet_address: Address,
    value: U256,
    inbox: &RollupInstance<(), &FilledProvider>,
) -> Result<(LocalSigner<SigningKey>, Address), eyre::Error> {
    let wallet_signer = PrivateKeySigner::random();
    let wallet_address = wallet_signer.address();
    let _ = inbox.depositEth(funding_wallet_address, wallet_address, value).send().await?;
    components.mine_both(0).await?; // Closes second slot
    components.mine_both(1).await?; // Closes third slot

    // Wait for deposit to be processed for the second wallet
    wait_until!(
        components.appchain_provider.get_balance(wallet_address).await? > U256::from(0),
        Duration::from_secs(10)
    );
    Ok((wallet_signer, wallet_address))
}

// Txn with higher nonce is accepted but doesn't make it on chain
#[tokio::test]
async fn e2e_maestro_higher_nonce_accepted() -> Result<(), eyre::Error> {
    TestComponents::run(
        &ConfigurationOptions { use_write_loop: true, ..Default::default() },
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

            let tx_hash0 = components.send_maestro_tx_successful(&tx0.encoded_2718()).await?;

            let tx_hash2 = components.send_maestro_tx_successful(&tx2.encoded_2718()).await?;

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
        &ConfigurationOptions { use_write_loop: true, ..Default::default() },
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
            let tx_hash1 = components.send_maestro_tx_successful(&tx1.encoded_2718()).await?;
            let tx_hash2 = components.send_maestro_tx_successful(&tx2.encoded_2718()).await?;

            // Assert that nonce didn't increase
            tokio::time::sleep(Duration::from_millis(500)).await;
            assert_eq!(
                components.appchain_provider.get_transaction_count(wallet_address).await?,
                nonce
            );

            // Assert waiting txns in cache
            let valkey_client = redis::Client::open(components.valkey_url.as_str()).unwrap();
            let mut valkey_conn = valkey_client.get_multiplexed_async_connection().await.unwrap();
            let waiting_txn_2 =
                valkey_conn.get_waiting_txn(chain_id, wallet_address, nonce + 2).await?;
            assert!(waiting_txn_2.is_some());

            // Send txn to "unstick" cache
            let tx_hash0 = components.send_maestro_tx_successful(&tx0.encoded_2718()).await?;

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

            // Nonce was updated
            assert_eq!(
                components.appchain_provider.get_transaction_count(wallet_address).await?,
                nonce + 3
            );

            // Waiting txns removed from cache
            let waiting_txn_2 =
                valkey_conn.get_waiting_txn(chain_id, wallet_address, nonce + 2).await?;
            assert!(waiting_txn_2.is_none());

            Ok(())
        },
    )
    .await
}
