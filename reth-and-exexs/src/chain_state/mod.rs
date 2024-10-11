use alloy_primitives::B256;

use crate::{
    engine::{Engine, EngineApi, ForkChoiceUpdate, ForkchoiceState, PayloadAttributes},
    l3_block::L3Block,
};

pub struct ChainState<E: Engine> {
    unsafe_block_hash: B256,
    safe_block_hash: B256,
    finalized_block_hash: B256,
    engine: E,
}

impl<E: Engine> ChainState<E> {
    pub async fn add_block(&mut self, block: L3Block) -> eyre::Result<()> {
        // Block to PayloadAttributes
        let payload_attributes = PayloadAttributes::from(block);

        // Start block build process
        let result = self
            .update_forkchoice_state(Some(payload_attributes))
            .await?;
        let payload_id = result.payload_id.unwrap();

        // Get Payload
        let payload = self.engine.get_payload(payload_id).await?;
        let new_hash = payload.block_hash;

        // Process payload
        self.engine.new_payload(payload).await?;

        // Update state
        self.unsafe_block_hash = new_hash;
        self.safe_block_hash = new_hash;

        // Call engine_ForkchoiceUpdated
        self.update_forkchoice_state(None).await?;

        Ok(())
    }

    async fn update_forkchoice_state(
        &mut self,
        attributes: Option<PayloadAttributes>,
    ) -> eyre::Result<ForkChoiceUpdate> {
        self.engine
            .forkchoice_updated(
                ForkchoiceState {
                    head_block_hash: self.unsafe_block_hash,
                    safe_block_hash: self.safe_block_hash,
                    finalized_block_hash: self.finalized_block_hash,
                },
                attributes,
            )
            .await
    }
}

impl ChainState<EngineApi> {
    pub fn new(engine: EngineApi) -> Self {
        Self {
            unsafe_block_hash: B256::default(),
            safe_block_hash: B256::default(),
            finalized_block_hash: B256::default(),
            engine,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{engine::MockEngine, l3_block::L3Block};
    use alloy_primitives::{b256, U256, U64};

    use super::*;
    use eyre::Result;

    #[tokio::test]
    async fn test_add_block() -> Result<()> {
        let new_hash = b256!("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
        let mut mock_engine = MockEngine::default();
        mock_engine.get_payload_res.block_hash = new_hash;

        let mut chain_state = ChainState {
            unsafe_block_hash: B256::default(),
            safe_block_hash: B256::default(),
            finalized_block_hash: B256::default(),
            engine: mock_engine,
        };
        let block = L3Block {
            parent_hash: B256::default(),
            epoch_number: U256::from(0),
            timestamp: U64::from(0),
            transaction_list: vec![],
        };

        chain_state.add_block(block).await?;

        assert_eq!(chain_state.unsafe_block_hash, new_hash);
        assert_eq!(chain_state.safe_block_hash, new_hash);
        assert_eq!(chain_state.finalized_block_hash, B256::default());

        Ok(())
    }
}
