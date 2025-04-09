//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a
//! consumer using a channel

use crate::{
    config::ChainIngestorConfig,
    ingestor::{check_reorg, IngestorError},
    metrics::IngestorMetrics,
};
use alloy::{primitives::Address, rpc::types::BlockNumberOrTag};
use common::{
    eth_client::RPCClient,
    types::{Block, BlockRef, Chain, PartialBlock, PartialLogWithTxdata, Receipt},
};
use std::{
    cmp::{max, min},
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{mpsc::Sender, oneshot},
    task::JoinSet,
};
use tracing::{debug, error, info, trace, warn};

struct BatchContext<'a> {
    client: &'a Arc<dyn RPCClient>,
    sender: &'a Sender<Arc<PartialBlock>>,
    metrics: &'a IngestorMetrics,
    current_block_number: &'a mut u64,
    last_block_sent: &'a mut Option<BlockRef>,
    initial_chain_head: u64,
    syncing_batch_size: u64,
    chain: Chain,
    addresses: &'a Vec<Address>,
    shutdown_rx: &'a mut oneshot::Receiver<()>,
    backoff_initial_interval: Duration,
    backoff_scaling_factor: u64,
    max_backoff: Duration,
}

/// Starts a new ingestor task.
///
/// This asynchronous function continuously polls for new blocks and their receipts
/// at a specified interval.
///
/// The polling process runs in an infinite loop, but it is designed to handle two
/// key scenarios:
/// 1. **Interval Tick**: On each interval tick, the function fetches the next block and receipts,
///    logs relevant details, and pushes the data to the consumer. If fetching or pushing fails, the
///    function retries automatically.
/// 2. **Cancellation Signal**: The function listens for cancellation signals (e.g., task abortion
///    or a `ctrl_c` event). When such a signal is received, the polling process gracefully stops.
/// # Errors
/// This function returns an `Error` if initialization or any critical operation
/// fails. Errors during polling (e.g., fetching blocks or pushing data) are logged
/// and retried within the loop.
///
/// # Arguments
/// - `chain`: Specifies whether the ingestor is targeting the `Settlement` or `Sequencing` chain.
/// - `config`: Configuration parameters, including the RPC endpoint URL and starting block number.
/// - `client`: An asynchronous RPC client used for fetching block data.
/// - `sender`: A channel for sending blocks to the consumer.
/// - `metrics`: Metrics collection for monitoring ingestion performance.
/// - `shutdown_rx`: A channel for receiving shutdown signals.
///
/// # Returns
/// A `Result` indicating the success or failure of the ingestor execution.
pub async fn run_http(
    chain: Chain,
    config: &ChainIngestorConfig,
    addresses: Vec<Address>,
    client: Arc<dyn RPCClient>,
    sender: Sender<Arc<PartialBlock>>,
    metrics: IngestorMetrics,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), IngestorError> {
    let initial_chain_head = fetch_block_with_retry(
        &*client,
        BlockNumberOrTag::Latest,
        chain,
        config.backoff_initial_interval,
        config.backoff_scaling_factor,
        config.max_backoff,
    )
    .await?
    .number;
    let batch_size = config.max_parallel_requests;
    let polling_interval = config.polling_interval;
    let start_block = config.start_block;

    info!(%chain, "Starting polling");

    let mut interval = tokio::time::interval(polling_interval);
    let mut current_block_number = start_block;
    let mut last_block_sent = None;

    loop {
        tokio::select! {
            biased;
            _ = &mut shutdown_rx => {
                info!(%chain, "Ingestor stopped");
                return Ok(());
            }
            _ = interval.tick() => {
               // Skip missed ticks
                interval.reset();
                fetch_and_push_batch(BatchContext {
                    client: &client,
                    sender: &sender,
                    metrics: &metrics,
                    current_block_number: &mut current_block_number,
                    last_block_sent: &mut last_block_sent,
                    initial_chain_head,
                    syncing_batch_size: batch_size,
                    chain,
                    addresses:&addresses,
                    shutdown_rx: &mut shutdown_rx,
                    backoff_initial_interval: config.backoff_initial_interval,
                    backoff_scaling_factor: config.backoff_scaling_factor,
                    max_backoff: config.max_backoff,
                }).await?;
            }
        }
    }
}

