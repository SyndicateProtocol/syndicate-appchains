use alloy::{
    consensus::{SignableTransaction, TxEip1559, TxLegacy},
    eips::Encodable2718,
    hex,
    primitives::{Address, Bytes, Signature, TxKind, U256},
};

/// Creates a sample legacy Ethereum transaction for testing
pub fn create_legacy_transaction(chain_id: u64, nonce: u64) -> Bytes {
    // Create a legacy transaction with common test values
    let tx = TxLegacy {
        chain_id: Some(chain_id),
        nonce,
        gas_price: 0x77359400, // 2 Gwei in hex
        gas_limit: 0x5208,     // 21000 in decimal (standard ETH transfer)
        to: TxKind::from(Address::from_slice(
            &hex::decode("4592d8f8d7b001e72cb26a73e4fa1806a51ac79d").unwrap(),
        )),
        value: U256::from_str_radix("0de0b6b3a7640000", 16).unwrap(), // 1 ETH in wei
        input: Bytes::new(),                                          // Empty input data
    };

    // Create a fixed test signature (this matches the one in your test)
    let r = U256::from_str_radix(
        "5924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562",
        16,
    )
    .unwrap();
    let s = U256::from_str_radix(
        "01b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772",
        16,
    )
    .unwrap();
    let signature = Signature::new(r, s, true);

    // Create the signed transaction
    let signed_tx = tx.into_signed(signature);

    // Encode the transaction
    Bytes::from(signed_tx.encoded_2718())
}

/// Creates an EIP-1559 transaction for testing
pub fn create_eip1559_transaction(chain_id: u64, nonce: u64) -> Bytes {
    // Create an EIP-1559 transaction
    let tx = TxEip1559 {
        chain_id,
        nonce,
        max_priority_fee_per_gas: 1_000_000_000, // 1 Gwei
        max_fee_per_gas: 2_000_000_000,          // 2 Gwei
        gas_limit: 21000,                        // Standard ETH transfer
        to: TxKind::from(Address::from_slice(
            &hex::decode("4592d8f8d7b001e72cb26a73e4fa1806a51ac79d").unwrap(),
        )),
        value: U256::from(1_000_000_000_000_000_000u64), // 1 ETH
        input: Bytes::new(),
        access_list: vec![].into(),
    };

    // Create a test signature
    let r = U256::from_str_radix(
        "5924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562",
        16,
    )
    .unwrap();
    let s = U256::from_str_radix(
        "01b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772",
        16,
    )
    .unwrap();
    let signature = Signature::new(r, s, false);

    // Create the signed transaction
    let signed_tx = tx.into_signed(signature);

    // Encode the transaction
    Bytes::from(signed_tx.encoded_2718())
}

/// Returns a valid RLP-encoded transaction hex string with 0x prefix
pub fn get_legacy_transaction_hex(chain_id: u64, nonce: u64) -> String {
    let tx_bytes = create_legacy_transaction(chain_id, nonce);
    format!("0x{}", hex::encode(tx_bytes))
}

/// Returns a valid EIP-1559 transaction hex string with 0x prefix
pub fn get_eip1559_transaction_hex(chain_id: u64, nonce: u64) -> String {
    let tx_bytes = create_eip1559_transaction(chain_id, nonce);
    format!("0x{}", hex::encode(tx_bytes))
}

/// Get the known valid test transaction used in unit tests
pub fn get_known_valid_transaction() -> String {
    "0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_encoding() {
        let tx_hex = get_legacy_transaction_hex(1, 0x02b2);
        assert_eq!(
            tx_hex,
            "0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a76400008026a05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772"
        );
    }
}
