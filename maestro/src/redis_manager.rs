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
const CONSUMER_GROUP: &str = "maestro-processors";
const CONSUMER_NAME: &str = "processor";
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

/// Redis Streams manager
#[derive(Debug)]
pub struct StreamManager {
    conn: MultiplexedConnection,
}

impl StreamManager {
    /// Create a new [`StreamManager` ]with the provided Redis connection
    pub const fn new(conn: MultiplexedConnection) -> Self {
        Self { conn }
    }

    /// Initialize stream consumer group
    pub async fn init_consumer_group(&mut self) -> Result<(), Error> {
        // Create consumer group if it doesn't exist
        // XGROUP CREATE key groupname id [MKSTREAM]
        let result: RedisResult<String> = redis::cmd("XGROUP")
            .arg("CREATE")
            .arg(TX_STREAM_KEY)
            .arg(CONSUMER_GROUP)
            .arg("$") // Start processing from new messages only
            .arg("MKSTREAM") // Create stream if it doesn't exist
            .query_async(&mut self.conn)
            .await;

        match result {
            Ok(_) => {
                info!("Created consumer group: {}", CONSUMER_GROUP);
                Ok(())
            }
            Err(e) => {
                // BUSYGROUP error means the group already exists, which is fine
                if e.to_string().contains("BUSYGROUP") {
                    info!("Consumer group already exists: {}", CONSUMER_GROUP);
                    Ok(())
                } else {
                    error!("Failed to create consumer group: {}", e);
                    Err(e.into())
                }
            }
        }
    }

    /// Enqueue a transaction request to the stream
    pub async fn enqueue_transaction(
        &mut self,
        tx_req: TransactionRequest,
    ) -> Result<String, Error> {
        // Serialize the transaction request
        let serialized = serde_json::to_string(&tx_req)?;

        // Add to stream with XADD
        // XADD key * field value [field value ...]
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

    /// Start a background task to process messages from the stream
    pub async fn start_processing_loop(&mut self) -> Result<(), Error> {
        // Initialize consumer group
        self.init_consumer_group().await?;

        info!("Starting Redis Streams processing loop");

        loop {
            match self.process_batch().await {
                Ok(count) => {
                    if count > 0 {
                        debug!("Processed {} transaction(s) from stream", count);
                    }
                }
                Err(e) => {
                    error!("Error processing stream batch: {}", e);
                }
            }

            // Small delay to avoid tight loop if there are persistent errors
            sleep(Duration::from_millis(100)).await;
        }
    }

    /// Process a batch of messages from the stream
    async fn process_batch(&mut self) -> Result<usize, Error> {
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
    ) -> Result<Vec<(String, HashMap<String, String>)>, Error> {
        // XPENDING to get info about pending messages
        let pending: Value = redis::cmd("XPENDING")
            .arg(TX_STREAM_KEY)
            .arg(CONSUMER_GROUP)
            .arg("-") // Start with any ID
            .arg("+") // End with any ID
            .arg(BATCH_SIZE) // Count
            .query_async(&mut self.conn)
            .await?;

        let pending_ids = match pending {
            Value::Array(items) => items
                .iter()
                .filter_map(|item| {
                    if let Value::Array(entry) = item {
                        if let Some(Value::BulkString(id_bytes)) = entry.first() {
                            if let Ok(id) = String::from_utf8(id_bytes.clone()) {
                                return Some(id);
                            }
                        }
                    }
                    None
                })
                .collect::<Vec<String>>(),
            _ => vec![],
        };

        if pending_ids.is_empty() {
            return Ok(vec![]);
        }

        // XCLAIM to claim these pending messages
        let claimed: Vec<(String, HashMap<String, String>)> = redis::cmd("XCLAIM")
            .arg(TX_STREAM_KEY)
            .arg(CONSUMER_GROUP)
            .arg(CONSUMER_NAME)
            .arg(0) // Min idle time (0 = claim immediately)
            .arg(pending_ids)
            .query_async(&mut self.conn)
            .await?;

        if !claimed.is_empty() {
            debug!("Claimed {} pending message(s)", claimed.len());
        }

        Ok(claimed)
    }

    /// Read new messages from the stream
    async fn read_new_messages(&mut self) -> Result<Vec<(String, HashMap<String, String>)>, Error> {
        // XREADGROUP to read new messages
        // Format: XREADGROUP GROUP group consumer [COUNT count] [BLOCK milliseconds] STREAMS key
        // [key ...] > [ID ...]
        let streams: Vec<(String, HashMap<String, String>)> = redis::cmd("XREADGROUP")
            .arg("GROUP")
            .arg(CONSUMER_GROUP)
            .arg(CONSUMER_NAME)
            .arg("COUNT")
            .arg(BATCH_SIZE)
            .arg("BLOCK")
            .arg(BLOCK_MS)
            .arg("STREAMS")
            .arg(TX_STREAM_KEY)
            .arg(">") // Only new messages
            .query_async(&mut self.conn)
            .await?;

        Ok(streams)
    }

    /// Process the received messages
    async fn process_messages(
        &mut self,
        messages: Vec<(String, HashMap<String, String>)>,
    ) -> Result<usize, Error> {
        if messages.is_empty() {
            return Ok(0);
        }

        let mut processed = 0;

        for (id, data) in messages {
            if let Some(tx_data) = data.get("data") {
                match self.process_transaction(tx_data).await {
                    Ok(_) => {
                        // Acknowledge successful processing
                        self.acknowledge_message(&id).await?;
                        processed += 1;
                    }
                    Err(e) => {
                        warn!("Failed to process transaction {}: {}", id, e);
                        // Note: we don't acknowledge, so it will be redelivered as pending
                    }
                }
            }
        }

        Ok(processed)
    }

    /// Process a single transaction
    async fn process_transaction(&self, tx_data: &str) -> Result<(), Error> {
        // Deserialize the transaction
        let tx_req: TransactionRequest = serde_json::from_str(tx_data)?;

        // TODO: Implement your actual transaction processing logic here
        info!("Processing transaction: {} from sender {}", tx_req.tx_hash, tx_req.sender);

        // Simulate some processing work
        sleep(Duration::from_millis(50)).await;

        Ok(())
    }

    /// Acknowledge a message as processed
    async fn acknowledge_message(&mut self, id: &str) -> Result<(), Error> {
        // XACK to acknowledge message
        let result: i32 = self.conn.xack(TX_STREAM_KEY, CONSUMER_GROUP, &[id]).await?;

        if result != 1 {
            warn!("Failed to acknowledge message {}: unexpected result {}", id, result);
        }

        Ok(())
    }
}
