//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a
//! consumer using a channel

use crate::{
    config::ChainIngestorConfig,
    ingestor::{check_reorg, IngestorError},
    metrics::IngestorMetrics,
};
use alloy::{
    consensus::{Transaction, TxEnvelope},
    primitives::Address,
    providers::Provider,
    rlp::Decodable,
    rpc::types::{BlockNumberOrTag, Filter, Header},
};
use common::types::{BlockRef, Chain, PartialBlock, PartialLogWithTxdata};
use eyre::Error;
use shared::bounded_join_set::BoundedJoinSet;
use std::{collections::BTreeMap, sync::Arc};
use tokio::{
    select,
    sync::{
        mpsc::{self, Sender},
        oneshot, Mutex,
    },
};
use tracing::{error, info, trace};

/// Runs a subscription-based ingestor that continuously monitors and processes blocks from an
/// Ethereum chain.
///
/// This function implements a subscription-based block ingestion strategy that:
/// 1. Initially syncs historical blocks from a starting point
/// 2. Subscribes to new block headers
/// 3. Processes blocks in parallel while maintaining order
/// 4. Handles chain reorganizations
/// 5. Tracks various metrics for monitoring
///
/// The ingestor maintains block order by:
/// - Processing blocks sequentially
/// - Storing out-of-order blocks in a `BTreeMap`
/// - Sending blocks in order once gaps are filled
///
/// # Arguments
/// * `chain` - The chain to monitor (Sequencing or Settlement)
/// * `config` - Configuration parameters for the ingestor
/// * `addresses` - List of contract addresses to filter logs for
/// * `client` - Ethereum client provider for RPC calls
/// * `sender` - Channel sender for processed blocks
/// * `metrics` - Metrics collector for monitoring performance
/// * `shutdown_rx` - Receiver for shutdown signals
///
/// # Returns
/// * `Result<(), IngestorError>` - Ok on successful completion, Err on failure
///
/// # Errors
/// * Returns `IngestorError` if:
///   - Initial subscription fails
///   - Block processing fails
///   - Chain reorganization is detected
///   - Channel send fails
///
/// # Metrics
/// Tracks:
/// * RPC call counts and durations
/// * Channel capacity
/// * Last block fetched
/// * Block processing latency
pub async fn run_subscription(
    chain: Chain,
    config: &ChainIngestorConfig,
    addresses: Vec<Address>,
    client: Arc<dyn Provider>,
    sender: Sender<Arc<PartialBlock>>,
    metrics: IngestorMetrics,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), IngestorError> {
    info!(%chain, "Starting ingestor subscription");
    let base_filter = Filter::new().address(addresses);
    let start_block = config.start_block;

    let start_time = std::time::Instant::now();
    let mut new_heads_sub = client.subscribe_blocks().await?;
    metrics.record_rpc_call(chain, "subscribe_blocks", start_time.elapsed());
    info!(%chain, "Subscribed to new block headers");

    //limit parallelism to the maximum batch size
    let tasks = Arc::new(Mutex::new(BoundedJoinSet::new(config.syncing_batch_size as usize + 1))); // +1 for the task that spawns all the others
    let (result_sender, mut result_receiver) = mpsc::channel(config.syncing_batch_size as usize);

    // Update channel capacity metric
    metrics.update_channel_capacity(chain, result_receiver.capacity());

    let initial_head_header =
        match tokio::time::timeout(config.rpc_timeout, new_heads_sub.recv()).await {
            Ok(Ok(header)) => header,
            Ok(Err(e)) => {
                error!(%chain, error = %e, "Subscription error during initial head fetch");
                panic!("Subscription for chain {} error during initial head fetch: {}", chain, e);
            }
            Err(_) => {
                info!(%chain, "Subscription timed out, fetching latest block manually");
                let start_time = std::time::Instant::now();
                let latest_block = client.get_block_by_number(BlockNumberOrTag::Latest).await?;
                metrics.record_rpc_call(chain, "get_block_by_number", start_time.elapsed());
                #[allow(clippy::unwrap_used)]
                // this is safe, if the RPC has no "latest" block, it's okay to panic
                latest_block.unwrap().header
            }
        };

    info!(
        %chain,
        %start_block,
        head_block = %initial_head_header.number,
        "Initial chain head received"
    );

    let tasks_clone = tasks.clone();
    let metrics_clone = metrics.clone();

    tasks.lock().await.spawn(async move {
        // ingest all blocks from `config.start_block` to the latest head
        info!(%chain, %start_block, initial_head_header = %initial_head_header.number, "Syncing - ingesting blocks");
        for number in start_block..=initial_head_header.number {
            let client_clone = client.clone();
            let base_filter_clone = base_filter.clone();
            let result_sender_clone = result_sender.clone();
            let tasks_clone = tasks_clone.clone();

            trace!(%chain, block = %number, "Syncing - spawning task for block");
            let metrics_clone = metrics_clone.clone();
            tasks_clone.lock().await.spawn(async move {
                let block = client_clone
                    .get_block_by_number(BlockNumberOrTag::Number(number))
                    .await?
                    .unwrap_or_else(|| {
                        panic!("No block found for number: {} on chain {}", number, chain)
                    });
                process_block(
                    &*client_clone,
                    block.header,
                    &base_filter_clone,
                    &result_sender_clone,
                    chain,
                    metrics_clone,
                )
                .await
            });
        }

        info!(%chain, "Historical sync complete, now following new blocks");
        // start ingesting new heads
        loop {
            let client_clone = client.clone();
            let base_filter_clone = base_filter.clone();
            let result_sender_clone = result_sender.clone();

            match new_heads_sub.recv().await {
                Ok(header) => {
                    info!(%chain, head_block_number = %header.number, "New chain head received");
                    let metrics_clone = metrics_clone.clone();
                    tasks_clone.lock().await.spawn(async move {
                        process_block(
                            &*client_clone,
                            header,
                            &base_filter_clone,
                            &result_sender_clone,
                            chain,
                            metrics_clone,
                        )
                        .await
                    });
                }
                Err(e) => {
                    error!(%chain, error = %e, "Error receiving block");
                }
            }
        }
    });

    let mut blocks_collection: BTreeMap<u64, Arc<PartialBlock>> = BTreeMap::new();
    let mut latest_processed_block: Option<BlockRef> = None;

    trace!(%chain, "Starting ingestor loop");
    loop {
        select! {
            biased;
            _ = &mut shutdown_rx => {
                info!(%chain, "Ingestor stopped");
                tasks.lock().await.abort_all();
                return Ok(());
            }
            received_block = result_receiver.recv() => {
                // Update channel capacity metric
                metrics.update_channel_capacity(chain, result_receiver.capacity());

                match received_block {
                    Some(block) => {
                        trace!(%chain, block = %block.number, "Received block");
                        // Handle first block or next sequential block
                        let is_valid_next_block = latest_processed_block.as_ref().map_or_else(|| block.number == start_block, |prev| block.number == prev.number + 1);

                        if !is_valid_next_block {
                            // Store out-of-sequence block for later sending
                            trace!(%chain, block = %block.number, "Storing out-of-sequence block for later sending");
                            blocks_collection.insert(block.number, block);
                            continue;
                        }

                        // Process the valid block
                        trace!(%chain, block = %block.number, logs = %block.logs.len(), "Sending block");
                        latest_processed_block = Some(block.clone().into());
                        metrics.record_last_block_fetched(chain, block.number);
                        sender.send(block).await?;

                        // Process any subsequent blocks we have stored
                        while let Some(ref last_block) = latest_processed_block {
                            let next_block_number = last_block.number + 1;
                            let Some((_number, next_block)) = blocks_collection.remove_entry(&next_block_number) else {
                                break;
                            };
                            trace!(%chain, block = %next_block.number, logs = %next_block.logs.len(), "sending stored block");
                            check_reorg(chain,last_block, &next_block)?;
                            latest_processed_block = Some(next_block.clone().into());
                            metrics.record_last_block_fetched(chain, next_block.number);
                            sender.send(next_block).await?;
                        }
                    }
                    None => {
                        error!(%chain, "Channel closed");
                        return Ok(());
                    }
                }
            }
        }
        tasks.lock().await.join_next().await; // clear the task that delivered the processed block
    }
}

