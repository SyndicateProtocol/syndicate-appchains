//! Anvil connector for the `MetaChain`

use crate::config::Configuration;
use crate::rollups::optimism::{
    batch::{new_batcher_tx, Batch},
    frame::to_data,
};
use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, B256, U256},
    providers::{
        ext::AnvilApi,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    rpc::types::{BlockId, BlockNumberOrTag, BlockTransactionsKind},
    signers::local::PrivateKeySigner,
    transports::http::Http,
};
use eyre::eyre;
use reqwest::{Client, Url};
use std::{net::TcpListener, str::FromStr};
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

/// Provider for the `MetaChain`
#[derive(Debug)]
pub struct MetaChainProvider {
    /// anvil instance
    pub anvil: AnvilInstance,
    /// provider
    provider: FilledProvider,
}

impl MetaChainProvider {
    /// Starts the Anvil instance and creates a provider for the `MetaChain`
    pub async fn start(config: Configuration) -> eyre::Result<Self> {
        Self::start_from_snapshot(config, "").await
    }

    /// Starts the Anvil instance from a snapshot and creates a provider for the `MetaChain`
    pub async fn start_from_snapshot(config: Configuration, snapshot: &str) -> eyre::Result<Self> {
        let port = find_available_port(config.port, 10)
            .ok_or_else(|| eyre!("No available ports found after 10 attempts"))?;

        if port != config.port {
            info!("Port {} is in use, switching to port {}", config.port, port);
        }

        let ts = config.genesis_timestamp.to_string();

        let mut args = vec![
            "--base-fee",
            "0",
            "--gas-limit",
            "30000000",
            "--timestamp",
            ts.as_str(),
            "--no-mining",
        ];

        if !snapshot.is_empty() {
            args.push("--load-state");
            args.push(snapshot);
        }

        let anvil = Anvil::new()
            .port(port)
            .chain_id(config.chain_id)
            .args(args)
            .try_spawn()?;

        let signer: PrivateKeySigner =
            "fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d"
                .parse()
                .map_err(|e| eyre!("Failed to parse private key: {}", e))?;

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(EthereumWallet::from(signer))
            .on_http(Url::parse(anvil.endpoint_url().as_str())?);

        Ok(Self { anvil, provider })
    }

    /// Mines a block on the `MetaChain`
    pub async fn mine_block(&self) -> eyre::Result<()> {
        let signer: PrivateKeySigner =
            "fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d"
                .parse()
                .map_err(|e| eyre!("Failed to parse private key: {}", e))?;
        let wallet = EthereumWallet::from(signer);

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_provider(self.provider.clone());

        // Set up the batcher and batch inbox
        let batcher = Address::from_str("0x063D87A885a9323831A688645647eD7d0e859C5d")
            .map_err(|e| eyre!("Failed to parse batcher address: {}", e))?;
        let batch_inbox = Address::from_str("0x97395dd253e2d096a0caa62a574895c3c2f2b2e0")
            .map_err(|e| eyre!("Failed to parse Batch Inbox address: {}", e))?;
        let balance = U256::MAX;
        provider.anvil_set_balance(batcher, balance).await?;

        let block = provider
            .get_block(
                BlockId::Number(BlockNumberOrTag::Number(0)),
                BlockTransactionsKind::Hashes,
            )
            .await?
            .ok_or_else(|| eyre!("Failed to get block"))?;

        info!("Block: {:?}", block);
        let single_batch = Batch {
            parent_hash: B256::from_str(
                "0xe009262cd1adf34cfaf845fd1c17a6ddb7f97c67b2992cd9f286ff4e1c6ad233",
            )
            .map_err(|e| eyre!("Failed to parse parent hash: {}", e))?,
            epoch_num: 0,
            epoch_hash: block.header.hash,
            timestamp: 1712500002,
            transactions: vec![],
        };
        let frames = single_batch
            .get_frames(1000000)
            .map_err(|e| eyre!("Failed to get frames: {}", e))?;
        let data =
            to_data(&frames).map_err(|e| eyre!("Failed to convert frames to data: {}", e))?;

        let tx = new_batcher_tx(batcher, batch_inbox, data.into());
        info!("Transaction: {:?}", tx);
        let builder = provider
            .send_transaction(tx.clone())
            .await
            .map_err(|e| eyre!("Failed to send transaction: {}", e))?;
        let hash = *builder.tx_hash();
        info!("Transaction hash: {:?}", hash);
        provider
            .anvil_mine(Some(U256::from(1)), None::<U256>)
            .await?;

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

// TODO : Implement generic deploy_contracts function
// async fn deploy_contracts(server_url:String) -> Result<(), Report> {
//     let anvil_provider = ProviderBuilder::new()
//         .with_recommended_fillers()
//         .on_http(Url::parse(server_url.clone().as_str())?);

//     // let event_emitter_bytecode = &EventEmitter::BYTECODE;
//     let event_emitter_bytecode = &EventEmitter::DEPLOYED_BYTECODE;

//     let addresses = [
//         "0x1234000000000000000000000000000000000000".parse()?,
//         "0x1234000000000000000000000000000000000001".parse()?,
//         "0x1234000000000000000000000000000000000002".parse()?];

//     for address in addresses.iter() {
//         anvil_provider.anvil_set_code(*address, event_emitter_bytecode.clone()).await?;
//     }
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_port_availability_checking() -> eyre::Result<()> {
        // Initial port should be available
        let base_port = 1111;
        assert!(
            is_port_available(base_port),
            "Base port should be available initially"
        );

        // Bind to the port to make it unavailable
        let _listener = TcpListener::bind(format!("127.0.0.1:{}", base_port))?;
        assert!(
            !is_port_available(base_port),
            "Base port should be unavailable after binding"
        );

        // Should find next available port
        let port = find_available_port(base_port, 10)
            .ok_or_else(|| eyre::eyre!("Failed to find available port"))?;

        // Port should be base_port + N*100 where N is 1..10
        assert!(port > base_port, "New port should be higher than base port");
        assert_eq!(
            (port - base_port) % 100,
            0,
            "Port increment should be multiple of 100"
        );
        assert!(
            port <= base_port + 900,
            "Port should not exceed max attempts range"
        );

        // New port should be available
        assert!(is_port_available(port), "New port should be available");

        Ok(())
    }
}
