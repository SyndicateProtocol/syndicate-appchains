// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SyndicateSequencingChain} from "../SyndicateSequencingChain.sol";

/**
 * @title SyndicateSequencingChainWithDecayingPriority
 * @notice Extension of SyndicateSequencingChain that implements a decaying priority mechanism for transactions.
 * This is only an example implementation. Not for production use.
 */
contract SyndicateSequencingChainWithDecayingPriority is SyndicateSequencingChain {
    /// @notice Modifier to checks if an address is allowed to submit txs based on the sender, origin and data
    /// @param proposer The address to check
    /// @param originator The address of tx.origin. Useful to know the sender originator in wrapper contracts
    /// @param data The calldata to check
    modifier onlyWhenAllowed(address proposer, address originator, bytes calldata data) {
        if (!isAllowed(proposer, originator, data)) revert TransactionOrProposerNotAllowed();
        _;
    }

    /// @notice The constant rate at which priority decays (10 units per second)
    uint256 public constant PRIORITY_DECAY_RATE = 10;

    /// @notice Emitted when a new transaction is processed with priority and timestamp
    event TransactionProcessed(address indexed sender, bytes data, uint256 originalPriority, uint256 timestamp);

    /// @notice Constructs the SyndicateSequencingChainWithDecayingPriority contract.
    /// @param _appChainId The ID of the App chain that this contract is sequencing transactions for.
    //#olympix-ignore-no-parameter-validation-in-constructor
    constructor(uint256 _appChainId) SyndicateSequencingChain(_appChainId) {}

    /// @notice Processes a single compressed transaction with priority.
    /// @param data The compressed transaction data.
    /// @param priority The initial priority of the transaction.
    //#olympix-ignore-required-tx-origin
    function processTransactionRaw(bytes calldata data, uint256 priority)
        external
        onlyWhenAllowed(msg.sender, tx.origin, data)
    {
        emit TransactionProcessed(msg.sender, data, priority, block.timestamp);
    }

    /// @notice Processes a single transaction with priority, prepending a zero byte.
    /// @dev Prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data The transaction data
    /// @param priority The initial priority of the transaction
    function processTransaction(bytes calldata data, uint256 priority)
        external
        onlyWhenAllowed(msg.sender, tx.origin, data)
    {
        emit TransactionProcessed(msg.sender, prependZeroByte(data), priority, block.timestamp); //#olympix-ignore-external-call-potential-out-of-gas
    }

    /// @notice Processes multiple transactions in bulk with individual priorities.
    /// @dev Prepends a zero byte to each transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    /// @param priorities An array of priorities for the transactions.
    //#olympix-ignore-reentrancy-events
    function processBulkTransactions(bytes[] calldata data, uint256[] calldata priorities) external {
        uint256 dataCount = data.length;
        require(dataCount == priorities.length, "Data and priority arrays must have the same length");

        // Process all transactions
        for (uint256 i = 0; i < dataCount; i++) {
            bool isAllowed = isAllowed(msg.sender, tx.origin, data[i]); //#olympix-ignore-any-tx-origin
            if (isAllowed) {
                // only emit the event if the transaction is allowed
                //#olympix-ignore-external-call-potential-out-of-gas
                emit TransactionProcessed(msg.sender, prependZeroByte(data[i]), priorities[i], block.timestamp);
            }
        }
    }

    /// @notice Calculate the effective priority after decay based on time elapsed
    /// @dev This is just a read function for calculation. It is not used internally by the contract.
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
