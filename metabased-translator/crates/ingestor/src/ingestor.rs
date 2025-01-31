//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a
//! consumer using a channel

use crate::{config::ChainIngestorConfig, eth_client::EthClient, metrics::IngestorMetrics};
use common::types::{BlockAndReceipts, Chain};
use eyre::{eyre, Error};
use std::time::Duration;
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    oneshot,
};
use tracing::{debug, error};

/// Polls and ingests blocks from an Ethereum chain
#[derive(Debug)]
pub struct Ingestor {
    chain: Chain,
    client: EthClient,
    current_block_number: u64,
    sender: Sender<BlockAndReceipts>,
    polling_interval: Duration,
    metrics: IngestorMetrics,
}

impl Ingestor {
    /// Creates a new `Ingestor` instance responsible for polling blocks.
    ///
    /// # Arguments
    /// - `chain`: Specifies whether the ingestor is targeting the `Settlement` or `Sequencing`
    ///   chain.
    /// - `config`: Configuration parameters, including the RPC endpoint URL and starting block
    ///   number.
    /// - `metrics`: Metrics collection for monitoring ingestion performance.
    ///
    /// # Returns
    /// A tuple containing the `Ingestor` instance and a `Receiver` for consuming blocks.
    pub async fn new(
        chain: Chain,
        config: ChainIngestorConfig,
        metrics: IngestorMetrics,
    ) -> Result<(Self, Receiver<BlockAndReceipts>), Error> {
        let client = EthClient::new(&config.rpc_url).await?;
        let (sender, receiver) = channel(config.buffer_size);
        Ok((
            Self {
                chain,
                client,
                current_block_number: config.start_block,
                sender,
                polling_interval: config.polling_interval(),
                metrics,
            },
            receiver,
        ))
    }

    /// Retrieves a block by its number.
    ///
    /// # Arguments
    /// - `block_number`: The block number to retrieve.
    ///
    /// # Returns
    /// The block and its receipts.
    async fn get_block_and_receipts(&self, block_number: u64) -> Result<BlockAndReceipts, Error> {
        let start_time_block = std::time::Instant::now();
        let block = self.client.get_block_by_number(block_number).await?;
        let duration_block = start_time_block.elapsed();

        let start_time_receipts = std::time::Instant::now();
        let receipts = self.client.get_block_receipts(block_number).await?;
        let duration_receipts = start_time_receipts.elapsed();

        self.metrics.record_rpc_call(self.chain, "eth_getBlockByNumber", duration_block);

        self.metrics.record_rpc_call(self.chain, "eth_getBlockReceipts", duration_receipts);

        debug!("Got block: {:?}", block.number);

        Ok(BlockAndReceipts { block, receipts })
    }

    /// Sends the retrieved block to the consumer and updates the current block number.
    ///
    /// # Arguments
    /// - `block_and_receipts`: The block and its receipts to be sent to the consumer.
    async fn push_block_and_receipts(
        &mut self,
        block_and_receipts: BlockAndReceipts,
    ) -> Result<(), Error> {
        if block_and_receipts.block.number != self.current_block_number {
            return Err(eyre!("Block number mismatch"));
        }
        self.sender.send(block_and_receipts.clone()).await?;
        self.current_block_number += 1;
        self.metrics.record_last_block_fetched(self.chain, self.current_block_number);
        Ok(())
    }

