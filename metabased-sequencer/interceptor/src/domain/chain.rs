use super::primitives::{Bytes, TxHash};
use jsonrpsee::core::async_trait;
use std::error::Error;

#[async_trait]
pub trait MetabasedSequencerChainService {
    type Error: Error;

    async fn process_transaction(&self, tx: Bytes) -> Result<TxHash, Self::Error>;

    async fn process_bulk_transactions(&self, tx: Vec<Bytes>) -> Result<TxHash, Self::Error>;

    // TODO (SEQ-248): Contract needs to be updated
    // async fn process_bulk_transactions_compressed(&self, txns: Bytes) -> Result<(), Self::Error>;
}

#[cfg(test)]
pub use tests::InMemoryMetabasedSequencerChain;

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[derive(Debug)]
    pub struct InMemoryMetabasedSequencerChain {
        transactions: Arc<RwLock<Vec<Bytes>>>,
    }

    impl InMemoryMetabasedSequencerChain {
        pub fn new(transactions: Arc<RwLock<Vec<Bytes>>>) -> Self {
            Self { transactions }
        }
    }

    #[async_trait]
    impl MetabasedSequencerChainService for InMemoryMetabasedSequencerChain {
        type Error = Infallible;

        async fn process_transaction(&self, tx: Bytes) -> Result<TxHash, Infallible> {
            self.transactions.write().await.push(tx);

            Ok(TxHash::ZERO)
        }

        async fn process_bulk_transactions(&self, tx: Vec<Bytes>) -> Result<TxHash, Infallible> {
            self.transactions.write().await.extend(tx);

            Ok(TxHash::ZERO)
        }

        // TODO SEQ-248: Implement bulk transactions
        // async fn process_bulk_transactions_compressed(&self, txns: Bytes) -> Result<(), Infallible> {
        //     self.transactions.write().await.push(txns);
        //
        //     Ok(())
        // }
    }
}
