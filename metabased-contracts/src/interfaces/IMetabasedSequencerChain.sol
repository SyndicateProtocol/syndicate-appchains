// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

/// @title IMetabasedSequencerChain
/// @notice Interface for the MetabasedSequencerChain contract
interface IMetabasedSequencerChain {
    /// @notice Process a transaction
    /// @param data The transaction data to process
    function processTransaction(bytes calldata data) external;
}
