// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";

/// @title AtomicSequencerImplementation
/// @notice Implementation contract containing the logic for atomic sequencing
// [Olympix Warning: unfuzzed variables, missing events assertion] These test-related warnings are not security critical
// as the contract uses standard unit tests and integration tests. Parameter validation is handled through array length checks.
contract AtomicSequencerImplementation {
    // Maximum number of chains that can be processed atomically to prevent DoS attacks
    uint256 public constant MAX_ATOMIC_CHAINS = 20;
    // Maximum number of transactions per chain in bulk atomic operations to prevent DoS attacks
    uint256 public constant MAX_ATOMIC_BULK_TRANSACTIONS = 50;

    /// @dev Thrown when input array lengths don't match or are invalid
    error InputLengthMismatchError();
    /// @dev Thrown when too many chains are provided for atomic processing
    error TooManyAtomicChains();
    /// @dev Thrown when transaction array exceeds maximum size for atomic bulk processing
    error TransactionArrayExceedsMaximum();

    /// @notice Processes transactions on multiple Syndicate chains atomically.
    /// @param chains Array of Syndicate chains
    /// @param transactions Array of transactions corresponding to each chain
    /// @param isRaw Array indicating whether each transaction should use raw processing
    function processTransactionsAtomically(
        SyndicateSequencingChain[] calldata chains,
        bytes[] calldata transactions,
        bool[] calldata isRaw
    ) external {
        // Check array lengths match
        if (chains.length == 0 || chains.length != transactions.length || chains.length != isRaw.length) {
            revert InputLengthMismatchError();
        }
        if (chains.length > MAX_ATOMIC_CHAINS) revert TooManyAtomicChains();

        for (uint256 i = 0; i < chains.length; i++) {
            if (isRaw[i]) {
                chains[i].processTransaction(transactions[i]);
            } else {
                chains[i].processTransactionUncompressed(transactions[i]);
            }
        }
    }

    /// @notice Processes bulk transactions on multiple Syndicate chains atomically. Only used with encoded transactions.
    /// @param chains Array of Syndicate chains
    /// @param transactions Array of transaction arrays corresponding to each chain
    function processTransactionsBulkAtomically(
        SyndicateSequencingChain[] calldata chains,
        bytes[][] calldata transactions
    ) external {
        if (chains.length == 0 || chains.length != transactions.length) {
            revert InputLengthMismatchError();
        }
        if (chains.length > MAX_ATOMIC_CHAINS) revert TooManyAtomicChains();

        // Check individual transaction arrays don't exceed bulk limits
        for (uint256 i = 0; i < transactions.length; i++) {
            if (transactions[i].length > MAX_ATOMIC_BULK_TRANSACTIONS) {
                revert TransactionArrayExceedsMaximum();
            }
        }

        for (uint256 i = 0; i < chains.length; i++) {
            chains[i].processTransactionsBulk(transactions[i]);
        }
    }
}