async fn push_block_and_receipts(
    sender: &Sender<Arc<PartialBlock>>,
    metrics: &IngestorMetrics,
    current_block_number: &mut u64,
    last_block_sent: &mut Option<BlockRef>,
    block_and_receipts: Arc<PartialBlock>,
    chain: Chain,
) -> Result<(), IngestorError> {
    if block_and_receipts.number != *current_block_number {
        error!(
            %chain,
            current_block = %current_block_number,
            received_block = %block_and_receipts.number,
            "Block number mismatch"
        );
        return Err(IngestorError::BlockNumberMismatch {
            current: *current_block_number,
            received: block_and_receipts.number,
        });
    }
    trace!(%chain, block_number = %block_and_receipts.number, "Attempting to send block");
    match last_block_sent {
        Some(last_block_sent) => check_reorg(chain, last_block_sent, &block_and_receipts)?,
        None => *last_block_sent = Some(BlockRef::new(&block_and_receipts)),
    }
    *last_block_sent = Some(BlockRef::new(&block_and_receipts));
    *current_block_number += 1;
    sender.send(block_and_receipts).await?;
    trace!(%chain, old_block_number = %*current_block_number, new_block_number = %(*current_block_number + 1), "Successfully sent block, incrementing block number");
    metrics.update_channel_capacity(chain, sender.capacity());
    Ok(())
}

