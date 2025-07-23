//! Integration tests for the synd-chain-ingestor service

use common::types::SequencingBlock;
use shared::types::{BlockBuilder, PartialBlock};
use std::{sync::Arc, time::Duration};
use synd_chain_ingestor::{
    client::{BlockStreamT, IngestorProvider, Provider},
    eth_client::EthClient,
};
use test_framework::components::chain_ingestor::ChainIngestorConfig;
use test_utils::{
    anvil::{mine_block, start_anvil},
    chain_info::ChainInfo,
    docker::{start_component, E2EProcess},
    port_manager::PortManager,
    utils::test_path,
    wait_until,
};
use tokio::time::sleep;
use tracing::info;

mod tests {
    use super::*;

    struct MockBlockBuilder;

    impl BlockBuilder<SequencingBlock> for MockBlockBuilder {
        fn build_block(&self, block: &PartialBlock) -> eyre::Result<SequencingBlock> {
            Ok(SequencingBlock {
                block_ref: block.block_ref.clone(),
                parent_hash: block.parent_hash,
                ..Default::default()
            })
        }
    }

    #[ctor::ctor]
    fn init() {
        shared::tracing::setup_global_logging();
    }

    #[allow(clippy::cognitive_complexity)]
    async fn setup(
        chain_rpc_url: Option<(String, u64)>,
    ) -> eyre::Result<(Option<ChainInfo>, E2EProcess, String)> {
        let mut chain_info = None;
        let ws_url;
        let start_block;
        match chain_rpc_url {
            Some((url, sb)) => {
                info!("Using chain RPC URL: {}", url);
                ws_url = url;
                start_block = sb;
            }
            None => {
                info!("Starting anvil...");
                let anvil = start_anvil(111111111).await?;
                ws_url = anvil.ws_url.clone();
                chain_info = Some(anvil);
                start_block = 0;
            }
        }

        info!("Starting chain ingestors...");
        let temp = test_path("chain_ingestor");
        let seq_chain_ingestor_cfg = ChainIngestorConfig {
            ws_urls: vec![ws_url],
            db_file: temp.clone() + "/sequencing_chain.db",
            start_block,
            port: PortManager::instance().next_port().await,
            metrics_port: PortManager::instance().next_port().await,
        };
        info!("seq_chain_ingestor_cfg: {:?}", seq_chain_ingestor_cfg);
        let sequencing_chain_ingestor = start_component(
            "synd-chain-ingestor",
            seq_chain_ingestor_cfg.port,
            seq_chain_ingestor_cfg.cli_args(),
            Default::default(),
        )
        .await?;

        let ingestor_ws_url = format!("ws://localhost:{}", seq_chain_ingestor_cfg.port);

        Ok((chain_info, sequencing_chain_ingestor, ingestor_ws_url))
    }

    #[tokio::test]
    async fn test_ingestor_start() -> eyre::Result<()> {
        let (_anvil, _ingestor, ingestor_ws_url) = setup(None).await?;

        let client = IngestorProvider::new(&ingestor_ws_url, Duration::from_secs(10)).await;
        sleep(Duration::from_secs(1)).await;

        let block_number = client.get_block_number().await.unwrap();
        assert_eq!(block_number, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_ingestor_get_blocks() -> eyre::Result<()> {
        let loop_count = 20;
        let (anvil, _ingestor, ingestor_ws_url) = setup(None).await?;
        let anvil = anvil.unwrap();

        let client = IngestorProvider::new(&ingestor_ws_url, Duration::from_secs(10)).await;

        for _ in 0..loop_count {
            mine_block(&anvil.provider, 10).await?;
        }
        wait_until!(
            client.get_block_number().await.unwrap() == loop_count,
            Duration::from_secs(10)
        );

        let eth_client = EthClient::new(
            vec![anvil.ws_url.clone()],
            Duration::from_secs(10),
            Duration::from_secs(10),
            100,
            Duration::from_secs(1),
        )
        .await;

        let mut block_stream =
            client.get_blocks(5, vec![], Arc::new(MockBlockBuilder), eth_client).await?;

        for _ in 0..10 {
            mine_block(&anvil.provider, 10).await?;
        }
        wait_until!(
            client.get_block_number().await.unwrap() == loop_count + 10,
            Duration::from_secs(10)
        );

        let block_number = client.get_block_number().await.unwrap();
        assert_eq!(block_number, loop_count + 10);

        for i in 5..=loop_count + 10 {
            let block = block_stream.recv(0).await?;
            assert_eq!(block.block_ref.number, i);
        }

        Ok(())
    }
}
