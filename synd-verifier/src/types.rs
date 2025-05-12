//! Types for the verifier

use alloy::{
    consensus::{Eip2718EncodableReceipt, Receipt as AlloyReceipt, RlpEncodableReceipt},
    eips::Typed2718,
    primitives::Bloom,
    rpc::types::Block,
};
use shared::types::Receipt;

/// A typed receipt for RLP and EIP-2718 encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedReceipt {
    /// The type of the receipt (0 = legacy).
    pub ty: u8,
    /// The underlying receipt data.
    pub receipt: AlloyReceipt,
}

impl RlpEncodableReceipt for TypedReceipt {
    fn rlp_encoded_length_with_bloom(&self, _bloom: &Bloom) -> usize {
        // This method is unused for TypedReceipt. Required by the trait.
        0
    }

    fn rlp_encode_with_bloom(&self, bloom: &Bloom, out: &mut dyn alloy::rlp::BufMut) {
        if self.ty != 0 {
            out.put_u8(self.ty);
        }
        self.receipt.rlp_encode_with_bloom(bloom, out);
    }
}

impl Eip2718EncodableReceipt for TypedReceipt {
    fn eip2718_encoded_length_with_bloom(&self, _bloom: &Bloom) -> usize {
        // This method is unused for TypedReceipt. Required by the trait.
        0
    }

    fn eip2718_encode_with_bloom(&self, bloom: &Bloom, out: &mut dyn alloy::rlp::BufMut) {
        if self.ty != 0 {
            out.put_u8(self.ty);
        }
        self.receipt.rlp_encode_with_bloom(bloom, out);
    }
}

impl Typed2718 for TypedReceipt {
    fn ty(&self) -> u8 {
        self.ty
    }
}

/// Input for verifying a batch of blocks and creating a new mchain block
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainVerificationInput {
    /// The blocks to verify
    pub blocks: Vec<Block>,
    /// The receipts to verify
    pub receipts: Vec<Vec<Receipt>>,
}
