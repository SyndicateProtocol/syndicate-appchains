use super::primitives::Bytes;
use jsonrpsee::core::async_trait;
use std::error::Error;

#[async_trait]
pub trait MetabasedSequencerChainService {
    type Error: Error;

    async fn process_transaction(&self, tx: Bytes) -> Result<(), Self::Error>;

    async fn process_bulk_transactions(&self, tx: Vec<Bytes>) -> Result<(), Self::Error>;
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

        async fn process_transaction(&self, tx: Bytes) -> Result<(), Infallible> {
            self.transactions.write().await.push(tx);

            Ok(())
        }

        async fn process_bulk_transactions(&self, tx: Vec<Bytes>) -> Result<(), Infallible> {
            self.transactions.write().await.extend(tx);

            Ok(())
        }
    }
}
