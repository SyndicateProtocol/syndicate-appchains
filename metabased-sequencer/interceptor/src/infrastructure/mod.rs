mod sol;
#[allow(dead_code)]
mod zlib_compression;

pub use sol::SolMetabasedSequencerChainService;
pub use zlib_compression::{compress_transactions};
use alloy::sol;
sol!(
    #[sol(rpc)]
    "../../metabased-contracts/src/MetabasedSequencerChain.sol"
);
pub use MetabasedSequencerChain as MetabasedSequencerChainInstance;