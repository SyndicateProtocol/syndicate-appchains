//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a
//! consumer using a channel

use crate::{config::ChainIngestorConfig, metrics::IngestorMetrics};
use alloy::rpc::types::BlockNumberOrTag;
use common::{
    eth_client::RPCClient,
    types::{Block, BlockAndReceipts, Chain, Receipt},
};
use eyre::{eyre, Error};
use std::{
    cmp::{max, min},
    collections::BTreeMap,
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{mpsc::Sender, oneshot},
    task::JoinSet,
};
use tracing::{debug, error, info, trace};

struct BatchContext<'a> {
    client: &'a Arc<dyn RPCClient>,
    sender: &'a Sender<Arc<BlockAndReceipts>>,
    metrics: &'a IngestorMetrics,
    current_block_number: &'a mut u64,
    initial_chain_head: u64,
    syncing_batch_size: u64,
    chain: Chain,
    shutdown_rx: &'a mut oneshot::Receiver<()>,
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
pub async fn run(
    chain: Chain,
    config: &ChainIngestorConfig,
    client: Arc<dyn RPCClient>,
    sender: Sender<Arc<BlockAndReceipts>>,
    metrics: IngestorMetrics,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    let initial_chain_head =
        fetch_block_with_retry(&*client, BlockNumberOrTag::Latest, chain).await?.number;
    // client.get_block_by_number().await?.number;
    let batch_size = config.syncing_batch_size;
    let polling_interval = config.polling_interval;
    let start_block = config.start_block;

    info!("Starting polling for {}", chain);

    let mut interval = tokio::time::interval(polling_interval);
    let mut current_block_number = start_block;

    loop {
        tokio::select! {
            biased;
            _ = &mut shutdown_rx => {
                info!("{} ingestor stopped", chain);
                return Ok(());
            }
            _ = interval.tick() => {
               // Skip missed ticks
                interval.reset();
                let should_terminate = fetch_and_push_batch(BatchContext {
                    client: &client,
                    sender: &sender,
                    metrics: &metrics,
                    current_block_number: &mut current_block_number,
                    initial_chain_head,
                    syncing_batch_size: batch_size,
                    chain,
                    shutdown_rx: &mut shutdown_rx,
                }).await;
                if should_terminate {
                    info!("{} ingestor stopped", chain);
                    return Ok(())
                }
            }
        }
    }
}

async fn push_block_and_receipts(
    sender: &Sender<Arc<BlockAndReceipts>>,
    metrics: &IngestorMetrics,
    current_block_number: &mut u64,
    block_and_receipts: Arc<BlockAndReceipts>,
    chain: Chain,
) -> Result<(), Error> {
    if block_and_receipts.block.number != *current_block_number {
        error!(
            "Block number mismatch on chain {:?}. Current block {:?}. Got {:?}",
            chain, current_block_number, block_and_receipts.block.number
        );
        return Err(eyre!("Block number mismatch"));
    }
    sender.send(block_and_receipts).await?;
    *current_block_number += 1;
    metrics.update_channel_capacity(chain, sender.capacity());
    Ok(())
}

async fn fetch_and_push_batch(ctx: BatchContext<'_>) -> bool {
    let block_numbers: Vec<u64> = (*ctx.current_block_number..
        min(
            max(ctx.initial_chain_head, *ctx.current_block_number) + 1,
            *ctx.current_block_number + ctx.syncing_batch_size,
        ))
        .collect();
    trace!("Fetching blocks {:?} on {:?}", block_numbers, ctx.chain);

    let mut tasks: JoinSet<Result<(u64, BlockAndReceipts), Error>> = JoinSet::new();

    // Spawn tasks for each block number
    for &block_number in &block_numbers {
        let client = ctx.client.clone();

        tasks.spawn(async move {
            // Fetch block and receipts
            let block =
                fetch_block_with_retry(&*client, BlockNumberOrTag::Number(block_number), ctx.chain)
                    .await?;
            let receipts = fetch_receipts_with_retry(&*client, block_number, ctx.chain).await?;

            // Return the block and receipts
            Ok((block_number, BlockAndReceipts { block, receipts }))
        });
    }

    // Create ordered map to collect results in correct sequence
    let mut results_map = BTreeMap::new();

    // Process results as they complete
    loop {
        let result = tokio::select! {
            biased;
            _ = &mut *ctx.shutdown_rx => {
                tasks.abort_all();
                return true;
            }
            result = tasks.join_next() => result
        };

        let Some(result) = result else { break };

        match result {
            Ok(Ok((block_num, block_and_receipts))) => {
                results_map.insert(block_num, block_and_receipts);
            }
            Ok(Err(err)) => {
                debug!("Failed to fetch block and receipts on {:?}: {:?}", ctx.chain, err);
            }
            Err(err) => {
                error!("Task failed: {:?}", err);
                panic!("unexpected task failure: {:?}", err);
            }
        }
    }

    // Process blocks in order
    for &block_number in &block_numbers {
        if let Some(block_and_receipts) = results_map.remove(&block_number) {
            if let Err(err) = push_block_and_receipts(
                ctx.sender,
                ctx.metrics,
                ctx.current_block_number,
                Arc::new(block_and_receipts),
                ctx.chain,
            )
            .await
            {
                error!("Failed to push block and receipts: {:?}", err);
                break;
            }
        } else {
            // no block found, stop iterating
            debug!("No block found for number: {:?}", block_number);
            break;
        }
    }
    false
}

// TODO make initial_backoff_ms configurable
// TODO make max_retries configurable (default 0)
// TODO try to re-use the exponential backoff logic duplicated in these 2 functions

