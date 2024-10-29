// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {RequireListManager} from "./RequireListManager.sol";

/// @title MetabasedSequencerChain
/// @notice The core contract for sequencing transactions using a modular architecture to determine who is allowed to sequence.
contract MetabasedSequencerChain is RequireListManager {
    /// @notice The ID of the L3 chain that this contract is sequencing transactions for.
    uint256 public immutable l3ChainId;

    /// @notice Emitted when a new transaction is processed.
    event TransactionProcessed(address indexed sender, bytes encodedTxn);

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

    /// @notice Processes a single encoded transaction.
    /// @param encodedTxn The encoded transaction data.
    /// @param isTxCompressed A flag to indicate if the transaction is compressed.
    function processTransaction(bytes calldata encodedTxn, bool isTxCompressed) public {
        // Check if msg.sender is allowed
        requireAllAllowed(msg.sender);
        requireAnyAllowed(msg.sender);

        // Emit event with transaction details
        if (isTxCompressed) {
            emit TransactionProcessed(msg.sender, encodedTxn);
        } else {
            emit TransactionProcessed(msg.sender, prependZeroByte(encodedTxn));
        }
    }

    /// @notice Processes multiple encoded transactions in bulk.
    /// @param encodedTxns An array of encoded transaction data.
    /// @param isTxsCompressed An array of flags to indicate if the transactions are compressed.
    function processBulkTransactions(bytes[] calldata encodedTxns, bool[] calldata isTxsCompressed) public {
        uint256 txnCount = encodedTxns.length;

        // Process all transactions
        for (uint256 i = 0; i < txnCount; i++) {
            processTransaction(encodedTxns[i], isTxsCompressed[i]);
        }
    }

    /// @notice Processes a chunk of transactions from a larger batch.
    /// @param encodedTxns An array of encoded transaction data.
    /// @param startIndex The starting index for this chunk in the overall batch.
    /// @param chunkSize The number of transactions to process in this chunk.
    /// @param chunkId A unique identifier for this chunk.
    /// @param isTxsCompressed A flag to indicate if the transactions are encoded.
    function processChunk(
        bytes[] calldata encodedTxns,
        uint256 startIndex,
        uint256 chunkSize,
        uint256 chunkId,
        bool isTxsCompressed
    ) public {
        if (chunkSize == 0) {
            revert InvalidChunkSize();
        }

        uint256 endIndex = startIndex + chunkSize;
        require(endIndex <= encodedTxns.length, "Chunk exceeds batch size");

        for (uint256 i = startIndex; i < endIndex; i++) {
            processTransaction(encodedTxns[i], isTxsCompressed);
        }

        emit ChunkProcessed(chunkId, chunkSize);
    }

    /// @notice Prepends a zero byte to the encoded transaction data
    /// @dev This helps op-translator identify uncompressed data
    /// @param encodedTxn The original encoded transaction data
    /// @return bytes The encoded transaction data with a prepended zero byte
    function prependZeroByte(bytes calldata encodedTxn) private pure returns (bytes memory) {
        bytes memory result = new bytes(encodedTxn.length + 1);
        result[0] = 0x00; // Add zero byte at the beginning
        for (uint256 i = 0; i < encodedTxn.length; i++) {
            result[i + 1] = encodedTxn[i];
        }
        return result;
    }
}
