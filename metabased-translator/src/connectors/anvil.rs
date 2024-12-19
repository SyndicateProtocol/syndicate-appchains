use crate::rollups::optimism::batch::{new_batcher_tx, Batch};
use crate::rollups::optimism::batcher_data::get_batcher_data;
use crate::rollups::optimism::frame::to_data;
use alloy::signers::k256::ecdsa;
use alloy_primitives::{Address, B256, U256};
use alloy_provider::ext::AnvilApi;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::{BlockId, BlockTransactionsKind, BlockNumberOrTag};
use op_alloy::protocol::SingleBatch;
use reqwest::Url;
use std::process::{Child, Command};
use std::str::FromStr;
use std::time::Duration;
use tracing::{error, info};
use alloy_network::EthereumWallet;
use alloy::signers::local::PrivateKeySigner;

use eyre::Report;
use crate::contract_bindings::eventemitter::EventEmitter;
use std::net::TcpListener;

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


fn cleanup_anvil(mut anvil: Child) {
if let Err(err) = anvil.kill() {
    error!("Failed to kill Anvil process: {}", err);
}
info!("Anvil process terminated.");
}

pub async fn run() -> eyre::Result<()> {

    // Start Anvil node on a specific port
    let port = 8888;
    let anvil = Command::new("anvil")
        .arg("--base-fee")
        .arg("0")
        .arg("--gas-limit")
        .arg("10000000000000000000")
        .arg("--chain-id")
        .arg("84532")
        .arg("--timestamp")
        .arg("1712500000")
        .arg("--port")
        .arg(port.to_string())
        .arg("--no-mining")
        .spawn()
        .expect("Failed to start Anvil. Is it installed?");

    let _guard = scopeguard::guard(anvil, cleanup_anvil);


    //sleep for 1 second to make sure anvil is ready
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Test JSON-RPC request to get the chain ID
    let client = reqwest::Client::new();
    let server_url = format!("http://localhost:{}", port);

    let response = client
        .post(server_url.clone())
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_chainId",
            "params": [],
            "id": 1
        }))
        .send()
        .await?;

    let json_response = response.json::<serde_json::Value>().await?;
    info!("Chain ID Response: {}", json_response);


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

    // let batch = Batch {
    //     parent_hash: B256::from_str(
    //         "0xe009262cd1adf34cfaf845fd1c17a6ddb7f97c67b2992cd9f286ff4e1c6ad233",
    //     )
    //     .unwrap(),
    //     epoch_num: 0,
    //     epoch_hash: block.header.hash,
    //     timestamp: 1712500002,
    //     transactions: vec![],
    // };

    // let frames = batch.get_frames(1000000).unwrap();
    // let data = to_data(&frames).unwrap();
    // info!("Data: {:?}", data);
    // Test the op_alloy batcher_data function

    let single_batch = SingleBatch {
        parent_hash: B256::from_str(
            "0xe009262cd1adf34cfaf845fd1c17a6ddb7f97c67b2992cd9f286ff4e1c6ad233",
        )
        .unwrap(),
        epoch_num: 0,
        epoch_hash: block.header.hash,
        timestamp: 1712500002,
        transactions: vec![],
    };
    let data = get_batcher_data(single_batch);
    info!("OP Data: {:?}", data);


    let tx = new_batcher_tx(batcher, batch_inbox, data.into());
    info!("Transaction: {:?}", tx);
    let builder = provider.send_transaction(tx.clone()).await.unwrap();
    let hash = *builder.tx_hash();
    info!("Transaction hash: {:?}", hash);
    provider
        .anvil_mine(Some(U256::from(1)), None::<U256>)
        .await?;

    //// END BATCHER TESTING ////

    // Keep the main thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

