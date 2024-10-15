use super::primitives::Bytes;
use jsonrpsee::core::async_trait;
use std::error::Error;

#[async_trait]
pub trait MetabasedSequencerChainService {
    type Error: Error;

    async fn process_transaction(&self, tx: Bytes) -> Result<(), Self::Error>;

    async fn process_bulk_transactions(&self, tx: Vec<Bytes>) -> Result<(), Self::Error>;
}
