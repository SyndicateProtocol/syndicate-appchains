use crate::rollups::optimism::batch::Batch;
use crate::rollups::optimism::frame::to_data;
use alloy::network::TransactionBuilder;
use alloy_primitives::{Address, Bytes, B256, U256};
use alloy_rpc_types::{TransactionInput, TransactionRequest};

pub fn new_batcher_tx(from: Address, to: Address, data: Bytes) -> TransactionRequest {
    let input = TransactionInput::new(data);
    let batcher_tx = TransactionRequest::default()
        .from(from)
        .to(to)
        .input(input);
    batcher_tx
}

// pub fn test() {
//     let batch = Batch {
//         parent_hash: B256::repeat_byte(0x11),
//         epoch_num: 42,
//         epoch_hash: B256::repeat_byte(0x22),
//         timestamp: 1712500000,
//         transactions: vec![b"tx1".to_vec(), b"tx2".to_vec()],
//     };
//     let frames = batch.get_frames(1000).unwrap();
//     let data = to_data(&frames).unwrap();
//     let batcher_tx = new_batcher_tx(Address::ZERO, Address::ZERO, data.into());
//     println!("{:?}", batcher_tx);
// }
