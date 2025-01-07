// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain_ReleaseCandidate} from "../MetabasedSequencerChain_ReleaseCandidate.sol";

/// @title AtomicSequencerImplementation
/// @notice Implementation contract containing the logic for atomic sequencing
contract AtomicSequencerImplementation_ReleaseCandidate {
    /// @dev Thrown when input array lengths don't match or are invalid
    error InputLengthMismatchError();

    /// @notice Processes transactions on multiple Metabased chains atomically.
    /// @param chains Array of Metabased chains
    /// @param transactions Array of transactions corresponding to each chain
    /// @param isRaw Array indicating whether each transaction should use raw processing
    function processTransactionsAtomically(
        MetabasedSequencerChain_ReleaseCandidate[] calldata chains,
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

    /// @notice Processes bulk transactions on multiple Metabased chains atomically. Only used with encoded transactions.
    /// @param chains Array of Metabased chains
    /// @param transactions Array of transaction arrays corresponding to each chain
    function processBulkTransactionsAtomically(
        MetabasedSequencerChain_ReleaseCandidate[] calldata chains,
        bytes[][] calldata transactions
    ) external {
        for (uint256 i = 0; i < chains.length; i++) {
            chains[i].processBulkTransactions(transactions[i]);
        }
    }
}
