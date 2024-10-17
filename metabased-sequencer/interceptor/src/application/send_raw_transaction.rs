use crate::application::Metrics;
use crate::domain::primitives::Bytes;
use crate::domain::MetabasedSequencerChainService;
use crate::presentation::json_rpc_errors::Error;
use crate::presentation::json_rpc_errors::Error::InvalidInput;
use crate::presentation::json_rpc_errors::InvalidInputError::MissingGasPrice;
use crate::presentation::transaction;
use alloy::consensus::{Transaction, TxEnvelope, TxType};
use alloy::primitives::private::alloy_rlp::Decodable;
use alloy::primitives::TxHash;
use alloy::primitives::U256;

/// Sends serialized and signed transaction `tx` using `chain`.
pub async fn send_raw_transaction<Chain, M: Metrics>(
    encoded: Bytes,
    chain: &Chain,
    metrics: &M,
) -> Result<TxHash, Error>
where
    Chain: MetabasedSequencerChainService,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
{
    metrics.inc_send_raw_transaction();

    // TODO remove or move up to function comment
    // 1. Decoding
    // 2. Validation
    // 3. Submission/forwarding

    // 1. Decoding:
    let mut slice: &[u8] = encoded.as_ref();
    let tx = TxEnvelope::decode(&mut slice)?;

    // 2. Validation:
    tx.recover_signer()?;

    if tx.tx_type() == TxType::Legacy {
        // TODO(SEQ-179): introduce optional global tx cap config. See op-geth's checkTxFee() + RPCTxFeeCap for equivalent
        // skip check if unset
        let tx_cap_in_wei = U256::from(1_000_000_000_000_000_000u64); // 1e18wei = 1 ETH
        let gas_price = tx.gas_price().ok_or(InvalidInput(MissingGasPrice))?;
        transaction::check_tx_fee(
            U256::try_from(gas_price)?,
            U256::try_from(tx.gas_limit())?,
            tx_cap_in_wei,
        )?;
    }

    chain.process_transaction(encoded).await?;

    Ok(tx.tx_hash().to_owned())
}
