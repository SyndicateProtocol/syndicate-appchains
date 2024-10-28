use crate::domain::primitives::{Address, Bytes};
use crate::domain::MetabasedSequencerChainService;
use alloy::network::Network;
use alloy::providers::Provider;
use alloy::sol;
use alloy::transports::Transport;
use async_trait::async_trait;
use std::marker::PhantomData;
use crate::infrastructure::sol::MetabasedSequencerChain::MetabasedSequencerChainInstance;

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
        Self {
            account,
            provider,
            phantom1: Default::default(),
            phantom2: Default::default(),
        }
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

    async fn process_transaction(&self, tx: Bytes) -> Result<(), Self::Error> {
        // Do not compress individual transaction, doesn't get smaller
        self.contract().processTransaction(tx).call().await?;
        Ok(())
    }

    async fn process_bulk_transactions(&self, tx: Vec<Bytes>) -> Result<(), Self::Error> {
        self.contract().processBulkTransactions(tx).call().await?;
        Ok(())
    }

    // TODO (SEQ-248): Implement bulk transactions
    // async fn process_bulk_transactions_compressed(&self, txns: Bytes) -> Result<(), Self::Error> {
    //     self.contract().processBulkTransactionsCompressed(txns).call().await?;
    //     Ok(())
    // }
}