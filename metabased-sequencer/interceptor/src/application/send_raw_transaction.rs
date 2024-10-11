use crate::presentation::json_rpc_errors::Error;
use crate::presentation::json_rpc_errors::Error::InvalidInput;
use crate::presentation::transaction;
use alloy::consensus::{Transaction, TxEnvelope, TxType};
use alloy::primitives::private::alloy_rlp::Decodable;
use alloy::primitives::TxHash;
use alloy_primitives::U256;
use bytes::Bytes;

/// Sends serialized and signed transaction `tx`.
pub fn send_raw_transaction(tx: Bytes) -> Result<TxHash, Error> {
    // TODO remove or move up to function comment
    // 1. Decoding
    // 2. Validation
    // 3. Submission/forwarding

    // 1. Decoding:
    let mut slice: &[u8] = tx.as_ref();
    let tx = TxEnvelope::decode(&mut slice)?;

    // 2. Validation:
    tx.recover_signer()?;

    if tx.tx_type() == TxType::Legacy {
        // TODO(SEQ-179): introduce optional global tx cap config. See op-geth's checkTxFee() + RPCTxFeeCap for equivalent
        // skip check if unset
        let tx_cap_in_wei = U256::from(1_000_000_000_000_000_000u64); // 1e18wei = 1 ETH
        let gas_price = tx
            .gas_price()
            .ok_or_else(|| InvalidInput("Legacy transaction missing gas price".to_string()))?;
        transaction::check_tx_fee(
            U256::try_from(gas_price)?,
            U256::try_from(tx.gas_limit())?,
            tx_cap_in_wei,
        )?;
    }

    Ok(tx.tx_hash().to_owned())
}
