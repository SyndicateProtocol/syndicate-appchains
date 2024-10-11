use crate::domain::primitives::{Address, Bytes};
use crate::domain::MetabasedSequencerChainService;
use alloy::providers::Provider;
use alloy::sol;
use async_trait::async_trait;

sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract MetabasedSequencerChain {
        event TransactionProcessed(address indexed sender, bytes encodedTxn);
        error InvalidTransactionForm();
        function emitTransactionProcessed(bytes calldata encodedTxn) public;
        function processTransaction(bytes calldata encodedTxn) public;
        function processBulkTransactions(bytes[] calldata encodedTxns) public;
    }
}

#[async_trait]
impl<P> MetabasedSequencerChainService for P
where
    P: Provider,
{
    async fn process_bulk_transactions(
        &self,
        account: Address,
        tx: Vec<Bytes>,
    ) -> anyhow::Result<()> {
        let contract = MetabasedSequencerChain::new(account, self);

        contract.processBulkTransactions(tx).call().await?;

        Ok(())
    }
}
