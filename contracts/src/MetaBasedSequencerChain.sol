// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireListManager} from "./RequireListManager.sol";

/// @title MetabasedSequencerChain
/// @notice The core contract for sequencing transactions using a modular architecture to determine who is allowed to sequence.
contract MetabasedSequencerChain is RequireListManager {
    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes encodedTxn);

    /// @dev Thrown when the transaction form is invalid.
    error InvalidTransactionForm();

    /// @notice Emits a TransactionProcessed event without additional processing
    /// @dev it assumes that the validation is done outise the contract, i.e. op-translator
    /// @param encodedTxn The encoded transaction data
    function emitTransactionProcessed(bytes calldata encodedTxn) public {
        emit TransactionProcessed(msg.sender, encodedTxn);
    }

    /// @notice Processes a single encoded transaction.
    /// @param encodedTxn The encoded transaction data.
    function processTransaction(bytes calldata encodedTxn) public {
        // Validate transaction form
        if (!isValidTransactionForm(encodedTxn)) {
            revert InvalidTransactionForm();
        }

        // Check if msg.sender is allowed
        requireAllAllowed(msg.sender);
        requireAnyAllowed(msg.sender);

        // Emit event with transaction details
        emit TransactionProcessed(msg.sender, encodedTxn);
    }

    /// @notice Processes multiple encoded transactions in bulk.
    /// @param encodedTxns An array of encoded transaction data.
    function processBulkTransactions(bytes[] calldata encodedTxns) public {
        uint256 txnCount = encodedTxns.length;

        // Process all transactions
        for (uint256 i = 0; i < txnCount; i++) {
            processTransaction(encodedTxns[i]);
        }
    }

    /// @dev Validates the form of the encoded transaction.
    /// @param encodedTxn The encoded transaction to validate.
    /// @return bool True if the transaction form is valid, false otherwise.
    function isValidTransactionForm(bytes calldata encodedTxn) internal pure returns (bool) {
        // TODO: Implement transaction form validation logic here
        // Linear task: https://linear.app/syndicate/issue/SEQ-80/implement-transaction-form-validation
        // It should be replaced with actual validation
        return encodedTxn.length > 0;
    }
}
