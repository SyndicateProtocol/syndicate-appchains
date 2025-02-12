use crate::{
    domain::{
        primitives::{Address, Bytes, TxHash},
        MetabasedSequencerChainService,
    },
    infrastructure::sol::MetabasedSequencerChain::MetabasedSequencerChainInstance,
};
use alloy::{hex, network::Network, providers::Provider, sol, transports::Transport};
use async_trait::async_trait;
use std::{marker::PhantomData, time::Duration};
use tracing::debug_span;

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
    fn format_tx_data(tx: &Bytes) -> String {
        format!("0x{}", hex::encode(tx))
    }

    pub fn new(account: Address, provider: P) -> Self {
        Self { account, provider, phantom1: Default::default(), phantom2: Default::default() }
    }

    pub fn contract(&self) -> MetabasedSequencerChainInstance<T, &P, N> {
        MetabasedSequencerChain::new(self.account, &self.provider)
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
                Ok(hash) => Ok(hash),
                Err(e) => {
                    tracing::error!(error = ?e, "Transaction submission failed");
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
                Ok(hash) => Ok(hash),
                Err(e) => {
                    tracing::error!(error = ?e, "Bulk transaction submission failed");
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
