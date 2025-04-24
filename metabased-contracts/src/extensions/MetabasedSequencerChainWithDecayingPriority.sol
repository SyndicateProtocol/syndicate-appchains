// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "../MetabasedSequencerChain.sol";

/**
 * @title MetabasedSequencerChainWithDecayingPriority
 * @notice This is an example on how to extend the MetabasedSequencerChain.
 * It is not necessarily a feature implemented in Metabased Sequencer.
 * It only serves as example and does not purport that this will be added in the future.
 */
contract MetabasedSequencerChainWithDecayingPriority is MetabasedSequencerChain {
    /// @notice The constant rate at which priority decays (10 units per second)
    uint256 public constant PRIORITY_DECAY_RATE = 10;

    /// @notice Emitted when a new transaction is processed with priority and timestamp
    event TransactionProcessed(address indexed sender, bytes data, uint256 originalPriority, uint256 timestamp);

    /// @notice Constructs the MetabasedSequencerChainWithDecayingPriority contract.
    /// @param _l3ChainId The ID of the L3 chain that this contract is sequencing transactions for.
    constructor(uint256 _l3ChainId) MetabasedSequencerChain(_l3ChainId) {}

    /// @notice Processes a single compressed transaction with priority.
    /// @param data The compressed transaction data.
    /// @param priority The initial priority of the transaction.
    function processTransactionRaw(bytes calldata data, uint256 priority)
        external
        onlyWhenAllowed(msg.sender)
        revertForUnallowedCalldata(data)
    {
        emit TransactionProcessed(msg.sender, data, priority, block.timestamp);
    }

    /// @notice Processes a single transaction with priority, prepending a zero byte.
    /// @dev Prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data The transaction data
    /// @param priority The initial priority of the transaction
    function processTransaction(bytes calldata data, uint256 priority)
        external
        onlyWhenAllowed(msg.sender)
        revertForUnallowedCalldata(data)
    {
        emit TransactionProcessed(msg.sender, prependZeroByte(data), priority, block.timestamp);
    }

    /// @notice Processes multiple transactions in bulk with individual priorities.
    /// @dev Prepends a zero byte to each transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    /// @param priorities An array of priorities for the transactions.
    function processBulkTransactions(bytes[] calldata data, uint256[] calldata priorities)
        external
        onlyWhenAllowed(msg.sender)
    {
        uint256 dataCount = data.length;
        require(dataCount == priorities.length, "Data and priority arrays must have the same length");

        // Process all transactions
        for (uint256 i = 0; i < dataCount; i++) {
            _revertForUnallowedCalldata(data[i]);
            emit TransactionProcessed(msg.sender, prependZeroByte(data[i]), priorities[i], block.timestamp);
        }
    }

    /// @notice Calculate the effective priority after decay based on time elapsed
    /// @dev This is just a read function for calculation. It is not used anywhere on purpose.
    /// @param originalPriority The original priority of the transaction
    /// @param submittedTimestamp The timestamp when the transaction was submitted
    /// @param currentTimestamp The current timestamp to calculate the decay against
    /// @return The effective priority after applying the decay formula
    function calculateEffectivePriority(uint256 originalPriority, uint256 submittedTimestamp, uint256 currentTimestamp)
        public
        pure
        returns (uint256)
    {
        // If current time is at or before submission time, no decay applies
        if (currentTimestamp <= submittedTimestamp) {
            return originalPriority;
        }

        // Calculate time elapsed in seconds
        uint256 elapsedTime = currentTimestamp - submittedTimestamp;

        // Calculate decay amount = time elapsed × decay rate
        uint256 decayAmount = elapsedTime * PRIORITY_DECAY_RATE;

        // If decay would reduce priority below zero, return zero
        if (decayAmount >= originalPriority) {
            return 0;
        }

        // Return original priority minus the decay amount
        return originalPriority - decayAmount;
    }
}
