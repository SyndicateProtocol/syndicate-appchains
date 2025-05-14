//! Types for the synd-verifier

use alloy::{
    consensus::{Receipt as AlloyReceipt, RlpEncodableReceipt, TxReceipt},
    eips::{Encodable2718, Typed2718},
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

impl Typed2718 for TypedReceipt {
    fn ty(&self) -> u8 {
        self.ty
    }
}

impl Encodable2718 for TypedReceipt {
    fn encode_2718_len(&self) -> usize {
        (if self.ty != 0 { 1 } else { 0 }) +
            self.receipt.rlp_encoded_length_with_bloom(&self.receipt.bloom())
    }

    fn encode_2718(&self, out: &mut dyn alloy::rlp::BufMut) {
        if self.ty != 0 {
            out.put_u8(self.ty);
        }
        self.receipt.rlp_encode_with_bloom(&self.receipt.bloom(), out)
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
