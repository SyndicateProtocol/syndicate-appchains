// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";

/// @title AtomicSequencerImplementation
/// @notice Implementation contract containing the logic for atomic sequencing
contract AtomicSequencerImplementation {
    /// @dev Thrown when the chain addresses are invalid
    error InvalidChainAddresses();

    /// @notice Processes single transactions on two Metabased chains atomically.
    /// @param chainA The first Metabased chain
    /// @param chainB The second Metabased chain
    /// @param txnA The transaction for the first chain
    /// @param txnB The transaction for the second chain
    /// @param isRaw Whether to use raw transaction processing
    function processTransactionsAtomically(
        MetabasedSequencerChain chainA,
        MetabasedSequencerChain chainB,
        bytes calldata txnA,
        bytes calldata txnB,
        bool isRaw
    ) external {
        // Validate chain addresses
        if (address(chainA) == address(0) || address(chainB) == address(0) || address(chainA) == address(chainB)) {
            revert InvalidChainAddresses();
        }

        // Process transactions using appropriate method
        if (isRaw) {
            chainA.processTransactionRaw(txnA);
            chainB.processTransactionRaw(txnB);
        } else {
            chainA.processTransaction(txnA);
            chainB.processTransaction(txnB);
        }
    }

    /// @notice Processes bulk transactions on two Metabased chains atomically.
    /// @param chainA The first Metabased chain
    /// @param chainB The second Metabased chain
    /// @param txnsA The transactions for the first chain
    /// @param txnsB The transactions for the second chain
    function processBulkTransactionsAtomically(
        MetabasedSequencerChain chainA,
        MetabasedSequencerChain chainB,
        bytes[] calldata txnsA,
        bytes[] calldata txnsB
    ) external {
        // Validate chain addresses
        if (address(chainA) == address(0) || address(chainB) == address(0) || address(chainA) == address(chainB)) {
            revert InvalidChainAddresses();
        }

        // Process bulk transactions on both chains
        chainA.processBulkTransactions(txnsA);
        chainB.processBulkTransactions(txnsB);
    }
}
