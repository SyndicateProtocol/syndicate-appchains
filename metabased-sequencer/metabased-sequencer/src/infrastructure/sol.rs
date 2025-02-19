use crate::{
    domain::{
        primitives::{Address, Bytes, TxHash},
        MetabasedSequencerChainService,
    },
    infrastructure::sol::MetabasedSequencerChain::MetabasedSequencerChainInstance,
};
use alloy::{
    hex, network::Network, primitives::U256, providers::Provider, sol, transports::Transport,
};
use async_trait::async_trait;
use std::{marker::PhantomData, time::Duration};
use tracing::{debug_span, error, info, warn};

sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract MetabasedSequencerChain {
        event TransactionProcessed(address indexed sender, bytes encodedTxn);
        error InvalidTransactionForm();
        function emitTransactionProcessed(bytes calldata encodedTxn) public;
        function processTransaction(bytes calldata encodedTxn) public;
        function processBulkTransactions(bytes[] calldata encodedTxns) public;
        // TODO(SEQ-248): Contract needs to be updated
        // function processBulkTransactionsCompressed(bytes calldata compressedTxns) public;
    }
}

#[derive(Debug)]
pub struct SolMetabasedSequencerChainService<P: Provider<T, N>, T: Transport + Clone, N: Network> {
    account: Address,
    provider: P,
    phantom1: PhantomData<T>,
    phantom2: PhantomData<N>,
}

impl<P: Provider<T, N>, T: Transport + Clone, N: Network>
    SolMetabasedSequencerChainService<P, T, N>
{
    pub fn new(account: Address, provider: P) -> Self {
        Self { account, provider, phantom1: Default::default(), phantom2: Default::default() }
    }

    pub fn contract(&self) -> MetabasedSequencerChainInstance<T, &P, N> {
        MetabasedSequencerChain::new(self.account, &self.provider)
    }

    /// Gets the current balance of the sequencer account
    async fn get_balance(&self) -> Result<U256, alloy::contract::Error> {
        Ok(self.provider.get_balance(self.account).await?)
    }

    fn format_tx_data(tx: &Bytes) -> String {
        format!("0x{}", hex::encode(tx))
    }

    /// Logs the current balance of the sequencer account with context
    async fn log_sequencer_balance(&self, context: &str) {
        match self.get_balance().await {
            Ok(balance) => {
                let decimals = U256::from(18); // ETH has 18 decimals
                let balance_synd = format_units_uint(&balance, &decimals);
                info!(
                    account = ?self.account,
                    balance_wei = ?balance,
                    balance_synd,
                    "Sequencer wallet balance {}", context
                );
            }
            Err(e) => {
                warn!(
                    account = ?self.account,
                    error = ?e,
                    "Could not fetch sequencer wallet balance {}", context
                );
            }
        }
    }
}

/// Format wei value with arbitrary precision
/// Ported from Foundry: https://github.com/foundry-rs/foundry/blob/master/crates/evm/abi/src/console/mod.rs#L11
pub fn format_units_uint(x: &U256, decimals: &U256) -> String {
    match alloy_primitives::utils::Unit::new(decimals.saturating_to::<u8>()) {
        Some(units) => alloy_primitives::utils::ParseUnits::U256(*x).format_units(units),
        None => x.to_string(),
    }
}

