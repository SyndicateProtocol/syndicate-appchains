// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.29;

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";

/// @title AtomicSequencerImplementation
/// @notice Implementation contract containing the logic for atomic sequencing
// [Olympix Warning: unfuzzed variables, missing events assertion] These test-related warnings are not security critical
// as the contract uses standard unit tests and integration tests. Parameter validation is handled through array length checks.
contract AtomicSequencerImplementation {
    /// @dev Thrown when input array lengths don't match or are invalid
    error InputLengthMismatchError();

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

        for (uint256 i = 0; i < chains.length; i++) {
            if (isRaw[i]) {
                chains[i].processTransactionRaw(transactions[i]);
            } else {
                chains[i].processTransaction(transactions[i]);
            }
        }
    }

    /// @notice Processes bulk transactions on multiple Syndicate chains atomically. Only used with encoded transactions.
    /// @param chains Array of Syndicate chains
    /// @param transactions Array of transaction arrays corresponding to each chain
    function processBulkTransactionsAtomically(
        SyndicateSequencingChain[] calldata chains,
        bytes[][] calldata transactions
    ) external {
        if (chains.length == 0 || chains.length != transactions.length) {
            revert InputLengthMismatchError();
        }

        for (uint256 i = 0; i < chains.length; i++) {
            chains[i].processBulkTransactions(transactions[i]);
        }
    }
}
