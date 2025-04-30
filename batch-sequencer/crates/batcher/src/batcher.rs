//! The Batcher service for the sequencer.

use crate::config::BatcherConfig;
use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use byte_unit::Byte;
use contract_bindings::metabased::walletpoolwrappermodule::WalletPoolWrapperModule::{
    self, WalletPoolWrapperModuleInstance,
};
use eyre::{eyre, Result};
use maestro::redis::consumer::StreamConsumer;
use redis::Client as RedisClient;
use shared::{additive_compression::AdditiveCompressor, types::FilledProvider};
use std::{mem::take, str::FromStr, time::Duration};
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

/// Batcher service
#[derive(Debug)]
struct Batcher {
    /// The max batch size for the batcher
    max_batch_size: Byte,
    /// The Redis consumer for the batcher
    redis_consumer: StreamConsumer,
    /// The compressor for the batcher
    compressor: AdditiveCompressor,
    /// The polling interval for the batcher
    polling_interval: Duration,
    /// The chain ID for the batcher
    chain_id: u64,
    /// Failed txns
    failed_txns: Vec<Bytes>,
    /// Track if submission is in-flight
    submission_in_flight: bool,
    /// Wallet pool
    wallet_pool: WalletPoolWrapperModuleInstance<(), FilledProvider>,
    /// The sequencing address
    sequencing_contract_address: Address,
}

#[derive(Debug, thiserror::Error)]
enum BatchError {
    #[error("Compressed batch too large: {0} bytes (limit: {1} bytes)")]
    BatchTooLarge(usize, usize),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] eyre::Report),

    #[error("Failed to send batch: {0}")]
    SendBatchFailed(String),
}

/// Run the batcher service. Starts the server and listens for batch requests.
pub async fn run_batcher(config: &BatcherConfig) -> Result<JoinHandle<()>> {
    let client = RedisClient::open(config.redis_url.as_str())
        .map_err(|e| eyre!("Failed to open Redis client: {}", e))?;
    let conn = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| eyre!("Failed to get Redis connection: {}", e))?;
    let redis_consumer = StreamConsumer::new(conn, config.chain_id, "0-0".to_string());

    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let sequencer_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .on_http(config.sequencing_rpc_url.clone());

    let wallet_pool = WalletPoolWrapperModule::new(config.wallet_pool_address, sequencer_provider);

    let mut batcher = Batcher::new(config, redis_consumer, wallet_pool);
    let polling_interval = config.polling_interval;

    let handle = tokio::spawn({
        async move {
            loop {
                if let Err(e) = batcher.read_and_batch_transactions().await {
                    error!("Batcher error: {:?}", e);
                }
                // In theory this could be removed, but we want to wait a reasonable amount of time
                // to batch as many transactions as possible.
                tokio::time::sleep(polling_interval).await;
            }
        }
    });
    info!("Batcher job started with {:?} poll interval", config.polling_interval);
    Ok(handle)
}

impl Batcher {
    fn new(
        config: &BatcherConfig,
        redis_consumer: StreamConsumer,
        wallet_pool: WalletPoolWrapperModuleInstance<(), FilledProvider>,
    ) -> Self {
        Self {
            max_batch_size: config.max_batch_size,
            redis_consumer,
            compressor: AdditiveCompressor::default(),
            polling_interval: config.polling_interval,
            chain_id: config.chain_id,
            failed_txns: Vec::new(),
            submission_in_flight: false,
            wallet_pool,
            sequencing_contract_address: config.sequencing_contract_address,
        }
    }
    async fn read_transactions(&mut self) -> Result<()> {
        loop {
            // TODO (SEQ-842): Configurable max msg count
            let transactions = self.redis_consumer.recv(1, self.polling_interval).await?;

            if transactions.is_empty() {
                debug!("No transactions available to batch.");
                break;
            }

            for (txn_bytes, _) in transactions {
                let txn = Bytes::from(txn_bytes);

                let compressed = self.compressor.clone_and_compress_with_txn(&txn)?;

                if compressed.len() as u64 >= self.max_batch_size.as_u64() {
                    // Stop consuming and let the caller finalize the batch
                    return Ok(());
                }
                debug!(
                    %self.chain_id, "Adding transaction to batch: {:?} - compressed size: {}",
                    txn,
                    compressed.len()
                );
                self.compressor.push_with_precompressed(&txn, compressed)?;
            }
        }
        Ok(())
    }

