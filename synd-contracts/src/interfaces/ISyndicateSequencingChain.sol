// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title ISyndicateSequencingChain
/// @notice Interface for the SyndicateSequencingChain contract
interface ISyndicateSequencingChain {
    /// @notice Process a transaction
    /// @param data The transaction data to process
    function processTransactionUncompressed(bytes calldata data) external;

    /// @notice Process a raw transaction
    /// @param data The transaction data to process
    function processTransaction(bytes calldata data) external;

    /// @notice Process bulk transactions
    /// @param data The array of transaction data to process
    function processTransactionsBulk(bytes[] calldata data) external;
}
