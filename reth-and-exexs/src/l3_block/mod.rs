use alloy::{
    primitives::{Bytes, FixedBytes, U256},
    providers::ProviderBuilder,
    sol,
    sol_types::SolCall,
};
use dotenv::dotenv;
use eyre::{Context, Result};
use reth_tracing::tracing::{error, warn};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

sol!(
    #[sol(rpc)]
    BasedSequencerChain,
    "../contracts/out/BasedSequencerChain.sol/BasedSequencerChain.json"
);

pub struct Config {
    pub contract_address: String,
    pub rpc_url: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let contract_address =
            env::var("CONTRACT_ADDRESS").context("CONTRACT_ADDRESS env var is not set")?;
        let rpc_url = env::var("RPC_URL").context("RPC_URL env var is not set")?;
        Ok(Config {
            contract_address,
            rpc_url,
        })
    }
}

#[derive(Debug, Clone)]
pub struct L3Block {
    pub parent_hash: FixedBytes<32>,
    pub epoch_number: U256,
    pub timestamp: U256,
    pub transaction_list: Vec<Bytes>,
}

pub struct L3BlockParser {
    #[allow(dead_code)]
    contract: BasedSequencerChain::BasedSequencerChainInstance<
        alloy::transports::http::Http<alloy::transports::http::Client>,
        alloy::providers::RootProvider<
            alloy::transports::http::Http<alloy::transports::http::Client>,
        >,
    >,
}

impl L3BlockParser {
    pub fn new() -> Result<Self> {
        dotenv().ok();

        let settings = Config::new()?;
        let provider = ProviderBuilder::new().on_http(settings.rpc_url.parse()?);

        let contract = BasedSequencerChain::new(settings.contract_address.parse()?, provider);

        Ok(Self { contract })
    }

    pub async fn parse_l3_block(&self, tx_data: &[u8]) -> Result<Option<L3Block>> {
        if let Ok(decoded) = BasedSequencerChain::sequenceNextBatchCall::abi_decode(tx_data, true) {
            let BasedSequencerChain::sequenceNextBatchCall { userProvidedBatch } = decoded;

            let current_timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(e) => {
                    error!("Failed to get current timestamp: {}", e);
                    warn!("Using UNIX_EPOCH as fallback timestamp");
                    0 // Fallback to UNIX_EPOCH if we somehow got a time before it
                }
            };

            let l3_block = L3Block {
                parent_hash: userProvidedBatch.non_empty_parent_hash,
                epoch_number: U256::from(0), // This will be calculated in the contract
                timestamp: U256::from(current_timestamp),
                transaction_list: userProvidedBatch.transaction_list,
            };

            Ok(Some(l3_block))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_l3_block() -> Result<()> {
        // Create the parser with the deployed contract
        let parser = L3BlockParser::new()?;

        // Create a sample transaction data
        let parent_hash = FixedBytes::from([1u8; 32]);
        let transaction_list = vec![Bytes::from(vec![1, 2, 3])];
        let user_provided_batch = BasedSequencerChain::UserProvidedBatch {
            non_empty_parent_hash: parent_hash,
            transaction_list: transaction_list.clone(),
        };
        let tx_data = BasedSequencerChain::sequenceNextBatchCall {
            userProvidedBatch: user_provided_batch,
        }
        .abi_encode();

        // Parse the L3 block
        let parsed_block = parser.parse_l3_block(&tx_data).await?;

        // Assert that the block was parsed successfully
        assert!(parsed_block.is_some());
        let parsed_block = parsed_block.unwrap();

        // Check the parsed block fields
        assert_eq!(parsed_block.parent_hash, parent_hash);
        assert_eq!(parsed_block.epoch_number, U256::from(0));
        assert_eq!(parsed_block.transaction_list, transaction_list);

        // The timestamp should be close to the current time
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let parsed_timestamp = parsed_block.timestamp.to::<u64>();
        assert!(
            parsed_timestamp >= current_timestamp - 5 && parsed_timestamp <= current_timestamp + 5
        );

        Ok(())
    }
}
