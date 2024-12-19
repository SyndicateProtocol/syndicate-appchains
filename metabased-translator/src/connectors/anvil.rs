use crate::rollups::optimism::batch::{new_batcher_tx, Batch};
use crate::rollups::optimism::frame::to_data;
use alloy_primitives::{Address, B256, U256};
use alloy_provider::ext::AnvilApi;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::{BlockId, BlockTransactionsKind, BlockNumberOrTag};
use reqwest::Url;
use std::str::FromStr;
use std::time::Duration;
use tracing::info;
use alloy_network::EthereumWallet;
use alloy::signers::local::PrivateKeySigner;
use std::net::TcpListener;
use alloy_node_bindings::Anvil;

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

pub async fn run() -> eyre::Result<()> {
    let base_port = 8888;
    let port = find_available_port(base_port, 10)
        .ok_or_else(|| eyre::eyre!("No available ports found after 10 attempts"))?;

    if port != base_port {
        info!("Port {} is in use, switching to port {}", base_port, port);
    }

    let _anvil = Anvil::new()
        .port(port)
        .chain_id(84532)
        .args(vec!["--base-fee", "0",
                   "--gas-limit", "30000000",
                   "--no-mining",
                   "--timestamp", "1712500000"
        ]).try_spawn()?;


    // Test JSON-RPC request to get the chain ID
    let server_url = format!("http://localhost:{}", port);

    let signer: PrivateKeySigner ="fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d".parse().unwrap();
    let wallet = EthereumWallet::from(signer);

    // Create a provider to interact with the node
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(Url::parse(server_url.clone().as_str())?);


    // Set up the batcher and batch inbox
    let batcher = Address::from_str("0x063D87A885a9323831A688645647eD7d0e859C5d").expect("Failed to parse batcher address");
    let batch_inbox = Address::from_str("0x97395dd253e2d096a0caa62a574895c3c2f2b2e0")
        .expect("Failed to parse Batch Inbox address");
    let balance = U256::MAX;
    provider.anvil_set_balance(batcher, balance).await?;

    let block = provider.get_block(
        BlockId::Number(BlockNumberOrTag::Number(0)),
        BlockTransactionsKind::Hashes
    ).await?
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
    let builder = provider.send_transaction(tx.clone()).await.unwrap();
    let hash = *builder.tx_hash();
    info!("Transaction hash: {:?}", hash);
    provider
        .anvil_mine(Some(U256::from(1)), None::<U256>)
        .await?;

    // Keep the main thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
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
