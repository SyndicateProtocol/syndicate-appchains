// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";

/// @title MetabasedSequencerChain
/// @notice The core contract for sequencing transactions using a modular architecture
/// to determine who is allowed to sequence.
contract MetabasedSequencerChain is SequencingModuleChecker {
    /// @notice The ID of the L3 chain that this contract is sequencing transactions for.
    uint256 public immutable l3ChainId;

    /// @notice Tracks the ownership and state of chunk processing
    struct ChunkState {
        address owner; // Address that initiated the chunk processing
        uint256 processedCount; // Number of chunks processed so far
        bool isComplete; // Whether the chunk processing is complete
        uint256 totalChunks; // Total number of chunks expected
        uint256 lastIndex; // Last processed index
    }

    /// @notice Mapping from transaction hash to chunk processing state
    mapping(bytes32 => ChunkState) public chunkStates;

    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes data);

    /// @notice Emitted when a chunk of transactions is processed.
    event TransactionChunkProcessed(bytes txChunk, uint256 index, uint256 totalChunks, bytes32 txHashForParent);

    /// @dev Thrown when an invalid chunk size is provided.
    error InvalidChunkSize();

    /// @dev Thrown when chunks are processed out of order
    error InvalidChunkOrder();

    /// @dev Thrown when an unauthorized address tries to process a chunk
    error UnauthorizedChunkProcessor();

    /// @dev Thrown when trying to process a chunk for a completed transaction
    error ChunkProcessingAlreadyComplete();

    /// @notice Constructs the MetabasedSequencerChain contract.
    /// @param _l3ChainId The ID of the L3 chain that this contract is sequencing transactions for.
    /// @param admin The address that will be set as the admin
    /// @param masterModule The address of the master permission module
    constructor(uint256 _l3ChainId, address admin, address masterModule) SequencingModuleChecker(admin, masterModule) {
        // chain id zero has no replay protection : https://eips.ethereum.org/EIPS/eip-3788
        require(_l3ChainId != 0, "L3 chain ID cannot be 0");
        l3ChainId = _l3ChainId;
    }

    /// @notice Processes a single compressed transaction.
    /// @param data The compressed transaction data.
    function processTransactionRaw(bytes calldata data) external onlyWhenAllowed(msg.sender) {
        emit TransactionProcessed(msg.sender, data);
    }

    /// @notice process transactions
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data The transaction data
    function processTransaction(bytes calldata data) external onlyWhenAllowed(msg.sender) {
        emit TransactionProcessed(msg.sender, prependZeroByte(data));
    }

    /// @notice Processes multiple transactions in bulk.
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    function processBulkTransactions(bytes[] calldata data) external onlyWhenAllowed(msg.sender) {
        uint256 dataCount = data.length;

        // Process all transactions
        for (uint256 i = 0; i < dataCount; i++) {
            emit TransactionProcessed(msg.sender, prependZeroByte(data[i]));
        }
    }

    /// @notice Processes a chunk of transactions from a larger batch.
    /// @param txChunk the compressed chunked transaction data.
    /// @param index The starting index for this chunk in the overall batch.
    /// @param totalChunks The number of transactions to process in this chunk.
    /// @param txHashForParent The hash of the parent transaction.
    function processChunk(bytes calldata txChunk, uint256 index, uint256 totalChunks, bytes32 txHashForParent)
        external
        onlyWhenAllowed(msg.sender)
    {
        if (totalChunks == 0) {
            revert InvalidChunkSize();
        }

        ChunkState storage state = chunkStates[txHashForParent];

        // If this is the first chunk for this txHashForParent
        if (state.owner == address(0)) {
            state.owner = msg.sender;
            state.totalChunks = totalChunks;
            state.lastIndex = type(uint256).max; // Initialize to max value
        } else {
            // Verify the sender is the original chunk processor
            if (state.owner != msg.sender) {
                revert UnauthorizedChunkProcessor();
            }

            // Verify chunk processing hasn't been completed
            if (state.isComplete) {
                revert ChunkProcessingAlreadyComplete();
            }

            // Verify chunks are processed in order
            if (index != state.lastIndex + 1) {
                revert InvalidChunkOrder();
            }
        }

        state.lastIndex = index;
        state.processedCount++;

        emit TransactionProcessed(msg.sender, txChunk);
        emit TransactionChunkProcessed(txChunk, index, totalChunks, txHashForParent);

        // Check if all chunks have been processed
        if (state.processedCount == state.totalChunks) {
            state.isComplete = true;
        }
    }

    /// @notice Prepends a zero byte to the transaction data
    /// @dev This helps op-translator identify uncompressed data
    /// @param _data The original transaction data
    /// @return bytes The transaction data
    function prependZeroByte(bytes calldata _data) private pure returns (bytes memory) {
        return abi.encodePacked(bytes1(0x00), _data);
    }
}
