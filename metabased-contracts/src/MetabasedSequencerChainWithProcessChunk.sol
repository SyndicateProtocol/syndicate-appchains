// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "./MetabasedSequencerChain.sol";

/// @title MetabasedSequencerChainWithProcessChunk
/// @notice extends MetabasedSequencerChain to allow processing of chunked transactions
contract MetabasedSequencerChainWithProcessChunk is MetabasedSequencerChain {
    /// @notice Tracks the processing state of chunked transactions
    struct ChunkState {
        uint256 chunkMask; // Bitmask tracking which chunks have been processed
        uint256 totalChunks; // Total number of expected chunks
        address owner; // Address that initiated the chunk processing
        bool isComplete; // Whether all chunks have been processed
    }

    /// @notice Mapping from transaction hash id to chunk processing state
    mapping(bytes32 txHashProcessId => ChunkState) public chunkStates;

    /// @notice Emitted when a chunk of transactions is processed.
    event TransactionChunkProcessed(bytes txChunk, uint256 index, uint256 totalChunks, bytes32 txHashForParent);

    /// @notice Emitted when all chunks for a transaction have been processed
    event ChunkProcessingCompleted(bytes32 indexed txHashForParent);

    /// @dev Thrown when an invalid chunk configuration is provided
    error InvalidChunkConfig();

    /// @dev Thrown when chunk index exceeds total chunks
    error InvalidChunkIndex();

    /// @dev Thrown when an unauthorized address tries to process a chunk
    error UnauthorizedChunkProcessor();

    /// @dev Thrown when trying to process a chunk for a completed transaction
    error ChunkProcessingAlreadyComplete();

    /// @notice Constructs the MetabasedSequencerChain contract.
    /// @param _l3ChainId The ID of the L3 chain that this contract is sequencing transactions for.
    /// @param admin The address that will be set as the admin
    /// @param masterModule The address of the master permission module
    constructor(uint256 _l3ChainId, address admin, address masterModule)
        MetabasedSequencerChain(_l3ChainId, admin, masterModule)
    {}

    /// @notice Processes a chunk of transactions from a larger batch.
    /// @param txChunk the compressed chunked transaction data.
    /// @param index The index for this chunk in the overall batch.
    /// @param totalChunks The total number of chunks in the batch. Due to bitmasking, this must be <= 256.
    /// @param txHashForParent The hash identifying this batch of chunks.
    function processChunk(bytes calldata txChunk, uint256 index, uint256 totalChunks, bytes32 txHashForParent)
        external
        onlyWhenAllowed(msg.sender)
    {
        // Validate chunk parameters
        if (totalChunks == 0 || totalChunks > 256) {
            revert InvalidChunkConfig();
        }

        if (index >= totalChunks) {
            revert InvalidChunkIndex();
        }
        bytes32 txHashProcessId = keccak256(abi.encode(txHashForParent, msg.sender, tx.origin));

        ChunkState storage state = chunkStates[txHashProcessId];

        // If this is the first chunk for this txHashForParent
        if (state.owner == address(0)) {
            state.owner = msg.sender;
            state.totalChunks = totalChunks;
        } else {
            // Verify the sender is the original chunk processor
            if (state.owner != msg.sender) {
                revert UnauthorizedChunkProcessor();
            }

            // Verify chunk processing hasn't been completed
            if (state.isComplete) {
                revert ChunkProcessingAlreadyComplete();
            }
        }

        // Update the bitmask to mark this chunk as received
        uint256 chunkBit = 1 << index;
        state.chunkMask = state.chunkMask | chunkBit;

        // Emit the chunk processing event
        emit TransactionProcessed(msg.sender, txChunk);
        emit TransactionChunkProcessed(txChunk, index, totalChunks, txHashForParent);

        // Check if all chunks have been received
        uint256 expectedMask = (1 << totalChunks) - 1;
        if (state.chunkMask == expectedMask) {
            state.isComplete = true;
            emit ChunkProcessingCompleted(txHashForParent);
        }
    }
}