    async fn read_and_batch_transactions(&mut self) -> Result<(), BatchError> {
        if self.submission_in_flight {
            debug!(%self.chain_id, "Submission in flight, skipping read.");
            return Ok(());
        }
        self.submission_in_flight = true;

        // Push failed txns to the compressor
        for txn in take(&mut self.failed_txns) {
            let compressed = self.compressor.clone_and_compress_with_txn(&txn)?;
            self.compressor.push_with_precompressed(&txn, compressed)?;
        }

        self.read_transactions().await?;

        if self.compressor.is_empty() {
            debug!(%self.chain_id, "No transactions available to batch.");
            return Ok(());
        }

        if let Err(e) = self.send_compressed_batch().await {
            error!(%self.chain_id, "Failed to send compressed batch: {:?}", e);
            self.failed_txns = self.compressor.drain_transactions(); // Save failed txns for next round
            self.submission_in_flight = false;
            return Err(e);
        }

        self.submission_in_flight = false;
        Ok(())
    }

    async fn send_compressed_batch(&mut self) -> Result<(), BatchError> {
        let num_transactions = self.compressor.num_transactions();
        let compressed = take(&mut self.compressor).finish()?;

        let compressed_len = compressed.len();
        let max_size = self.max_batch_size.as_u64() as usize;

        if compressed_len >= max_size {
            error!(%self.chain_id, "Compressed batch size ({}) exceeds limit ({})", compressed_len, max_size);
            return Err(BatchError::BatchTooLarge(compressed_len, max_size));
        }

        self.send_batch(compressed.clone()).await?;

        self.compressor.reset(); // Reset for next round

        debug!(
            %self.chain_id, "Batch sent: {} txs, compressed size: {} bytes",
            num_transactions,
            compressed_len
        );
        Ok(())
    }

    async fn send_batch(&self, data: Bytes) -> Result<(), BatchError> {
        debug!(%self.chain_id, "Sending batch to wallet pool");
        let result = self
            .wallet_pool
            .processTransactionRaw(self.sequencing_contract_address, data)
            .send()
            .await;

        if let Err(e) = result {
            return Err(BatchError::SendBatchFailed(e.to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maestro::redis::producer::StreamProducer;
    use test_utils::docker::start_redis;
    use url::Url;

    fn test_config() -> BatcherConfig {
        BatcherConfig {
            max_batch_size: Byte::from_u64(1024),
            redis_url: "dummy".to_string(),
            chain_id: 1,
            polling_interval: Duration::from_millis(10),
            private_key: "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890"
                .to_string(),
            wallet_pool_address: Address::ZERO,
            sequencing_rpc_url: Url::parse("http://localhost:8545").unwrap(),
            sequencing_contract_address: Address::ZERO,
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_read_transactions_adds_to_compressor() {
        let mut config = test_config();
        // Start Redis container
        let (_redis, redis_url) = start_redis().await.unwrap();
        config.redis_url = redis_url.clone();

        // Connect to Redis
        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let chain_id = 1;
        let redis_consumer = StreamConsumer::new(conn.clone(), chain_id, "0-0".to_string());
        let producer =
            StreamProducer::new(conn, chain_id, Duration::from_secs(60), Duration::from_secs(60));

        // Send transaction
        let test_data1 = b"test transaction data 1".to_vec();
        producer.enqueue_transaction(test_data1.clone()).await.unwrap();
        let signer = PrivateKeySigner::from_str(&config.private_key).unwrap_or_else(|err| {
            panic!("Failed to parse default private key for signer: {}", err)
        });
        let sequencer_provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(signer))
            .on_http(config.sequencing_rpc_url.clone());
        let wallet_pool =
            WalletPoolWrapperModule::new(config.wallet_pool_address, sequencer_provider);
        let mut batcher = Batcher::new(&config, redis_consumer, wallet_pool);

        let result = batcher.read_transactions().await;
        assert!(result.is_ok());
        assert!(!batcher.compressor.is_empty(), "Compressor should not be empty after read");
    }

    #[tokio::test]
    async fn test_send_compressed_batch_returns_error_if_too_large() {
        let config = BatcherConfig {
            max_batch_size: Byte::from_u64(1), // force failure
            ..test_config()
        };
        // Start Redis container
        let (_redis, redis_url) = start_redis().await.unwrap();

        // Connect to Redis
        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn, 1, "0-0".to_string());

        let signer = PrivateKeySigner::from_str(&config.private_key).unwrap_or_else(|err| {
            panic!("Failed to parse default private key for signer: {}", err)
        });
        let sequencer_provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(signer))
            .on_http(config.sequencing_rpc_url.clone());
        let wallet_pool =
            WalletPoolWrapperModule::new(config.wallet_pool_address, sequencer_provider);

        let mut batcher = Batcher::new(&config, redis_consumer, wallet_pool);

        // Insert dummy data
        batcher.compressor.clone_and_compress_with_txn(&Bytes::from(vec![2, 3, 4])).unwrap();

        let result = batcher.send_compressed_batch().await;
        assert!(result.is_err());
    }
}