#[async_trait]
impl<P: Provider<T, N>, T: Transport + Clone, N: Network> MetabasedSequencerChainService
    for SolMetabasedSequencerChainService<P, T, N>
{
    type Error = alloy::contract::Error;

    async fn process_transaction(&self, tx: Bytes) -> Result<TxHash, Self::Error> {
        let result = debug_span!(
            "process_transaction",
            account = ?self.account,
            tx_data = ?Self::format_tx_data(&tx),
            tx_size = tx.len()
        )
        .in_scope(|| async {
            let pending_tx = self.contract().processTransaction(tx).send().await?;

            match pending_tx
                .with_required_confirmations(2)
                .with_timeout(Some(Duration::from_secs(60)))
                .watch()
                .await
            {
                Ok(hash) => {
                    self.log_sequencer_balance("after transaction").await;
                    Ok(hash)
                }
                Err(e) => {
                    error!(error = ?e, "Transaction submission failed");
                    Err(alloy::contract::Error::from(e))
                }
            }
        })
        .await?;

        Ok(result)
    }

    async fn process_bulk_transactions(&self, txs: Vec<Bytes>) -> Result<TxHash, Self::Error> {
        debug_span!(
            "process_bulk_transactions",
            account = ?self.account,
            tx_count = txs.len(),
            total_size = txs.iter().map(|tx| tx.len()).sum::<usize>(),
            tx_data = ?txs.iter().map(|tx| Self::format_tx_data(tx)).collect::<Vec<_>>()
        )
        .in_scope(|| async {
            let pending_tx = self.contract().processBulkTransactions(txs).send().await?;

            match pending_tx
                .with_required_confirmations(2)
                .with_timeout(Some(Duration::from_secs(60)))
                .watch()
                .await
            {
                Ok(hash) => {
                    self.log_sequencer_balance("after bulk transaction").await;
                    Ok(hash)
                }
                Err(e) => {
                    error!(error = ?e, "Bulk transaction submission failed");
                    Err(alloy::contract::Error::from(e))
                }
            }
        })
        .await
    }

    // TODO (SEQ-248): Implement bulk transactions
    // async fn process_bulk_transactions_compressed(&self, txns: Bytes) -> Result<(), Self::Error>
    // {     self.contract().processBulkTransactionsCompressed(txns).call().await?;
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        network::Ethereum,
        providers::{ProviderCall::BoxedFuture, RootProvider, RpcWithBlock},
        transports::BoxTransport,
    };

    #[derive(Debug, Clone)]
    struct MockProvider {
        balance: U256,
    }

    impl MockProvider {
        fn new(balance: U256) -> Self {
            Self { balance }
        }
    }

    impl Provider<BoxTransport, Ethereum> for MockProvider {
        fn root(&self) -> &RootProvider<BoxTransport, Ethereum> {
            panic!("Not implemented")
        }

        fn get_balance(
            &self,
            _address: Address,
        ) -> RpcWithBlock<BoxTransport, Address, U256, U256, fn(U256) -> U256> {
            let balance = self.balance;
            RpcWithBlock::new_provider(move |_block_id| {
                let fut = Box::pin(async move { Ok(balance) });
                BoxedFuture(fut)
            })
        }
    }

    #[tokio::test]
    async fn test_get_balance() {
        let expected_balance = U256::from(100);
        let provider = MockProvider::new(expected_balance);
        let service: SolMetabasedSequencerChainService<MockProvider, BoxTransport, Ethereum> =
            SolMetabasedSequencerChainService::new(Address::default(), provider);
        let balance = service.get_balance().await.unwrap();
        assert_eq!(balance, expected_balance);
    }

    #[tokio::test]
    async fn test_get_zero_balance() {
        let expected_balance = U256::from(0);
        let provider = MockProvider::new(expected_balance);
        let service: SolMetabasedSequencerChainService<MockProvider, BoxTransport, Ethereum> =
            SolMetabasedSequencerChainService::new(Address::default(), provider);
        let balance = service.get_balance().await.unwrap();
        assert_eq!(balance, expected_balance);
    }

    /// Function under test is from Foundry so it's already reliable. This test is to confirm
    /// expected values
    #[test]
    fn test_format_units_uint() {
        let eth_decimals = U256::from(18);

        // Test zero
        let zero = U256::from(0);
        assert_eq!(format_units_uint(&zero, &eth_decimals), "0.000000000000000000");

        // Test small value (1 wei)
        let one_wei = U256::from(1);
        assert_eq!(format_units_uint(&one_wei, &eth_decimals), "0.000000000000000001");

        // Test 1 ETH exactly
        let one_eth = U256::from(10).pow(U256::from(18));
        assert_eq!(format_units_uint(&one_eth, &eth_decimals), "1.000000000000000000");

        // Test value less than 1 ETH
        let point_1_eth = U256::from(100000000000000000u64); // 0.1 ETH
        assert_eq!(format_units_uint(&point_1_eth, &eth_decimals), "0.100000000000000000");

        // Test value greater than 1 ETH
        let one_point_5_eth = U256::from(1500000000000000000u64); // 1.5 ETH
        assert_eq!(format_units_uint(&one_point_5_eth, &eth_decimals), "1.500000000000000000");

        // Test large value
        let thousand_eth = U256::from(1000) * U256::from(10).pow(U256::from(18)); // 1000 ETH
        assert_eq!(format_units_uint(&thousand_eth, &eth_decimals), "1000.000000000000000000");

        // Test very small value
        let tiny_value = U256::from(123); // 123 wei
        assert_eq!(format_units_uint(&tiny_value, &eth_decimals), "0.000000000000000123");

        // Test precise value with many decimals
        let precise_value: U256 = "123456789123456789".parse().unwrap(); // ~0.123 ETH
        assert_eq!(format_units_uint(&precise_value, &eth_decimals), "0.123456789123456789");
    }
}
