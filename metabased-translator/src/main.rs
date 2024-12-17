use alloy::{hex, sol};
use alloy::primitives::U256;
use alloy::providers::ProviderBuilder;
use alloy_provider::ext::AnvilApi;

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

// PoC deploying a contract using `anvil_set_code`, then interacting with it
#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();

    // Deploy the `Counter` contract.
    // let contract = Counter::deploy(&provider).await?;
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

    Ok(())




    // // Set the number to 42.
    // let builder = contract.setNumber(U256::from(42));
    // let tx_hash = builder.send().await?.watch().await?;
    //
    // println!("Set number to 42: {tx_hash}");
    //
    // // Increment the number to 43.
    // let builder = contract.increment();
    // let tx_hash = builder.send().await?.watch().await?;
    //
    // println!("Incremented number: {tx_hash}");

    // Retrieve the number, which should be 43.

    // let builder = contract.number();
    // let result = builder.call().await;
    // let uint = result?.number;
    // let number = uint.to_string();
    //
    // println!("Retrieved number: {number}");
    //
    // Ok(())
}
