use alloy::consensus::TxEnvelope;
use alloy::primitives::private::alloy_rlp::Decodable;
use alloy::primitives::TxHash;
use bytes::Bytes;

/// Sends serialized and signed transaction `tx`.
pub fn send_raw_transaction(tx: Bytes) -> anyhow::Result<TxHash> {
    let mut slice: &[u8] = tx.as_ref();
    let tx = TxEnvelope::decode(&mut slice)?;

    // TODO validate tx

    Ok(tx.tx_hash().to_owned())
}
