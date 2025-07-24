//! This module provides the consumer implementation for Valkey Streams used to queue
//! and process transactions across different chains.

use crate::{
    valkey::{streams::producer::tx_stream_key, valkey_metrics::ValkeyMetrics},
    with_cache_metrics,
};
use alloy::primitives::ChainId;
use derivative::Derivative;
use redis::{
    aio::ConnectionManager,
    streams::{StreamReadOptions, StreamReadReply},
    AsyncCommands,
};
use shared::tracing::{otel_global, OpenTelemetrySpanExt, SpanKind, TraceContextExt};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tracing::{info_span, instrument, Span};

/// A consumer for Valkey Streams that reads transaction data.
///
/// This consumer reads from a Valkey Stream using the XREAD command, maintaining its position
/// in the stream using the last processed message ID. It's designed to work with transaction
/// data that has been enqueued by a `StreamProducer`.
///
/// # Example
/// ```no_compile
/// use valkey::aio::ConnectionManager;
/// use synd-maestro::valkey::consumer::StreamConsumer;
///
/// let conn: ConnectionManager = // ... get Valkey connection
/// let chain_id = 1;
/// let start_from_id = "0-0";
/// let mut consumer = StreamConsumer::new(conn, chain_id, start_from_id);
///
/// // Read next transaction
/// if let Some((tx_data, msg_id)) = consumer.recv().await? {
///     // Process transaction data
/// }
/// ```
#[derive(Derivative)]
#[derivative(Debug)]
pub struct StreamConsumer {
    #[derivative(Debug = "ignore")]
    conn: ConnectionManager,
    stream_key: String,
    last_id: String,
    valkey_metrics: Arc<ValkeyMetrics>,
}

impl StreamConsumer {
    /// Creates a new `StreamConsumer` for a specific chain.
    ///
    /// # Arguments
    /// * `conn` - A Valkey multiplexed connection to use for reading from the stream
    /// * `chain_id` - The ID of the chain to consume transactions for
    /// * `start_from_id` - The ID of the message to start reading from ("0-0" to start from the
    ///   beginning)
    /// # Returns
    /// A new `StreamConsumer` instance configured to read from the stream for the given chain.
    pub fn new(
        conn: ConnectionManager,
        chain_id: ChainId,
        start_from_id: String,
        valkey_metrics: Arc<ValkeyMetrics>,
    ) -> Self {
        // NOTE maybe we don't need ConnectionManager here (unless we want to have multiple
        // consumers per service)
        Self { conn, stream_key: tx_stream_key(chain_id), last_id: start_from_id, valkey_metrics }
    }

    /// Receives the next transaction from the stream.
    ///
    /// This method reads one message at a time and updates the internal `last_id` to track the
    /// position in the stream.
    ///
    /// # Arguments
    /// * `number_of_messages` - The number of messages to read from the stream
    ///
    /// # Returns
    /// * `Ok(Some((Vec<u8>, String)))` - The raw transaction data and the last ID if a new message
    ///   was received
    /// * `Ok([])` - If no new messages are available
    /// * `Err(_)` - If there was an error reading from the stream or parsing the message
    ///
    /// # Notes
    /// * Currently reads one message at a time for simplicity. For high-throughput scenarios,
    ///   consider implementing batch reading with an internal buffer.
    /// * Blocks for `block_duration` waiting for a new message to arrive
    /// * For automatic cache metrics, wrap calls to this method with `with_cache_metrics!` macro
    #[instrument(
        name = "redis_receive",
        skip(self),
        err,
        fields(
            otel.kind = ?SpanKind::Consumer,
        )
    )]
    pub async fn recv(
        &mut self,
        max_msg_count: usize,
        block_duration: Duration,
    ) -> eyre::Result<Vec<(Vec<u8>, String)>, StreamError> {
        let opts = StreamReadOptions::default()
            .block(block_duration.as_millis() as usize)
            .count(max_msg_count);

        let reply: Option<StreamReadReply> = with_cache_metrics!(
            self.valkey_metrics,
            self.conn.xread_options(&[self.stream_key.as_str()], &[self.last_id.as_str()], &opts)
        )?;

        let reply = match reply {
            Some(r) => r,
            None => return Ok(vec![]),
        };

        let key_data = reply.keys.first().ok_or_else(|| {
            StreamError::Parse("Expected at least one stream key in reply".into())
        })?;

        if key_data.ids.is_empty() {
            return Ok(vec![]);
        }

        let mut results = Vec::with_capacity(key_data.ids.len());

        for id in &key_data.ids {
            let traceparent = id
                .map
                .get("traceparent")
                .ok_or_else(|| StreamError::Parse("No traceparent found in message".into()))?;
            let producer_context = match traceparent {
                redis::Value::BulkString(data) => {
                    let carrier: HashMap<String, String> =
                        [("traceparent".to_string(), String::from_utf8_lossy(data).to_string())]
                            .into();
                    otel_global::get_text_map_propagator(|propagator| propagator.extract(&carrier))
                }
                _ => {
                    return Err(StreamError::Parse(
                        "Expected binary data, got different type".into(),
                    ))
                }
            };
            let producer_span = producer_context.span();
            let span = info_span!(
                parent: Span::current(),
                "redis_recv_transaction",
                otel.kind = ?SpanKind::Consumer,
            );
            span.add_link(producer_span.span_context().clone());
            let _guard = span.enter();

            let raw_tx = id.map.get("data").ok_or(StreamError::NoData)?;

            match raw_tx {
                redis::Value::BulkString(data) => {
                    results.push((data.clone(), id.id.clone()));
                }
                _ => {
                    return Err(StreamError::Parse(
                        "Expected binary data, got different type".into(),
                    ))
                }
            }
        }

        if let Some(last_id) = reply.keys[0].ids.last() {
            self.last_id = last_id.id.clone();
        }

        Ok(results)
    }
}

