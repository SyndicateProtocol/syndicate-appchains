use super::primitives::{Bytes, TxHash};
use jsonrpsee::core::async_trait;
use std::error::Error;

/// Represents the interface with the layer-2 blockchain.
///
/// The chain interface is defined by the following operations:
/// * [`Self::process_transaction`] to process a single transaction.
/// * [`Self::process_bulk_transaction`] to process multiple transactions in one call.
#[async_trait]
pub trait MetabasedSequencerChainService {
    /// The associated [`Error`] that can occur during the transaction processing.
    type Error: Error;

    /// Sends a single transaction into the layer-2 smart contract inbox to be picked up for
    /// processing.
    async fn process_transaction(&self, tx: Bytes) -> Result<TxHash, Self::Error>;

    /// Sends an unbounded amount of transactions into the layer-2 smart contract inbox to be picked
    /// up for processing.
    async fn process_bulk_transactions(&self, tx: Vec<Bytes>) -> Result<TxHash, Self::Error>;
}

#[cfg(test)]
pub use tests::InMemoryMetabasedSequencerChain;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{convert::Infallible, sync::Arc};
    use tokio::sync::RwLock;

    /// A test-only fake implementation of [`MetabasedSequencerChainService`].
    ///
    /// Collects enqueued transactions entirely in-memory and makes no external communication
    /// requests or connections.
    ///
    /// This chain is defined by the operations:
    /// * [`Self::new`] to construct a new instance passing in the memory buffer.
    /// * The methods from [`MetabasedSequencerChainService`] to fill in the memory buffer with
    ///   layer-3 transactions. Normally, it should wrap the given transactions into layer-2
    ///   blockchain transaction and return its hash. Since this implementation creates no
    ///   transactions, it simply returns zero.
    #[derive(Debug)]
    pub struct InMemoryMetabasedSequencerChain {
        transactions: Arc<RwLock<Vec<Bytes>>>,
    }

    impl InMemoryMetabasedSequencerChain {
        /// Constructs a new [`InMemoryMetabasedSequencerChain`] using the provided memory buffer.
        ///
        /// The caller may clone the passed in [`Arc`] pointer to keep access to it and inspect the
        /// changes made by the chain. This is useful for asserting some expectations as a part of
        /// a test scenario to analyse some behavior.
        ///
        /// # Example
        /// ```
        /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// # use interceptor::domain::primitives::Bytes;
        /// # use interceptor::domain::{InMemoryMetabasedSequencerChain, MetabasedSequencerChainService};
        /// use std::sync::Arc;
        /// use tokio::sync::RwLock;
        ///
        /// # let encoded_tx = Bytes::new();
        /// let transactions = Arc::new(RwLock::new(Vec::new()));
        /// let chain = InMemoryMetabasedSequencerChain::new(transactions.clone());
        ///
        /// chain.process_transaction(encoded_tx.clone()).await?;
        ///
        /// let actual_transactions = transactions.read().await.clone();
        /// let expected_transactions = vec![encoded_tx];
        ///
        /// assert_eq!(actual_transactions, expected_transactions);
        /// # Ok(())
        /// # }
        /// ```
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
    }
}
