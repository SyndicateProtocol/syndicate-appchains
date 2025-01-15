//! Anvil connector for the `MetaChain`

use crate::config::BlockBuilderConfig;
use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::U256,
    signers::local::PrivateKeySigner,
    transports::http::Http,
};
use alloy_node_bindings::Anvil;
use alloy_provider::{
    ext::AnvilApi,
    fillers::{
        BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
    },
    Identity, Provider, ProviderBuilder, RootProvider,
};
use alloy_rpc_types::TransactionRequest;
use eyre::eyre;
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

/// Provider for the `MetaChain`
#[derive(Debug)]
pub struct MetaChainProvider {
    provider: FilledProvider,
}

impl MetaChainProvider {
    /// Starts the Anvil instance and creates a provider for the `MetaChain`
    pub async fn start(config: BlockBuilderConfig) -> eyre::Result<Self> {
        let port = find_available_port(config.port, 10)
            .ok_or_else(|| eyre!("No available ports found after 10 attempts"))?;

        if port != config.port {
            info!("Port {} is in use, switching to port {}", config.port, port);
        }

        let anvil = Anvil::new()
            .port(port)
            .chain_id(config.chain_id)
            .args(vec![
                "--base-fee",
                "0",
                "--gas-limit",
                "30000000",
                "--timestamp",
                config.genesis_timestamp.to_string().as_str(),
                "--no-mining",
            ])
            .try_spawn()?;

        let signer: PrivateKeySigner =
            "fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d"
                .parse()
                .map_err(|e| eyre!("Failed to parse private key: {}", e))?;

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(EthereumWallet::from(signer))
            .on_http(anvil.endpoint_url());

        Ok(Self { provider })
    }

    /// Submits a transaction to the `MetaChain`
    pub async fn submit_txn(&self, txn: TransactionRequest) -> eyre::Result<String> {
        let pending_txn = self
            .provider
            .send_transaction(txn)
            .await
            .map_err(|e| eyre!("Failed to send transaction: {}", e))?;

        Ok(pending_txn.tx_hash().to_string())
    }

    /// Mines a block on the `MetaChain`
    pub async fn mine_block(&self) -> eyre::Result<()> {
        self.provider
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
