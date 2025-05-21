// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title ISyndicateSequencingChain
/// @notice Interface for the SyndicateSequencingChain contract
interface ISyndicateSequencingChain {
    /// @notice Processes an uncompressed transaction.
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data An array of transaction data without prepended zero bytes.
    function processTransactionUncompressed(bytes calldata data) external;

    //// @notice Processes a compressed transaction.
    /// @param data The compressed transaction data.
    function processTransaction(bytes calldata data) external;

    /// @notice Process bulk transactions
    /// @param data The array of transaction data to process
    function processTransactionsBulk(bytes[] calldata data) external;
}