async fn fetch_and_push_batch(ctx: BatchContext<'_>) -> Result<(), IngestorError> {
    let start_block_num = *ctx.current_block_number;
    let upper_bound = min(
        max(ctx.initial_chain_head, start_block_num) + 1,
        start_block_num + ctx.syncing_batch_size,
    );
    let block_numbers: Vec<u64> = (start_block_num..upper_bound).collect();

    if block_numbers.is_empty() {
        warn!(%ctx.chain, current_block = %start_block_num, initial_chain_head = %ctx.initial_chain_head, syncing_batch_size = %ctx.syncing_batch_size, "Calculated empty block range, skipping fetch cycle.");
        return Ok(());
    }
    info!(%ctx.chain, range = ?block_numbers, "Calculated fetch range");

    let mut tasks: JoinSet<Result<(u64, PartialBlock), IngestorError>> = JoinSet::new();

    // Spawn tasks for each block number
    for &block_number in &block_numbers {
        trace!(%ctx.chain, block_number, "Spawning fetch task");
        let client = ctx.client.clone();
        let addresses = ctx.addresses.clone();
        let backoff_initial_interval = ctx.backoff_initial_interval;
        let backoff_scaling_factor = ctx.backoff_scaling_factor;
        let max_backoff = ctx.max_backoff;
        let chain = ctx.chain;

        tasks.spawn(async move {
            // Fetch block and receipts
            let block = fetch_block_with_retry(
                &*client,
                BlockNumberOrTag::Number(block_number),
                chain,
                backoff_initial_interval,
                backoff_scaling_factor,
                max_backoff,
            )
            .await?;
            let receipts = fetch_receipts_with_retry(
                &*client,
                block_number,
                chain,
                backoff_initial_interval,
                backoff_scaling_factor,
                max_backoff,
            )
            .await?;
            // Filter receipts that include logs for any of the addresses in ctx.addresses
            let filtered_logs: Vec<PartialLogWithTxdata> = receipts
                .into_iter()
                .flat_map(|receipt| {
                    // Keep the relevant logs and related tx calldata
                    let logs: Vec<PartialLogWithTxdata> = receipt
                        .logs
                        .into_iter()
                        .filter(|log| addresses.contains(&log.address))
                        .map(|log| PartialLogWithTxdata {
                            address: log.address,
                            topics: log.topics,
                            data: log.data,
                            tx_calldata: block.transactions[log.transaction_index as usize]
                                .input
                                .clone(),
                        })
                        .collect();
                    logs
                })
                .collect();

            // Return the block and receipts
            Ok((
                block_number,
                PartialBlock {
                    number: block.number,
                    hash: block.hash,
                    timestamp: block.timestamp,
                    parent_hash: block.parent_hash,
                    logs: filtered_logs,
                },
            ))
        });
    }

    // Create a vector to collect results in the correct order
    let mut results = vec![None; block_numbers.len()];

    // Process results as they complete
    loop {
        let result = tokio::select! {
            biased;
            _ = &mut *ctx.shutdown_rx => {
                warn!(%ctx.chain, "Shutdown signal received during task joining, aborting all fetch tasks.");
                tasks.abort_all();
                return Ok(());
            }
            result = tasks.join_next() => result
        };

        let Some(result) = result else {
            trace!(%ctx.chain, "No more tasks in JoinSet.");
            break;
        };

        match result {
            Ok(Ok((block_num, block_and_receipts))) => {
                trace!(%ctx.chain, block_num, "Task completed successfully");
                let index = (block_num - block_numbers[0]) as usize;
                results[index] = Some(block_and_receipts);
            }
            Ok(Err(err)) => {
                if !matches!(err, IngestorError::ResourceNotAvailable { .. }) {
                    warn!(%ctx.chain, %err, "Failed to fetch block and receipts");
                }
            }
            Err(err) => {
                error!(%ctx.chain, %err, "Task failed");
                panic!("unexpected task failure: {:?}", err);
            }
        }
    }

    let num_fetched = results.iter().filter(|r| r.is_some()).count();
    debug!(%ctx.chain, fetched = %num_fetched, total_requested = %block_numbers.len(), "Processing fetched results");

    // Process blocks in order, stopping at first gap
    for (i, block_and_receipts) in results.into_iter().enumerate() {
        let current_processing_block_num = block_numbers[i];
        match block_and_receipts {
            Some(block_and_receipts) => {
                trace!(%ctx.chain, block_number = %current_processing_block_num, "Processing result: Some, attempting push");
                push_block_and_receipts(
                    ctx.sender,
                    ctx.metrics,
                    ctx.current_block_number,
                    ctx.last_block_sent,
                    Arc::new(block_and_receipts),
                    ctx.chain,
                )
                .await?;
            }
            None => {
                debug!(%ctx.chain, block_number = %current_processing_block_num, "Processing result: None, stopping batch processing at first gap.");
                break;
            }
        }
    }
    Ok(())
}

