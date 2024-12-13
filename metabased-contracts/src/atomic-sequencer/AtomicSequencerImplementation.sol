// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";

/// @title AtomicSequencerImplementation
/// @notice Implementation contract containing the logic for atomic sequencing
contract AtomicSequencerImplementation {
    /// @dev Thrown when a chain address is zero
    error InvalidChainAddresses();

    /// @dev Thrown when input array lengths don't match or are invalid
    error InputLengthMismatchError();

    /// @dev Validates chain addresses and array lengths
    /// @param chains Array of Metabased chains to validate
    /// @param transactions Array of transactions to validate
    /// @param isRawArray Array of flags indicating raw transaction processing
    modifier onlyValidChains(
        MetabasedSequencerChain[] calldata chains,
        bytes[] calldata transactions,
        bool[] calldata isRawArray
    ) {
        // Check array lengths match
        if (chains.length == 0 || chains.length != transactions.length || chains.length != isRawArray.length) {
            revert InputLengthMismatchError();
        }

        // Check for zero addresses
        for (uint256 i = 0; i < chains.length; i++) {
            if (address(chains[i]) == address(0)) {
                revert InvalidChainAddresses();
            }
        }
        _;
    }

    /// @notice Processes transactions on multiple Metabased chains atomically.
    /// @param chains Array of Metabased chains
    /// @param transactions Array of transactions corresponding to each chain
    /// @param isRawArray Array indicating whether each transaction should use raw processing
    function processTransactionsAtomically(
        MetabasedSequencerChain[] calldata chains,
        bytes[] calldata transactions,
        bool[] calldata isRawArray
    ) external onlyValidChains(chains, transactions, isRawArray) {
        // Process transactions using appropriate method per transaction
        for (uint256 i = 0; i < chains.length; i++) {
            if (isRawArray[i]) {
                chains[i].processTransactionRaw(transactions[i]);
            } else {
                chains[i].processTransaction(transactions[i]);
            }
        }
    }

    /// @notice Processes bulk transactions on multiple Metabased chains atomically.
    /// @param chains Array of Metabased chains
    /// @param transactions Array of transaction arrays corresponding to each chain
    /// @param isRawArray Array indicating whether each chain should use raw processing
    function processBulkTransactionsAtomically(
        MetabasedSequencerChain[] calldata chains,
        bytes[][] calldata transactions,
        bool[] calldata isRawArray
    ) external onlyValidChains(chains, transactions, isRawArray) {
        // Process bulk transactions using appropriate method per chain
        for (uint256 i = 0; i < chains.length; i++) {
            if (isRawArray[i]) {
                chains[i].processBulkTransactionsRaw(transactions[i]);
            } else {
                chains[i].processBulkTransactions(transactions[i]);
            }
        }
    }
}