// TODO - replace addresses with OP and Arb precompile addresses
async fn deploy_contracts(server_url:String) -> Result<(), Report> {
    let anvil_provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_http(Url::parse(server_url.clone().as_str())?);

    // let event_emitter_bytecode = &EventEmitter::BYTECODE;
    let event_emitter_bytecode = &EventEmitter::DEPLOYED_BYTECODE;

    let addresses = [
        "0x1234000000000000000000000000000000000000".parse()?,
        "0x1234000000000000000000000000000000000001".parse()?,
        "0x1234000000000000000000000000000000000002".parse()?];

    for address in addresses.iter() {
        anvil_provider.anvil_set_code(*address, event_emitter_bytecode.clone()).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{hex, sol};
    use alloy::primitives::U256;
    use alloy_primitives::{keccak256, Address};
    use alloy_provider::ext::AnvilApi;
    use alloy::providers::ProviderBuilder;
    use std::str::FromStr;
    use std::sync::Arc;
    use alloy::transports::BoxTransport;
    use alloy_provider::fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller};
    use alloy_provider::{Identity, Provider, RootProvider};
    use alloy_provider::layers::AnvilProvider;
    use alloy_provider::network::{Ethereum, EthereumWallet};
    use crate::contract_bindings::eventemitter::EventEmitter::EventEmitterInstance;

    // Create a type alias for our complex provider type
    type AnvilFillProvider =
    FillProvider<
        JoinFill<JoinFill<Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>,
            WalletFiller<EthereumWallet>>, AnvilProvider<RootProvider<BoxTransport>, BoxTransport>,
        BoxTransport,
        Ethereum
    >;

    // Helper struct for Anvil instance management
    struct AnvilInstance {
        url: String,
        provider: Arc<AnvilFillProvider>,
    }

    impl AnvilInstance {
        async fn new() -> eyre::Result<Self> {
            Self::with_port(8545).await
        }

        async fn with_port(port: u16) -> eyre::Result<Self> {
            // Init provider as part of creating Anvil layer in one step. 
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .on_anvil_with_wallet_and_config(|anvil| {
                    anvil
                        .port(port)
                        // .args(vec!["--no-mining"]) // Need to mine blocks to check txns
                });

            let url = format!("http://localhost:{}", port);

            Ok(Self {
                url,
                provider: Arc::new(provider)
            })
        }

        fn url(&self) -> &str {
            &self.url
        }
    }

    // Existing simple Counter contract definition
    sol! {
        #[allow(missing_docs)]
        #[sol(rpc, bytecode="6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033")]
        contract Counter {
            uint256 public number;

            function setNumber(uint256 newNumber) public {
                number = newNumber;
            }

            function increment() public {
                number++;
            }
        }
    }


    #[tokio::test]
    async fn test_counter_contract_with_anvil_set_code() -> eyre::Result<()> {
        // Spin up a local Anvil node.
        // Ensure `anvil` is available in $PATH.
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();

        let address = "0x1234000000000000000000000000000000000000".parse()?;

        let bytecode = hex::decode(
            "6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033"
        )?;

        provider.anvil_set_code(address, bytecode.into()).await?;

        let contract = Counter::new(address, provider.clone());
        let result = contract.setNumber(U256::from(42)).send().await?;
        let receipt = result.watch().await?;
        info!("Set number transaction: {:?}", receipt);

        let number = contract.number().call().await?;
        assert_eq!(number.number, U256::from(42), "Number should be 42");

        Ok(())
    }

    #[tokio::test]
    async fn test_deploy_contracts_invalid_url() {
        let result = deploy_contracts("invalid-url".to_string()).await;
        assert!(result.is_err(), "Expected error for invalid URL");
    }

    #[tokio::test]
    async fn test_deploy_contracts_no_anvil() {
        let result = deploy_contracts("http://localhost:9999".to_string()).await;
        assert!(result.is_err(), "Expected error when Anvil is not running");
    }

    #[tokio::test]
    async fn test_deploy_multiple_times() -> eyre::Result<()> {
        let anvil = AnvilInstance::new().await?;

        for _ in 0..3 {
            deploy_contracts(anvil.url().to_string()).await?;
        }

        Ok(())
    }

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
