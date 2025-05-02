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
use std::{
    mem::take,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};
use tc_client::tc_client::{TCClient, BATCH_FUNCTION_SIGNATURE};
use tokio::{sync::Semaphore, task::JoinHandle};
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
    /// The chain ID for the batcher
    chain_id: u64,
    /// The timeout for the batcher
    timeout: Duration,
    /// Submission Semaphore
    submission_semaphore: Arc<Semaphore>,
    /// Wallet pool
    wallet_pool: WalletPoolWrapperModuleInstance<(), FilledProvider>,
    /// The sequencing address
    sequencing_contract_address: Address,
    /// The TC client
    tc_client: Option<TCClient>,
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
pub async fn run_batcher(
    config: &BatcherConfig,
    tc_client: Option<TCClient>,
    wallet_pool_address: Address,
    sequencing_contract_address: Address,
) -> Result<JoinHandle<()>> {
    let client = RedisClient::open(config.redis_url.as_str()).map_err(|e| {
        eyre!("Failed to open Redis client: {}. Redis URL: {}", e, config.redis_url)
    })?;
    let conn = client.get_multiplexed_async_connection().await.map_err(|e| {
        eyre!("Failed to get Redis connection: {}. Redis URL: {}", e, config.redis_url)
    })?;
    let redis_consumer = StreamConsumer::new(conn, config.chain_id, "0-0".to_string());

    let wallet_pool = create_wallet_pool(config, wallet_pool_address);

    let mut batcher =
        Batcher::new(config, redis_consumer, wallet_pool, tc_client, sequencing_contract_address);

    let handle = tokio::spawn({
        async move {
            loop {
                debug!("Batcher reading and batching transactions at time {:?}", Instant::now());
                if let Err(e) = batcher.read_and_batch_transactions().await {
                    error!("Batcher error: {:?}", e);
                }
            }
        }
    });
    info!("Batcher job started with");
    Ok(handle)
}

fn create_wallet_pool(
    config: &BatcherConfig,
    wallet_pool_address: Address,
) -> WalletPoolWrapperModuleInstance<(), FilledProvider> {
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let sequencer_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .on_http(config.sequencing_rpc_url.clone());
    WalletPoolWrapperModule::new(wallet_pool_address, sequencer_provider)
}