/// Executes an operation with exponential backoff retry logic.
///
/// This utility function provides a resilient way to execute remote operations (like RPC calls)
/// with built-in retry behavior using an exponential backoff strategy.
///
/// # Features
///
/// - Exponential backoff: Retry intervals increase exponentially up to a configurable maximum
/// - Special handling for "not found" errors: Allows differentiating between temporary failures and
///   genuine "not found" conditions
///
/// # Arguments
///
/// * `operation` - A function that returns a future which performs the operation being retried
/// * `context` - A descriptive string that identifies the operation (used in logs and error
///   messages)
/// * `backoff_initial_interval` - The initial delay between retry attempts
/// * `backoff_scaling_factor` - The multiplier applied to the backoff time after each failure
/// * `max_backoff` - The maximum duration to wait between retry attempts
/// * `is_not_found_error` - A predicate function that identifies special "not found" error cases
///
/// # Type Parameters
///
/// * `T` - The return type of the operation
/// * `F` - The type of the function that returns the future
/// * `Fut` - The future returned by the function
/// * `P` - The type of the predicate function
///
/// # Returns
///
/// Returns a `Result<T, Error>` where:
/// - `Ok(T)` is returned when the operation succeeds
/// - `Err(Error)` is returned either immediately for "not found" errors
///
/// # Behavior
///
/// 1. Calls the provided operation
/// 2. If successful, returns the result
/// 3. If the error is identified as a "not found" error by the predicate, returns an error
///    immediately
/// 4. For other errors, waits for the current backoff duration, increases the backoff duration, and
///    retries
/// 5. Retries indefinitely until either success or a "not found" error
///
/// # Examples
///
/// ```no_compile
/// use common::eth_client::RPCClientError;
/// use std::time::Duration;
///
/// async fn example_usage(client: &dyn RPCClient) -> Result<Block, Error> {
///     let context = "fetch block #100 on Settlement chain";
///
///     fetch_with_retry(
///         || client.get_block_by_number(BlockNumberOrTag::Number(100)),
///         context.to_string(),
///         Duration::from_millis(100),
///         2,
///         Duration::from_secs(5),
///         |err| matches!(err, RPCClientError::BlockNotFound(_)),
///     )
///     .await
/// }
/// ```
async fn fetch_with_retry<T: Send, F, Fut, P>(
    operation: F,
    context: String,
    backoff_initial_interval: Duration,
    backoff_scaling_factor: u64,
    max_backoff: Duration,
    is_not_found_error: P,
    chain: Chain,
) -> Result<T, IngestorError>
where
    F: Fn() -> Fut + Send,
    Fut: std::future::Future<Output = Result<T, common::eth_client::RPCClientError>> + Send,
    P: Fn(&common::eth_client::RPCClientError) -> bool + Send,
{
    let mut retry_count = 0;
    let mut backoff = backoff_initial_interval;

    loop {
        trace!(%chain, %context, attempt = %(retry_count + 1), "Attempting operation");
        match operation().await {
            Ok(response) => {
                trace!(%chain, %context, "Operation successful");
                return Ok(response);
            }
            Err(err) => {
                if is_not_found_error(&err) {
                    debug!(%chain, %context, error = %err, "Resource not yet available");
                    return Err(IngestorError::ResourceNotAvailable { resource: context.clone() });
                }
                retry_count += 1;
                error!(
                    %chain,
                    %context, %retry_count, error = %err,
                    next_retry_in = ?backoff,
                    "Operation failed, retrying"
                );
                tokio::time::sleep(backoff).await;

                // Calculate next backoff duration with scaling factor
                let next_backoff = backoff.mul_f64(backoff_scaling_factor as f64);

                // Cap at max_backoff
                backoff = min(next_backoff, max_backoff);
            }
        }
    }
}

async fn fetch_block_with_retry(
    client: &dyn RPCClient,
    b: BlockNumberOrTag,
    chain: Chain,
    backoff_initial_interval: Duration,
    backoff_scaling_factor: u64,
    max_backoff: Duration,
) -> Result<Block, IngestorError> {
    let context = format!("fetch block #{}", b);

    fetch_with_retry(
        || client.get_block_by_number(b),
        context,
        backoff_initial_interval,
        backoff_scaling_factor,
        max_backoff,
        |err| matches!(err, common::eth_client::RPCClientError::BlockNotFound(_)),
        chain,
    )
    .await
}

