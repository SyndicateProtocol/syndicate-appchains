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
pub async fn send_raw_transaction<Chain>(encoded: Bytes, chain: &Chain) -> Result<TxHash, Error>
where
    Chain: MetabasedSequencerChainService,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
{
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

    // 3. Submission/forwarding:
    Ok(chain.process_transaction(encoded).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::primitives::Bytes;
    use crate::domain::InMemoryMetabasedSequencerChain;
    use alloy::consensus::{SignableTransaction, TxEip1559, TxEnvelope};
    use alloy::network::TxSignerSync;
    use alloy::primitives::TxKind;
    use alloy::signers::local::PrivateKeySigner;
    use alloy_primitives::private::alloy_rlp::Encodable;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    const PRIVATE_KEY: [u8; 32] = [0x11; 32];

    fn create_transaction(to: TxKind, input: Vec<u8>) -> TxEnvelope {
        let mut tx = TxEip1559 {
            chain_id: 0,
            nonce: 0,
            gas_limit: u64::MAX,
            max_fee_per_gas: 0,
            max_priority_fee_per_gas: 0,
            to,
            value: Default::default(),
            access_list: Default::default(),
            input: input.into(),
        };
        let signer = PrivateKeySigner::from_bytes(&PRIVATE_KEY.into()).unwrap();
        let signature = signer.sign_transaction_sync(&mut tx).unwrap();

        TxEnvelope::Eip1559(tx.into_signed(signature))
    }

    #[tokio::test]
    async fn test_send_raw_transaction_writes_tx_to_metabased_chain() {
        let tx = create_transaction(TxKind::Create, Vec::new());
        let mut encoded_tx = Vec::new();
        tx.encode(&mut encoded_tx);
        let encoded_tx = Bytes::from(encoded_tx);

        let transactions = Arc::new(RwLock::new(Vec::new()));
        let chain = InMemoryMetabasedSequencerChain::new(transactions.clone());

        send_raw_transaction(encoded_tx.clone(), &chain)
            .await
            .unwrap();

        let expected_transactions = vec![encoded_tx];
        let actual_transactions = transactions.read().await.clone();

        assert_eq!(actual_transactions, expected_transactions);
    }
}