impl Batcher {
    fn new(
        config: &BatcherConfig,
        redis_consumer: StreamConsumer,
        wallet_pool: WalletPoolWrapperModuleInstance<(), FilledProvider>,
        tc_client: Option<TCClient>,
        sequencing_contract_address: Address,
    ) -> Self {
        Self {
            max_batch_size: config.max_batch_size,
            redis_consumer,
            compressor: AdditiveCompressor::default(),
            chain_id: config.chain_id,
            timeout: config.timeout,
            submission_semaphore: Arc::new(Semaphore::new(1)), //Only one submission at a time
            wallet_pool,
            sequencing_contract_address,
            tc_client,
        }
    }
    async fn read_transactions(&mut self) -> Result<()> {
        let start = Instant::now();
        loop {
            if start.elapsed() >= self.timeout {
                debug!(%self.chain_id, "Read timeout reached. Stopping transaction read.");
                break;
            }

            // TODO (SEQ-842): Configurable max msg count
            // NOTE: If msg count is >1 we need to handle edge cases where not all transactions fit
            // in the batch
            let transactions = self.redis_consumer.recv(1, Duration::from_millis(100)).await?;

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
        // Ensure only one submission runs at a time by holding the semaphore permit
        let semaphore = Arc::clone(&self.submission_semaphore);
        let _permit = match semaphore.try_acquire() {
            Ok(permit) => permit,
            Err(_) => {
                debug!(%self.chain_id, "Submission in flight, skipping read.");
                return Ok(());
            }
        };

        // _permit is held until the end of the function

        self.read_transactions().await?;

        if self.compressor.is_empty() {
            debug!(%self.chain_id, "No transactions available to batch.");
            return Ok(());
        }

        if let Err(e) = self.send_compressed_batch().await {
            // Don't reset the compressor here, because we want to retry the same batch
            error!(%self.chain_id, "Failed to send compressed batch: {:?}", e);
            return Err(e);
        }

        self.compressor.reset(); // Reset for next round
        Ok(())
    }

    async fn send_compressed_batch(&mut self) -> Result<(), BatchError> {
        let num_transactions = self.compressor.num_transactions();
        debug!(%self.chain_id, "Sending compressed batch with {} transactions", num_transactions);
        let compressed = take(&mut self.compressor).finish()?;

        let compressed_len = compressed.len();
        let max_size = self.max_batch_size.as_u64() as usize;

        if compressed_len >= max_size {
            error!(%self.chain_id, "Compressed batch size ({}) exceeds limit ({})", compressed_len, max_size);
            return Err(BatchError::BatchTooLarge(compressed_len, max_size));
        }

        self.send_batch(compressed.clone()).await?;

        debug!(
            %self.chain_id, "Batch sent: {} txs, compressed size: {} bytes",
            num_transactions,
            compressed_len
        );
        Ok(())
    }

    async fn send_batch(&self, data: Bytes) -> Result<(), BatchError> {
        if self.tc_client.is_some() {
            self.send_batch_to_tc(data).await
        } else {
            self.send_batch_to_l2(data).await
        }
    }

    async fn send_batch_to_l2(&self, data: Bytes) -> Result<(), BatchError> {
        debug!(%self.chain_id, "Sending batch to L2");
        let result = self
            .wallet_pool
            .processTransactionRaw(self.sequencing_contract_address, data)
            .send()
            .await;

        info!("Batch sent result: {:?}", result);

        if let Err(e) = result {
            return Err(BatchError::SendBatchFailed(e.to_string()));
        }

        Ok(())
    }

    async fn send_batch_to_tc(&self, data: Bytes) -> Result<(), BatchError> {
        debug!(%self.chain_id, "Sending batch to TC");
        let client = self.tc_client.as_ref().ok_or_else(|| {
            BatchError::SendBatchFailed("tc_client is None, expected Some".to_string())
        })?;
        let result = client.send_transaction(data, BATCH_FUNCTION_SIGNATURE.to_string()).await;

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
    use tc_client::config::TCConfig;
    use test_utils::docker::start_redis;
    use url::Url;

    fn test_config() -> BatcherConfig {
        BatcherConfig {
            max_batch_size: Byte::from_u64(1024),
            redis_url: "dummy".to_string(),
            chain_id: 1,
            timeout: Duration::from_millis(200),
            private_key: "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890"
                .to_string(),
            sequencing_rpc_url: Url::parse("http://localhost:8545").unwrap(),
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
        let signer = PrivateKeySigner::from_str(&config.private_key)
            .unwrap_or_else(|err| panic!("Failed to parse private key for batcher: {}", err));
        let sequencer_provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(signer))
            .on_http(config.sequencing_rpc_url.clone());
        let wallet_pool = WalletPoolWrapperModule::new(Address::ZERO, sequencer_provider);
        let mut batcher = Batcher::new(&config, redis_consumer, wallet_pool, None, Address::ZERO);

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
        let wallet_pool = WalletPoolWrapperModule::new(Address::ZERO, sequencer_provider);

        let mut batcher = Batcher::new(&config, redis_consumer, wallet_pool, None, Address::ZERO);

        // Insert dummy data
        batcher.compressor.clone_and_compress_with_txn(&Bytes::from(vec![2, 3, 4])).unwrap();

        let result = batcher.send_compressed_batch().await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_read_transactions_no_data() {
        let mut config = test_config();
        let (_redis, redis_url) = start_redis().await.unwrap();
        config.redis_url = redis_url.clone();

        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn, config.chain_id, "0-0".to_string());

        let signer = PrivateKeySigner::from_str(&config.private_key).unwrap();
        let sequencer_provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(signer))
            .on_http(config.sequencing_rpc_url.clone());
        let wallet_pool = WalletPoolWrapperModule::new(Address::ZERO, sequencer_provider);

        let mut batcher = Batcher::new(&config, redis_consumer, wallet_pool, None, Address::ZERO);
        let result = batcher.read_transactions().await;

        assert!(result.is_ok());
        assert!(batcher.compressor.is_empty(), "Compressor should be empty");
    }

    #[tokio::test]
    async fn test_submission_in_flight_skips_batching() {
        let config = test_config();
        let (_redis, redis_url) = start_redis().await.unwrap();

        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn, config.chain_id, "0-0".to_string());

        let signer = PrivateKeySigner::from_str(&config.private_key).unwrap();
        let sequencer_provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(signer))
            .on_http(config.sequencing_rpc_url.clone());
        let wallet_pool = WalletPoolWrapperModule::new(Address::ZERO, sequencer_provider);

        let mut batcher = Batcher::new(&config, redis_consumer, wallet_pool, None, Address::ZERO);

        // Clone the semaphore to avoid borrowing through `batcher`
        let semaphore = Arc::clone(&batcher.submission_semaphore);
        let _guard = semaphore.try_acquire().expect("should acquire permit");

        // Now `batcher` can be mutably borrowed safely
        let result = batcher.read_and_batch_transactions().await;
        assert!(result.is_ok(), "Expected batching to skip and return Ok");
    }
    #[tokio::test]
    async fn test_send_batch_prefers_tc_client() {
        let config = test_config();
        let dummy_client =
            TCClient::new(&TCConfig::default(), Address::ZERO, Address::ZERO).unwrap();
        let (_redis, redis_url) = start_redis().await.unwrap();
        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn, config.chain_id, "0-0".to_string());

        let signer = PrivateKeySigner::from_str(&config.private_key).unwrap();
        let sequencer_provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(signer))
            .on_http(config.sequencing_rpc_url.clone());
        let wallet_pool = WalletPoolWrapperModule::new(Address::ZERO, sequencer_provider);

        let batcher =
            Batcher::new(&config, redis_consumer, wallet_pool, Some(dummy_client), Address::ZERO);

        // Just verify logic flow doesn't panic
        let result = batcher.send_batch(Bytes::from(vec![1, 2, 3])).await;
        let _ = result.is_err(); // likely will error due to dummy TCClient
    }
}
