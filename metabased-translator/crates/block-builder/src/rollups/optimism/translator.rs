use crate::connectors::anvil::MetaChainProvider;
use crate::rollups::optimism::batch::{new_batcher_tx, Batch};
use crate::rollups::optimism::frame::to_data;
use crate::rollups::translator::Translator;
use alloy::signers::local::PrivateKeySigner;
use alloy_network::EthereumWallet;
use alloy_primitives::{Address, B256, U256};
use alloy_provider::ext::AnvilApi;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::{Block, BlockId, BlockNumberOrTag, BlockTransactionsKind, Transaction};
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

// TODO [SEQ-433]: Correctly implement dynamically constructing the batch
pub struct OPTranslator {
    meta_chain: Arc<MetaChainProvider>,
}

impl Translator for OPTranslator {
    fn translate_blocks(
        &self,
        settlement_blocks: Vec<Block>,
        sequence_blocks: Vec<Block>,
    ) -> Vec<Transaction> {
        info!("Translating blocks for Optimism");
        info!("Settlement blocks: {:?}", settlement_blocks);
        info!("Sequence blocks: {:?}", sequence_blocks);

        // TODO [SEQ-433]: Implement Optimism-specific block translation logic
        Vec::new()
    }
}

impl OPTranslator {
    pub fn new(meta_chain: Arc<MetaChainProvider>) -> Self {
        Self { meta_chain }
    }

    // Copied from generic anvil file to here for now
    pub async fn send_batcher_transaction(&self) -> eyre::Result<()> {
        let signer: PrivateKeySigner =
            "fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d"
                .parse()
                .unwrap();
        let wallet = EthereumWallet::from(signer);

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_provider(self.meta_chain.base_provider.as_ref().unwrap());

        // Set up the batcher and batch inbox
        let batcher = Address::from_str("0x063D87A885a9323831A688645647eD7d0e859C5d")
            .expect("Failed to parse batcher address");
        let batch_inbox = Address::from_str("0x97395dd253e2d096a0caa62a574895c3c2f2b2e0")
            .expect("Failed to parse Batch Inbox address");

        let block = provider
            .get_block(
                BlockId::Number(BlockNumberOrTag::Number(0)),
                BlockTransactionsKind::Hashes,
            )
            .await?
            .expect("Failed to get block");

        info!("Block: {:?}", block);
        let single_batch = Batch {
            parent_hash: B256::from_str(
                "0xe009262cd1adf34cfaf845fd1c17a6ddb7f97c67b2992cd9f286ff4e1c6ad233",
            )
            .unwrap(),
            epoch_num: 0,
            epoch_hash: block.header.hash,
            timestamp: 1712500002,
            transactions: vec![],
        };
        let frames = single_batch.get_frames(1000000).unwrap();
        let data = to_data(&frames).unwrap();

        let tx = new_batcher_tx(batcher, batch_inbox, data.into());
        info!("Transaction: {:?}", tx);

        provider.anvil_set_balance(batcher, U256::MAX).await?;
        let tx_hash = provider.send_transaction(tx).await?;
        info!("Transaction hash: {:?}", tx_hash);

        Ok(())
    }
}
