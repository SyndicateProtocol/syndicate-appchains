use alloy::sol;

sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract MetabasedSequencerChain {
        function processTransaction(bytes calldata encodedTxn) public;
    }
}
