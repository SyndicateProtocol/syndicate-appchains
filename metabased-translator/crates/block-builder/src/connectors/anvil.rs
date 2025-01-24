//! Anvil connector for the `MetaChain`

use crate::{block_builder::BlockBuilderError, config::BlockBuilderConfig, rollups::arbitrum};
use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, U256},
    providers::{
        ext::AnvilApi,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    rpc::types::{anvil::MineOptions, TransactionRequest},
    signers::local::PrivateKeySigner,
    transports::http::Http,
};
use contract_bindings::arbitrum::rollup::Rollup;
use eyre::Result;
use reqwest::Client;
use std::net::TcpListener;
use tracing::info;

type FilledProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

// Anvil is automatically stopped when the `MetaChainProvider` is dropped.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct MetaChainProvider {
    pub anvil: AnvilInstance,
    pub provider: FilledProvider,
    pub rollup: Rollup::RollupInstance<Http<Client>, FilledProvider>,
    pub rollup_info: String,
}

impl MetaChainProvider {
    /// Starts the Anvil instance and creates a provider for the `MetaChain`
    pub async fn start(config: BlockBuilderConfig) -> Result<Self> {
        let port = find_available_port(config.port, 10).ok_or_else(|| {
            BlockBuilderError::AnvilStartError("No available ports found after 10 attempts")
        })?;

        if port != config.port {
            info!("Port {} is in use, switching to port {}", config.port, port);
        }

        let ts = config.genesis_timestamp.to_string();

        let args = vec![
            "--base-fee",
            "0",
            "--gas-limit",
            "30000000",
            "--timestamp",
            ts.as_str(),
            "--no-mining",
        ];

        let anvil = Anvil::new().port(port).chain_id(config.chain_id).args(args).try_spawn()?;

        // TODO (SEQ-515) Move to a config value
        let signer: PrivateKeySigner =
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
                .parse()
                .map_err(|_| BlockBuilderError::AnvilStartError("Failed to parse private key"))?;

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(EthereumWallet::from(signer))
            .on_http(anvil.endpoint_url());

        let rollup_config = Self::rollup_config(U256::from(config.mchain_id));

        let deploy_tx =
            Rollup::deploy_builder(&provider, U256::from(config.mchain_id), rollup_config.clone())
                .send()
                .await?;

        provider.anvil_set_block_timestamp_interval(1).await?;
        provider.anvil_mine(Some(U256::from(1)), None::<U256>).await?;

        let receipt = deploy_tx.get_receipt().await?;
        let rollup = Rollup::new(
            receipt.contract_address.ok_or(BlockBuilderError::NoContractAddress())?,
            provider.clone(),
        );

        let rollup_info = Self::rollup_info(
            "test",
            U256::from(config.chain_id),
            &rollup_config,
            rollup.address(),
            U256::from(receipt.block_number.ok_or(BlockBuilderError::NoBlockNumber())?),
        );

        Ok(Self { anvil, provider, rollup, rollup_info })
    }

    /// Submits a transaction to the `MetaChain`
    pub async fn submit_txn(&self, txn: TransactionRequest) -> Result<String> {
        let pending_txn =
            self.provider.send_transaction(txn).await.map_err(BlockBuilderError::SubmitTxnError)?;

        Ok(pending_txn.tx_hash().to_string())
    }

    /// Mines a block on the `MetaChain`
    pub async fn mine_block(&self, block_timestamp_secs: u64) -> Result<()> {
        let opts = MineOptions::Options {
            timestamp: (block_timestamp_secs > 0).then_some(block_timestamp_secs),
            blocks: Some(1),
        };
        let result = self.provider.anvil_mine_detailed(Some(opts)).await;
        info!("{}", format!("Mined block on MetaChain {:?}", result));
        result?;

        Ok(())
    }