async fn fetch_receipts_with_retry(
    client: &dyn RPCClient,
    block_number: u64,
    chain: Chain,
    backoff_initial_interval: Duration,
    backoff_scaling_factor: u64,
    max_backoff: Duration,
) -> Result<Vec<Receipt>, IngestorError> {
    let context = format!("fetch block receipts #{}", block_number);
    let block_number_hex = format!("0x{:x}", block_number);

    fetch_with_retry(
        || client.get_block_receipts(&block_number_hex),
        context,
        backoff_initial_interval,
        backoff_scaling_factor,
        max_backoff,
        |err| matches!(err, common::eth_client::RPCClientError::BlockReceiptsNotFound(_)),
        chain,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::ChainIngestorConfig, metrics::IngestorMetrics};
    use alloy::{primitives::B256, rpc::types::BlockNumberOrTag};
    use async_trait::async_trait;
    use common::{
        eth_client::RPCClientError,
        types::{Block, PartialBlock},
    };
    use eyre::Result;
    use mockall::{mock, predicate::*};
    use prometheus_client::registry::Registry;
    use std::str::FromStr;
    use tokio::sync::mpsc::channel;

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }

    fn get_dummy_block(number: u64) -> Block {
        Block {
            hash: B256::from_str(
                "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number,
            parent_hash: B256::from_str(
                // all blocks have same hash/parent hash to prevent reorg detection triggering
                "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            logs_bloom: "0xLog".to_string(),
            transactions_root: "0xTra".to_string(),
            state_root: "0xSta".to_string(),
            receipts_root: "0xRec".to_string(),
            timestamp: 1000000000,
            transactions: vec![],
        }
    }

    mock! {
        #[derive(Debug)]
        pub RPCClientMock {}

        #[async_trait]
        impl RPCClient for RPCClientMock {
            async fn get_block_by_number(&self, block_number: BlockNumberOrTag) -> Result<Block, RPCClientError>;
            async fn get_block_receipts(&self, block_number_hex: &str) -> Result<Vec<Receipt>, RPCClientError>;
        }
    }

    #[tokio::test]
    async fn test_push_block_and_receipts() -> Result<(), IngestorError> {
        let start_block = 19486923;
        let (sender, mut receiver) = channel(10);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);
        let mut current_block_number = start_block;
        let mut last_block_sent = None;

        let block = get_dummy_block(start_block);
        let arc_block = Arc::new(PartialBlock {
            number: block.number,
            hash: block.hash,
            timestamp: block.timestamp,
            parent_hash: block.parent_hash,
            logs: vec![],
        });

        push_block_and_receipts(
            &sender,
            &metrics,
            &mut current_block_number,
            &mut last_block_sent,
            arc_block,
            Chain::Sequencing,
        )
        .await
        .expect("Failed to poll block");

        if let Some(arc_block) = receiver.recv().await {
            assert_eq!(arc_block.number, start_block);
        } else {
            panic!("No block received");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_block_number_mismatch() {
        let start_block = 100;
        let wrong_block = 101;
        let (sender, _) = channel(10);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);
        let mut current_block_number = start_block;
        let mut last_block_sent = None;

        let block = get_dummy_block(wrong_block);
        let arc_block = Arc::new(PartialBlock {
            number: block.number,
            hash: block.hash,
            timestamp: block.timestamp,
            parent_hash: block.parent_hash,
            logs: vec![],
        });

        let err = push_block_and_receipts(
            &sender,
            &metrics,
            &mut current_block_number,
            &mut last_block_sent,
            arc_block,
            Chain::Sequencing,
        )
        .await
        .expect_err("Should fail with block number mismatch");

        matches!(err, IngestorError::BlockNumberMismatch { current, received } if current == start_block && received == wrong_block);
    }

    #[tokio::test]
    async fn test_start_polling_simple_and_shutdown() -> Result<(), IngestorError> {
        let start_block = 1;
        let polling_interval = Duration::from_millis(10);
        let config = ChainIngestorConfig {
            start_block,
            polling_interval,
            backoff_initial_interval: Duration::from_millis(50),
            backoff_scaling_factor: 2,
            max_backoff: Duration::from_secs(30),
            ..Default::default()
        };

        let (sender, _receiver) = channel(10);
        let mut mock_client = MockRPCClientMock::new();
        mock_client.expect_get_block_by_number().returning(move |num| match num {
            BlockNumberOrTag::Number(num) => Ok(get_dummy_block(num)),
            BlockNumberOrTag::Latest => Ok(get_dummy_block(start_block)),
            _ => Err(RPCClientError::BlockNotFound(num.to_string())),
        });
        mock_client.expect_get_block_receipts().returning(move |_| Ok(vec![]));
        let client: Arc<dyn RPCClient> = Arc::new(mock_client);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let polling_handle = tokio::spawn(async move {
            let result =
                run_http(Chain::Sequencing, &config, vec![], client, sender, metrics, shutdown_rx)
                    .await;
            assert!(result.is_ok(), "Polling task failed: {:?}", result);
        });

        tokio::time::sleep(Duration::from_millis(50)).await;

        let _ = shutdown_tx.send(());
        polling_handle.await.unwrap();

        Ok(())
    }

    #[tokio::test]
    async fn test_start_polling_batching() -> Result<(), IngestorError> {
        let start_block = 100;
        let syncing_batch_size = 5;
        let config = ChainIngestorConfig {
            start_block,
            max_parallel_requests: syncing_batch_size,
            polling_interval: Duration::from_millis(10),
            backoff_initial_interval: Duration::from_millis(50),
            backoff_scaling_factor: 2,
            max_backoff: Duration::from_secs(30),
            ..Default::default()
        };
        let chain_head = start_block + syncing_batch_size;

        let (sender, mut receiver) = channel(10);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);

        let mut mock_client = MockRPCClientMock::new();

        mock_client
            .expect_get_block_by_number()
            .times((syncing_batch_size as usize) + 1)
            .returning(move |num| match num {
                BlockNumberOrTag::Number(num) => Ok(get_dummy_block(num)),
                BlockNumberOrTag::Latest => Ok(get_dummy_block(chain_head)),
                _ => Err(RPCClientError::BlockNotFound(num.to_string())),
            });
        mock_client
            .expect_get_block_receipts()
            .times(syncing_batch_size as usize)
            .returning(move |_| Ok(vec![]));

        mock_client
            .expect_get_block_by_number()
            .with(eq(BlockNumberOrTag::Number(chain_head)))
            .returning(move |_| Err(RPCClientError::BlockNotFound(chain_head.to_string())));

        let client: Arc<dyn RPCClient> = Arc::new(mock_client);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let polling_handle = tokio::spawn(async move {
            let result =
                run_http(Chain::Sequencing, &config, vec![], client, sender, metrics, shutdown_rx)
                    .await;
            assert!(result.is_ok(), "Polling task failed: {:?}", result);
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut received_blocks = Vec::new();
        while let Ok(block) = receiver.try_recv() {
            received_blocks.push(block.number);
        }

        assert_eq!(received_blocks.len(), syncing_batch_size as usize);
        assert_eq!(
            received_blocks,
            (start_block..start_block + syncing_batch_size).collect::<Vec<_>>()
        );

        let _ = shutdown_tx.send(());
        polling_handle.await.unwrap();

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_with_retry_success_first_try() {
        let expected_value = 42u64;
        let operation = || async { Ok::<u64, RPCClientError>(expected_value) };

        let result = fetch_with_retry(
            operation,
            "test operation".to_string(),
            Duration::from_millis(10),
            2,
            Duration::from_millis(100),
            |_| false,
            Chain::Sequencing,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }

    #[tokio::test]
    async fn test_fetch_with_retry_not_found_error() {
        let operation = || async {
            Result::<u64, RPCClientError>::Err(RPCClientError::BlockNotFound(
                "test block".to_string(),
            ))
        };

        let err = fetch_with_retry(
            operation,
            "test operation".to_string(),
            Duration::from_millis(10),
            2,
            Duration::from_millis(100),
            |err| matches!(err, RPCClientError::BlockNotFound(_)),
            Chain::Sequencing,
        )
        .await
        .expect_err("Should fail with resource not available");

        matches!(err, IngestorError::ResourceNotAvailable { resource } if resource == "test operation");
    }

    #[tokio::test]
    async fn test_fetch_with_retry_success_after_retries() {
        // Counter to track how many times the operation is called
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counter_clone = counter.clone();

        // Operation that fails twice then succeeds
        let operation = move || {
            let count = counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async move {
                if count < 2 {
                    Result::<u64, RPCClientError>::Err(RPCClientError::RpcError(
                        alloy::transports::RpcError::Transport(
                            alloy::transports::TransportErrorKind::Custom(
                                eyre::eyre!("temporary failure").into(),
                            ),
                        ),
                    ))
                } else {
                    Ok(42u64)
                }
            }
        };

        let result = fetch_with_retry(
            operation,
            "test operation".to_string(),
            Duration::from_millis(10), // Small duration for test speed
            2,
            Duration::from_millis(100),
            |err| matches!(err, RPCClientError::BlockNotFound(_)),
            Chain::Sequencing,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }
}
