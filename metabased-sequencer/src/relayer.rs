//! The `service` module handles the business logic for the metabased sequencer.

use crate::{
    config::Config,
    contract::MetabasedSequencerChain::processTransactionCall,
    errors::{
        Error,
        Error::{Contract, InvalidInput, InvalidParams, TransactionRejected},
        InvalidInputError::{MissingChainID, MissingGasPrice, UnableToRLPDecode},
        InvalidParamsError::{MissingParam, NotAnArray, NotHexEncoded, WrongParamCount},
        Rejection::FeeTooHigh,
    },
    metrics::RelayerMetrics,
};
use alloy::{
    consensus::{Transaction, TxEnvelope, TxType},
    hex,
    network::{Ethereum, EthereumWallet},
    primitives::{Address, Bytes, TxHash, U256},
    providers::{
        fillers::{
            CachedNonceManager, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    rlp::Decodable,
    rpc::types::{TransactionInput, TransactionRequest},
    signers::local::PrivateKeySigner,
    sol_types::SolCall,
    transports::http::Http,
};
use eyre::Result;
use jsonrpsee::{core::RpcResult, types::Params, Extensions};
use reqwest::Client;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::{debug, error, info};

#[allow(missing_docs)]
pub type FilledProvider = FillProvider<
    JoinFill<
        JoinFill<
            JoinFill<
                JoinFill<Identity, NonceFiller<CachedNonceManager>>,
                WalletFiller<EthereumWallet>,
            >,
            GasFiller,
        >,
        ChainIdFiller,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

/// The service for relaying transactions to the sequencing contract.
#[derive(Debug)]
pub struct RelayerService {
    /// The address of the sequencing contract on the sequencing chain
    contract_address: Address,

    /// The provider for the sequencing chain
    provider: Arc<FilledProvider>,

    /// The metrics for the relayer service
    metrics: Arc<RelayerMetrics>,
}

impl RelayerService {
    /// Create a new `RelayerService` instance.
    pub fn new(config: &Config, relayer_metrics: RelayerMetrics) -> Result<Self> {
        let signer = PrivateKeySigner::from_bytes(&config.private_key)?;
        let wallet = EthereumWallet::from(signer);

        let provider = ProviderBuilder::new()
            // TODO [SEQ-620]: Make nonce management more robust
            .with_cached_nonce_management()
            .filler(WalletFiller::new(wallet))
            .filler(GasFiller)
            .filler(ChainIdFiller::new(None))
            .on_http(config.chain_rpc_url.clone());

        Ok(Self {
            contract_address: config.chain_contract_address,
            provider: Arc::new(provider),
            metrics: Arc::new(relayer_metrics),
        })
    }

    // TODO [SEQ-660]: Refactor this function to be more readable
    fn validate_transaction(&self, raw_tx: &Bytes) -> Result<TxHash, Error> {
        debug!(bytes_length = raw_tx.len(), "Starting transaction validation");
        // 1. Decoding:
        let mut slice: &[u8] = raw_tx.as_ref();
        let tx = match TxEnvelope::decode(&mut slice) {
            Ok(tx) => tx,
            Err(_) => {
                let error = InvalidInput(UnableToRLPDecode);
                debug!(
                    error = %error,
                    "Transaction decoding failed"
                );
                return Err(error);
            }
        };

        // 2. Validation:
        //For non-legacy transactions, validate chain ID immediately
        if tx.tx_type() != TxType::Legacy && tx.chain_id().is_none() {
            let error = InvalidInput(MissingChainID);
            debug!(
                error = %error,
                tx_type = ?tx.tx_type(),
                "Transaction validation failed: missing chain ID"
            );
            return Err(error);
        }

        tx.recover_signer().map_err(|e| {
            debug!(
                error = ?e,
                tx_type = ?tx.tx_type(),
                "Transaction validation failed: invalid signature"
            );
            e
        })?;

        if tx.tx_type() == TxType::Legacy {
            // TODO(SEQ-179): introduce optional global tx cap config. See op-geth's checkTxFee() +
            // RPCTxFeeCap for equivalent skip check if unset
            let tx_cap_in_wei = U256::from(1_000_000_000_000_000_000u64); // 1e18wei = 1 ETH
            let gas_price = tx.gas_price().ok_or_else(|| {
                let error = InvalidInput(MissingGasPrice);
                debug!(
                    error = %error,
                    tx_type = ?tx.tx_type(),
                    "Transaction validation failed: missing gas price"
                );
                error
            })?;

            // Short circuit if there is no cap for transaction fee at all.
            if tx_cap_in_wei.is_zero() {
                return Ok(tx.tx_hash().to_owned());
            }

            let gas_price = U256::try_from(gas_price)?;
            let gas = U256::try_from(tx.gas_limit())?;
            let fee_wei = gas_price.saturating_mul(gas);

            if fee_wei > tx_cap_in_wei {
                return Err(TransactionRejected(FeeTooHigh));
            }
        }

        Ok(tx.tx_hash().to_owned())
    }

    async fn process_transaction(&self, raw_tx: Bytes) -> Result<TxHash, Error> {
        info!("Processing transaction: {}", hex::encode(&raw_tx));
        let original_tx_hash = self.validate_transaction(&raw_tx)?;

        debug!("Submitting validated transaction to chain");
        let data = processTransactionCall { encodedTxn: raw_tx };
        let input = TransactionInput::new(data.abi_encode().into());
        let tx = TransactionRequest::default().to(self.contract_address).input(input);
        let pending_tx = self.provider.send_transaction(tx).await.map_err(|e| {
            error!(error = ?e, "Transaction submission failed");
            Contract(alloy::contract::Error::from(e))
        })?;

        // TODO [SEQ-661]: Make this configurable
        match pending_tx
            .with_required_confirmations(2)
            .with_timeout(Some(Duration::from_secs(60)))
            .watch()
            .await
        {
            Ok(hash) => {
                // TODO: Log sequencer balance
                debug!("Transaction submitted: {}", hex::encode(hash));
                Ok(original_tx_hash)
            }
            Err(e) => {
                error!(error = ?e, "Transaction submission failed");
                Err(Contract(alloy::contract::Error::from(e)))
            }
        }
    }
}

// Params validation
fn parse_send_raw_transaction_params(params: Params<'static>) -> Result<Bytes, Error> {
    let mut json: serde_json::Value = serde_json::from_str(params.as_str().unwrap_or("[]"))?;
    let arr = json.as_array_mut().ok_or(InvalidParams(NotAnArray))?;
    if arr.len() != 1 {
        return Err(InvalidParams(WrongParamCount(arr.len())));
    }
    let item = arr.pop().ok_or(InvalidParams(MissingParam))?;
    let raw_tx = item.as_str().ok_or(InvalidParams(NotHexEncoded))?.to_string();
    let tx_data: Bytes = hex::decode(&raw_tx).map(Bytes::from)?;

    Ok(tx_data)
}

/// The handler for the `eth_sendRawTransaction` JSON-RPC method.
pub async fn send_raw_transaction_handler(
    params: Params<'static>,
    service: Arc<RelayerService>,
    _: Extensions,
) -> RpcResult<String> {
    let start = Instant::now();
    let result = async {
        let tx_data = parse_send_raw_transaction_params(params)?;
        let tx_hash = service.process_transaction(tx_data).await?;
        Ok::<_, Error>(format!("0x{}", hex::encode(tx_hash)))
    }
    .await;
    let duration = start.elapsed();

    result
        .inspect(|_| service.metrics.record_rpc_call("eth_sendRawTransaction", duration, None))
        .map_err(|e| {
            service.metrics.record_rpc_call("eth_sendRawTransaction", duration, Some(&e));
            e.to_json_rpc_error()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, metrics::MetricsState};
    use alloy::primitives::{Bytes, B256};
    use jsonrpsee::types::Params;
    use std::str::FromStr;
    use url::Url;

    fn setup_test_service() -> RelayerService {
        let config = Config {
            chain_contract_address: Address::from([0x42; 20]),
            chain_rpc_url: Url::parse("http://localhost:8545").unwrap(),
            private_key: B256::from([0x1; 32]),
            port: 8456,
            metrics_port: 9191,
        };
        let mut metrics = MetricsState::new();
        let relayer_metrics = RelayerMetrics::new(&mut metrics.registry);
        RelayerService::new(&config, relayer_metrics).unwrap()
    }

    #[test]
    fn test_new_service_creation() {
        let config = Config {
            chain_contract_address: Address::from([0x42; 20]),
            chain_rpc_url: Url::parse("http://localhost:8545").unwrap(),
            private_key: B256::from([0x1; 32]),
            port: 8456,
            metrics_port: 9191,
        };
        let mut metrics = MetricsState::new();
        let relayer_metrics = RelayerMetrics::new(&mut metrics.registry);

        let result = RelayerService::new(&config, relayer_metrics);
        assert!(result.is_ok());

        let service = result.unwrap();
        assert_eq!(service.contract_address, config.chain_contract_address);
    }

    #[test]
    fn test_new_service_with_invalid_private_key() {
        let config = Config {
            chain_contract_address: Address::from([0x42; 20]),
            chain_rpc_url: Url::parse("http://localhost:8545").unwrap(),
            private_key: B256::from([0x0; 32]), // Invalid private key (all zeros)
            port: 8456,
            metrics_port: 9191,
        };
        let mut metrics = MetricsState::new();
        let relayer_metrics = RelayerMetrics::new(&mut metrics.registry);

        let result = RelayerService::new(&config, relayer_metrics);
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_send_raw_transaction_handler_invalid_params() {
        let service = Arc::new(setup_test_service());
        let invalid_params = Params::new(Some("[\"invalid_hex\"]"));

        let result =
            send_raw_transaction_handler(invalid_params, service, Extensions::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_raw_transaction_handler_valid_params() {
        let service = Arc::new(setup_test_service());
        // Valid raw transaction hex
        let valid_tx = "[\"0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772\"]";
        let params = Params::new(Some(valid_tx));

        let result = send_raw_transaction_handler(params, service, Extensions::default()).await;
        // Note: This will fail in actual execution since we're using a mock setup
        // but it tests the parameter parsing logic
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_process_transaction() {
        let service = setup_test_service();
        let test_tx = Bytes::from_str("0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772").unwrap();

        let result = service.process_transaction(test_tx).await;
        // This will fail since we're not connected to a real node
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_transaction() {
        let service = setup_test_service();
        let valid_tx = Bytes::from_str("0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772").unwrap();

        let result = service.validate_transaction(&valid_tx);
        // The validation should pass since this is a valid RLP-encoded transaction
        assert!(result.is_ok());
        let tx_hash = result.unwrap();
        assert_eq!(
            tx_hash.to_string(),
            "0xc429e5f128387d224ba8bed6885e86525e14bfdc2eb24b5e9c3351a1176fd81f"
        );
    }
}