    /// Starts the polling process.
    ///
    /// This asynchronous function continuously polls for new blocks and their receipts
    /// at a specified interval (`self.polling_interval`). For each block, it fetches the
    /// block and receipt data using `get_block_and_receipts` and sends the result to the
    /// consumer via the provided channel.
    ///
    /// The polling process runs in an infinite loop, but it is designed to handle two
    /// key scenarios:
    /// 1. **Interval Tick**: On each interval tick, the function fetches the next block and
    ///    receipts, logs relevant details, and pushes the data to the consumer. If fetching or
    ///    pushing fails, the function retries automatically.
    /// 2. **Cancellation Signal**: The function listens for cancellation signals (e.g., task
    ///    abortion or a `ctrl_c` event). When such a signal is received, the polling process
    ///    gracefully stops.
    /// # Errors
    /// This function returns an `Error` if initialization or any critical operation
    /// fails. Errors during polling (e.g., fetching blocks or pushing data) are logged
    /// and retried within the loop.
    pub async fn start_polling(
        mut self,
        mut shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), Error> {
        debug!("Starting polling");

        let mut interval = tokio::time::interval(self.polling_interval);

        loop {
            tokio::select! {
                _ = &mut shutdown_rx => {
                    debug!("Received shutdown signal, stopping polling");
                    drop(self.sender);
                    break;
                }
                _ = interval.tick() => {
                    match self.get_block_and_receipts(self.current_block_number).await {
                        Ok(block_and_receipts) => {
                            debug!("Pushing block: {:?}", block_and_receipts.block.number);
                            if let Err(err) = self.push_block_and_receipts(block_and_receipts).await {
                                error!("Failed to push block and receipts: {:?}, retrying...", err);
                            }
                        }
                        Err(err) => {
                            error!("Failed to fetch block and receipts: {:?}, retrying...", err);
                        }
                    }
                }
            }
        }

        debug!("Polling stopped");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{ChainIngestorConfig, IngestionPipelineConfig},
        metrics::IngestorMetrics,
    };
    use alloy::primitives::B256;
    use common::types::{Block, BlockAndReceipts};
    use eyre::Result;
    use prometheus_client::registry::Registry;
    use std::str::FromStr;

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }

    fn test_config() -> IngestionPipelineConfig {
        IngestionPipelineConfig {
            sequencing: ChainIngestorConfig {
                buffer_size: 100,
                polling_interval_secs: 1,
                rpc_url: "https://sequencing.io".into(),
                start_block: 19486923,
            }
            .into(),
            settlement: ChainIngestorConfig {
                buffer_size: 100,
                polling_interval_secs: 1,
                rpc_url: "https://settlement.io".into(),
                start_block: 19486923,
            }
            .into(),
        }
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

    #[tokio::test]
    async fn test_ingestor_new() -> Result<(), Error> {
        let start_block = 19486923;
        let buffer_size = 100;
        let polling_interval = Duration::from_secs(1);
        let config = test_config();

        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);

        let (ingestor, receiver) =
            Ingestor::new(Chain::Sequencing, config.sequencing.into(), metrics).await?;

        assert_eq!(ingestor.current_block_number, start_block);
        assert_eq!(receiver.capacity(), buffer_size);
        assert_eq!(ingestor.polling_interval, polling_interval);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_block_and_receipts() -> Result<(), Error> {
        let start_block = 19486923;
        let polling_interval = Duration::from_secs(1);

        let (sender, mut receiver) = channel(10);
        let client = EthClient::new(test_config().sequencing.sequencing_rpc_url.as_str()).await?;
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);
        let mut ingestor = Ingestor {
            chain: Chain::Sequencing,
            client,
            current_block_number: start_block,
            sender,
            polling_interval,
            metrics,
        };

        let block = get_dummy_block_and_receipts(start_block);

        ingestor.push_block_and_receipts(block).await.expect("Failed to poll block");

        if let Some(BlockAndReceipts { block, .. }) = receiver.recv().await {
            assert_eq!(block.number, start_block);
        } else {
            panic!("No block received");
        }
        Ok(())
    }
    #[tokio::test]
    async fn test_start_polling_simple() -> Result<(), Error> {
        let start_block = 1;
        let polling_interval = Duration::from_millis(10);

        let (sender, _) = channel(10);
        let client = EthClient::new(test_config().sequencing.sequencing_rpc_url.as_str()).await?;
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);
        let ingestor = Ingestor {
            chain: Chain::Sequencing,
            client,
            current_block_number: start_block,
            sender,
            polling_interval,
            metrics,
        };

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let polling_handle = tokio::spawn(async move {
            let result = ingestor.start_polling(shutdown_rx).await;
            assert!(result.is_ok(), "Polling task failed: {:?}", result);
        });

        tokio::time::sleep(Duration::from_millis(50)).await;

        let _ = shutdown_tx.send(());
        polling_handle.await?;

        Ok(())
    }
}
