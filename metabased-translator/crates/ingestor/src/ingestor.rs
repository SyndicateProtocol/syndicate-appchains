//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a
//! consumer using a channel

use crate::{config::ChainIngestorConfig, eth_client::RPCClient, metrics::IngestorMetrics};
use alloy::rpc::types::BlockNumberOrTag;
use common::types::{BlockAndReceipts, Chain};
use eyre::{eyre, Error};
use std::{
    cmp::{max, min},
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        oneshot,
    },
    time::Instant,
};
use tracing::{debug, error};

/// Polls and ingests blocks from an Ethereum chain
#[derive(Debug)]
pub struct Ingestor {
    chain: Chain,
    client: Arc<dyn RPCClient>,
    current_block_number: u64,
    initial_chain_head: u64,
    syncing_batch_size: u64,
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
    /// - `client`: An asynchronous RPC client used for fetching block data.
    /// - `config`: Configuration parameters, including the RPC endpoint URL and starting block
    ///   number.
    /// - `metrics`: Metrics collection for monitoring ingestion performance.
    ///
    /// # Returns
    /// A tuple containing the `Ingestor` instance and a `Receiver` for consuming blocks.
    pub async fn new(
        chain: Chain,
        client: Arc<dyn RPCClient>,
        config: &ChainIngestorConfig,
        metrics: IngestorMetrics,
    ) -> Result<(Self, Receiver<BlockAndReceipts>), Error> {
        let (sender, receiver) = channel(config.buffer_size);
        let client_clone = client.clone();
        let chain_head = client_clone.get_block_by_number(BlockNumberOrTag::Latest).await?;
        Ok((
            Self {
                chain,
                client,
                current_block_number: config.start_block,
                initial_chain_head: chain_head.number,
                syncing_batch_size: config.syncing_batch_size,
                sender,
                polling_interval: config.polling_interval,
                metrics,
            },
            receiver,
        ))
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
        self.metrics.update_channel_capacity(self.chain, self.sender.capacity());
        Ok(())
    }

