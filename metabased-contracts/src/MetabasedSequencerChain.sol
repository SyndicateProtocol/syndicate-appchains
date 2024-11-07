// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireListManager} from "./RequireListManager.sol";

/// @title MetabasedSequencerChain
/// @notice The core contract for sequencing transactions using a modular architecture to determine who is allowed to sequence.
contract MetabasedSequencerChain is RequireListManager {
    /// @notice The ID of the L3 chain that this contract is sequencing transactions for.
    uint256 public immutable l3ChainId;

    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes tx);

    /// @notice Emitted when a chunk of transactions is processed.
    event ChunkProcessed(uint256 indexed chunkId, uint256 transactionsProcessed);

    /// @dev Thrown when the transaction form is invalid.
    error InvalidTransactionForm();

    /// @dev Thrown when an invalid chunk size is provided.
    error InvalidChunkSize();

    /// @notice Constructs the MetabasedSequencerChain contract.
    /// @param _l3ChainId The ID of the L3 chain that this contract is sequencing transactions for.
    /// @param admin The address that will be set as the admin
    constructor(uint256 _l3ChainId, address admin) RequireListManager(admin) {
        // chain id zero has no replay protection : https://eips.ethereum.org/EIPS/eip-3788
        require(_l3ChainId != 0, "L3 chain ID cannot be 0");

        l3ChainId = _l3ChainId;
    }

    modifier onlyWhenAllowed(address sender) {
        // Check if msg.sender is allowed
        requireAllAllowed(msg.sender);
        requireAnyAllowed(msg.sender);
        _;
    }

    /// @notice Processes a single transaction.
    /// @param txn The transaction data.
    function processTransactionRaw(bytes calldata txn) external onlyWhenAllowed(msg.sender) {
        emit TransactionProcessed(msg.sender, txn);
    }

    /// @notice process transactions
    /// @dev It prepends a zero byte to the transaction data
    /// @param txn The transaction data
    function processTransaction(bytes calldata txn) external onlyWhenAllowed(msg.sender) {
        emit TransactionProcessed(msg.sender, prependZeroByte(txn));
    }

    /// @notice Processes multiple transactions in bulk.
    /// @dev It prepends a zero byte to the transaction data
    /// @param txns An array of  transaction data.
    function processBulkTransactions(bytes[] calldata txns) external onlyWhenAllowed(msg.sender) {
        uint256 txnCount = txns.length;

        // Process all transactions
        for (uint256 i = 0; i < txnCount; i++) {
            emit TransactionProcessed(msg.sender, prependZeroByte(txns[i]));
        }
    }

    /// @notice Processes a chunk of transactions from a larger batch.
    /// @param txs An array of  transaction data.
    /// @param startIndex The starting index for this chunk in the overall batch.
    /// @param chunkSize The number of transactions to process in this chunk.
    /// @param chunkId A unique identifier for this chunk.
    function processChunk(bytes[] calldata txs, uint256 startIndex, uint256 chunkSize, uint256 chunkId)
        external
        onlyWhenAllowed(msg.sender)
    {
        if (chunkSize == 0) {
            revert InvalidChunkSize();
        }

        uint256 endIndex = startIndex + chunkSize;
        require(endIndex <= txs.length, "Chunk exceeds batch size");

        for (uint256 i = startIndex; i < endIndex; i++) {
            emit TransactionProcessed(msg.sender, txs[i]);
        }

        emit ChunkProcessed(chunkId, chunkSize);
    }

    /// @notice Prepends a zero byte to the transaction data
    /// @dev This helps op-translator identify un data
    /// @param _tx The original transaction data
    /// @return bytes The transaction data
    function prependZeroByte(bytes calldata _tx) private pure returns (bytes memory) {
        return abi.encodePacked(bytes1(0x00), _tx);
    }
}
