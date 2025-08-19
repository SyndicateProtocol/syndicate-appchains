//! The Batcher service for the sequencer.

use crate::{
    config::BatcherConfig,
    metrics::BatcherMetrics,
    sequencing_batch::{SequencingBatch, TxWithVakeyId},
};
use alloy::{
    network::EthereumWallet,
    primitives::{keccak256, Bytes},
    providers::Provider,
    signers::local::PrivateKeySigner,
    transports::TransportError,
};
use contract_bindings::synd::syndicate_sequencing_chain::SyndicateSequencingChain::SyndicateSequencingChainInstance;
use derivative::Derivative;
use eyre::{eyre, Result};
use redis::{aio::ConnectionManager, Client as ValkeyClient};
use shared::{
    multi_rpc_provider::{MultiRpcProvider, RetryConfig},
    service_start_utils::{
        cache_health_check_handler, start_http_server_with_aux_handlers, MetricsState,
    },
    tracing::SpanKind,
};
use std::{
    collections::VecDeque,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};
use synd_maestro::{
    valkey::{
        models::consumer_last_id::ConsumerLastIdExt as _,
        streams::consumer::StreamConsumer,
        valkey_metrics::{CacheType, Operation, ValkeyMetrics},
    },
    with_cache_metrics,
};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, instrument, trace, warn};

/// Batcher service
#[derive(Derivative)]
#[derivative(Debug)]
struct Batcher {
    /// Batcher configuration
    config: BatcherConfig,
    /// The max batch size for the batcher (default: 90KB)
    max_batch_size: usize,
    /// The Stream consumer for the batcher
    stream_consumer: StreamConsumer,
    /// The Valkey connection for the batcher
    #[derivative(Debug = "ignore")]
    valkey_conn: ConnectionManager,
    /// The sequencing contract provider for the batcher
    sequencing_contract_instance: SyndicateSequencingChainInstance<MultiRpcProvider>,
    /// Metrics
    metrics: BatcherMetrics,
    /// Outstanding transactions that didn't fit in the last batch and their stream ID
    /// [(0xbytes, <milliseconds-since-epoch>-<sequence>)]
    outstanding_txs: Vec<TxWithVakeyId>,
}

#[derive(Debug, thiserror::Error)]
enum BatchError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] eyre::Report),

    #[error("Failed to send batch: {0}")]
    SendBatchFailed(String),
}

/// Get the consumer last id from the Valkey cache.
///
/// If the consumer last id is not found, return "0-0".
/// If the consumer last id is found but fails to parse, return "0-0".
/// `0-0` means the consumer will start from the beginning of all transactions in the cache
pub async fn get_cached_consumer_last_id(
    conn: &mut ConnectionManager,
    chain_id: u64,
    valkey_metrics: ValkeyMetrics,
) -> String {
    let consumer_last_id =
        match with_cache_metrics!(&valkey_metrics, conn.get_consumer_last_id(chain_id)) {
            Ok(Some(id)) => id,
            Err(e) => {
                warn!(%e, %chain_id, "Failed to get consumer last id, using 0-0");
                "0-0".to_string()
            }
            _ => "0-0".to_string(),
        };

    consumer_last_id
}

