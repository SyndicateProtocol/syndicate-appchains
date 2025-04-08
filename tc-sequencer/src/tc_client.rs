//! The `service` module handles the business logic for the tc sequencer.

use crate::config::Config;
use alloy::{
    consensus::Transaction,
    hex::{self},
    primitives::{Address, Bytes, TxHash},
};
use eyre::Result;
use jsonrpsee::{
    core::{RpcResult, Serialize},
    types::Params,
    Extensions,
};
use reqwest::Client;
use serde as _;
use shared::{
    json_rpc::{parse_send_raw_transaction_params, Error},
    tx_validation::validate_transaction,
};
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error, info};
use url::Url;

const DEFAULT_SEQUENCING_CHAIN_ID: u64 = 5113;
const DEFAULT_FUNCTION_SIGNATURE: &str = "processTransaction(bytes data)";
const TC_DATA_KEY: &str = "data";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SendTransactionRequest {
    project_id: String,
    contract_address: String,
    chain_id: u64,
    function_signature: String,
    args: HashMap<String, String>,
}

impl SendTransactionRequest {
    fn new(project_id: String, contract_address: String, data: Bytes) -> Self {
        Self {
            project_id,
            contract_address,
            chain_id: DEFAULT_SEQUENCING_CHAIN_ID,
            function_signature: DEFAULT_FUNCTION_SIGNATURE.to_string(),
            args: HashMap::from([(TC_DATA_KEY.to_string(), hex::encode(data))]),
        }
    }
}

/// The service for relaying transactions to the sequencing contract.
#[derive(Debug)]
pub struct TCClient {
    tc_url: Url,
    tc_project_id: String,
    tc_api_key: String,

    sequencing_addresses: HashMap<u64, Address>,
    client: Client,
}

impl TCClient {
    /// Create a new `TCClient` instance.
    pub fn new(config: &Config) -> Result<Self> {
        let client = Client::new();

        Ok(Self {
            tc_url: config.tc_endpoint.get_url(),
            tc_project_id: config.tc_project_id.clone(),
            tc_api_key: config.tc_api_key.clone(),
            sequencing_addresses: config.sequencing_addresses.clone(),
            client,
        })
    }

    fn get_contract_address(&self, chain_id: u64) -> Result<Address, Error> {
        self.sequencing_addresses.get(&chain_id).copied().ok_or_else(|| {
            Error::Internal(format!("Sequencing on chain {} is not supported", chain_id))
        })
    }

    async fn send_transaction(
        &self,
        contract_address: Address,
        raw_tx: Bytes,
    ) -> Result<(), Error> {
        let request = SendTransactionRequest::new(
            self.tc_project_id.clone(),
            contract_address.to_string(),
            raw_tx,
        );

        let response = self
            .client
            .post(format!("{}/transact/sendTransaction", self.tc_url))
            .bearer_auth(self.tc_api_key.clone())
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                let error_msg = format!("Failed to send transaction to TC: {}", e);
                error!(error_msg);
                Error::Internal(error_msg)
            })?;

        if !response.status().is_success() {
            let error_msg = format!("Failed to send transaction to TC: {}", response.status());
            error!(error_msg);
            return Err(Error::Internal(error_msg));
        }

        Ok(())
    }

    async fn process_transaction(&self, raw_tx: Bytes) -> Result<TxHash, Error> {
        info!("Processing transaction: {}", hex::encode(&raw_tx));
        let original_tx = validate_transaction(&raw_tx)?;

        // Determine the contract address
        let contract_address =
            self.get_contract_address(original_tx.chain_id().unwrap_or_default())?;

        debug!("Submitting validated transaction to TC");
        self.send_transaction(contract_address, raw_tx).await?;

        Ok(*original_tx.tx_hash())
    }
}

/// The handler for the `eth_sendRawTransaction` JSON-RPC method.
pub async fn send_raw_transaction_handler(
    params: Params<'static>,
    service: Arc<TCClient>,
    _: Extensions,
) -> RpcResult<String> {
    let tx_data = parse_send_raw_transaction_params(params)?;
    let tx_hash = service.process_transaction(tx_data).await?;

    Ok(format!("0x{}", hex::encode(tx_hash)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{get_sequencing_addresses_default, Config};
    use alloy::primitives::Bytes;
    use std::str::FromStr;

    fn setup_test_service() -> TCClient {
        let config = Config::default();
        TCClient::new(&config).unwrap()
    }

    #[test]
    fn test_new_service_creation() {
        let config = Config::default();

        let result = TCClient::new(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_contract_address() {
        // Set up a test service with a mock config
        let config = Config {
            sequencing_addresses: get_sequencing_addresses_default(),
            ..Default::default()
        };
        let service = TCClient::new(&config).unwrap();

        // Manchego Chain
        let contract_address = service.get_contract_address(510000).unwrap();
        assert_eq!(
            contract_address,
            Address::from_str("0x180972BF154c9Aea86c43149D83B7Ea078c33f48").unwrap()
        );

        // Burrata Chain
        let contract_address = service.get_contract_address(510001).unwrap();
        assert_eq!(
            contract_address,
            Address::from_str("0xC1cacFC14624c4E241286Ade61DF545b90f850B4").unwrap()
        );

        // IRL Chain
        let contract_address = service.get_contract_address(63821).unwrap();
        assert_eq!(
            contract_address,
            Address::from_str("0x536EA7C009ebE418501a1DB133b281a4a01d50f5").unwrap()
        );

        // Commerce Chain
        let contract_address = service.get_contract_address(63822).unwrap();
        assert_eq!(
            contract_address,
            Address::from_str("0x7C8d3922298AbbEF7beE5F3dACC4238326482789").unwrap()
        );

        // Dream Chain
        let contract_address = service.get_contract_address(63823).unwrap();
        assert_eq!(
            contract_address,
            Address::from_str("0x62B82d1AF6D61DdfE5b4af38Eb5dE982A7f7565f").unwrap()
        );

        // Amino Chain
        let contract_address = service.get_contract_address(63824).unwrap();
        assert_eq!(
            contract_address,
            Address::from_str("0x8CcaC248CcFCA1283981678B7291F48f6e26ad39").unwrap()
        );

        // Eco Chain
        let contract_address = service.get_contract_address(63825).unwrap();
        assert_eq!(
            contract_address,
            Address::from_str("0x47ec452FA5035C24217daCC66aA305802F1d0fbe").unwrap()
        );

        // Playground Chain
        let contract_address = service.get_contract_address(63826).unwrap();
        assert_eq!(
            contract_address,
            Address::from_str("0x4e001110D16bE154EB586e73d2da823721E1a9cD").unwrap()
        );
    }

    #[test]
    fn test_get_contract_address_with_override() {
        let config = Config {
            sequencing_addresses: HashMap::from([(
                510001,
                Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            )]),
            ..Default::default()
        };
        let service = TCClient::new(&config).unwrap();

        let contract_address = service.get_contract_address(510001).unwrap();
        assert_eq!(
            contract_address,
            Address::from_str("0x0000000000000000000000000000000000000001").unwrap()
        );
    }

    #[tokio::test]
    async fn test_process_transaction() {
        let service = setup_test_service();
        let test_tx = Bytes::from_str("0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772").unwrap();

        let result = service.process_transaction(test_tx).await;
        // This will fail since we're not connected to a real node
        assert!(result.is_err());
    }
}
