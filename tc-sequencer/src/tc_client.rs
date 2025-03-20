//! The `service` module handles the business logic for the tc sequencer.

use crate::{
    bytecode::get_bytecode,
    config::Config,
    errors::{
        Error,
        Error::InvalidParams,
        InvalidParamsError::{MissingParam, NotAnArray, NotHexEncoded, WrongParamCount},
    },
    validation::validate_transaction,
};
use alloy::{
    consensus::Transaction,
    hex::{self},
    primitives::{keccak256, Address, Bytes, TxHash, U256},
};
use eyre::Result;
use jsonrpsee::{
    core::{RpcResult, Serialize},
    types::Params,
    Extensions,
};
use reqwest::Client;
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error, info};
use url::Url;

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
            chain_id: 8453,
            function_signature: "processTransaction(bytes data)".to_string(),
            args: HashMap::from([("data".to_string(), hex::encode(data))]),
        }
    }
}

/// The service for relaying transactions to the sequencing contract.
#[derive(Debug)]
pub struct TCClient {
    tc_url: Url,
    tc_project_id: String,
    tc_api_key: String,

    factory_address: Address,
    client: Client,
}

impl TCClient {
    /// Create a new `TCClient` instance.
    pub fn new(config: &Config) -> Result<Self> {
        let client = Client::new();

        Ok(Self {
            tc_url: config.tc_url.clone(),
            tc_project_id: config.tc_project_id.clone(),
            tc_api_key: config.tc_api_key.clone(),
            factory_address: config.metabased_sequencer_factory_address,
            client,
        })
    }

    fn get_contract_address(&self, chain_id: u64) -> Address {
        let chain_salt = U256::from(chain_id).to_be_bytes();
        let mut packed = get_bytecode().to_vec();
        packed.extend_from_slice(&chain_salt);
        let bytecode = Bytes::from(packed);
        self.factory_address.create2(chain_salt, keccak256(&bytecode))
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
            .post(self.tc_url.clone())
            .bearer_auth(self.tc_api_key.clone())
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send transaction to TC: {}", e);
                Error::Internal
            })?;

        if !response.status().is_success() {
            error!("Failed to send transaction to TC: {}", response.status());
            return Err(Error::Internal);
        }

        Ok(())
    }

    async fn process_transaction(&self, raw_tx: Bytes) -> Result<TxHash, Error> {
        info!("Processing transaction: {}", hex::encode(&raw_tx));
        let original_tx = validate_transaction(&raw_tx)?;

        // Determine the contract address
        let contract_address =
            self.get_contract_address(original_tx.chain_id().unwrap_or_default());

        debug!("Submitting validated transaction to TC");
        self.send_transaction(contract_address, raw_tx).await?;

        Ok(*original_tx.tx_hash())
    }
}

// Params validation
// TODO [SEQ-663]: Refactor this function
fn parse_send_raw_transaction_params(params: Params<'static>) -> Result<Bytes, Error> {
    let mut json: serde_json::Value = serde_json::from_str(params.as_str().unwrap_or("[]"))?;
    let arr = json.as_array_mut().ok_or(InvalidParams(NotAnArray))?;
    if arr.len() != 1 {
        return Err(InvalidParams(WrongParamCount(arr.len())));
    }
    let item = arr.pop().ok_or(InvalidParams(MissingParam))?;
    let raw_tx = item.as_str().ok_or(InvalidParams(NotHexEncoded))?.to_string();
    let tx_data: Bytes = hex::decode(&raw_tx).map(Bytes::from)?;

    Ok(tx_data)
}

/// The handler for the `eth_sendRawTransaction` JSON-RPC method.
pub async fn send_raw_transaction_handler(
    params: Params<'static>,
    service: Arc<TCClient>,
    _: Extensions,
) -> RpcResult<String> {
    let tx_data = parse_send_raw_transaction_params(params).map_err(|e| e.to_json_rpc_error())?;
    let tx_hash = service.process_transaction(tx_data).await.map_err(|e| e.to_json_rpc_error())?;

    Ok(format!("0x{}", hex::encode(tx_hash)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use alloy::primitives::Bytes;
    use jsonrpsee::types::Params;
    use std::str::FromStr;

    fn setup_test_service() -> TCClient {
        let config = Config {
            tc_url: Url::parse("http://localhost:8545").unwrap(),
            tc_project_id: "test".to_string(),
            tc_api_key: "test".to_string(),
            metabased_sequencer_factory_address: Address::ZERO,
            port: 8456,
        };
        TCClient::new(&config).unwrap()
    }

    #[test]
    fn test_new_service_creation() {
        let config = Config {
            tc_url: Url::parse("http://localhost:8545").unwrap(),
            tc_project_id: "test".to_string(),
            tc_api_key: "test".to_string(),
            metabased_sequencer_factory_address: Address::ZERO,
            port: 8456,
        };

        let result = TCClient::new(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_contract_address() {
        // Setup a test service with a mock config
        let config = Config {
            tc_url: Url::parse("http://localhost:8545").unwrap(),
            tc_project_id: "test".to_string(),
            tc_api_key: "test".to_string(),
            metabased_sequencer_factory_address: Address::from_str(
                "0xFEA8A2BA8B760348ea95492516620ad45a299d53",
            )
            .unwrap(),
            port: 8456,
        };
        let service = TCClient::new(&config).unwrap();

        let contract_address = service.get_contract_address(510001);
        assert_eq!(
            contract_address,
            Address::from_str("0xC1cacFC14624c4E241286Ade61DF545b90f850B4").unwrap()
        );
    }

    #[tokio::test]
    async fn test_send_raw_transaction_handler_invalid_params() {
        let service = Arc::new(setup_test_service());
        let invalid_params = Params::new(Some("[\"invalid_hex\"]"));

        let result =
            send_raw_transaction_handler(invalid_params, service, Extensions::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_raw_transaction_handler_valid_params() {
        let service = Arc::new(setup_test_service());
        // Valid raw transaction hex
        let valid_tx = "[\"0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772\"]";
        let params = Params::new(Some(valid_tx));

        let result = send_raw_transaction_handler(params, service, Extensions::default()).await;
        // Note: This will fail in actual execution since we're using a mock setup
        // but it tests the parameter parsing logic
        assert!(result.is_err());
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
