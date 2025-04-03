//! The `redis_manager` module is for connecting to `redis` instances and sending messages

use crate::{config::Config, errors::Error};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client, RedisResult, Value};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

/// Connect to a Redis instance
pub async fn connect(config: Config) -> eyre::Result<(Client, MultiplexedConnection), Error> {
    info!("Connecting to Redis server at {}...", config.redis_address);
    let client = Client::open(config.redis_address)?;
    info!("Got Redis client");
    let mut conn = client.get_multiplexed_async_connection().await?;
    info!("Got Redis connection");

    // Test an operation on Redis
    conn.set::<&str, &str, ()>("test_key", "test_value").await?;

    let value: String = conn.get("test_key").await?;

    info!("Redis test value set: {}", value);
    Ok((client, conn))
}

// Stream constants
const TX_STREAM_KEY: &str = "maestro:transactions";
const DEFAULT_GROUP: &str = "maestro-processors";
const BLOCK_MS: usize = 2000; // How long to block waiting for new messages
const BATCH_SIZE: usize = 10; // How many messages to process at once
const _MAX_RETRIES: usize = 3; // Number of retries for failed processing

/// Message structure for the transaction stream
#[derive(Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct TransactionRequest {
    pub tx_hash: String,
    pub sender: String,
    pub raw_tx: String,
    pub timestamp: u64,
}

/// Redis Streams manager for stream configuration and setup
#[derive(Debug)]
pub struct StreamManager {
    conn: MultiplexedConnection,
}

impl StreamManager {
    /// Create a new StreamManager with the provided Redis connection
    pub const fn new(conn: MultiplexedConnection) -> Self {
        Self { conn }
    }

    /// Initialize stream consumer group
    pub async fn init_consumer_group(&mut self) -> Result<(), Error> {
        self.init_consumer_group_with_name(DEFAULT_GROUP).await
    }

    /// Initialize stream consumer group with a specific name
    pub async fn init_consumer_group_with_name(&mut self, group_name: &str) -> Result<(), Error> {
        // Create consumer group if it doesn't exist
        let result: RedisResult<String> =
            self.conn.xgroup_create_mkstream(TX_STREAM_KEY, group_name, "$").await;

        match result {
            Ok(_) => {
                info!("Created consumer group: {}", group_name);
                Ok(())
            }
            Err(e) => {
                // BUSYGROUP error means the group already exists, which is fine
                if e.to_string().contains("BUSYGROUP") {
                    info!("Consumer group already exists: {}", group_name);
                    Ok(())
                } else {
                    error!("Failed to create consumer group: {}", e);
                    Err(e.into())
                }
            }
        }
    }

    /// Create a producer for this stream
    pub fn create_producer(&self) -> Result<StreamProducer, Error> {
        let conn = self.conn.clone();
        Ok(StreamProducer::new(conn))
    }

    /// Create a consumer with a specific ID using the default group
    pub fn create_consumer(&self, id: usize) -> Result<StreamConsumer, Error> {
        self.create_consumer_with_group(DEFAULT_GROUP, id)
    }

    /// Create a consumer with a specific ID and group
    pub fn create_consumer_with_group(
        &self,
        group_name: &str,
        id: usize,
    ) -> Result<StreamConsumer, Error> {
        let conn = self.conn.clone();
        Ok(StreamConsumer::new(conn, group_name.to_string(), format!("consumer-{}", id)))
    }
}

/// Redis Stream producer for enqueueing transactions
#[derive(Debug)]
pub struct StreamProducer {
    conn: MultiplexedConnection,
}

impl StreamProducer {
    /// Create a new producer with the provided Redis connection
    pub fn new(conn: MultiplexedConnection) -> Self {
        Self { conn }
    }

    /// Enqueue a transaction request to the stream
    pub async fn enqueue_transaction(
        &mut self,
        tx_req: TransactionRequest,
    ) -> Result<String, Error> {
        // Serialize the transaction request
        let serialized = serde_json::to_string(&tx_req)?;

        // Add to stream with XADD
        let id: String = self
            .conn
            .xadd(
                TX_STREAM_KEY,
                "*", // Auto-generate ID
                &[("data", serialized)],
            )
            .await?;

        debug!("Enqueued transaction with ID: {}", id);
        Ok(id)
    }
}

/// Redis Stream consumer for processing transactions
#[derive(Debug)]
pub struct StreamConsumer {
    conn: MultiplexedConnection,
    group_name: String,
    consumer_name: String,
}

impl StreamConsumer {
    /// Create a new consumer with a specific group and consumer name
    pub fn new(conn: MultiplexedConnection, group_name: String, consumer_name: String) -> Self {
        Self { conn, group_name, consumer_name }
    }

