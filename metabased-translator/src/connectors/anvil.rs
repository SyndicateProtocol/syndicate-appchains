use crate::rollups::optimism::batch::{new_batcher_tx, Batch};
use crate::rollups::optimism::frame::to_data;
use alloy_primitives::{Address, B256, U256};
use alloy_provider::ext::AnvilApi;
use alloy_provider::ProviderBuilder;
use eyre::Report;
use reqwest::Url;
use std::process::{Child, Command};
use std::str::FromStr;
use std::time::Duration;
use tracing::{error, info};
use crate::contract_bindings::eventemitter::EventEmitter;

// Graceful shutdown for Anvil process
fn cleanup_anvil(mut anvil: Child) {
    if let Err(err) = anvil.kill() {
        error!("Failed to kill Anvil process: {}", err);
    }
    info!("Anvil process terminated.");
}

pub async fn run() -> eyre::Result<()> {
    // Start Anvil node on a specific port
    let port = 8545;
    let anvil = Command::new("anvil")
        .arg("--base-fee")
        .arg("0")
        .arg("--gas-limit")
        .arg("9999999999999999999999999")
        .arg("--port")
        .arg(port.to_string())
        .arg("--no-mining")
        .spawn()
        .expect("Failed to start Anvil. Is it installed?");

    info!("Started Anvil node with PID: {}", anvil.id());

    // Gracefully handle Anvil shutdown on program exit
    let _guard = scopeguard::guard(anvil, cleanup_anvil);

    // Wait for Anvil to start
    tokio::time::sleep(Duration::from_secs(2)).await;

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

    // Create a provider to interact with the node
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_http(Url::parse(server_url.clone().as_str())?);

    // Deploy a contract using anvil_setCode
    let address = "0x1234000000000000000000000000000000000000".parse()?;
    let bytecode = alloy::hex::decode(
        "6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033"
    )?;
    provider.anvil_set_code(address, bytecode.clone().into()).await?;
    info!("Deployed contract code at: {}", address);

    deploy_contracts(server_url.clone()).await?;

    // // Create contract instance
    // let contract = Contract::default(address, provider.clone());
    //
    // // Interact with the deployed contract
    // let tx_result = contract.setNumber(U256::from(42)).send().await?;
    // let receipt = tx_result.await?;
    // println!("Set number transaction receipt: {:?}", receipt);
    //
    // let number = contract.number().call().await?;
    // println!("Number value: {}", number);

    info!(
        "Server is running at http://localhost:{}. Press Ctrl+C to stop.",
        port
    );

    // Test server call to anvil_setCode
    let json_response = client
        .post(server_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "anvil_setCode",
            "params": ["0x1235000000000000000000000000000000000000", "0x"],
            "id": 1
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    info!("anvil_setCode Response: {}", json_response);

    //// BATCHER TESTING ////

    // Set up the batcher and batch inbox
    let batcher = Address::from_str("0x1234000000000000000000000000000000000000")
        .expect("Failed to parse Batcher address");
    let batch_inbox = Address::from_str("0x1234000000000000000000000000000000000001")
        .expect("Failed to parse Batch Inbox address");
    let balance = U256::MAX;
    provider.anvil_set_balance(batcher, balance).await?;

    let batch = Batch {
        parent_hash: B256::from_str(
            "0xfe705b3c9f7e9154dc17baf8f5d6b62456cf1f607dffcdbb0b4f00fcdfbfa16b",
        )?,
        epoch_num: 0,
        epoch_hash: B256::from_str(
            "0xfe705b3c9f7e9154dc17baf8f5d6b62456cf1f607dffcdbb0b4f00fcdfbfa16b",
        )?,
        timestamp: 1712500000,
        transactions: vec![],
    };
    let frames = batch.get_frames(1000).unwrap();
    let data = to_data(&frames)?;
    let txn = new_batcher_tx(batcher, batch_inbox, data.into());

    info!("Sending transaction to batch inbox: {:?}", txn);
    provider.eth_send_unsigned_transaction(txn).await?;
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
    use std::process::Child;
    use std::str::FromStr;
    use alloy_provider::Provider;
    use scopeguard::ScopeGuard;
    use crate::contract_bindings::eventemitter::EventEmitter::EventEmitterInstance;

    // Helper struct for Anvil instance management
    struct AnvilInstance {
        url: String,
        _guard: ScopeGuard<Child, fn(Child)>,
    }

    impl AnvilInstance {
        async fn new() -> eyre::Result<Self> {
            Self::with_port(8545).await
        }

        async fn with_port(port: u16) -> eyre::Result<Self> {
            let anvil = Command::new("anvil")
                .arg("--port")
                .arg(port.to_string())
                // .arg("--no-mining") // Want to mine blocks to confirm txn 
                .spawn()
                .expect("Failed to start Anvil");

            info!("Started Anvil node with PID: {}", anvil.id());

            // Wait for anvil to start
            tokio::time::sleep(Duration::from_secs(2)).await;

            let url = format!("http://localhost:{}", port);
            let _guard: ScopeGuard<Child, fn(Child)> = scopeguard::guard(anvil, cleanup_anvil);

            Ok(Self { url, _guard })
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
    async fn test_deploy_event_emitter_contracts() -> eyre::Result<()> {
        let anvil = AnvilInstance::with_port(8457).await?;
        deploy_contracts(anvil.url().to_string()).await?;

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_http(Url::parse(anvil.url())?);

        provider.anvil_set_logging(true).await?;

        let address = Address::from_str("0x1234000000000000000000000000000000000000")?;
        let contract = EventEmitterInstance::new(address, provider.clone());

        // Create meaningful test data
        let sig_hash = keccak256("TestEvent(bytes32)"); // Using appropriate event signature
        let non_indexed = [1u8; 32]; // Some test data

        // Send transaction and wait for it
        let tx = contract.emitEvent1(sig_hash, non_indexed.into())
            .gas(1_000_000)
            .nonce(0)
            .from("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse()?)// Prefunded testing address
            .send()
            .await?;

        let hash = tx.watch().await?;
        let receipt =
            provider.get_transaction_receipt(hash).await?.expect("Transaction receipt not found");

        // Verify the transaction succeeded
        assert!(receipt.status());

        // Uncomment me to see logs
        // receipt.inner.logs().iter().for_each(|log| {
        //     println!("Log: {:?}", log);
        // });

        // // Optionally verify events were emitted
        assert!(!receipt.inner.logs().is_empty());

        Ok(())
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
}
