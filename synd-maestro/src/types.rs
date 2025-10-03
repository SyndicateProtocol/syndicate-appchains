//! Types used by the `synd-maestro` package.

use alloy::{
    consensus::{Transaction, TxEnvelope},
    hex,
    primitives::Address,
};
use serde::{Deserialize, Serialize};

/// Structured representation of a `TxEnvelope` for logging
#[derive(Debug, Serialize, Deserialize)]
struct TxEnvelopeLog {
    hash: String,
    signer_address: String,
    tx_type: String,
    chain_id: u64,
    nonce: u64,
    to: String,
    gas_limit: u64,
    max_fee_per_gas: u128,
    max_priority_fee_per_gas: u128,
    gas_price: u128,
    value: String, // Using string to handle U256 properly
    input_len: usize,
}

/// Formats a `TxEnvelope` for human-friendly logging as JSON.
pub fn fmt_tx_envelope_for_logging(tx: &TxEnvelope, signer: &Address) -> String {
    let tx_log = TxEnvelopeLog {
        hash: format!("0x{}", hex::encode(tx.hash())),
        signer_address: signer.to_string(),
        tx_type: match tx {
            TxEnvelope::Legacy(_) => "legacy",
            TxEnvelope::Eip2930(_) => "eip2930",
            TxEnvelope::Eip1559(_) => "eip1559",
            TxEnvelope::Eip4844(_) => "eip4844",
            _ => "unknown",
        }
        .to_string(),
        chain_id: tx.chain_id().unwrap_or_default(),
        nonce: tx.nonce(),
        to: tx.to().map_or_else(|| "none".to_string(), |to| format!("0x{}", hex::encode(to))),
        gas_limit: tx.gas_limit(),
        max_fee_per_gas: tx.max_fee_per_gas(),
        max_priority_fee_per_gas: tx.max_priority_fee_per_gas().unwrap_or_default(),
        gas_price: tx.gas_price().unwrap_or_default(),
        value: tx.value().to_string(), // Convert U256 to string for JSON
        input_len: tx.input().len(),
    };

    // Serialize to JSON string
    serde_json::to_string(&tx_log).unwrap_or_else(|e| {
        format!("failed to serialize TxEnvelopeLog to JSON - this should never happen! {e}")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        network::{EthereumWallet, TransactionBuilder},
        primitives::{utils::parse_ether, Address, Bytes, U256},
        rpc::types::TransactionRequest,
        signers::local::PrivateKeySigner,
    };
    use serde_json::Value;
    use shared::tx_validation::{check_signature, decode_transaction};
    use test_utils::transaction::create_legacy_transaction;
    use tracing::info;

    #[tokio::test]
    async fn test_fmt_tx_envelope_for_logging_json_structure() {
        // Create a fully populated EIP-1559 transaction
        let signer = PrivateKeySigner::random();
        let wallet = EthereumWallet::from(signer);

        let chain_id = 1u64;
        let nonce = 100u64;
        let to_address = Address::from([0x12; 20]);
        let value = parse_ether("1.5").unwrap();
        let gas_limit = 21000u64;
        let max_fee_per_gas = 50_000_000_000u128; // 50 gwei
        let max_priority_fee_per_gas = 2_000_000_000u128; // 2 gwei
        let input_data = Bytes::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);

        let tx = TransactionRequest::default()
            .with_to(to_address)
            .with_value(value)
            .with_nonce(nonce)
            .with_gas_limit(gas_limit)
            .with_max_fee_per_gas(max_fee_per_gas)
            .with_max_priority_fee_per_gas(max_priority_fee_per_gas)
            .with_chain_id(chain_id)
            .with_input(input_data.clone())
            .build(&wallet)
            .await
            .expect("Failed to build transaction");

        let signer_address = wallet.default_signer().address();
        let formatted = fmt_tx_envelope_for_logging(&tx, &signer_address);

        // Parse as JSON and verify structure
        let json: Value = serde_json::from_str(&formatted).expect("Should be valid JSON");

        // Verify all expected fields are present
        assert!(json.get("hash").is_some());
        assert!(json.get("signer_address").is_some());
        assert!(json.get("tx_type").is_some());
        assert!(json.get("chain_id").is_some());
        assert!(json.get("nonce").is_some());
        assert!(json.get("to").is_some());
        assert!(json.get("gas_limit").is_some());
        assert!(json.get("max_fee_per_gas").is_some());
        assert!(json.get("max_priority_fee_per_gas").is_some());
        assert!(json.get("gas_price").is_some());
        assert!(json.get("value").is_some());
        assert!(json.get("input_len").is_some());

        // Verify specific values
        assert_eq!(json["tx_type"], "eip1559");
        assert_eq!(json["chain_id"], chain_id);
        assert_eq!(json["nonce"], nonce);
        assert_eq!(json["to"], format!("0x{}", hex::encode(to_address)));
        assert_eq!(json["gas_limit"], gas_limit);
        assert_eq!(json["max_fee_per_gas"].to_string(), max_fee_per_gas.to_string());
        assert_eq!(
            json["max_priority_fee_per_gas"].to_string(),
            max_priority_fee_per_gas.to_string()
        );
        assert_eq!(json["value"], value.to_string());
        assert_eq!(json["input_len"], input_data.len());
        assert_eq!(json["signer_address"], signer_address.to_string());

        // Verify hash format
        let hash_str = json["hash"].as_str().unwrap();
        assert!(hash_str.starts_with("0x"));
        assert_eq!(hash_str.len(), 66); // 0x + 64 hex chars
        info!(tx_log=%fmt_tx_envelope_for_logging(&tx, &signer_address), "sample log");
    }

    #[tokio::test]
    async fn test_fmt_tx_envelope_for_logging_with_legacy_transaction() {
        let chain_id = 4u64;
        let nonce = 42u64;
        let raw_tx = create_legacy_transaction(chain_id, nonce);

        let tx = decode_transaction(&raw_tx).expect("Failed to decode transaction");
        let signer = check_signature(&tx).expect("Failed to check signature");

        let formatted = fmt_tx_envelope_for_logging(&tx, &signer);

        // Verify it's valid JSON
        let json: Value = serde_json::from_str(&formatted).expect("Should be valid JSON");

        assert_eq!(json["tx_type"], "legacy");
        assert_eq!(json["chain_id"], chain_id);
        assert_eq!(json["nonce"], nonce);
        assert!(json["hash"].as_str().unwrap().starts_with("0x"));
        info!(tx_log=%fmt_tx_envelope_for_logging(&tx, &signer), "sample log");
    }

    #[tokio::test]
    async fn test_fmt_tx_envelope_for_logging_contract_creation() {
        let signer = PrivateKeySigner::random();
        let wallet = EthereumWallet::from(signer);

        let tx = TransactionRequest::default()
            .with_value(U256::ZERO)
            .with_to(Address::ZERO)
            .with_nonce(0)
            .with_gas_limit(2_000_000u64)
            .with_max_fee_per_gas(20_000_000_000u128)
            .with_max_priority_fee_per_gas(1_000_000_000u128)
            .with_chain_id(1u64)
            .with_input(Bytes::from(vec![0x60, 0x80, 0x60, 0x40, 0x52]))
            .build(&wallet)
            .await
            .expect("Failed to build contract creation transaction");

        let signer_address = wallet.default_signer().address();
        let formatted = fmt_tx_envelope_for_logging(&tx, &signer_address);

        let json: Value = serde_json::from_str(&formatted).expect("Should be valid JSON");

        // Contract creation should have "none" as the to address
        assert_eq!(json["to"], "0x0000000000000000000000000000000000000000");
        assert_eq!(json["input_len"], 5);
        info!(tx_log=%fmt_tx_envelope_for_logging(&tx, &signer_address), "sample log");
    }

    #[tokio::test]
    async fn test_fmt_tx_envelope_for_logging_deserializable() {
        let signer = PrivateKeySigner::random();
        let wallet = EthereumWallet::from(signer);

        let tx = TransactionRequest::default()
            .with_to(Address::ZERO)
            .with_value(parse_ether("2.5").unwrap())
            .with_nonce(99)
            .with_gas_limit(30000)
            .with_max_fee_per_gas(15_000_000_000u128)
            .with_max_priority_fee_per_gas(1_500_000_000u128)
            .with_chain_id(42u64)
            .build(&wallet)
            .await
            .expect("Failed to build transaction");

        let signer_address = wallet.default_signer().address();
        let formatted = fmt_tx_envelope_for_logging(&tx, &signer_address);

        // Deserialize back to our struct to verify it's complete
        let tx_log: TxEnvelopeLog =
            serde_json::from_str(&formatted).expect("Should deserialize back to TxEnvelopeLog");

        assert_eq!(tx_log.tx_type, "eip1559");
        assert_eq!(tx_log.chain_id, 42);
        assert_eq!(tx_log.nonce, 99);
        assert_eq!(tx_log.gas_limit, 30000);
        assert_eq!(tx_log.max_fee_per_gas, 15_000_000_000u128);
        assert_eq!(tx_log.max_priority_fee_per_gas, 1_500_000_000u128);
        assert_eq!(tx_log.value, parse_ether("2.5").unwrap().to_string());
        assert_eq!(tx_log.to, "0x0000000000000000000000000000000000000000");
        assert_eq!(tx_log.signer_address, signer_address.to_string());
        info!(tx_log=%fmt_tx_envelope_for_logging(&tx, &signer_address), "sample log");
    }
}
