//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a
//! consumer using a channel

use crate::{config::ChainIngestorConfig, metrics::IngestorMetrics, run_http, run_subscription};
use alloy::{
    primitives::{Address, B256},
    transports::{RpcError, TransportErrorKind},
};
use common::{
    eth_client::Client,
    types::{BlockRef, Chain, PartialBlock},
};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{
    mpsc::{error::SendError, Sender},
    oneshot,
};
use tracing::trace;

/// Runs the ingestor component for a given chain.
/// it will use the correct client based on the URL scheme
pub async fn run(
    chain: Chain,
    config: &ChainIngestorConfig,
    addresses: Vec<Address>,
    client: &Client,
    sender: Sender<Arc<PartialBlock>>,
    metrics: IngestorMetrics,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), IngestorError> {
    match client {
        Client::Http(client) => {
            run_http(chain, config, addresses, client.clone(), sender, metrics, shutdown_rx).await
        }
        Client::Subscription(client) => {
            run_subscription(chain, config, addresses, client.clone(), sender, metrics, shutdown_rx)
                .await
        }
    }
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, Error)]
pub enum IngestorError {
    #[error(
        "{chain} chain reorg detected. Current: #{current_block}, Received: #{received_block}, Received parent hash: #{received_parent_hash}"
    )]
    ReorgDetected {
        chain: Chain,
        current_block: Box<BlockRef>,
        received_block: Box<BlockRef>,
        received_parent_hash: B256,
    },

    #[error("Failed to send slot through channel: {0}")]
    Send(#[from] SendError<Arc<PartialBlock>>),

    #[error("Failed to get initial chain head: {0}")]
    GetInitialChainHead(#[from] RpcError<TransportErrorKind>),

    #[error("{resource} not yet available")]
    ResourceNotAvailable { resource: String },

    #[error("Block number mismatch: current={current}, got={received}")]
    BlockNumberMismatch { current: u64, received: u64 },
}

/// Checks if the next block is a valid continuation of the current chain, will return an error if a
/// reorg has taken place
pub fn check_reorg(
    chain: Chain,
    current_block: &BlockRef,
    next_block: &PartialBlock,
) -> Result<(), IngestorError> {
    if next_block.number != current_block.number + 1 ||
        next_block.parent_hash != current_block.hash ||
        next_block.timestamp < current_block.timestamp
    {
        return Err(IngestorError::ReorgDetected {
            chain,
            current_block: Box::new(current_block.clone()),
            received_block: Box::new(BlockRef::new(next_block)),
            received_parent_hash: next_block.parent_hash,
        });
    }
    Ok(())
}

/// Process and send a block, handling common logic between HTTP and subscription ingestors.
///
/// This function:
/// 1. Checks for chain reorganizations
/// 2. Updates the latest processed block reference
/// 3. Records metrics
/// 4. Sends the block through the provided channel
///
/// Returns an error if a chain reorganization is detected or if sending fails.
pub async fn process_and_send_block(
    sender: &Sender<Arc<PartialBlock>>,
    metrics: &IngestorMetrics,
    last_block_sent: &mut Option<BlockRef>,
    block: Arc<PartialBlock>,
    chain: Chain,
) -> Result<(), IngestorError> {
    trace!(%chain, block_number = %block.number, "Processing and sending block");

    // Check for chain reorganizations if we have a previous block
    if let Some(last_block) = last_block_sent {
        check_reorg(chain, last_block, &block)?;
    }

    // Update the reference to the latest processed block
    *last_block_sent = Some(block.clone().into());

    // Record metrics
    metrics.record_last_block_fetched(chain, block.number);

    let block_number = block.number;
    // Send the block
    sender.send(block).await?;

    // Update channel capacity metric
    metrics.update_channel_capacity(chain, sender.capacity());

    trace!(%chain, block_number, "Successfully sent block");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::B256;
    use assert_matches::assert_matches;
    use common::types::{BlockRef, PartialBlock};
    use std::str::FromStr;

    fn create_test_block(number: u64, timestamp: u64) -> PartialBlock {
        PartialBlock {
            hash: B256::from_str(
                "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number,
            parent_hash: B256::from_str(
                "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            timestamp,
            logs: vec![],
        }
    }

    #[test]
    fn test_parent_hash_mismatch_reorg() {
        // Create and set the first block
        let first_block = create_test_block(1, 50);
        let first_block_ref = BlockRef::new(&first_block);

        // Create a second block with correct number (2) but different parent hash
        let second_block = PartialBlock {
            hash: B256::from_str(
                "2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number: 2, // Correct sequential number
            parent_hash: B256::from_str(
                "0000000000abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(), // Different hash than first_block.hash
            timestamp: 60,
            logs: vec![],
        };

        // Check for reorg - should fail due to parent hash mismatch
        let result = check_reorg(Chain::Sequencing, &first_block_ref, &second_block);

        // Verify that a reorg was detected
        assert_matches!(result, Err(IngestorError::ReorgDetected { .. }));
    }

    #[test]
    fn test_reorg_detected_block_number() {
        // Set up first block at number 2
        let first_block = PartialBlock {
            hash: B256::from_str(
                "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number: 2,
            parent_hash: B256::from_str(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            timestamp: 100,
            logs: vec![],
        };
        let first_block_ref = BlockRef::new(&first_block);

        // Create block with block number 1 (lower than current)
        let reorg_block = PartialBlock {
            hash: B256::from_str(
                "2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number: 1, // Lower block number should trigger reorg
            parent_hash: B256::from_str(
                "0000000000abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            timestamp: 50,
            logs: vec![],
        };

        let result = check_reorg(Chain::Sequencing, &first_block_ref, &reorg_block);
        assert_matches!(result, Err(IngestorError::ReorgDetected { .. }));
    }

    #[test]
    fn test_valid_next_block() {
        // Create a current block
        let current_block = PartialBlock {
            hash: B256::from_str(
                "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number: 1,
            parent_hash: B256::from_str(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            timestamp: 100,
            logs: vec![],
        };
        let current_block_ref = BlockRef::new(&current_block);

        // Create a valid next block
        let next_block = PartialBlock {
            hash: B256::from_str(
                "2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number: 2,                       // Higher number
            parent_hash: current_block.hash, // Correct parent hash
            timestamp: 200,
            logs: vec![],
        };

        // Check should pass for both chain types
        let result_seq = check_reorg(Chain::Sequencing, &current_block_ref, &next_block);
        let result_set = check_reorg(Chain::Settlement, &current_block_ref, &next_block);

        assert!(result_seq.is_ok());
        assert!(result_set.is_ok());
    }

    #[test]
    fn test_equal_block_number() {
        // Create a current block
        let current_block = PartialBlock {
            hash: B256::from_str(
                "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number: 5,
            parent_hash: B256::from_str(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            timestamp: 100,
            logs: vec![],
        };
        let current_block_ref = BlockRef::new(&current_block);

        // Create a block with the same number
        let equal_block = PartialBlock {
            hash: B256::from_str(
                "2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number: 5, // Same number should trigger reorg
            parent_hash: B256::from_str(
                "3334567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            timestamp: 105,
            logs: vec![],
        };

        let result = check_reorg(Chain::Sequencing, &current_block_ref, &equal_block);
        assert_matches!(result, Err(IngestorError::ReorgDetected { .. }));
    }
}
