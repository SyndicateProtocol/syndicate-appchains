//! Slotter module for `synd-translator`

use crate::metrics::SlotterMetrics;
use alloy::primitives::FixedBytes;
use common::types::{Chain, SequencingBlock, SettlementBlock};
use shared::tracing::SpanKind;
use synd_chain_ingestor::client::BlockStreamT;
use synd_mchain::{
    client::Provider,
    db::{ArbitrumBatch, MBlock, Slot},
};
use thiserror::Error;
use tracing::{error, info, instrument, trace};

/// Ingests blocks from the sequencing and settlement chains, slots them into slots, and sends the
/// slots to the slot processor to generate `synd-mchain` blocks.
#[allow(clippy::expect_used)]
#[allow(clippy::cognitive_complexity)]
#[instrument(
    skip_all,
    err,
    fields(otel.kind = ?SpanKind::Internal),
)]
pub async fn run(
    settlement_delay: u64,
    mut sequencing: impl BlockStreamT<SequencingBlock> + Send,
    mut settlement: impl BlockStreamT<SettlementBlock> + Send,
    provider: &impl Provider,
    metrics: &SlotterMetrics,
) -> Result<(), SlotterError> {
    info!("Starting Slotter");

    trace!("Waiting for settlement block");
    let mut set_block = settlement
        .recv(0)
        .await
        .map_err(|e| SlotterError::IngestorError(Chain::Settlement, e.to_string()))?;

    loop {
        trace!("Waiting for sequencing block");
        let seq_block = sequencing
            .recv(0)
            .await
            .map_err(|e| SlotterError::IngestorError(Chain::Sequencing, e.to_string()))?;

        metrics.record_last_processed_block(seq_block.block_ref.number, Chain::Sequencing);
        metrics.update_chain_timestamp_lag(seq_block.block_ref.timestamp, Chain::Sequencing);

        let mut mblock = MBlock {
            timestamp: seq_block.block_ref.timestamp,
            slot: Slot {
                seq_block_number: seq_block.block_ref.number,
                seq_block_hash: seq_block.block_ref.hash,
                set_block_hash: FixedBytes::ZERO,
                set_block_number: 0,
            },
            payload: None,
        };

        let mut messages = vec![];

        let mut blocks_per_slot: u64 = 1;
        let slot_end_ts = if seq_block.block_ref.timestamp >= settlement_delay {
            seq_block.block_ref.timestamp - settlement_delay + 1
        } else {
            Default::default()
        };
        while set_block.block_ref.timestamp < slot_end_ts {
            blocks_per_slot += 1;
            messages.append(&mut set_block.messages);
            set_block = settlement
                .recv(slot_end_ts)
                .await
                .map_err(|e| SlotterError::IngestorError(Chain::Settlement, e.to_string()))?;
            metrics.record_last_processed_block(set_block.block_ref.number, Chain::Sequencing);
            metrics.update_chain_timestamp_lag(set_block.block_ref.timestamp, Chain::Sequencing);
        }

        if seq_block.tx_count > 0 || !messages.is_empty() {
            mblock.payload = Some(ArbitrumBatch::new(seq_block.batch, messages));
        }
        mblock.slot.set_block_hash = set_block.block_ref.hash;
        mblock.slot.set_block_number = set_block.block_ref.number;

        trace!("Processing slot {:?}", mblock.slot);
        let time = std::time::Instant::now();
        provider
            .add_batch(&mblock)
            .await
            .map_err(|e| SlotterError::SlotProcessorError(e.to_string()))?;
        if let Some(payload) = mblock.payload {
            info!(
                "Sent slot {} ({} seq, {} set) with timestamp {} in {:?}",
                mblock.slot.seq_block_number,
                seq_block.tx_count,
                payload.delayed_messages.len(),
                mblock.timestamp,
                time.elapsed()
            );
        }
        metrics.record_blocks_per_slot(blocks_per_slot);
        metrics.record_last_slot(mblock.slot.seq_block_number);
    }
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SlotterError {
    #[error("Slot processor error: {0}")]
    SlotProcessorError(String),

    #[error("{0} chain ingestor error: {1}")]
    IngestorError(Chain, String),
}

#[cfg(test)]
mod tests {
    use crate::{metrics::SlotterMetrics, slotter::run};
    use alloy::primitives::U256;
    use async_trait::async_trait;
    use common::types::{Chain, SettlementBlock};
    use prometheus_client::registry::Registry;
    use shared::types::BlockRef;
    use synd_chain_ingestor::client::BlockStreamT;
    use synd_mchain::client::{DeserializeOwned, Provider, ToRpcParams};

    #[ctor::ctor]
    fn init() {
        shared::tracing::setup_global_logging();
    }

    #[allow(dead_code)]
    struct PanicProvider {}

    #[async_trait]
    impl Provider for PanicProvider {
        async fn request<Params: ToRpcParams + Send, T: DeserializeOwned>(
            &self,
            _method: &'static str,
            _params: Params,
        ) -> Result<T, ClientError> {
            panic!("unexpected call to raw_request");
        }
    }

    struct MockBlockStream<Block>(pub Vec<Block>);

    #[async_trait]
    impl<Block: Send> BlockStreamT<Block> for MockBlockStream<Block> {
        async fn recv(&mut self, timestamp: u64) -> eyre::Result<Block> {
            assert_eq!(timestamp, 0);
            Ok(self.0.pop().unwrap())
        }
    }

    #[tokio::test]
    async fn test_slotter() -> eyre::Result<()> {
        // TODO
        Ok(())
    }
}
