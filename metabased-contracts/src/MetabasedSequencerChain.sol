// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";

/// @title MetabasedSequencerChain
/// @notice The core contract for sequencing transactions using a modular architecture
/// to determine who is allowed to sequence.
contract MetabasedSequencerChain is SequencingModuleChecker {
    /// @notice The ID of the L3 chain that this contract is sequencing transactions for.
    uint256 public immutable l3ChainId;

    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes data, uint256 priority);

    /// @notice Emitted when a chunk of transactions is processed.
    event TransactionChunkProcessed(
        bytes txChunk, uint256 index, uint256 totalChunks, bytes32 txHashForParent, uint256 priority
    );

    /// @dev Thrown when an invalid chunk size is provided.
    error InvalidChunkSize();

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

    /// @notice Processes a chunk of transactions from a larger batch.
    /// @param txChunk the compressed chunked transaction data.
    /// @param index The starting index for this chunk in the overall batch.
    /// @param totalChunks The number of transactions to process in this chunk.
    /// @param txHashForParent The hash of the parent transaction.
    /// @param priority The priority of the transaction.
    function processChunk(
        bytes calldata txChunk,
        uint256 index,
        uint256 totalChunks,
        bytes32 txHashForParent,
        uint256 priority
    ) external onlyWhenAllowed(msg.sender) {
        if (totalChunks == 0) {
            revert InvalidChunkSize();
        }

        emit TransactionProcessed(msg.sender, txChunk, priority);

        emit TransactionChunkProcessed(txChunk, index, totalChunks, txHashForParent, priority);
    }

    /// @notice Prepends a zero byte to the transaction data
    /// @dev This helps op-translator identify uncompressed data
    /// @param _data The original transaction data
    /// @return bytes The transaction data
    function prependZeroByte(bytes calldata _data) private pure returns (bytes memory) {
        return abi.encodePacked(bytes1(0x00), _data);
    }
}
