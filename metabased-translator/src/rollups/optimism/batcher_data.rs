use alloy_primitives::B256;
use op_alloy::protocol::{
    Batch, ChannelOut, CompressionAlgo, SingleBatch, VariantCompressor
};
use op_alloy::genesis::RollupConfig;


// Constants
const BATCHER_TRANSACTION_VERSION_BYTE: u8 = 0x00;
const FRAME_SIZE: usize = 1000000;

pub fn get_batcher_data(batch: SingleBatch) -> Vec<u8> {

    // Calculate the id from the epoch hash
    let id = B256::from(batch.epoch_hash)[..16]
    .try_into()
    .expect("16 bytes always fit");

    let config = RollupConfig::default();
    let compressor: VariantCompressor = CompressionAlgo::Zlib.into();
    // let compressor: VariantCompressor = CompressionAlgo::Brotli10.into();
    let mut channel_out = ChannelOut::new(id, &config, compressor);

    channel_out.add_batch(Batch::Single(batch)).unwrap();
    channel_out.close();

    // loop through and get all frames
    let mut frames = Vec::new();
    while channel_out.ready_bytes() > 0 {
        let frame = channel_out.output_frame(FRAME_SIZE).expect("Failed to output frame");
        frames.push(frame);
    }

    let mut buf = Vec::new();
    buf.push(BATCHER_TRANSACTION_VERSION_BYTE); // Add version byte at the beginning

    for frame in frames {
        buf.extend_from_slice(&frame.encode());
    }

    buf
}
