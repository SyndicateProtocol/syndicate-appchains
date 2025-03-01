use crate::contract::MetabasedSequencerChain::processTransactionCall;
use alloy::{
    consensus::{Transaction, TxEnvelope, TxType},
    hex,
    network::{Ethereum, EthereumWallet},
    primitives::{Address, Bytes, TxHash, B256, U256},
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
use jsonrpsee::{
    core::RpcResult,
    types::{error::ErrorCode, Params},
};
use reqwest::Client;
use std::sync::Arc;
use tracing::{debug, error, info};
use url::Url;

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

pub struct MetabasedService {
    contract_address: Address,
    provider: Arc<FilledProvider>,
}

impl MetabasedService {
    pub fn new(contract_address: Address, rpc_url: Url, private_key: B256) -> Result<Self> {
        let signer = PrivateKeySigner::from_bytes(&private_key)?;
        let wallet = EthereumWallet::from(signer);

        let provider = ProviderBuilder::new()
            .with_cached_nonce_management()
            .filler(WalletFiller::new(wallet))
            .filler(GasFiller)
            .filler(ChainIdFiller::new(None))
            .on_http(rpc_url);

        Ok(Self { contract_address, provider: Arc::new(provider) })
    }

    fn validate_transaction(&self, raw_tx: &Bytes) -> Result<()> {
        debug!(bytes_length = raw_tx.len(), "Starting transaction validation");
        // 1. Decoding:
        let mut slice: &[u8] = raw_tx.as_ref();
        let tx = match TxEnvelope::decode(&mut slice) {
            Ok(tx) => tx,
            Err(_) => {
                debug!(
                    // error = %error,
                    "Transaction decoding failed"
                );
                return Err(eyre::eyre!("Transaction decoding failed"));
            }
        };

        // 2. Validation:
        //For non-legacy transactions, validate chain ID immediately
        if tx.tx_type() != TxType::Legacy && tx.chain_id().is_none() {
            debug!(
                tx_type = ?tx.tx_type(),
                "Transaction validation failed: missing chain ID"
            );
            return Err(eyre::eyre!("Transaction validation failed: missing chain ID"));
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
                // let error = InvalidInput(MissingGasPrice);
                debug!(
                    // error = %error,
                    tx_type = ?tx.tx_type(),
                    "Transaction validation failed: missing gas price"
                );
                eyre::eyre!("Transaction validation failed: missing gas price")
            })?;

            // Short circuit if there is no cap for transaction fee at all.
            if tx_cap_in_wei.is_zero() {
                return Ok(());
            }

            let gas_price = U256::try_from(gas_price)?;
            let gas = U256::try_from(tx.gas_limit())?;
            let fee_wei = gas_price.saturating_mul(gas);

            if fee_wei > tx_cap_in_wei {
                return Err(eyre::eyre!("Transaction fee too high"));
            }
        }

        Ok(())
    }

    async fn process_transaction(&self, raw_tx: Bytes) -> Result<TxHash> {
        info!("Processing transaction: {}", hex::encode(&raw_tx));
        self.validate_transaction(&raw_tx)?;

        debug!("Submitting validated transaction to chain");
        let data = processTransactionCall { encodedTxn: raw_tx };
        let input = TransactionInput::new(data.abi_encode().into());
        let tx = TransactionRequest::default().to(self.contract_address).input(input);
        let pending = self.provider.send_transaction(tx).await?;

        Ok(pending.tx_hash().to_owned())
    }
}

pub async fn send_raw_transaction_handler(
    params: Params<'static>,
    service: Arc<MetabasedService>,
) -> RpcResult<String> {
    let tx_data: Bytes =
        params.one::<String>().map_err(|_| ErrorCode::InvalidRequest)?.parse().map_err(|e| {
            error!("Failed to parse transaction data: {}", e);
            ErrorCode::InvalidParams
        })?;

    let tx_hash = service.process_transaction(tx_data).await.map_err(|e| {
        error!("Failed to process transaction: {}", e);
        ErrorCode::InternalError
    })?;

    Ok(format!("0x{}", hex::encode(tx_hash)))
}
