// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "../MetabasedSequencerChain.sol";

/// @title MetabasedSequencerChainWithPriority
/// @notice extends MetabasedSequencerChain to allow processing of transactions with priority
contract MetabasedSequencerChainWithPriority is MetabasedSequencerChain {
    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes data, uint256 priority);

    /// @notice Constructs the MetabasedSequencerChainWithPriority contract.
    /// @param _l3ChainId The ID of the L3 chain that this contract is sequencing transactions for.
    /// @param admin The address that will be set as the admin
    /// @param masterModule The address of the master permission module
    constructor(uint256 _l3ChainId, address admin, address masterModule)
        MetabasedSequencerChain(_l3ChainId, admin, masterModule)
    {}

    /// @notice Processes a single compressed transaction.
    /// @param data The compressed transaction data.
    /// @param priority The priority of the transaction.
    function processTransactionRaw(bytes calldata data, uint256 priority) external onlyWhenAllowed(msg.sender) {
        emit TransactionProcessed(msg.sender, data, priority);
    }

    /// @notice process transactions
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data The transaction data
    /// @param priority The priority of the transaction
    function processTransaction(bytes calldata data, uint256 priority) external onlyWhenAllowed(msg.sender) {
        emit TransactionProcessed(msg.sender, prependZeroByte(data), priority);
    }

    /// @notice Processes multiple transactions in bulk.
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    /// @param priority An array of priorities for the transactions.
    function processBulkTransactions(bytes[] calldata data, uint256[] calldata priority)
        external
        onlyWhenAllowed(msg.sender)
    {
        uint256 dataCount = data.length;

        // Process all transactions
        for (uint256 i = 0; i < dataCount; i++) {
            emit TransactionProcessed(msg.sender, prependZeroByte(data[i]), priority[i]);
        }
    }
}
