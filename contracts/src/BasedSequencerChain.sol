// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireListManager} from "./RequireListManager.sol";

/// @title BasedSequencerChain
/// @notice The core contract for sequencing chains using a modular architecture to determine who is allowed to sequence.
contract BasedSequencerChain is RequireListManager {
    /// @notice The length of an epoch in seconds.
    /// @dev Ideally, epochs are as fast as possible to allow a wide variety of based sequencers to participate.
    uint256 public constant EPOCH_LENGTH = 1; // 1 second.

    /// @notice The initial epoch number of the chain.
    /// @dev We use an initial epoch number rather than starting the chain at block 1 to keep block numbers consistent across L3s on the same L2.
    uint256 public immutable INITIAL_EPOCH_NUMBER;

    /// @notice The maximum size of the bid list.
    /// @dev This logic will eventually be modularized, so take it as more of a reference example than the final implementation.
    uint256 public constant MAX_BID_LIST_SIZE = 5;

    /// @notice The epoch number of the last non-empty block.
    /// @dev This value is necessary to keep track of the previous parent hashes. Otherwise, the parent hash would be the empty block.
    uint256 public lastNonEmptyEpochNumber;

    /// @notice The data that the user provides when submitting a batch.
    /// @dev The rest of the batch struct will be auto-filled.
    struct UserProvidedBatch {
        /// @notice The parent hash of the last non-empty block.
        /// @dev We'll get the epoch number automatically, so users don't need to pass it in.
        bytes32 non_empty_parent_hash;
        /// @notice The list of transactions that the user wants to submit.
        bytes[] transaction_list; // EIP-2718 encoded transactions
    }

    /// @notice The structure of a batch based on the Optimism batch structure.
    struct Batch {
        /// @notice The parent hash of the batch.
        bytes32 parent_hash;
        /// @notice The epoch number of the parent batch.
        uint256 parent_epoch_number;
        /// @notice The epoch number of the batch, equivalent to a block number.
        uint256 epoch_number;
        /// @notice The hash of the transaction list, which will be referenced by the next block as the parent hash.
        bytes32 epoch_hash;
        /// @notice The list of transactions in the batch.
        bytes[] transaction_list; // EIP-2718 encoded transactions
    }

    /// @notice Mapping from epoch number to batch.
    /// @dev If an epoch number has no corresponding batch, it is treated as an empty block.
    mapping(uint256 epochNumber => Batch batch) public batches;

    /// @notice Emitted when a new batch is sequenced.
    event LatestBatchProcessed(
        bytes32 parent_hash,
        uint256 parent_epoch_number,
        uint256 indexed epoch_number,
        bytes32 epoch_hash,
        bytes[] transaction_list
    );

    /// @dev Emitted when the parent hash of a batch does not match the expected parent hash.
    /// @param expectedParentHash The expected parent hash.
    /// @param actualParentHash The actual parent hash provided in the batch.
    error ParentHashDoesNotMatch(bytes32 expectedParentHash, bytes32 actualParentHash);

    /// @dev Constructor function.
    /// @notice Sets the initial epoch number to the current epoch number.
    constructor() {
        INITIAL_EPOCH_NUMBER = calculateCurrentEpochNumber();
    }

    /// @notice Sequences the next batch.
    /// @param userProvidedBatch The user-provided batch data.
    function sequenceNextBatch(UserProvidedBatch calldata userProvidedBatch) public {
        if (!checkParentHash(userProvidedBatch.non_empty_parent_hash)) {
            revert ParentHashDoesNotMatch(
                batches[lastNonEmptyEpochNumber].epoch_hash, userProvidedBatch.non_empty_parent_hash
            );
        }

        requireAllAllowed(msg.sender);
        requireAnyAllowed(msg.sender);

        Batch memory batch = Batch({
            parent_hash: userProvidedBatch.non_empty_parent_hash,
            parent_epoch_number: lastNonEmptyEpochNumber,
            epoch_number: calculateCurrentEpochNumber(),
            epoch_hash: keccak256(abi.encode(userProvidedBatch.transaction_list)),
            transaction_list: userProvidedBatch.transaction_list
        });

        batches[batch.epoch_number] = batch;

        lastNonEmptyEpochNumber = batch.epoch_number;

        emit LatestBatchProcessed(
            batch.parent_hash, batch.parent_epoch_number, batch.epoch_number, batch.epoch_hash, batch.transaction_list
        );
    }

    /// @notice Calculates the epoch number based on a given timestamp.
    /// @param timestamp The timestamp to calculate the epoch number for.
    /// @return The calculated epoch number.
    function calculateEpochNumber(uint256 timestamp) public pure returns (uint256) {
        return timestamp / EPOCH_LENGTH;
    }

    /// @notice Calculates the current epoch number based on the current block timestamp.
    /// @return The current epoch number.
    function calculateCurrentEpochNumber() public view returns (uint256) {
        return calculateEpochNumber(block.timestamp);
    }

    /// @notice Checks if the provided parent hash matches the last non-empty epoch's hash.
    /// @param parentHash The parent hash to check.
    /// @return True if the parent hash matches, false otherwise.
    function checkParentHash(bytes32 parentHash) public view returns (bool) {
        return parentHash == batches[lastNonEmptyEpochNumber].epoch_hash;
    }
}
