use crate::{config::MetabasedConfig, types::RuntimeError};
use alloy::eips::BlockNumberOrTag;
use block_builder::{
    block_builder::BlockBuilder, connectors::mchain::MetaChainProvider,
    rollups::shared::RollupAdapter,
};
use common::types::{BlockAndReceipts, BlockRef, Chain, KnownState};
use eyre::Result;
use ingestor::{
    config::ChainIngestorConfig,
    eth_client::{EthClient, RPCClient},
    ingestor::Ingestor,
};
use metrics::metrics::{start_metrics, MetricsState, TranslatorMetrics};
use prometheus_client::registry::Registry;
use serde_json::{json, value::Value};
use slotter::Slotter;
use std::sync::Arc;
use tokio::{sync::mpsc::Receiver, task::JoinHandle};

pub async fn clients(
    config: &MetabasedConfig,
) -> Result<(Arc<dyn RPCClient>, Arc<dyn RPCClient>), RuntimeError> {
    let sequencing_client: Arc<dyn RPCClient> = Arc::new(
        EthClient::new(&config.sequencing.sequencing_rpc_url, Chain::Sequencing)
            .await
            .map_err(RuntimeError::RPCClient)?,
    );
    let settlement_client: Arc<dyn RPCClient> = Arc::new(
        EthClient::new(&config.settlement.settlement_rpc_url, Chain::Settlement)
            .await
            .map_err(RuntimeError::RPCClient)?,
    );
    Ok((sequencing_client, settlement_client))
}

/// These extra fields are added to every log event for additional context. Add more as needed
pub fn get_extra_fields_for_logging(base_config: MetabasedConfig) -> Vec<(String, Value)> {
    vec![("chain_id".to_string(), json!(base_config.block_builder.target_chain_id))]
}

pub async fn init_metrics(config: &MetabasedConfig) -> (TranslatorMetrics, JoinHandle<()>) {
    let registry = Registry::default();
    let mut metrics_state = MetricsState { registry };
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    let metrics_task = start_metrics(metrics_state, config.metrics.metrics_port).await;
    (metrics, metrics_task)
}

// TODO(SEQ-628): `init` all components without a channel, `start` all components with required
//channel
pub async fn create_node_components<R: RollupAdapter>(
    config: &MetabasedConfig,
    sequencing_client: Arc<dyn RPCClient>,
    settlement_client: Arc<dyn RPCClient>,
    rollup_adapter: R,
    safe_state: Option<KnownState>,
    metrics: TranslatorMetrics,
) -> Result<
    (
        Ingestor,
        Receiver<Arc<BlockAndReceipts>>,
        Ingestor,
        Receiver<Arc<BlockAndReceipts>>,
        Slotter,
        BlockBuilder<R>,
    ),
    RuntimeError,
> {
    let sequencing_config: ChainIngestorConfig = (&config.sequencing).into();
    let settlement_config: ChainIngestorConfig = (&config.settlement).into();

    // Initialize components
    let (sequencing_ingestor, sequencing_rx) = Ingestor::new(
        Chain::Sequencing,
        sequencing_client,
        &sequencing_config,
        metrics.ingestor_sequencing,
    )
    .await?;

    let (settlement_ingestor, settlement_rx) = Ingestor::new(
        Chain::Settlement,
        settlement_client,
        &settlement_config,
        metrics.ingestor_settlement,
    )
    .await?;

    let (slotter, slot_rx) = Slotter::new(&config.slotter, safe_state, metrics.slotter);
    let block_builder =
        BlockBuilder::new(slot_rx, &config.block_builder, rollup_adapter, metrics.block_builder)
            .await?;
    Ok((
        sequencing_ingestor,
        sequencing_rx,
        settlement_ingestor,
        settlement_rx,
        slotter,
        block_builder,
    ))
}

async fn validate_block_add_timestamp(
    client: &Arc<dyn RPCClient>,
    expected_block: &mut BlockRef,
) -> bool {
    match client.get_block_by_number(BlockNumberOrTag::Number(expected_block.number)).await {
        Ok(block) => {
            expected_block.timestamp = block.timestamp;
            block.hash == expected_block.hash
        }
        Err(_) => false,
    }
}

// TODO (SEQ-651) - re-use this function in case of reorg
pub async fn get_safe_state(
    mchain: &MetaChainProvider,
    sequencing_client: Arc<dyn RPCClient>,
    settlement_client: Arc<dyn RPCClient>,
    rollup_adapter: &impl RollupAdapter,
) -> Result<(Option<KnownState>, Option<u64>)> {
    let mut current_block = BlockNumberOrTag::Latest;
    loop {
        match rollup_adapter.get_processed_blocks(&mchain.provider, current_block).await? {
            Some((mut state, block_number)) => {
                let seq_valid =
                    validate_block_add_timestamp(&sequencing_client, &mut state.sequencing_block)
                        .await;
                let settle_valid =
                    validate_block_add_timestamp(&settlement_client, &mut state.settlement_block)
                        .await;

                if seq_valid && settle_valid {
                    return Ok((Some(state), Some(block_number)));
                }
                current_block = BlockNumberOrTag::Number(block_number.saturating_sub(1));
            }
            None => return Ok((None, None)),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex, B256};
    use async_trait::async_trait;
    use common::types::Block;
    use ingestor::eth_client::RPCClientError;
    use std::fmt::Debug;

    #[derive(Debug, Clone)]
    struct MockRPCClient {
        block_hash: B256,
        timestamp: u64,
    }

    #[async_trait]
    impl RPCClient for MockRPCClient {
        async fn get_block_by_number(&self, _: BlockNumberOrTag) -> Result<Block, RPCClientError> {
            Ok(Block {
                hash: self.block_hash,
                timestamp: self.timestamp,
                number: 1,
                ..Default::default()
            })
        }

        async fn batch_get_blocks_and_receipts(
            &self,
            _block_numbers: Vec<u64>,
        ) -> Result<Vec<BlockAndReceipts>, RPCClientError> {
            unimplemented!("Not needed for this test")
        }
    }

    #[tokio::test]
    async fn test_validate_block() {
        let expected_hash = B256::from_slice(&hex!(
            "1234567890123456789012345678901234567890123456789012345678901234"
        ));
        let expected_timestamp = 12345;

        let mut test_block = BlockRef {
            hash: expected_hash,
            number: 1,
            timestamp: 0, // Initial timestamp
        };

        let client: Arc<dyn RPCClient> =
            Arc::new(MockRPCClient { block_hash: expected_hash, timestamp: expected_timestamp });

        assert!(validate_block_add_timestamp(&client, &mut test_block).await);
        assert_eq!(test_block.timestamp, expected_timestamp);

        // Test mismatch case
        let client_mismatch: Arc<dyn RPCClient> = Arc::new(MockRPCClient {
            block_hash: B256::from_slice(&hex!(
                "4321432143214321432143214321432143214321432143214321432143214321"
            )),
            timestamp: expected_timestamp,
        });

        let mut test_block = BlockRef { hash: expected_hash, number: 1, timestamp: 0 };

        assert!(!validate_block_add_timestamp(&client_mismatch, &mut test_block).await);
    }
}
