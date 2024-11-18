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
    use alloy::consensus::{SignableTransaction, TxEip1559, TxEnvelope, TxLegacy};
    use alloy::network::TxSignerSync;
    use alloy::primitives::TxKind;
    use alloy::signers::local::PrivateKeySigner;
    use alloy_primitives::private::alloy_rlp::Encodable;
    use alloy_primitives::{b256, Signature};
    use async_trait::async_trait;
    use std::fmt::{Debug, Display, Formatter};
    use std::sync::Arc;
    use test_case::test_case;
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
    async fn test_send_raw_transaction_writes_tx_to_metabased_chain_successfully() {
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

    fn encode_legacy_transaction_without_chain_id() -> Vec<u8> {
        let mut tx = TxLegacy {
            chain_id: None,
            nonce: 0,
            gas_price: u128::MAX,
            gas_limit: u64::MAX,
            to: TxKind::Create,
            value: Default::default(),
            input: Default::default(),
        };
        let signer = PrivateKeySigner::from_bytes(&PRIVATE_KEY.into()).unwrap();
        let signature = signer.sign_transaction_sync(&mut tx).unwrap();

        let tx = TxEnvelope::Legacy(tx.into_signed(signature));
        let mut encoded_tx = Vec::new();
        tx.encode(&mut encoded_tx);
        encoded_tx
    }

    #[tokio::test]
    #[test_case(vec![0, 1, 2, 3]; "Non-rlp")]
    #[test_case(vec![]; "Empty")]
    #[test_case(encode_legacy_transaction_without_chain_id(); "Legacy transaction without chain ID")]
    async fn test_decoding_invalid_rlp_transaction_fails(encoded_tx: Vec<u8>) {
        let encoded_tx = Bytes::from(encoded_tx);

        let transactions = Arc::new(RwLock::new(Vec::new()));
        let chain = InMemoryMetabasedSequencerChain::new(transactions.clone());

        let error = send_raw_transaction(encoded_tx.clone(), &chain)
            .await
            .unwrap_err();

        let expected_error = "invalid input: unable to RLP decode";
        let actual_error = error.to_string();

        assert_eq!(actual_error, expected_error);
    }

    #[tokio::test]
    async fn test_sending_transaction_singed_with_wrong_signature_fails() {
        fn create_transaction_with_wrong_signature() -> TxEnvelope {
            let tx = TxEip1559 {
                chain_id: 0,
                nonce: 0,
                gas_limit: u64::MAX,
                max_fee_per_gas: 0,
                max_priority_fee_per_gas: 0,
                to: TxKind::Create,
                value: Default::default(),
                access_list: Default::default(),
                input: Default::default(),
            };
            let signature = Signature::from_scalars_and_parity(
                b256!("0000000000000000000000000000000000000000000000000000000000000000"),
                b256!("0000000000000000000000000000000000000000000000000000000000000000"),
                false,
            )
            .unwrap();

            TxEnvelope::Eip1559(tx.into_signed(signature))
        }

        let tx = create_transaction_with_wrong_signature();
        let mut encoded_tx = Vec::new();
        tx.encode(&mut encoded_tx);
        let encoded_tx = Bytes::from(encoded_tx);

        let transactions = Arc::new(RwLock::new(Vec::new()));
        let chain = InMemoryMetabasedSequencerChain::new(transactions.clone());

        let error = send_raw_transaction(encoded_tx.clone(), &chain)
            .await
            .unwrap_err();

        let expected_error = "invalid input: invalid transaction signature";
        let actual_error = error.to_string();

        assert_eq!(actual_error, expected_error);
    }

    #[tokio::test]
    async fn test_sending_transaction_with_failing_chain_fails() {
        #[derive(Debug)]
        struct Failure;

        impl Display for Failure {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "failure")
            }
        }

        impl std::error::Error for Failure {}

        impl From<Failure> for Error {
            fn from(_: Failure) -> Self {
                Self::Server
            }
        }

        struct FailingSequencerChain;

        #[async_trait]
        impl MetabasedSequencerChainService for FailingSequencerChain {
            type Error = Failure;

            async fn process_transaction(&self, _tx: Bytes) -> Result<TxHash, Self::Error> {
                Err(Failure)
            }

            async fn process_bulk_transactions(
                &self,
                _tx: Vec<Bytes>,
            ) -> Result<TxHash, Self::Error> {
                Err(Failure)
            }
        }

        let tx = create_transaction(TxKind::Create, Vec::new());
        let mut encoded_tx = Vec::new();
        tx.encode(&mut encoded_tx);
        let encoded_tx = Bytes::from(encoded_tx);

        let chain = FailingSequencerChain;

        let error = send_raw_transaction(encoded_tx.clone(), &chain)
            .await
            .unwrap_err();

        let expected_error = "server error";
        let actual_error = error.to_string();

        assert_eq!(actual_error, expected_error);
    }

    #[tokio::test]
    async fn test_sending_legacy_transaction_exceeding_gas_price_cap_fails() {
        fn create_transaction() -> TxEnvelope {
            let mut tx = TxLegacy {
                chain_id: Some(0),
                nonce: 0,
                gas_price: u128::MAX,
                gas_limit: u64::MAX,
                to: TxKind::Create,
                value: Default::default(),
                input: Default::default(),
            };
            let signer = PrivateKeySigner::from_bytes(&PRIVATE_KEY.into()).unwrap();
            let signature = signer.sign_transaction_sync(&mut tx).unwrap();

            TxEnvelope::Legacy(tx.into_signed(signature))
        }

        let tx = create_transaction();
        let mut encoded_tx = Vec::new();
        tx.encode(&mut encoded_tx);
        let encoded_tx = Bytes::from(encoded_tx);

        let transactions = Arc::new(RwLock::new(Vec::new()));
        let chain = InMemoryMetabasedSequencerChain::new(transactions.clone());

        let error = send_raw_transaction(encoded_tx.clone(), &chain)
            .await
            .unwrap_err();

        let expected_error = "transaction rejected: transaction fee too high";
        let actual_error = error.to_string();

        assert_eq!(actual_error, expected_error);
    }
}