    /// Starts the polling process.
    ///
    /// This asynchronous function continuously polls for new blocks and their receipts
    /// at a specified interval (`self.polling_interval`).
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
                    drop(self.sender);
                    debug!("{} ingestor stopped", self.chain);
                    return Ok(());
                }
                _ = interval.tick() => {
                    self.fetch_and_push_batch().await;
                }
            }
        }
    }

    async fn fetch_and_push_batch(&mut self) {
        let block_numbers = (self.current_block_number..
            min(
                max(self.initial_chain_head, self.current_block_number) + 1,
                self.current_block_number + self.syncing_batch_size,
            ))
            .collect();

        let start_time = Instant::now();
        match self.client.batch_get_blocks_and_receipts(block_numbers).await {
            Ok(blocks) => {
                let duration = start_time.elapsed();
                self.metrics.record_rpc_call(
                    self.chain,
                    "batch(eth_getBlockByNumber + eth_getBlockReceipts)",
                    duration,
                );
                for block in blocks {
                    if let Err(err) = self.push_block_and_receipts(block).await {
                        error!("Failed to push block and receipts: {:?}, retrying...", err);
                    }
                }
            }
            Err(err) => {
                error!("Failed to fetch multiple blocks and receipts: {:?}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{ChainIngestorConfig, IngestionPipelineConfig},
        eth_client::{EthClient, RPCClientError},
        metrics::IngestorMetrics,
    };
    use alloy::{primitives::B256, rpc::types::BlockNumberOrTag};
    use async_trait::async_trait;
    use common::types::{Block, BlockAndReceipts};
    use eyre::Result;
    use mockall::{mock, predicate::*};
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
                polling_interval: Duration::from_secs(1),
                rpc_url: "https://sequencing.io".into(),
                start_block: 19486923,
                syncing_batch_size: 50,
            }
            .into(),
            settlement: ChainIngestorConfig {
                buffer_size: 100,
                polling_interval: Duration::from_secs(1),
                rpc_url: "https://settlement.io".into(),
                start_block: 19486923,
                syncing_batch_size: 50,
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

    mock! {
        #[derive(Debug)]
        pub RPCClientMock {}

        #[async_trait]
        impl RPCClient for RPCClientMock {
            async fn get_block_by_number(&self, block_number: BlockNumberOrTag) -> Result<Block, RPCClientError>;
            async fn batch_get_blocks_and_receipts(&self, block_numbers: Vec<u64>) -> Result<Vec<BlockAndReceipts>, RPCClientError>;
        }
    }

    #[tokio::test]
    async fn test_ingestor_new() -> Result<(), Error> {
        let start_block = 19486923;
        let chain_head = start_block + 10;
        let buffer_size = 100;
        let polling_interval = Duration::from_secs(1);
        let config = test_config();

        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);
        let config = config.sequencing.into();

        let mut mock = MockRPCClientMock::new();
        mock.expect_get_block_by_number()
            .with(eq(BlockNumberOrTag::Latest))
            .times(1)
            .returning(move |_| Ok(Block { number: chain_head, ..Default::default() }));

        let client: Arc<dyn RPCClient> = Arc::new(mock);
        let (ingestor, receiver) =
            Ingestor::new(Chain::Sequencing, client, &config, metrics).await?;

        assert_eq!(ingestor.current_block_number, start_block);
        assert_eq!(receiver.capacity(), buffer_size);
        assert_eq!(ingestor.polling_interval, polling_interval);
        assert_eq!(ingestor.initial_chain_head, chain_head);
        Ok(())
    }

    #[tokio::test]
    async fn test_push_block_and_receipts() -> Result<(), Error> {
        let start_block = 19486923;
        let polling_interval = Duration::from_secs(1);

        let (sender, mut receiver) = channel(10);
        let client: Arc<dyn RPCClient> =
            Arc::new(EthClient::new(&test_config().sequencing.sequencing_rpc_url).await?);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);
        let mut ingestor = Ingestor {
            chain: Chain::Sequencing,
            client,
            current_block_number: start_block,
            initial_chain_head: start_block + 100,
            syncing_batch_size: 50,
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
        let client: Arc<dyn RPCClient> =
            Arc::new(EthClient::new(&test_config().sequencing.sequencing_rpc_url).await?);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);
        let ingestor = Ingestor {
            chain: Chain::Sequencing,
            client,
            current_block_number: start_block,
            initial_chain_head: start_block + 100,
            syncing_batch_size: 50,
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

    #[tokio::test]
    async fn test_start_polling_batching() -> Result<(), Error> {
        let start_block = 100;
        let syncing_batch_size = 5;
        let chain_head = start_block + 5;
        let polling_interval = Duration::from_millis(10);

        let (sender, mut receiver) = channel(10);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = IngestorMetrics::new(&mut metrics_state.registry);

        let mut mock_client = MockRPCClientMock::new();

        mock_client
            .expect_batch_get_blocks_and_receipts()
            .withf(move |block_numbers| block_numbers.len() == syncing_batch_size as usize)
            .times(1)
            .returning(move |block_numbers| {
                Ok(block_numbers.iter().map(|&num| get_dummy_block_and_receipts(num)).collect())
            });

        mock_client
            .expect_batch_get_blocks_and_receipts()
            .with(eq(vec![chain_head]))
            .returning(move |_| Err(RPCClientError::BlockNotFound(chain_head.to_string())));

        let client: Arc<dyn RPCClient> = Arc::new(mock_client);
        let ingestor = Ingestor {
            chain: Chain::Sequencing,
            client,
            current_block_number: start_block,
            initial_chain_head: chain_head,
            syncing_batch_size,
            sender,
            polling_interval,
            metrics,
        };

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let polling_handle = tokio::spawn(async move {
            let result = ingestor.start_polling(shutdown_rx).await;
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
