use alloy::transports::http::Http;
use alloy_node_bindings::{Anvil, AnvilInstance};
use alloy_primitives::U256;
use alloy_provider::ext::AnvilApi;
use alloy_provider::{ProviderBuilder, RootProvider};
use reqwest::Client;
use std::net::TcpListener;
use tracing::info;

pub struct MetaChainProvider {
    port: u16,
    chain_id: u64,
    genesis_timestamp: u64,

    anvil: Option<AnvilInstance>,
    pub base_provider: Option<RootProvider<Http<Client>>>,
}

impl Default for MetaChainProvider {
    fn default() -> Self {
        Self {
            port: 8888,
            chain_id: 84532,
            genesis_timestamp: 1712500000,
            anvil: None,
            base_provider: None,
        }
    }
}

impl MetaChainProvider {
    pub fn start(&mut self) -> eyre::Result<()> {
        let base_port = self.port;
        let port = find_available_port(base_port, 10)
            .ok_or_else(|| eyre::eyre!("No available ports found after 10 attempts"))?;

        if port != base_port {
            info!("Port {} is in use, switching to port {}", base_port, port);
        }

        let anvil = Anvil::new()
            .port(port)
            .chain_id(self.chain_id)
            .args(vec![
                "--base-fee",
                "0",
                "--gas-limit",
                "30000000",
                "--timestamp",
                self.genesis_timestamp.to_string().as_str(),
                "--no-mining",
            ])
            .try_spawn()?;

        let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());

        self.anvil = Some(anvil);
        self.base_provider = Some(provider);

        Ok(())
    }

    pub async fn mine_block(&self) -> eyre::Result<()> {
        self.base_provider
            .as_ref()
            .ok_or_else(|| eyre::eyre!("Provider not initialized"))?
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

/// Try to find an available port starting from base_port
/// Increments by 100 each try, up to max_attempts
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