    /// Start a background task to process messages from the stream
    pub async fn start_processing_loop(&mut self) -> Result<(), Error> {
        info!("Starting Redis Streams processing loop for consumer {}", self.consumer_name);

        loop {
            match self.process_batch().await {
                Ok(count) => {
                    if count > 0 {
                        debug!(
                            "Consumer {} processed {} transaction(s) from stream",
                            self.consumer_name, count
                        );
                    }
                }
                Err(e) => {
                    error!(
                        "Error processing stream batch by consumer {}: {}",
                        self.consumer_name, e
                    );
                }
            }

            // Small delay to avoid tight loop if there are persistent errors
            sleep(Duration::from_millis(100)).await;
        }
    }

    /// Process a batch of messages from the stream
    pub async fn process_batch(&mut self) -> Result<usize, Error> {
        // Read pending messages first (messages that were delivered but not acknowledged)
        let pending_msgs = self.read_pending_messages().await?;

        if !pending_msgs.is_empty() {
            return self.process_messages(pending_msgs).await;
        }

        // If no pending messages, read new ones
        let new_msgs = self.read_new_messages().await?;
        self.process_messages(new_msgs).await
    }

    /// Read pending messages (delivered but not acknowledged)
    async fn read_pending_messages(
        &mut self,
    ) -> Result<Vec<(String, HashMap<String, Value>)>, Error> {
        // Get all pending message IDs
        let pending_info: redis::streams::StreamPendingCountReply =
            self.conn.xpending_count(TX_STREAM_KEY, &self.group_name, "-", "+", BATCH_SIZE).await?;

        if pending_info.ids.is_empty() {
            return Ok(vec![]);
        }

        // Extract the message IDs
        let pending_ids: Vec<String> =
            pending_info.ids.iter().map(|item| item.id.clone()).collect();

        // Claim these pending messages using XCLAIM
        let claimed: Vec<(String, HashMap<String, Value>)> = self
            .conn
            .xclaim(
                TX_STREAM_KEY,
                &self.group_name,
                &self.consumer_name,
                0, // Min idle time (claim immediately)
                &pending_ids,
            )
            .await?;

        if !claimed.is_empty() {
            debug!("Consumer {} claimed {} pending message(s)", self.consumer_name, claimed.len());
        }

        Ok(claimed)
    }

    /// Read new messages from the stream
    async fn read_new_messages(&mut self) -> Result<Vec<(String, HashMap<String, Value>)>, Error> {
        // Use StreamReadOptions to read from the group
        let options = redis::streams::StreamReadOptions::default()
            .group(&self.group_name, &self.consumer_name)
            .count(BATCH_SIZE)
            .block(BLOCK_MS);

        // Use XREADGROUP to read new messages
        let result: redis::streams::StreamReadReply =
            self.conn.xread_options(&[TX_STREAM_KEY], &[">"], &options).await?;

        // Convert the StreamReadReply to the expected format
        let mut messages = Vec::new();
        for stream_key in result.keys {
            for message in stream_key.ids {
                messages.push((message.id, message.map));
            }
        }

        Ok(messages)
    }

    /// Process the received messages
    async fn process_messages(
        &mut self,
        messages: Vec<(String, HashMap<String, Value>)>,
    ) -> Result<usize, Error> {
        if messages.is_empty() {
            return Ok(0);
        }

        let mut processed = 0;

        for (id, data) in messages {
            info!("Consumer {} processing message {}", self.consumer_name, id);
            if let Some(tx_data) = data.get("data") {
                match self.process_transaction(tx_data).await {
                    Ok(_) => {
                        // Acknowledge successful processing
                        self.acknowledge_message(&id).await?;
                        info!("Consumer {} acknowledged message {}", self.consumer_name, id);
                        processed += 1;
                    }
                    Err(e) => {
                        warn!(
                            "Consumer {} failed to process transaction {}: {}",
                            self.consumer_name, id, e
                        );
                        // Note: we don't acknowledge, so it will be redelivered as pending
                    }
                }
            }
        }

        Ok(processed)
    }

    /// Process a single transaction
    async fn process_transaction(&self, tx_data: &Value) -> Result<(), Error> {
        // Deserialize the transaction
        let json_string = match tx_data {
            Value::BulkString(bytes) => std::str::from_utf8(bytes)?.to_string(),
            Value::SimpleString(s) => s.clone(),
            Value::Int(i) => i.to_string(),
            Value::Double(d) => d.to_string(),
            _ => {
                return Err(Error::Redis(redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Expected string value for transaction data",
                ))))
            }
        };

        let tx_req: TransactionRequest = serde_json::from_str(json_string.as_str())?;

        // TODO: Implement txn processing logic here
        info!("Processing transaction: {} from sender {}", tx_req.tx_hash, tx_req.sender);

        // Simulate some processing work
        sleep(Duration::from_millis(50)).await;

        Ok(())
    }

    /// Acknowledge a message as processed
    async fn acknowledge_message(&mut self, id: &str) -> Result<(), Error> {
        // Use XACK to acknowledge message
        let result: i32 = self.conn.xack(TX_STREAM_KEY, &self.group_name, &[id]).await?;

        if result != 1 {
            warn!(
                "Consumer {} failed to acknowledge message {}: unexpected result {}",
                self.consumer_name, id, result
            );
        }

        Ok(())
    }
}
