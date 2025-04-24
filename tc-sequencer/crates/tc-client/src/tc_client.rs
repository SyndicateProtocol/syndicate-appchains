//! The `service` module handles the business logic for the tc sequencer.

use crate::config::TCConfig;
use alloy::{
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
use serde::{self, Deserialize};
use shared::{
    json_rpc::{parse_send_raw_transaction_params, RpcError},
    tx_validation::validate_transaction,
};
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error, info};
use url::Url;

const DEFAULT_SEQUENCING_CHAIN_ID: u64 = 5113;
/// The function signature for the transaction function
const DEFAULT_FUNCTION_SIGNATURE: &str = "processTransaction(address chainAddress, bytes data)";
/// The function signature for the batch function
pub const BATCH_FUNCTION_SIGNATURE: &str =
    "processTransactionRaw(address chainAddress, bytes[] data)";
const TC_CHAIN_ADDRESS_KEY: &str = "chainAddress";
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendTransactionResponse {
    transaction_id: String,
}

impl SendTransactionRequest {
    fn new(
        project_id: String,
        contract_address: Address,
        chain_address: Address,
        data: Bytes,
        function_signature: String,
    ) -> Self {
        Self {
            project_id,
            contract_address: contract_address.to_string(),
            chain_id: DEFAULT_SEQUENCING_CHAIN_ID,
            function_signature,
            args: HashMap::from([
                (TC_CHAIN_ADDRESS_KEY.to_string(), chain_address.to_string()),
                (TC_DATA_KEY.to_string(), format!("0x{}", hex::encode(data))),
            ]),
        }
    }
}

/// The service for relaying transactions to the sequencing contract.
#[derive(Debug, Clone)]
pub struct TCClient {
    tc_url: Url,
    tc_project_id: String,
    tc_api_key: String,

    wallet_pool_address: Address,
    sequencing_address: Address,
    client: Client,
}

impl TCClient {
    /// Create a new `TCClient` instance.
    pub fn new(config: &TCConfig) -> Result<Self> {
        let client = Client::new();
        Ok(Self {
            tc_url: config.tc_endpoint.get_url(),
            tc_project_id: config.tc_project_id.clone(),
            tc_api_key: config.tc_api_key.clone(),
            wallet_pool_address: config.wallet_pool_address,
            sequencing_address: config.sequencing_address,
            client,
        })
    }
    async fn send_transaction(
        &self,
        raw_tx: Bytes,
        function_signature: String,
    ) -> Result<String, RpcError> {
        let request = SendTransactionRequest::new(
            self.tc_project_id.clone(),
            self.wallet_pool_address,
            self.sequencing_address,
            raw_tx,
            function_signature,
        );

        let response = self
            .client
            .post(format!("{}/transact/sendTransaction", self.tc_url))
            .bearer_auth(self.tc_api_key.clone())
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send transaction to TC: {}", e);
                RpcError::Internal("failed to submit transaction to sequencer".to_string())
            })?;

        if !response.status().is_success() {
            let error_msg = response.text().await.unwrap_or_default();
            error!("Failed to send transaction to TC: {}", error_msg);
            return Err(RpcError::Internal("failed to submit transaction to sequencer".to_string()));
        }

        let response_body: SendTransactionResponse = response.json().await.map_err(|e| {
            error!("Failed to parse response from TC: {}", e);
            RpcError::Internal("failed to parse response from sequencer".to_string())
        })?;

        Ok(response_body.transaction_id)
    }

    /// Process a transaction by submitting it to the TC
    pub async fn process_transaction(
        &self,
        raw_tx: Bytes,
        function_signature: String,
    ) -> Result<TxHash, RpcError> {
        info!("Processing transaction: {}", hex::encode(&raw_tx));
        let original_tx = validate_transaction(&raw_tx)?;
        let original_tx_hash = *original_tx.tx_hash();
        let raw_tx_clone = raw_tx.clone();
        let transaction_id = self.send_transaction(raw_tx, function_signature).await?;
        debug!(
            "Transaction submitted successfully - Original raw tx: 0x{}, Original tx hash: 0x{}, TC transaction ID: {}",
            hex::encode(&raw_tx_clone),
            hex::encode(original_tx_hash),
            transaction_id
        );

        Ok(original_tx_hash)
    }
}

/// The handler for the `eth_sendRawTransaction` JSON-RPC method.
pub async fn send_raw_transaction_handler(
    params: Params<'static>,
    service: Arc<TCClient>,
    _: Extensions,
) -> RpcResult<String> {
    let tx_data = parse_send_raw_transaction_params(params)?;
    let tx_hash =
        service.process_transaction(tx_data, DEFAULT_FUNCTION_SIGNATURE.to_string()).await?;

    Ok(format!("0x{}", hex::encode(tx_hash)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TCConfig;
    use alloy::primitives::Bytes;
    use std::str::FromStr;

    fn setup_test_service() -> TCClient {
        let config = TCConfig::default();
        TCClient::new(&config).unwrap()
    }

    #[test]
    fn test_new_service_creation() {
        let config = TCConfig::default();

        let result = TCClient::new(&config);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_transaction() {
        let service = setup_test_service();
        let test_tx = Bytes::from_str("0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772").unwrap();

        let result =
            service.process_transaction(test_tx, DEFAULT_FUNCTION_SIGNATURE.to_string()).await;
        // This will fail since we're not connected to a real node
        assert!(result.is_err());
    }
}
