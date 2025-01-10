use tracing::Level;
use std::convert::TryFrom;
use std::fmt::Debug;
use crate::domain::primitives::Bytes;
use crate::domain::MetabasedSequencerChainService;
use crate::presentation::json_rpc_errors::Error;
use crate::presentation::json_rpc_errors::Error::{InvalidInput, InvalidParams};
use crate::presentation::json_rpc_errors::InvalidInputError::{
    MissingChainID, MissingGasPrice, UnableToRLPDecode,
};
use crate::presentation::json_rpc_errors::InvalidParamsError::{
    MissingParam, NotAnArray, NotHexEncoded, WrongParamCount,
};
use crate::presentation::transaction;
use alloy::consensus::{Transaction, TxEnvelope, TxType};
use alloy::primitives::private::alloy_rlp::Decodable;
use alloy::primitives::TxHash;
use alloy::primitives::U256;
use jsonrpsee::types::Params;
use tracing::{debug, instrument};
use crate::presentation::json_rpc_errors::InvalidParamsError::{MissingParam, NotAnArray, NotHexEncoded, WrongParamCount};
use std::convert::TryFrom;

/// Sends serialized and signed transaction `tx` using `chain`.
#[instrument(level = Level::DEBUG, skip(chain), fields(encoded))]
pub async fn send_raw_transaction<Chain>(encoded: Bytes, chain: &Chain) -> Result<TxHash, Error>
where
    Chain: MetabasedSequencerChainService + Debug,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
{
    debug!(
        bytes_length = encoded.len(),
        "Starting transaction validation"
    );
    // 1. Decoding:
    let mut slice: &[u8] = encoded.as_ref();
    let tx = match TxEnvelope::decode(&mut slice) {
        Ok(tx) => tx,
        Err(_) =>
            {
                let error = InvalidInput(UnableToRLPDecode);
                debug!(
                    error = %error
                );
                return Err(error)
            },
    };

    // 2. Validation:
    //For non-legacy transactions, validate chain ID immediately
    if tx.tx_type() != TxType::Legacy && tx.chain_id().is_none() {
        let error = InvalidInput(MissingChainID);
        debug!(error = %error);
        return Err(error);
    }

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

    debug!("Submitting validated transaction to chain");
    // 3. Submission/forwarding:
    let result = chain.process_transaction(encoded).await?;
    debug!(
        tx_hash = ?result,
        "Chain processed transaction successfully"
    );
    Ok(result)
}

#[derive(Debug)]
pub struct SendRawTransactionParams {
    pub(crate) raw_tx: String,
}

impl TryFrom<Params<'static>> for SendRawTransactionParams {
    type Error = Error;

    // Params validation
    fn try_from(params: Params<'static>) -> Result<Self, Self::Error> {
        let mut json: serde_json::Value = serde_json::from_str(params.as_str().unwrap())?;
        let arr = json.as_array_mut().ok_or(InvalidParams(NotAnArray))?;
        if arr.len() != 1 {
            return Err(InvalidParams(WrongParamCount(arr.len())));
        }
        let item = arr.pop().ok_or(InvalidParams(MissingParam))?;
        let raw_tx = item
            .as_str()
            .ok_or(InvalidParams(NotHexEncoded))?
            .to_string();

        Ok(Self { raw_tx })
    }
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
    use alloy_primitives::{b256, PrimitiveSignature};
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

    fn encode_legacy_transaction_without_chain_id(gas_price: u128) -> Vec<u8> {
        let mut tx = TxLegacy {
            chain_id: None,
            nonce: 0,
            gas_price,
            gas_limit: 1,
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
    async fn test_decoding_legacy_transaction_without_chain_id_succeeds() {
        let encoded_tx = encode_legacy_transaction_without_chain_id(1);
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
            let signature = PrimitiveSignature::from_scalars_and_parity(
                b256!("0000000000000000000000000000000000000000000000000000000000000000"),
                b256!("0000000000000000000000000000000000000000000000000000000000000000"),
                false,
            );

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

        #[derive(Debug)]
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
        let encoded_tx = encode_legacy_transaction_without_chain_id(u128::MAX);
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

    // Params tests
    #[test]
    fn test_valid_params() {
        let params = Params::new(Some(r#"["0x1234"]"#));
        let result = SendRawTransactionParams::try_from(params);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().raw_tx, "0x1234");
    }

    #[test]
    fn test_invalid_array() {
        let params = Params::new(Some(r#"{"tx": "0x1234"}"#));
        let result = SendRawTransactionParams::try_from(params);
        assert!(matches!(result, Err(InvalidParams(NotAnArray))));
    }

    #[test]
    fn test_wrong_param_count() {
        let params = Params::new(Some(r#"["0x1234", "extra"]"#));
        let result = SendRawTransactionParams::try_from(params);
        assert!(matches!(result, Err(InvalidParams(WrongParamCount(2)))));
    }

    #[test]
    fn test_missing_param() {
        let params = Params::new(Some(r#"[]"#));
        let result = SendRawTransactionParams::try_from(params);
        assert!(matches!(result, Err(InvalidParams(WrongParamCount(0)))));
    }

    #[test]
    fn test_not_hex_encoded() {
        let params = Params::new(Some(r#"[123]"#));
        let result = SendRawTransactionParams::try_from(params);
        assert!(matches!(result, Err(InvalidParams(NotHexEncoded))));
    }
}
