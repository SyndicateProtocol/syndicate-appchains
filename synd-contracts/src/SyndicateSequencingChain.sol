// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";

/// @title SyndicateSequencingChain
/// @notice The core contract for sequencing transactions using a modular architecture
/// to determine the address that is allowed to sequence and whether the calldata is allowed.
contract SyndicateSequencingChain is SequencingModuleChecker {
    /// @notice The ID of the App chain that this contract is sequencing transactions for.
    uint256 public immutable appChainId;

    /// @notice Constructs the SyndicateSequencingChain contract.
    /// @param _appChainId The ID of the App chain that this contract is sequencing transactions for.
    //#olympix-ignore-missing-revert-reason-tests
    constructor(uint256 _appChainId) SequencingModuleChecker() {
        // chain id zero has no replay protection: https://eips.ethereum.org/EIPS/eip-3788
        require(_appChainId != 0, "App chain ID cannot be 0");
        appChainId = _appChainId;
    }

    /// @notice Processes a single compressed transaction.
    /// @param data The compressed transaction data.
    //#olympix-ignore-required-tx-origin
    function processTransactionRaw(bytes calldata data) external {
        if (!transactionProcessed(data)) {
            revert TransactionOrProposerNotAllowed();
        }
    }

    /// @notice Process transactions
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data The transaction data
    //#olympix-ignore-required-tx-origin
    function processTransaction(bytes calldata data) external {
        if (!uncompressedTransactionProcessed(data)) {
            revert TransactionOrProposerNotAllowed();
        }
    }

    /// @notice Processes multiple transactions in bulk.
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    //#olympix-ignore-reentrancy-events
    function processBulkTransactions(bytes[] calldata data) external {
        uint256 dataCount = data.length;

        // Process all transactions
        for (uint256 i = 0; i < dataCount; i++) {
            uncompressedTransactionProcessed(data[i]);
        }
    }
}
