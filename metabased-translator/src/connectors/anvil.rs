use std::backtrace;
use std::process::{Child, Command};
use std::str::FromStr;
use std::time::Duration;
use alloy_provider::ext::AnvilApi;
use alloy_provider::{Provider, ProviderBuilder};
use reqwest::Url;
use tracing::{error, info};
use alloy_primitives::{Address, U256, B256};
use crate::rollups::optimism::batch::Batch;
use crate::rollups::optimism::frame::to_data;

use crate::rollups::optimism::batcher_transaction::new_batcher_tx;

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

    let response = client.post(server_url.clone())
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
    provider.anvil_set_code(address, bytecode.into()).await?;
    info!("Deployed contract code at: {}", address);

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
    let json_response = client.post(server_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "anvil_setCode",
            "params": ["0x1235000000000000000000000000000000000000", "0x"],
            "id": 1
        }))
        .send()
        .await?.json::<serde_json::Value>().await?;

    info!("anvil_setCode Response: {}", json_response);




    //// BATCHER TESTING ////

    // Set up the batcher and batch inbox
    let batcher = Address::from_str("0x1234000000000000000000000000000000000000").expect("Failed to parse Batcher address");
    let batch_inbox = Address::from_str("0x1234000000000000000000000000000000000000").expect("Failed to parse Batch Inbox address");
    let balance = U256::MAX;
    provider.anvil_set_balance(batcher, balance).await?;



    let batch = Batch {
                parent_hash: B256::from_str("0xfe705b3c9f7e9154dc17baf8f5d6b62456cf1f607dffcdbb0b4f00fcdfbfa16b").unwrap(),
                epoch_num: 0,
                epoch_hash: B256::from_str("0xfe705b3c9f7e9154dc17baf8f5d6b62456cf1f607dffcdbb0b4f00fcdfbfa16b").unwrap(),
                timestamp: 1712500000,
                transactions: vec![],
            };
    let frames = batch.get_frames(1000).unwrap();
    let data = to_data(&frames).unwrap();
    let txn = new_batcher_tx(batcher, batch_inbox, data.into());
    info!("Sending transaction to batch inbox: {:?}", txn);
    provider.eth_send_unsigned_transaction(txn).await?;
    provider.anvil_mine(Some(U256::from(1)), None::<U256>).await?;

    //// END BATCHER TESTING ////


    // Keep the main thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[cfg(test)]
mod tests {
    // Simple contract from Allou-rs/examples repo
    // Codegen from embedded Solidity code and precompiled bytecode.
    sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc Counter.sol --via-ir --optimize --bin
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

    use alloy::{hex, sol};
    use alloy::primitives::U256;
    use alloy_provider::ext::AnvilApi;
    use alloy::providers::ProviderBuilder;

    #[tokio::test]
    async fn test_counter_contract_with_anvil_set_code() -> eyre::Result<()> {
        // Spin up a local Anvil node.
        // Ensure `anvil` is available in $PATH.
        let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();

        let address = "0x1234000000000000000000000000000000000000".parse()?;
        let contract = Counter::new(address, provider.clone());

        println!("Deployed contract at address: {}", contract.address());

        // Get the runtime bytecode (not the deployment bytecode)
        let bytecode = hex::decode(
            "6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033"
        )?;

        // Set the code at the address
        provider.anvil_set_code(address, bytecode.into()).await?;

        // Create contract instance
        let contract = Counter::new(address, provider.clone());

        // Try to interact with the contract
        let result = contract.setNumber(U256::from(42)).send().await?;
        let receipt = result.watch().await?;
        println!("Set number transaction: {:?}", receipt);

        // Read the number
        let number = contract.number().call().await?;
        println!("Number: {}", number.number);

        // Add an actual test assertion
        assert_eq!(number.number, U256::from(42), "Number should be 42");

        Ok(())
    }
}