/// Errors that can occur when reading from the cache stream.
#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    /// Cache error
    #[error("Cache error: {0}")]
    Cache(#[from] redis::RedisError),
    /// Stream parsing error
    #[error("Stream parsing error: {0}")]
    Parse(String),
    /// No data found in the message
    #[error("No data found in message")]
    NoData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::valkey::streams::producer::{CheckFinalizationResult, StreamProducer};
    use prometheus_client::registry::Registry;
    use std::{sync::Arc, time::Duration};
    use test_utils::docker::start_valkey;

    #[tokio::test]
    async fn test_produce_consume_transaction() {
        // Start Valkey container
        let (_valkey, valkey_url) = start_valkey().await.unwrap();

        let mut registry = Registry::default();
        let valkey_metrics = Arc::new(ValkeyMetrics::new(&mut registry));

        // Connect to Valkey
        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();

        // Setup test data
        let chain_id = 1;
        let test_data = b"test transaction data".to_vec();

        // Create producer and consumer
        let producer = StreamProducer::new(
            conn.clone(),
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            valkey_metrics.clone(),
        )
        .await;
        let mut consumer =
            StreamConsumer::new(conn, chain_id, "0-0".to_string(), valkey_metrics.clone());

        // Send transaction
        producer.enqueue_transaction(&test_data).await.unwrap();

        // Receive and verify
        let received = consumer.recv(1, Duration::from_secs(1)).await.unwrap();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].0, test_data);
        assert!(!received[0].1.is_empty());
        tokio::time::sleep(Duration::from_secs(30)).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_produce_consume_multiple_transactions() {
        // Start Valkey container
        let (_valkey, valkey_url) = start_valkey().await.unwrap();

        let mut registry = Registry::default();
        let valkey_metrics = Arc::new(ValkeyMetrics::new(&mut registry));

        // Connect to Valkey
        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();

        // Setup test data
        let chain_id = 1;
        let test_data1 = b"test transaction data 1".to_vec();
        let test_data2 = b"test transaction data 2".to_vec();

        // Create producer and consumer
        let producer = StreamProducer::new(
            conn.clone(),
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            valkey_metrics.clone(),
        )
        .await;
        let mut consumer =
            StreamConsumer::new(conn, chain_id, "0-0".to_string(), valkey_metrics.clone());

        // Send transactions
        producer.enqueue_transaction(&test_data1).await.unwrap();
        producer.enqueue_transaction(&test_data2).await.unwrap();

        // Receive and verify
        let received = consumer.recv(2, Duration::from_secs(1)).await.unwrap();
        assert_eq!(received.len(), 2);
        assert_eq!(received[0].0, test_data1);
        assert_eq!(received[1].0, test_data2);
        assert!(!received[0].1.is_empty());
        assert!(!received[1].1.is_empty());
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