const INITIAL_BACKOFF_MS: u64 = 50;

async fn fetch_block_with_retry(
    client: &dyn RPCClient,
    b: BlockNumberOrTag,
    chain: Chain,
) -> Result<Block, Error> {
    let mut retry_count = 0;
    let mut backoff_ms = INITIAL_BACKOFF_MS;
    let context = format!("fetched block #{}", b);

    loop {
        match client.get_block_by_number(b).await {
            Ok(response) => {
                trace!("Successfully {} on {:?} chain", context, chain);
                return Ok(response);
            }
            Err(err) => {
                match err {
                    common::eth_client::RPCClientError::BlockNotFound(_) => {
                        trace!("Block {} not yet available on {:?} chain", b, chain);
                        return Err(eyre!("Block {} not yet available", b));
                    }
                    _ => {
                        retry_count += 1;
                        error!(
                            "Failed to get block {} on {:?} chain: {} retry_count: {}",
                            b, chain, err, retry_count
                        );
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                        backoff_ms *= 2; // Exponential backoff
                    }
                }
            }
        }
    }
}

async fn fetch_receipts_with_retry(
    client: &dyn RPCClient,
    block_number: u64,
    chain: Chain,
) -> Result<Vec<Receipt>, Error> {
    let mut retry_count = 0;
    let mut backoff_ms = INITIAL_BACKOFF_MS;
    let context = format!("fetched block #{}", block_number);
    let block_number_hex = format!("0x{:x}", block_number);

    loop {
        match client.get_block_receipts(&block_number_hex).await {
            Ok(response) => {
                trace!("Successfully {} on {:?} chain", context, chain);
                return Ok(response);
            }
            Err(err) => {
                match err {
                    common::eth_client::RPCClientError::BlockReceiptsNotFound(_) => {
                        trace!("Block {} not yet available on {:?} chain", block_number, chain);
                        return Err(eyre!("Block {} not yet available", block_number));
                    }
                    _ => {
                        retry_count += 1;
                        error!(
                            "Failed to get receipts for block {} on {:?} chain: {} retry_count: {}",
                            block_number, chain, err, retry_count
                        );
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                        backoff_ms *= 2; // Exponential backoff
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::ChainIngestorConfig, metrics::IngestorMetrics};
    use alloy::{primitives::B256, rpc::types::BlockNumberOrTag};
    use async_trait::async_trait;
    use common::{
        eth_client::RPCClientError,
        types::{Block, BlockAndReceipts},
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

    fn get_dummy_block_and_receipts(number: u64) -> BlockAndReceipts {
        let block: Block = Block {
            hash: B256::from_str(
                "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number,
            parent_hash: B256::from_str(
                "0234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            logs_bloom: "0xLog".to_string(),
            transactions_root: "0xTra".to_string(),
            state_root: "0xSta".to_string(),
            receipts_root: "0xRec".to_string(),
            timestamp: 1000000000,
            transactions: vec![],
        };
        BlockAndReceipts { block, receipts: vec![] }
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
    async fn test_push_block_and_receipts() -> Result<(), Error> {
        let start_block = 19486923;
        let (sender, mut receiver) = channel(10);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);
        let mut current_block_number = start_block;

        let block = get_dummy_block_and_receipts(start_block);
        let arc_block = Arc::new(block);

        push_block_and_receipts(
            &sender,
            &metrics,
            &mut current_block_number,
            arc_block,
            Chain::Sequencing,
        )
        .await
        .expect("Failed to poll block");

        if let Some(arc_block) = receiver.recv().await {
            assert_eq!(arc_block.block.number, start_block);
        } else {
            panic!("No block received");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_start_polling_simple_and_shutdown() -> Result<(), Error> {
        let start_block = 1;
        let polling_interval = Duration::from_millis(10);
        let config = ChainIngestorConfig { start_block, polling_interval, ..Default::default() };

        let (sender, _) = channel(10);
        let mut mock_client = MockRPCClientMock::new();
        mock_client
            .expect_get_block_by_number()
            .returning(move |_| Ok(get_dummy_block_and_receipts(start_block).block));
        mock_client.expect_get_block_receipts().returning(move |_| Ok(vec![]));
        let client: Arc<dyn RPCClient> = Arc::new(mock_client);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let polling_handle = tokio::spawn(async move {
            let result =
                run(Chain::Sequencing, &config, client, sender, metrics, shutdown_rx).await;
            assert!(result.is_ok(), "Polling task failed: {:?}", result);
        });

        tokio::time::sleep(Duration::from_millis(50)).await;

        let _ = shutdown_tx.send(());
        polling_handle.await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_start_polling_batching() -> Result<(), Error> {
        let start_block = 100;
        let syncing_batch_size = 5;
        let config = ChainIngestorConfig {
            start_block,
            syncing_batch_size,
            polling_interval: Duration::from_millis(10),
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
                BlockNumberOrTag::Number(num) => Ok(get_dummy_block_and_receipts(num).block),
                BlockNumberOrTag::Latest => Ok(get_dummy_block_and_receipts(chain_head).block),
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
                run(Chain::Sequencing, &config, client, sender, metrics, shutdown_rx).await;
            assert!(result.is_ok(), "Polling task failed: {:?}", result);
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut received_blocks = Vec::new();
        while let Ok(block) = receiver.try_recv() {
            received_blocks.push(block.block.number);
        }

        assert_eq!(received_blocks.len(), syncing_batch_size as usize);
        assert_eq!(
            received_blocks,
            (start_block..start_block + syncing_batch_size).collect::<Vec<_>>()
        );

        let _ = shutdown_tx.send(());
        polling_handle.await?;

        Ok(())
    }
}
