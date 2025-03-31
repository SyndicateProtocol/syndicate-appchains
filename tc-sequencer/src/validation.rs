//! Utility function for validating transactions

use alloy::{
    consensus::{Transaction, TxEnvelope},
    primitives::{Bytes, U256},
    rlp::Decodable,
};
use eyre::Result;
use shared::json_rpc::{
    Error,
    Error::{InvalidInput, TransactionRejected},
    InvalidInputError::{MissingChainID, UnableToRLPDecode},
    Rejection::FeeTooHigh,
};
use tracing::debug;

fn decode_transaction(raw_tx: &Bytes) -> Result<TxEnvelope, Error> {
    let mut slice: &[u8] = raw_tx.as_ref();
    TxEnvelope::decode(&mut slice).map_err(|_| {
        let error = InvalidInput(UnableToRLPDecode);
        debug!(error = %error, "Transaction decoding failed");
        error
    })
}

fn check_chain_id(tx: &TxEnvelope) -> Result<(), Error> {
    if tx.chain_id().is_none() {
        let error = InvalidInput(MissingChainID);
        debug!(
            error = %error,
            tx_type = ?tx.tx_type(),
            "Transaction validation failed: missing chain ID"
        );
        return Err(error);
    }

    Ok(())
}

fn check_signature(tx: &TxEnvelope) -> Result<(), Error> {
    tx.recover_signer().map_err(|e| {
        debug!(
            error = ?e,
            tx_type = ?tx.tx_type(),
            "Transaction validation failed: invalid signature"
        );
        e
    })?;
    Ok(())
}

fn check_gas_price(tx: &TxEnvelope) -> Result<(), Error> {
    // TODO(SEQ-179): introduce optional global tx cap config. See op-geth's checkTxFee() +
    // RPCTxFeeCap for equivalent skip check if unset
    let tx_fee_cap_in_wei = U256::from(1_000_000_000_000_000_000u64); // 1e18wei = 1 ETH

    let gas_price = U256::try_from(tx.max_fee_per_gas())?;
    let gas = U256::try_from(tx.gas_limit())?;
    let fee_wei = gas_price.saturating_mul(gas);

    if fee_wei > tx_fee_cap_in_wei {
        return Err(TransactionRejected(FeeTooHigh));
    }
    Ok(())
}

/// Validate a transaction
pub fn validate_transaction(raw_tx: &Bytes) -> Result<TxEnvelope, Error> {
    debug!(bytes_length = raw_tx.len(), "Starting transaction validation");
    // Decoding:
    let tx = decode_transaction(raw_tx)?;

    // Validation Checks:
    // Check chain ID
    check_chain_id(&tx)?;

    // Check signature
    check_signature(&tx)?;

    // Check gas price
    check_gas_price(&tx)?;

    Ok(tx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        consensus::{Signed, TxEip1559, TxLegacy},
        primitives::{b256, Bytes, PrimitiveSignature},
    };
    use std::str::FromStr;

    #[test]
    fn test_validate_transaction() {
        let valid_tx = Bytes::from_str("0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772").unwrap();

        let result = validate_transaction(&valid_tx);
        // The validation should pass since this is a valid RLP-encoded transaction
        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(
            tx.tx_hash().to_string(),
            "0xc429e5f128387d224ba8bed6885e86525e14bfdc2eb24b5e9c3351a1176fd81f"
        );
    }
    #[test]
    fn test_decode_transaction() {
        // Valid transaction
        let valid_tx = Bytes::from_str("0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772").unwrap();
        let result = decode_transaction(&valid_tx);
        assert!(result.is_ok());

        // Invalid transaction
        let invalid_tx = Bytes::from_str("0x1234").unwrap();
        let result = decode_transaction(&invalid_tx);
        assert!(matches!(result, Err(InvalidInput(UnableToRLPDecode))));
    }

    fn wrap_txn_legacy(tx: TxLegacy) -> TxEnvelope {
        TxEnvelope::Legacy(Signed::new_unchecked(
            tx,
            PrimitiveSignature::test_signature(),
            Default::default(),
        ))
    }

    fn wrap_txn_eip1559(tx: TxEip1559) -> TxEnvelope {
        TxEnvelope::Eip1559(Signed::new_unchecked(
            tx,
            PrimitiveSignature::test_signature(),
            Default::default(),
        ))
    }

    #[test]
    fn test_check_chain_id() {
        // Valid legacy transaction
        let valid_tx = wrap_txn_legacy(TxLegacy { chain_id: Some(1), ..Default::default() });
        assert!(check_chain_id(&valid_tx).is_ok());

        // Legacy ransaction without chain ID should fail
        let invalid_tx = wrap_txn_legacy(TxLegacy { chain_id: None, ..Default::default() });
        assert!(matches!(check_chain_id(&invalid_tx), Err(InvalidInput(MissingChainID))));
    }

    #[test]
    fn test_check_signature() {
        // Valid transaction with valid signature
        let valid_tx = wrap_txn_legacy(TxLegacy::default());
        assert!(check_signature(&valid_tx).is_ok());

        // Transaction with invalid signature should fail
        let invalid_tx = TxEnvelope::Legacy(Signed::new_unchecked(
            TxLegacy::default(),
            PrimitiveSignature::from_scalars_and_parity(
                b256!("0x0000000000000000000000000000000000000000000000000000000000000000"),
                b256!("0x0000000000000000000000000000000000000000000000000000000000000000"),
                true,
            ),
            Default::default(),
        ));
        assert!(check_signature(&invalid_tx).is_err());
    }

    #[test]
    fn test_check_gas_price() {
        // Valid transaction with acceptable gas price
        let valid_tx = wrap_txn_legacy(TxLegacy::default());
        assert!(check_gas_price(&valid_tx).is_ok());

        // Legacy transaction with extremely high gas price should fail
        let invalid_legacy_tx = wrap_txn_legacy(TxLegacy {
            gas_limit: 2u64,
            gas_price: 1_000_000_000_000_000_000u128,
            ..Default::default()
        });
        assert!(matches!(
            check_gas_price(&invalid_legacy_tx),
            Err(TransactionRejected(FeeTooHigh))
        ));

        // EIP-1559 transaction with extremely high gas price should fail
        let invalid_eip1559_tx = wrap_txn_eip1559(TxEip1559 {
            gas_limit: 2u64,
            max_fee_per_gas: 1_000_000_000_000_000_000u128,
            ..Default::default()
        });
        assert!(matches!(
            check_gas_price(&invalid_eip1559_tx),
            Err(TransactionRejected(FeeTooHigh))
        ));
    }
}