async fn process_block(
    client: &dyn Provider,
    header: Header,
    base_filter: &Filter,
    tx: &Sender<Arc<PartialBlock>>,
    chain: Chain,
    metrics: IngestorMetrics,
) -> Result<(), Error> {
    trace!(%chain, block = %header.number, "Processing block");
    let filter = base_filter
        .clone()
        .from_block(BlockNumberOrTag::Number(header.number))
        .to_block(header.number);

    trace!(%chain, block = %header.number, "Fetching logs for block");
    let start_time = std::time::Instant::now();
    let logs = client.get_logs(&filter).await?;
    metrics.record_rpc_call(chain, "get_logs", start_time.elapsed());

    let mut logs_with_tx = Vec::new();
    for log in logs {
        let Some(tx_hash) = log.transaction_hash else {
            panic!("No transaction hash found for log on chain {}", chain);
        };

        let start_time = std::time::Instant::now();
        let raw_tx_res = client.get_raw_transaction_by_hash(tx_hash).await?;
        metrics.record_rpc_call(chain, "get_raw_transaction_by_hash", start_time.elapsed());

        let Some(raw_tx) = raw_tx_res else {
            panic!("No transaction found for hash: {} on chain {}", tx_hash, chain);
        };
        let tx = TxEnvelope::decode(&mut raw_tx.as_ref())?;
        logs_with_tx.push(PartialLogWithTxdata::new(log, tx.input().to_owned()));
    }
    trace!(%chain, block = %header.number, logs = %logs_with_tx.len(), "Block processed");
    tx.send(Arc::new(PartialBlock::new(header, logs_with_tx))).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        consensus::{SignableTransaction, TxEip1559},
        eips::Encodable2718,
        primitives::{Address, Bytes, LogData, TxHash, B256},
        providers::ProviderCall,
        rpc::types::Log,
        signers::{local::PrivateKeySigner, Signer},
        transports::{RpcError, TransportErrorKind},
    };
    use tokio::sync::mpsc;

    // Simple mock implementation of Provider that we control manually
    struct MockProvider {
        logs_to_return: Vec<Log>,
        transactions: std::collections::HashMap<B256, Bytes>,
    }

    impl MockProvider {
        const fn new(logs: Vec<Log>, transactions: std::collections::HashMap<B256, Bytes>) -> Self {
            Self { logs_to_return: logs, transactions }
        }
    }

    #[::async_trait::async_trait]
    impl Provider for MockProvider {
        fn root(&self) -> &alloy::providers::RootProvider<alloy::network::Ethereum> {
            unimplemented!("root is not needed for these tests")
        }

        async fn get_logs(
            &self,
            _filter: &Filter,
        ) -> Result<Vec<Log>, RpcError<TransportErrorKind>> {
            Ok(self.logs_to_return.clone())
        }

        fn get_raw_transaction_by_hash(
            &self,
            hash: TxHash,
        ) -> ProviderCall<(TxHash,), Option<Bytes>> {
            let result = self.transactions.get(&hash).cloned();
            ProviderCall::BoxedFuture(Box::pin(async move { Ok(result) }))
        }
    }

    // Helper to create a test Header
    fn create_test_header(number: u64, hash: B256, parent_hash: B256, timestamp: u64) -> Header {
        // Convert it to a proper Header object
        Header {
            hash,
            inner: alloy::consensus::Header {
                parent_hash,
                number,
                timestamp,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    // Helper to create a proper transaction
    async fn create_test_transaction(input_data: Vec<u8>) -> Result<Bytes, Error> {
        let tx = TxEip1559 { input: input_data.into(), ..Default::default() };

        let signer = PrivateKeySigner::random();
        let signature = signer.sign_message(tx.signature_hash().as_ref()).await?;
        let signed_tx = tx.into_signed(signature);

        let mut buf = Vec::new();
        signed_tx.encode_2718(&mut buf);
        Ok(buf.into())
    }

    #[tokio::test]
    async fn test_process_block_empty_block() {
        // Setup mock provider
        let provider = MockProvider::new(Vec::new(), std::collections::HashMap::new());

        // Create test header
        let block_number = 100;
        let block_hash = B256::from_slice(&[1u8; 32]);
        let parent_hash = B256::from_slice(&[2u8; 32]);
        let header = create_test_header(block_number, block_hash, parent_hash, 12345);

        // Set up channel to receive the processed block
        let (tx, mut rx) = mpsc::channel(10);
        let chain = Chain::Sequencing;
        let metrics = IngestorMetrics::new_noop();

        // Process block
        let result =
            process_block(&provider, header.clone(), &Filter::new(), &tx, chain, metrics).await;

        // Assertions
        assert!(result.is_ok());

        // Check block received from channel
        let received_block = rx.try_recv().unwrap();
        assert_eq!(received_block.number, block_number);
        assert_eq!(received_block.hash, block_hash);
        assert_eq!(received_block.timestamp, 12345);
        assert!(received_block.logs.is_empty());
    }

    #[tokio::test]
    async fn test_process_block_with_logs() {
        // Create test data
        let block_number = 200;
        let block_hash = B256::from_slice(&[3u8; 32]);
        let parent_hash = B256::from_slice(&[4u8; 32]);
        let contract_address = Address::from_slice(&[0xA1; 20]);
        let topic = B256::from_slice(&[0xB1; 32]);
        let tx_hash1 = B256::from_slice(&[0xC1; 32]);
        let tx_hash2 = B256::from_slice(&[0xC2; 32]);

        // Create properly encoded transactions
        let tx_input1_raw = b"transaction data 1".to_vec();
        let tx_input2_raw = b"transaction data 2".to_vec();
        let tx_data1 = create_test_transaction(tx_input1_raw.clone()).await.unwrap();
        let tx_data2 = create_test_transaction(tx_input2_raw.clone()).await.unwrap();

        // Create header
        let header = create_test_header(block_number, block_hash, parent_hash, 54321);

        // Create mock logs
        let log1 = Log {
            inner: alloy::primitives::Log {
                address: contract_address,
                data: LogData::new(vec![topic], Bytes::from(b"log data 1".to_vec())).unwrap(),
            },
            transaction_hash: Some(tx_hash1),
            block_number: Some(block_number),
            block_hash: Some(block_hash),
            transaction_index: Some(0),
            log_index: Some(0),
            removed: false,
            ..Default::default()
        };
        let log2 = Log {
            inner: alloy::primitives::Log {
                address: contract_address,
                data: LogData::new(vec![topic], Bytes::from(b"log data 2".to_vec())).unwrap(),
            },
            transaction_hash: Some(tx_hash2),
            block_number: Some(block_number),
            block_hash: Some(block_hash),
            transaction_index: Some(0),
            log_index: Some(0),
            removed: false,
            ..Default::default()
        };

        // Create mock transactions
        let mut txs = std::collections::HashMap::new();
        txs.insert(tx_hash1, tx_data1);
        txs.insert(tx_hash2, tx_data2);

        // Setup mock provider with logs and transactions
        let provider = MockProvider::new(vec![log1, log2], txs);

        // Set up channel to receive the processed block
        let (tx, mut rx) = mpsc::channel(10);
        let chain = Chain::Sequencing;
        let metrics = IngestorMetrics::new_noop();

        // Process block
        let result =
            process_block(&provider, header.clone(), &Filter::new(), &tx, chain, metrics).await;

        println!("result: {:?}", result);

        // Assertions
        assert!(result.is_ok());

        // Check block received from channel
        let received_block = rx.try_recv().unwrap();
        assert_eq!(received_block.number, block_number);
        assert_eq!(received_block.hash, block_hash);
        assert_eq!(received_block.logs.len(), 2);

        // Check log data
        assert_eq!(received_block.logs[0].address, contract_address);
        assert_eq!(received_block.logs[0].data, Bytes::from(b"log data 1".to_vec()));
        assert_eq!(received_block.logs[0].tx_calldata, Bytes::from(tx_input1_raw));

        assert_eq!(received_block.logs[1].address, contract_address);
        assert_eq!(received_block.logs[1].data, Bytes::from(b"log data 2".to_vec()));
        assert_eq!(received_block.logs[1].tx_calldata, Bytes::from(tx_input2_raw));
    }
}
