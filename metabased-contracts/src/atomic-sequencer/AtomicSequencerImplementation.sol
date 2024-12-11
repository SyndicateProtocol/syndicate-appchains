// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";

/// @title AtomicSequencerImplementation
/// @notice Implementation contract containing the logic for atomic sequencing
contract AtomicSequencerImplementation {
    /// @dev Thrown when the chain addresses are invalid
    error InvalidChainAddresses();

    /// @notice Processes transactions on multiple Metabased chains atomically.
    /// @param chains Array of Metabased chains
    /// @param transactions Array of transactions corresponding to each chain
    /// @param isRaw Whether to use raw transaction processing
    function processTransactionsAtomically(
        MetabasedSequencerChain[] calldata chains,
        bytes[] calldata transactions,
        bool isRaw
    ) external {
        // Validate input arrays
        if (chains.length == 0 || chains.length != transactions.length) {
            revert InvalidChainAddresses();
        }

        // Validate chain addresses (only check for zero address)
        for (uint256 i = 0; i < chains.length; i++) {
            if (address(chains[i]) == address(0)) {
                revert InvalidChainAddresses();
            }
        }

        // Process transactions using appropriate method
        for (uint256 i = 0; i < chains.length; i++) {
            if (isRaw) {
                chains[i].processTransactionRaw(transactions[i]);
            } else {
                chains[i].processTransaction(transactions[i]);
            }
        }
    }

    /// @notice Processes bulk transactions on multiple Metabased chains atomically.
    /// @param chains Array of Metabased chains
    /// @param transactions Array of transaction arrays corresponding to each chain
    function processBulkTransactionsAtomically(
        MetabasedSequencerChain[] calldata chains,
        bytes[][] calldata transactions
    ) external {
        // Validate input arrays
        if (chains.length == 0 || chains.length != transactions.length) {
            revert InvalidChainAddresses();
        }

        // Validate chain addresses (only check for zero address)
        for (uint256 i = 0; i < chains.length; i++) {
            if (address(chains[i]) == address(0)) {
                revert InvalidChainAddresses();
            }
        }

        // Process bulk transactions on all chains
        for (uint256 i = 0; i < chains.length; i++) {
            chains[i].processBulkTransactions(transactions[i]);
        }
    }
}
