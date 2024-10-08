// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {AccessControl} from "openzeppelin-contracts//contracts/access/AccessControl.sol";

/// @title L3BackfillData
/// @notice This contract is used to emit events containing L3 chain block and transaction data
/// @dev This contract uses AccessControl for managing permissions
contract L3BackfillData is AccessControl {
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    /// @notice Struct representing the header of an L3 block
    struct BlockHeader {
        bytes32 parentHash;
        bytes32 uncleHash;
        address coinbase;
        bytes32 root;
        bytes32 txHash;
        bytes32 receiptHash;
        bytes32[8] bloom;
        uint256 difficulty;
        uint256 number;
        uint256 gasLimit;
        uint256 gasUsed;
        uint256 time;
        bytes32 extra;
        bytes32 mixDigest;
        uint64 nonce;
        uint256 baseFee;
        bytes32 withdrawalsHash;
        uint64 blobGasUsed;
        uint256 excessBlobGas;
        bytes32 parentBeaconRoot;
    }

    /// @notice Event emitted when a new block is submitted
    /// @param blockNumber The number of the block
    /// @param blockHeader The header of the block
    /// @param blockHash The hash of the block
    event Block(uint256 indexed blockNumber, BlockHeader blockHeader, bytes32 blockHash);

    /// @notice Event emitted when transactions for a block are submitted
    /// @param blockNumber The number of the block these transactions belong to
    /// @param txData The compressed transaction data
    /// @param segmentIndex The index of this segment in the total transaction data
    /// @param segmentTotal The total number of segments for this block's transactions
    event Transactions(uint256 indexed blockNumber, bytes txData, uint256 segmentIndex, uint256 segmentTotal);

    /// @notice Constructor that sets up the default admin and manager roles
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role
    constructor(address admin, address manager) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, manager);
    }

    /// @notice Submits a block to be emitted as an event
    /// @param blockHeader The header of the block
    /// @param blockHash The hash of the block
    function submitBlock(BlockHeader calldata blockHeader, bytes32 blockHash) external onlyRole(MANAGER_ROLE) {
        emit Block(blockHeader.number, blockHeader, blockHash);
    }

    /// @notice Submits transactions for a block to be emitted as an event
    /// @param blockNumber The number of the block these transactions belong to
    /// @param txData The compressed transaction data
    /// @param segmentIndex The index of this segment in the total transaction data
    /// @param segmentTotal The total number of segments for this block's transactions
    function submitTransactions(uint256 blockNumber, bytes calldata txData, uint256 segmentIndex, uint256 segmentTotal)
        external
        onlyRole(MANAGER_ROLE)
    {
        emit Transactions(blockNumber, txData, segmentIndex, segmentTotal);
    }
}
