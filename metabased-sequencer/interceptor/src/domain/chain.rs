use super::primitives::{Address, Bytes};
use jsonrpsee::core::async_trait;

#[async_trait]
pub trait MetabasedSequencerChainService {
    async fn process_bulk_transactions(
        &self,
        account: Address,
        tx: Vec<Bytes>,
    ) -> anyhow::Result<()>;
}