/// Run the batcher service. Starts the server and listens for batch requests.
pub async fn run_batcher(config: BatcherConfig) -> Result<JoinHandle<()>> {
    let client = ValkeyClient::open(config.valkey_url.as_str()).map_err(|e| {
        eyre!("Failed to open Valkey client: {}. Valkey URL: {}", e, config.valkey_url)
    })?;
    let mut conn = ConnectionManager::new(client).await.map_err(|e| {
        eyre!("Failed to get Valkey connection: {}. Valkey URL: {}", e, config.valkey_url)
    })?;

    // Start metrics and health endpoints
    let mut metrics_state = MetricsState::default();
    let registry = &mut metrics_state.registry;
    let metrics = BatcherMetrics::new(registry);
    let valkey_metrics = metrics.valkey.clone();

    let cache_health_handler = cache_health_check_handler(conn.clone());
    tokio::spawn(start_http_server_with_aux_handlers(
        metrics_state,
        config.port,
        Some(cache_health_handler.clone()),
        Some(cache_health_handler), // same /ready behavior as /health
    ));

    let sequencing_contract_instance = create_sequencing_contract_instance(&config).await?;

    let consumer_last_id =
        get_cached_consumer_last_id(&mut conn, config.chain_id, valkey_metrics.clone()).await;

    debug!(%config.chain_id, %consumer_last_id, "Using consumer last id");

    let stream_consumer = StreamConsumer::new(
        conn.clone(),
        config.chain_id,
        consumer_last_id,
        Arc::new(valkey_metrics),
    );

    let mut batcher = Batcher::new(
        config,
        conn.clone(),
        stream_consumer,
        sequencing_contract_instance,
        metrics.clone(),
    );

    let handle = tokio::spawn({
        async move {
            loop {
                debug!(
                    "Batcher reading and batching transactions at time {}",
                    humantime::format_rfc3339_millis(SystemTime::now()),
                );
                if let Err(e) = batcher.process_transactions().await {
                    error!("Batcher error: {e:?}");
                    // Reset wallet nonce by recreating the contract instance
                    batcher.reset_sequencing_contract_instance().await;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });
    info!("Batcher job started");
    Ok(handle)
}

async fn create_sequencing_contract_instance(
    config: &BatcherConfig,
) -> Result<SyndicateSequencingChainInstance<MultiRpcProvider>, TransportError> {
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {err}"));

    let retry_config = RetryConfig::new(
        config.rpc_max_retries,
        config.rpc_initial_backoff_ms,
        config.rpc_compute_units_per_second,
        config.rpc_compute_units_avg_request_cost,
    );

    let multi_provider = MultiRpcProvider::new_with_wallet_and_retry(
        config.sequencing_rpc_urls.clone(),
        None,
        EthereumWallet::from(signer),
        Some(retry_config),
    )
    .await
    .map_err(|e| TransportError::local_usage_str(&e.to_string()))?;

    Ok(SyndicateSequencingChainInstance::new(config.sequencing_address, multi_provider))
}

impl Batcher {
    fn new(
        config: BatcherConfig,
        valkey_conn: ConnectionManager,
        stream_consumer: StreamConsumer,
        sequencing_contract_instance: SyndicateSequencingChainInstance<MultiRpcProvider>,
        metrics: BatcherMetrics,
    ) -> Self {
        Self {
            config: config.clone(),
            max_batch_size: config.max_batch_size.as_u64() as usize,
            stream_consumer,
            valkey_conn,
            sequencing_contract_instance,
            metrics,
            outstanding_txs: Vec::new(),
        }
    }

    async fn reset_sequencing_contract_instance(&mut self) {
        debug!("Resetting sequencing contract instance");
        let contract_instance = match create_sequencing_contract_instance(&self.config).await {
            Ok(instance) => instance,
            Err(e) => {
                error!("Failed to reset sequencing contract instance: {:?}", e);
                return
            }
        };

        self.sequencing_contract_instance = contract_instance
    }

    #[instrument(
        skip(self),
        err,
        fields(
            otel.kind = ?SpanKind::Consumer,
            chain_id = %self.config.chain_id
        )
    )]
    async fn process_transactions(&mut self) -> Result<(), BatchError> {
        let start = Instant::now();

        let (batch, last_included_id) = self.read_and_batch_transactions().await?;
        if batch.is_empty() {
            trace!(%self.config.chain_id, "No transactions available to batch.");
            return Ok(());
        }

        let submission_start = Instant::now();
        if let Err(e) = self.send_batch(&batch).await {
            // Don't reset the compressor here, because we want to retry the same batch
            error!(%self.config.chain_id, "send_batch failed: {:?}", e);
            self.outstanding_txs.extend(batch.into_owned_txs());
            return Err(e);
        }

        self.update_consumer_last_id(last_included_id);
        self.metrics.record_outstanding_txs(self.outstanding_txs.len());
        self.metrics.record_submission_latency(submission_start.elapsed());
        self.metrics.record_processing_time(start.elapsed());

        Ok(())
    }

    #[instrument(
        skip(self),
        err,
        fields(
            otel.kind = ?SpanKind::Consumer,
            chain_id = %self.config.chain_id
        )
    )]
    async fn read_and_batch_transactions(&mut self) -> Result<(SequencingBatch, String)> {
        let start = Instant::now();
        let mut batch = match self.config.compression_enabled {
            true => SequencingBatch::Compressed(vec![], vec![]),
            false => SequencingBatch::Uncompressed(vec![]),
        };
        let mut uncompressed_size = 0;
        let mut last_included_id = "0-0".to_string();

        'outer: loop {
            if start.elapsed() >= self.config.timeout {
                debug!(%self.config.chain_id, "Read timeout reached. Stopping transaction read.");
                break;
            }

            // TODO (SEQ-842): Configurable max msg count
            // NOTE: If msg count is >1 we need to handle edge cases where not all transactions fit
            // in the batch
            let incoming_txs = self.stream_consumer.recv(1, Duration::from_millis(100)).await?;

            // Combine outstanding transactions with incoming transactions
            let mut pending_txs: VecDeque<(Vec<u8>, String)> =
                std::mem::take(&mut self.outstanding_txs).into_iter().chain(incoming_txs).collect();

            // Process transactions until one doesn't fit
            while let Some((tx_bytes, tx_id)) = pending_txs.front() {
                let proposed_batch = batch.create_new_with_tx((tx_bytes.clone(), tx_id.clone()))?;

                if proposed_batch.len() > self.max_batch_size {
                    // If the current txs vector is empty, that means the transaction we are trying
                    // to add is too large to fit in a single batch by itself.
                    // We need to discard it, or this loop will get stuck trying
                    // to add the same transaction over and over.
                    if batch.txs().is_empty() {
                        let (bad_tx, bad_tx_id) = pending_txs.pop_front().unwrap_or_default();
                        error!(%self.config.chain_id, tx_hash=%keccak256(bad_tx), tx_id=%bad_tx_id, "Transaction is too large to fit in the batch. Discarding");
                        continue;
                    }

                    // Stop consuming
                    // Save all remaining transactions
                    self.outstanding_txs = pending_txs.into();
                    break 'outer;
                }

                uncompressed_size += tx_bytes.len();
                debug!(
                    %self.config.chain_id, %tx_id, batch_size = %proposed_batch.len(), "Adding transaction to batch",
                );
                batch = proposed_batch;
                last_included_id = tx_id.clone();
                pending_txs.pop_front(); // remove the tx from the pending txs
            }
        }
        self.metrics.record_batch_transactions(batch.txs().len());
        if self.config.compression_enabled {
            if batch.len() > uncompressed_size {
                warn!(%self.config.chain_id, "Batch compressed size is larger than uncompressed size.");
            }
            self.metrics.record_compression_space_saving_pct(uncompressed_size, batch.len());
        }
        Ok((batch, last_included_id))
    }

    #[instrument(
        skip_all,
        err,
        fields(
            otel.kind = ?SpanKind::Producer,
            chain_id = %self.config.chain_id,
            batch_size = %batch.len()
        )
    )]
    async fn send_batch(&mut self, batch: &SequencingBatch) -> Result<(), BatchError> {
        debug!(
            %self.config.chain_id, "Sending Batch - size: {} bytes",
            batch.len()
        );

        let transaction_request = match batch {
            SequencingBatch::Compressed(compressed_bytes, _) => self
                .sequencing_contract_instance
                .processTransaction(Bytes::from(compressed_bytes.clone()))
                .into_transaction_request(),
            SequencingBatch::Uncompressed(batch) => self
                .sequencing_contract_instance
                .processTransactionsBulk(batch.iter().map(|tx| Bytes::from(tx.0.clone())).collect())
                .into_transaction_request(),
        };

        let pending_tx = self
            .sequencing_contract_instance
            .provider()
            .send_transaction(transaction_request.clone())
            .await
            .map_err(|e| {
                BatchError::SendBatchFailed(format!("error: {e:?}, tx: {transaction_request:?}",))
            })?;

        if self.config.wait_for_receipt {
            let receipt = pending_tx
                .with_timeout(self.config.wait_for_receipt_timeout)
                .get_receipt()
                .await
                .map_err(|e| BatchError::SendBatchFailed(e.to_string()))?;
            warn!(%self.config.chain_id, "Batch submitted and receipt received: {:?}", receipt);
            if !receipt.status() {
                error!(%self.config.chain_id, "Batch submission failed. tx: {:?}, receipt: {:?}", transaction_request, receipt);
                return Err(BatchError::SendBatchFailed("Batch submission failed".to_string()));
            }
        }
        debug!(%self.config.chain_id, "Batch submitted");

        // Record the wallet balance in the background
        self.record_wallet_balance();

        Ok(())
    }

    fn record_wallet_balance(&self) {
        let provider = self.sequencing_contract_instance.provider().clone();
        let metrics = self.metrics.clone();
        tokio::spawn(async move {
            let wallet_address = provider.signer_address();

            match provider.get_balance(wallet_address).await {
                Ok(balance) => {
                    metrics.record_wallet_balance(balance.to());
                }
                Err(e) => {
                    error!("Failed to get wallet balance: {:?}", e);
                }
            }
        });
    }

    fn update_consumer_last_id(&self, last_included_id: String) {
        if last_included_id.is_empty() {
            return;
        }
        let mut conn = self.valkey_conn.clone();
        let chain_id = self.config.chain_id;
        let metrics = self.metrics.clone();
        tokio::spawn(async move {
            let cache_result = with_cache_metrics!(
                &metrics.valkey,
                Operation::Write,
                CacheType::ValkeyCache,
                conn.set_consumer_last_id(chain_id, last_included_id.clone())
            );

            match cache_result {
                Ok(_) => {
                    debug!(%chain_id, %last_included_id, "Updated consumer last id");
                }
                Err(e) => {
                    warn!(%e, %chain_id, %last_included_id, "Failed to update consumer last id");
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        eips::Encodable2718,
        network::{EthereumWallet, TransactionBuilder},
        primitives::{Address, U256},
        providers::ext::AnvilApi,
        rpc::types::TransactionRequest,
        sol,
    };
    use prometheus_client::registry::Registry;
    use reqwest;
    use shared::{tracing::setup_global_logging, types::FilledProvider};
    use std::sync::Arc;
    use synd_maestro::valkey::{
        streams::producer::{CheckFinalizationResult, StreamProducer},
        valkey_metrics::ValkeyMetrics,
    };
    use test_utils::{
        anvil::start_anvil,
        chain_info::{test_account1, ChainInfo},
        docker::{start_valkey, E2EProcess},
        port_manager::PortManager,
        wait_until,
    };
    use url::Url;

    #[ctor::ctor]
    fn init() {
        setup_global_logging();
    }

    // Create a mock provider that always succeeds
    async fn create_mock_contract(
        anvil: Option<&ChainInfo>,
    ) -> SyndicateSequencingChainInstance<MultiRpcProvider> {
        let mock_address = Address::from([0; 20]); // Use a dummy address

        let signer = PrivateKeySigner::from_str(
            "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890",
        )
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {err}"));

        let mock_provider = if let Some(anvil) = anvil {
            let signer_address =
                Address::from_str("0x06A85356DCb5b307096726FB86A78c59D38e08ee").unwrap();

            let multi_provider = MultiRpcProvider::new_with_wallet(
                vec![Url::parse(&anvil.http_url).unwrap()],
                None,
                EthereumWallet::from(signer.clone()),
            )
            .await
            .unwrap();

            // Set balance at 100 ETH using the underlying provider
            let provider = multi_provider.active_provider();
            let balance = U256::from(100) * U256::from(10).pow(U256::from(18));
            provider.anvil_set_balance(signer_address, balance).await.unwrap();
            multi_provider
        } else {
            // For mocked client, create providers directly without network connection
            use alloy::{providers::ProviderBuilder, transports::mock::Asserter};
            let asserter = Asserter::new();
            let mock_provider = ProviderBuilder::new()
                .wallet(EthereumWallet::from(signer))
                .connect_mocked_client(asserter);

            MultiRpcProvider::from_providers(vec![Arc::new(mock_provider)])
        };
        SyndicateSequencingChainInstance::new(mock_address, mock_provider)
    }

    fn test_config() -> BatcherConfig {
        BatcherConfig {
            max_batch_size: byte_unit::Byte::from_u64(1024),
            valkey_url: "dummy".to_string(),
            chain_id: 1,
            compression_enabled: true,
            timeout: Duration::from_millis(200),
            private_key: test_account1().private_key.to_string(),
            sequencing_rpc_urls: vec![Url::parse("http://localhost:8545").unwrap()],
            wait_for_receipt_timeout: Some(Duration::from_secs(1)),
            ..Default::default()
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_read_transactions() {
        let mut config = test_config();
        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let chain_id = 1;
        let valkey_metrics = Arc::new(ValkeyMetrics::default());
        let stream_consumer =
            StreamConsumer::new(conn.clone(), chain_id, "0-0".to_string(), valkey_metrics);
        let producer = StreamProducer::new(
            conn.clone(),
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
        )
        .await;

        let test_data1 = b"test transaction data 1".to_vec();
        producer.enqueue_transaction(&test_data1).await.unwrap();
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let sequencing_contract_instance = create_mock_contract(None).await;
        let mut batcher = Batcher::new(
            config,
            conn.clone(),
            stream_consumer,
            sequencing_contract_instance,
            metrics,
        );

        let result = batcher.read_and_batch_transactions().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_compressed_batch_returns_error_if_too_large() {
        let mut config = test_config();
        config.max_batch_size = byte_unit::Byte::from_u64(1); // force failure
        config.compression_enabled = false;
        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();
        let valkey_metrics = Arc::new(ValkeyMetrics::default());

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let chain_id = 1;
        let stream_consumer =
            StreamConsumer::new(conn.clone(), chain_id, "0-0".to_string(), valkey_metrics);
        let producer = StreamProducer::new(
            conn.clone(),
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
        )
        .await;

        let test_data1 = b"test transaction data 1".to_vec();
        producer.enqueue_transaction(&test_data1).await.unwrap();
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let sequencing_contract_instance = create_mock_contract(None).await;
        let mut batcher = Batcher::new(
            config,
            conn.clone(),
            stream_consumer,
            sequencing_contract_instance,
            metrics,
        );

        let result = batcher.read_and_batch_transactions().await;
        assert!(result.is_ok(), "result: {result:?}");
        assert!(result.unwrap().0.is_empty());
    }

    #[tokio::test]
    async fn test_read_transactions_no_data() {
        let mut config = test_config();
        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();
        let valkey_metrics = Arc::new(ValkeyMetrics::default());

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let stream_consumer =
            StreamConsumer::new(conn.clone(), config.chain_id, "0-0".to_string(), valkey_metrics);

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let sequencing_contract_instance = create_mock_contract(None).await;
        let mut batcher = Batcher::new(
            config,
            conn.clone(),
            stream_consumer,
            sequencing_contract_instance,
            metrics,
        );
        let result = batcher.read_and_batch_transactions().await;

        assert!(result.is_ok());
        assert!(result.unwrap().0.is_empty());
    }

    #[tokio::test]
    async fn test_multiple_txs() {
        let mut config = test_config();
        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();
        let valkey_metrics = Arc::new(ValkeyMetrics::default());

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let stream_consumer =
            StreamConsumer::new(conn.clone(), config.chain_id, "0-0".to_string(), valkey_metrics);
        let producer = StreamProducer::new(
            conn.clone(),
            config.chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
        )
        .await;

        // Add 100 test transactions of ~50KB each
        // Create a 50KB transaction by repeating the pattern
        let base_data = b"test transaction data";
        let mut test_data = Vec::with_capacity(50 * 1024); // 50KB
        while test_data.len() < 50 * 1024 {
            test_data.extend_from_slice(base_data);
        }
        test_data.truncate(50 * 1024); // Ensure exact 50KB
        for _ in 0..100 {
            producer.enqueue_transaction(&test_data).await.unwrap();
        }

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);
        let metrics_clone = metrics.clone();

        let anvil = start_anvil(1).await.unwrap();
        config.sequencing_rpc_urls = vec![Url::parse(&anvil.http_url).unwrap()];
        let sequencing_contract_instance = create_mock_contract(Some(&anvil)).await;
        let mut batcher = Batcher::new(
            config,
            conn.clone(),
            stream_consumer,
            sequencing_contract_instance,
            metrics,
        );

        // Run the batcher and verify it sends the batch
        let _handle = tokio::spawn(async move {
            loop {
                batcher.process_transactions().await.unwrap();
            }
        });

        wait_until!(metrics_clone.total_txs_processed.get() == 100, Duration::from_secs(10));
        drop(anvil);
    }

    #[tokio::test]
    async fn test_multiple_txs_uncompressed() {
        let mut config = test_config();
        config.compression_enabled = false;
        config.max_batch_size = byte_unit::Byte::from_u64(51 * 1024); // 51KB

        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();
        let valkey_metrics = Arc::new(ValkeyMetrics::default());

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let stream_consumer =
            StreamConsumer::new(conn.clone(), config.chain_id, "0-0".to_string(), valkey_metrics);
        let producer = StreamProducer::new(
            conn.clone(),
            config.chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
        )
        .await;

        // Add 20 test transactions of ~10KB each
        // Create a 10KB transaction by repeating the pattern
        let base_data = b"test transaction data";
        let mut test_data = Vec::with_capacity(10 * 1024); // 10KB
        while test_data.len() < 10 * 1024 {
            test_data.extend_from_slice(base_data);
        }
        test_data.truncate(10 * 1024); // Ensure exact 10KB
        for _ in 0..20 {
            producer.enqueue_transaction(&test_data).await.unwrap();
        }

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);
        let metrics_clone = metrics.clone();

        let anvil = start_anvil(1).await.unwrap();
        config.sequencing_rpc_urls = vec![Url::parse(&anvil.http_url).unwrap()];
        let sequencing_contract_instance = create_mock_contract(Some(&anvil)).await;
        let mut batcher = Batcher::new(
            config,
            conn.clone(),
            stream_consumer,
            sequencing_contract_instance,
            metrics,
        );

        // Run the batcher and verify it sends the batch
        let _handle = tokio::spawn(async move {
            loop {
                batcher.process_transactions().await.unwrap();
            }
        });

        wait_until!(metrics_clone.total_txs_processed.get() == 20, Duration::from_secs(10));
        drop(anvil);
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let (valkey, valkey_url) = start_valkey().await.unwrap();
        let anvil = start_anvil(1).await.unwrap();

        let port = PortManager::instance().next_port().await;
        let config = BatcherConfig {
            max_batch_size: byte_unit::Byte::from_u64(1024),
            valkey_url,
            chain_id: 1,
            compression_enabled: true,
            timeout: Duration::from_millis(200),
            private_key: "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890"
                .to_string(),
            sequencing_rpc_urls: vec![Url::parse(&anvil.http_url).unwrap()],
            rpc_max_retries: 10,
            port,
            sequencing_address: Address::ZERO,
            ..Default::default()
        };

        let _handle = run_batcher(config).await.unwrap();

        let url = format!("http://0.0.0.0:{}/health", port);

        let client = reqwest::Client::new();

        // Should succeed
        wait_until!(
            client.get(&url).send().await.is_ok_and(|resp| resp.status() == 200),
            Duration::from_secs(2)
        );

        // Close Valkey container and test failure
        drop(valkey);

        wait_until!(
            client.get(&url).send().await.is_ok_and(|resp| resp.status() == 500),
            Duration::from_secs(2)
        );
    }

    #[tokio::test]
    async fn test_consumer_last_id_update_on_tx() {
        let mut config = test_config();
        config.compression_enabled = false;
        config.max_batch_size = byte_unit::Byte::from_u64(51 * 1024); // 51KB

        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();
        let valkey_metrics = Arc::new(ValkeyMetrics::default());

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();

        let anvil = start_anvil(1).await.unwrap();
        config.sequencing_rpc_urls = vec![Url::parse(&anvil.http_url).unwrap()];

        let stream_consumer = StreamConsumer::new(
            conn.clone(),
            config.chain_id,
            "0-0".to_string(),
            valkey_metrics.clone(),
        );
        let producer = StreamProducer::new(
            conn.clone(),
            config.chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
        )
        .await;

        // Enqueue one transaction
        let test_data = b"hello world".to_vec();
        producer.enqueue_transaction(&test_data).await.unwrap();

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let sequencing_contract_instance = create_mock_contract(Some(&anvil)).await;
        let chain_id = config.chain_id;
        let mut batcher = Batcher::new(
            config,
            conn.clone(),
            stream_consumer,
            sequencing_contract_instance,
            metrics,
        );

        // Process the transaction so that last_included_id is updated and persisted
        batcher.process_transactions().await.unwrap();

        // get_cached_consumer_last_id reads what update_consumer_last_id wrote,
        // which is done on a spawned task; wait until it shows up.
        wait_until!(
            {
                let mut c = conn.clone();
                let vm = (*valkey_metrics).clone();
                get_cached_consumer_last_id(&mut c, chain_id, vm).await != "0-0"
            },
            Duration::from_secs(5)
        );

        // Final assert
        let mut conn_clone = conn.clone();
        let updated =
            get_cached_consumer_last_id(&mut conn_clone, chain_id, (*valkey_metrics).clone()).await;
        assert_ne!(updated, "0-0", "Expected consumer last id to be updated after processing a tx");
    }

    // Mock sequencing contract that mimics the real SyndicateSequencingChain interface
    sol! {
        #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b506109548061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610055575f3560e01c806306661abd1461005957806346e2cc0914610077578063804e512314610093578063cdafb978146100af578063f0ba8440146100cb575b5f5ffd5b6100616100fb565b60405161006e91906102bc565b60405180910390f35b610091600480360381019061008c919061033e565b610100565b005b6100ad60048036038101906100a8919061033e565b610141565b005b6100c960048036038101906100c491906103de565b610182565b005b6100e560048036038101906100e09190610453565b610204565b6040516100f291906104ee565b60405180910390f35b5f5481565b818160015f546103e881106101185761011761050e565b5b01918261012692919061076f565b505f5f81548092919061013890610869565b91905055505050565b818160015f546103e881106101595761015861050e565b5b01918261016792919061076f565b505f5f81548092919061017990610869565b91905055505050565b5f5f90505b828290508110156101ff578282828181106101a5576101a461050e565b5b90506020028101906101b791906108bc565b60015f546103e881106101cd576101cc61050e565b5b0191826101db92919061076f565b505f5f8154809291906101ed90610869565b91905055508080600101915050610187565b505050565b6001816103e88110610214575f80fd5b015f9150905080546102259061059f565b80601f01602080910402602001604051908101604052809291908181526020018280546102519061059f565b801561029c5780601f106102735761010080835404028352916020019161029c565b820191905f5260205f20905b81548152906001019060200180831161027f57829003601f168201915b505050505081565b5f819050919050565b6102b6816102a4565b82525050565b5f6020820190506102cf5f8301846102ad565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126102fe576102fd6102dd565b5b8235905067ffffffffffffffff81111561031b5761031a6102e1565b5b602083019150836001820283011115610337576103366102e5565b5b9250929050565b5f5f60208385031215610354576103536102d5565b5b5f83013567ffffffffffffffff811115610371576103706102d9565b5b61037d858286016102e9565b92509250509250929050565b5f5f83601f84011261039e5761039d6102dd565b5b8235905067ffffffffffffffff8111156103bb576103ba6102e1565b5b6020830191508360208202830111156103d7576103d66102e5565b5b9250929050565b5f5f602083850312156103f4576103f36102d5565b5b5f83013567ffffffffffffffff811115610411576104106102d9565b5b61041d85828601610389565b92509250509250929050565b610432816102a4565b811461043c575f5ffd5b50565b5f8135905061044d81610429565b92915050565b5f60208284031215610468576104676102d5565b5b5f6104758482850161043f565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f6104c08261047e565b6104ca8185610488565b93506104da818560208601610498565b6104e3816104a6565b840191505092915050565b5f6020820190508181035f83015261050681846104b6565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f60028204905060018216806105b657607f821691505b6020821081036105c9576105c8610572565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261062b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826105f0565b61063586836105f0565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61067061066b610666846102a4565b61064d565b6102a4565b9050919050565b5f819050919050565b61068983610656565b61069d61069582610677565b8484546105fc565b825550505050565b5f5f905090565b6106b46106a5565b6106bf818484610680565b505050565b5b818110156106e2576106d75f826106ac565b6001810190506106c5565b5050565b601f821115610727576106f8816105cf565b610701846105e1565b81016020851015610710578190505b61072461071c856105e1565b8301826106c4565b50505b505050565b5f82821c905092915050565b5f6107475f198460080261072c565b1980831691505092915050565b5f61075f8383610738565b9150826002028217905092915050565b610779838361053b565b67ffffffffffffffff81111561079257610791610545565b5b61079c825461059f565b6107a78282856106e6565b5f601f8311600181146107d4575f84156107c2578287013590505b6107cc8582610754565b865550610833565b601f1984166107e2866105cf565b5f5b82811015610809578489013582556001820191506020850194506020810190506107e4565b868310156108265784890135610822601f891682610738565b8355505b6001600288020188555050505b50505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610873826102a4565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036108a5576108a461083c565b5b600182019050919050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f833560016020038436030381126108d8576108d76108b0565b5b80840192508235915067ffffffffffffffff8211156108fa576108f96108b4565b5b602083019250600182023603831315610916576109156108b8565b5b50925092905056fea26469706673582212204ff9b99c0e542d742d48d5cb59e53d58788dfc360267673e76b6165ed68e3afb64736f6c634300081e0033")]
        contract MockSequencingContract {
            uint256 public count;
            bytes[1000] public data;

            function processTransaction(bytes calldata _data) external {
                data[count] = _data;
                count++;
            }
            function processTransactionUncompressed(bytes calldata _data) external {
                data[count] = _data;
                count++;
            }
            function processTransactionsBulk(bytes[] calldata _data) external {
                for (uint256 i = 0; i < _data.length; i++) {
                    data[count] = _data[i];
                    count++;
                }
            }
        }
    }

    #[tokio::test]
    async fn test_nonce_too_low_recovery() {
        let count = 2;
        let (anvil, mock_contract, producer, _valkey, _handle) = setup_nonce_test(count).await;
        let sequencer_address = test_account1().signer.address();

        let nonce = anvil.provider.get_transaction_count(sequencer_address).await.unwrap();
        info!("Sequencer nonce: {}", nonce);

        let tx = TransactionRequest::default()
            .with_to(sequencer_address)
            .with_from(sequencer_address)
            .with_value(U256::from(1))
            .with_gas_limit(100_000)
            .with_chain_id(1)
            .with_max_fee_per_gas(100000000)
            .with_nonce(nonce)
            .with_max_priority_fee_per_gas(0)
            .build(&EthereumWallet::from(test_account1().signer.clone()))
            .await
            .unwrap()
            .encoded_2718();
        let receipt =
            anvil.provider.send_raw_transaction(&tx).await.unwrap().get_receipt().await.unwrap();

        assert!(receipt.status());

        let test_data = b"transaction 11".to_vec();
        producer.enqueue_transaction(&test_data).await.unwrap();

        wait_until!(
            mock_contract.count().call().await.unwrap() == U256::from(count + 1),
            Duration::from_secs(5)
        );
    }

    #[tokio::test]
    async fn test_nonce_too_high_recovery() {
        let count = 5;
        let (anvil, mock_contract, producer, _valkey, _handle) = setup_nonce_test(count).await;
        let sequencer_address = test_account1().signer.address();

        // Get current block number before rollback
        let current_block = anvil.provider.get_block_number().await.unwrap();
        info!("Current block before rollback: {}", current_block);

        let nonce = anvil.provider.get_transaction_count(sequencer_address).await.unwrap();
        info!("Sequencer nonce: {}", nonce);

        // Rollback anvil to a previous state (this should cause nonce mismatch)
        let rollback_depth = 3;
        anvil.provider.anvil_rollback(Some(rollback_depth)).await.unwrap();
        assert_eq!(
            anvil.provider.get_block_number().await.unwrap(),
            current_block - rollback_depth
        );
        assert_eq!(
            anvil.provider.get_transaction_count(sequencer_address).await.unwrap(),
            nonce - rollback_depth
        );
        info!("Rolled back to block: {}", current_block - 1);
        assert_eq!(mock_contract.count().call().await.unwrap(), U256::from(count - rollback_depth));

        info!("count: {}", mock_contract.count().call().await.unwrap()); // TODO REMOVE

        // Add more transactions after rollback
        let test_data = b"transaction 11".to_vec();
        producer.enqueue_transaction(&test_data).await.unwrap();

        tokio::time::sleep(Duration::from_secs(10)).await;
        info!("count: {}", mock_contract.count().call().await.unwrap()); // TODO REMOVE

        wait_until!(
            mock_contract.count().call().await.unwrap() == U256::from(count - rollback_depth + 1),
            Duration::from_secs(5)
        );

        assert_eq!(
            mock_contract.data(U256::from(count - rollback_depth)).call().await.unwrap(),
            test_data
        );
    }

    async fn setup_nonce_test(
        count: u64,
    ) -> (
        ChainInfo,
        MockSequencingContract::MockSequencingContractInstance<FilledProvider>,
        StreamProducer,
        E2EProcess,
        JoinHandle<()>,
    ) {
        let mut config = test_config();
        config.compression_enabled = false;
        config.wait_for_receipt = true;

        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();

        let anvil = start_anvil(config.chain_id).await.unwrap();
        anvil.provider.anvil_set_auto_mine(true).await.unwrap();
        config.sequencing_rpc_urls = vec![Url::parse(&anvil.http_url).unwrap()];

        // Deploy the mock contract
        let mock_contract = MockSequencingContract::deploy(anvil.provider.clone()).await.unwrap();
        config.sequencing_address = *mock_contract.address();

        let producer = StreamProducer::new(
            conn.clone(),
            config.chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
        )
        .await;

        config.private_key = test_account1().private_key.to_string();

        let _handle = run_batcher(config).await.unwrap();

        // Send some initial transactions
        for i in 0..count {
            let test_data = format!("transaction {}", i).as_bytes().to_vec();
            producer.enqueue_transaction(&test_data).await.unwrap();
            wait_until!(
                mock_contract.count().call().await.unwrap() == U256::from(i + 1),
                Duration::from_secs(2)
            );
        }

        assert_eq!(mock_contract.count().call().await.unwrap(), U256::from(count));

        (anvil, mock_contract, producer, _valkey, _handle)
    }
}
