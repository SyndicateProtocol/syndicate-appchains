// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

/// @title IMetabasedSequencerChain
/// @notice Interface for the MetabasedSequencerChain contract
interface IMetabasedSequencerChain {
    /// @notice Process a transaction
    /// @param data The transaction data to process
    function processTransaction(bytes calldata data) external;

    /// @notice Process a raw transaction
    /// @param data The transaction data to process
    function processTransactionRaw(bytes calldata data) external;

    /// @notice Process bulk transactions
    /// @param data The array of transaction data to process
    function processBulkTransactions(bytes[] calldata data) external;
}
