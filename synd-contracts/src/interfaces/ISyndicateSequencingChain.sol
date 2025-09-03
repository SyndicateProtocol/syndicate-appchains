// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title ISyndicateSequencingChain
/// @notice Interface for the SyndicateSequencingChain contract
interface ISyndicateSequencingChain {
    /// @notice The ID of the App chain that this contract is sequencing transactions for
    function appchainId() external view returns (uint256);

    function encodeTransaction(bytes calldata data) external returns (bytes memory);

    /// @notice Process a signed transaction.
    /// @param data Transaction data
    function processTransaction(bytes calldata data) external;

    /// @notice Processes multiple signed transactions in bulk.
    /// @param data An array of transaction data.
    function processTransactionsBulk(bytes[] calldata data) external;
}