    fn rollup_config(chain_id: U256) -> String {
        format!(
            r#"{{
            "chainId": {},
            "homesteadBlock": 0,
            "daoForkBlock": null,
            "daoForkSupport": true,
            "eip150Block": 0,
            "eip150Hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "eip155Block": 0,
            "eip158Block": 0,
            "byzantiumBlock": 0,
            "constantinopleBlock": 0,
            "petersburgBlock": 0,
            "istanbulBlock": 0,
            "muirGlacierBlock": 0,
            "berlinBlock": 0,
            "londonBlock": 0,
            "clique": {{
              "period": 0,
              "epoch": 0
            }},
            "arbitrum": {{
              "EnableArbOS": true,
              "AllowDebugPrecompiles": false,
              "DataAvailabilityCommittee": false,
              "InitialArbOSVersion": 10,
              "InitialChainOwner": "{}",
              "GenesisBlockNum": 0
            }}
          }}"#,
            chain_id,
            Address::ZERO
        )
    }

    fn rollup_info(
        chain_name: &str,
        chain_id: U256,
        rollup_config: &str,
        rollup: &Address,
        deployed_at: U256,
    ) -> String {
        format!(
            r#"[{{
    "chain-name": "{}",
    "parent-chain-id": {},
    "parent-chain-is-arbitrum": false,
    "sequencer-url": "",
    "secondary-forwarding-target": "",
    "feed-url": "",
    "secondary-feed-url": "",
    "das-index-url": "",
    "has-genesis-state": false,
    "chain-config": {},
    "rollup": {{
      "bridge": "{}",
      "inbox": "{}",
      "sequencer-inbox": "{}",
      "deployed-at": {},
      "rollup": "{}",
      "native-token": "{}",
      "upgrade-executor": "{}",
      "validator-wallet-creator": "{}"
    }}
  }}]"#,
            chain_name,
            chain_id,
            rollup_config,
            rollup,
            rollup,
            rollup,
            deployed_at,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO
        )
    }

    /// post a rollup batch to the mchain and mine the next block
    pub async fn send_batch(&self, batch: &arbitrum::batch::Batch) -> Result<()> {
        let delayed_messages_read = self.rollup.totalDelayedMessagesRead().call().await?._0;
        let tx = self
            .rollup
            .postBatch(
                delayed_messages_read
                    .checked_add(U256::from(
                        batch
                            .0
                            .iter()
                            .filter(|x| matches!(x, arbitrum::batch::BatchMessage::Delayed))
                            .count(),
                    ))
                    .ok_or(BlockBuilderError::Overflow())?, // after delayed messages read
                batch.encode()?,
            )
            .send()
            .await?
            .watch();
        self.mine_block(0).await?;
        tx.await?;
        Ok(())
    }
}

/// Check if a port is available by attempting to bind to it
///
/// The port will be used for both HTTP and WebSocket connections, a feature provided by Anvil.
/// See: <https://book.getfoundry.sh/reference/anvil/#supported-transport-layers>
pub fn is_port_available(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    TcpListener::bind(addr).is_ok()
}

/// Try to find an available port starting from `base_port`
/// Increments by 100 each try, up to `max_attempts`
pub fn find_available_port(base_port: u16, max_attempts: u16) -> Option<u16> {
    for attempt in 0..max_attempts {
        let port = base_port.saturating_add(attempt * 100);
        if is_port_available(port) {
            return Some(port);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_port_availability_checking() -> Result<()> {
        // Initial port should be available
        let base_port = 1111;
        assert!(is_port_available(base_port), "Base port should be available initially");

        // Bind to the port to make it unavailable
        let _listener = TcpListener::bind(format!("127.0.0.1:{}", base_port))?;
        assert!(!is_port_available(base_port), "Base port should be unavailable after binding");

        // Should find next available port
        let port = find_available_port(base_port, 10)
            .ok_or_else(|| eyre::eyre!("Failed to find available port"))?;

        // Port should be base_port + N*100 where N is 1..10
        assert!(port > base_port, "New port should be higher than base port");
        assert_eq!((port - base_port) % 100, 0, "Port increment should be multiple of 100");
        assert!(port <= base_port + 900, "Port should not exceed max attempts range");

        // New port should be available
        assert!(is_port_available(port), "New port should be available");

        Ok(())
    }
}
