// SPDX-License-Identifier: MIT
pragma solidity 0.8.29;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/// @title SyndicateStorage
/// @notice This contract is used to emit events containing App chain block and transaction data
/// @dev This contract uses AccessControl for managing permissions
contract SyndicateStorage is AccessControl {
    uint256 public immutable appChainId;
    uint256 public indexFromBlock;

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    /// @notice Emits a EpochRangeProcessed indicating the range of epochs that have been processed
    /// @param startEpochNumber The starting epoch number
    /// @param endEpochNumber The ending epoch number
    event EpochRangeProcessed(uint256 indexed startEpochNumber, uint256 indexed endEpochNumber); //#olympix-ignore-missing-events-assertion

    /// @notice Constructor that sets up the default admin and manager roles
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role
    /// @param appChainId_ The App chain ID
    constructor(address admin, address manager, uint256 appChainId_) {
        require(admin != address(0), "Admin address cannot be 0");
        require(manager != address(0), "Manager address cannot be 0");
        // chain id zero has no replay protection : https://eips.ethereum.org/EIPS/eip-3788
        require(appChainId_ != 0, "App chain ID cannot be 0");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, manager);
        appChainId = appChainId_;
    }

    /// @notice Sets the index from block
    /// @param blockNumber The block number to start indexing from
    function setIndexFromBlock(uint256 blockNumber) external onlyRole(MANAGER_ROLE) {
        indexFromBlock = blockNumber;
    }

    /// @notice Emits a Batch
    /// @param epochNumber The epoch number
    /// @dev The third parameter is a batch: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format
    function save(uint256 epochNumber, bytes32, bytes calldata) external onlyRole(MANAGER_ROLE) {
        emit EpochRangeProcessed(epochNumber, epochNumber);
    }

    /// @notice Emits many Batches
    /// @param epochNumbers The epoch numbers
    /// @param epochHashes The epoch hashes
    /// @param batches The batches
    /// @dev https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format
    function saveForMany(uint256[] calldata epochNumbers, bytes32[] calldata epochHashes, bytes[] calldata batches)
        external
        onlyRole(MANAGER_ROLE)
    {
        require(
            epochNumbers.length == epochHashes.length && epochHashes.length == batches.length,
            "Array lengths must be equal"
        );
        uint256 length = epochNumbers.length;
        emit EpochRangeProcessed(epochNumbers[0], epochNumbers[length - 1]);
    }
}
