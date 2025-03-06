//! The `contract` module contains the contract bindings for the metabased sequencer.

use alloy::sol;

// TODO [SEQ-658]: Use contract bindings
sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract MetabasedSequencerChain {
        function processTransaction(bytes calldata encodedTxn) public;
    }
}
