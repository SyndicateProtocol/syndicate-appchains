use crate::connectors::anvil::MetaChainProvider;
use crate::rollups::translator::Translator;
use alloy_primitives::Address;
use alloy_provider::{ext::AnvilApi, Provider, ProviderBuilder};
use alloy_rpc_types::{Block, Transaction, TransactionInput, TransactionRequest};
use std::{str::FromStr, sync::Arc};
use tracing::info;

/// Translates blocks from L1 and L2 chains into a list of transactions for Arbitrum
pub struct ArbTranslator {
    bridge_address: Address,
    meta_chain: Arc<MetaChainProvider>,
}

impl Translator for ArbTranslator {
    fn translate_blocks(
        &self,
        settlement_blocks: Vec<Block>,
        sequence_blocks: Vec<Block>,
    ) -> Vec<Transaction> {
        info!("Translating blocks for Arbitrum");
        info!("Settlement blocks: {:?}", settlement_blocks);
        info!("Sequence blocks: {:?}", sequence_blocks);

        // TODO: Implement Arbitrum-specific block translation logic
        Vec::new()
    }
}

impl ArbTranslator {
    pub fn new(meta_chain: Arc<MetaChainProvider>) -> Self {
        Self {
            bridge_address: Address::from_str("0x063D87A885a9323831A688645647eD7d0e859C5d")
                .unwrap(),
            meta_chain,
        }
    }

    pub async fn init_rollup(&self) -> eyre::Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_provider(self.meta_chain.base_provider.as_ref().unwrap());

        // TODO: deploy contracts

        // Impersonate batcher account
        provider
            .anvil_impersonate_account(self.bridge_address)
            .await?;

        Ok(())
    }

    pub async fn send_batcher_transaction(&self) -> eyre::Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_provider(self.meta_chain.base_provider.as_ref().unwrap());

        let transaction = TransactionRequest::default()
            .to(self.bridge_address)
            .from(self.bridge_address)
            .input(TransactionInput::from(vec![0x00]));

        let _receipt = provider.send_transaction(transaction).await?;

        Ok(())
    }
}
