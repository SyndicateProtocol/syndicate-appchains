use alloy_primitives::FixedBytes;
use async_trait::async_trait;
use eyre::Result;
use std::default::Default;

use super::{
    Engine, ExecutionPayload, ForkChoiceUpdate, ForkchoiceState, PayloadAttributes, PayloadId,
    PayloadStatus, Status,
};

/// Mock L2 Engine API that returns preset responses
#[derive(Debug, Clone)]
pub struct MockEngine {
    /// Forkchoice updated call response when payload is Some
    pub forkchoice_updated_payloads_res: ForkChoiceUpdate,
    /// Forkchoice updated call response when payload is None
    pub forkchoice_updated_res: ForkChoiceUpdate,
    /// New payload call response
    pub new_payload_res: PayloadStatus,
    /// Get payload call response
    pub get_payload_res: ExecutionPayload,
}

impl Default for MockEngine {
    fn default() -> Self {
        Self {
            forkchoice_updated_payloads_res: ForkChoiceUpdate {
                payload_status: PayloadStatus {
                    status: Status::Valid,
                    latest_valid_hash: Some(FixedBytes::default()),
                    validation_error: None,
                },
                payload_id: Some(PayloadId::default()),
            },
            forkchoice_updated_res: ForkChoiceUpdate {
                payload_status: PayloadStatus {
                    status: Status::Valid,
                    latest_valid_hash: Some(FixedBytes::default()),
                    validation_error: None,
                },
                payload_id: Some(PayloadId::default()),
            },
            new_payload_res: PayloadStatus {
                status: Status::Valid,
                latest_valid_hash: Some(FixedBytes::default()),
                validation_error: None,
            },
            get_payload_res: ExecutionPayload::default(),
        }
    }
}

#[async_trait]
impl Engine for MockEngine {
    async fn forkchoice_updated(
        &self,
        _forkchoice_state: ForkchoiceState,
        payload_attributes: Option<PayloadAttributes>,
    ) -> Result<ForkChoiceUpdate> {
        Ok(if payload_attributes.is_some() {
            self.forkchoice_updated_payloads_res.clone()
        } else {
            self.forkchoice_updated_res.clone()
        })
    }

    async fn new_payload(&self, _execution_payload: ExecutionPayload) -> Result<PayloadStatus> {
        Ok(self.new_payload_res.clone())
    }

    async fn get_payload(&self, _payload_id: PayloadId) -> Result<ExecutionPayload> {
        Ok(self.get_payload_res.clone())
    }
}
