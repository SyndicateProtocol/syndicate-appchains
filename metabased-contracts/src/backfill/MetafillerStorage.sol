// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {AccessControl} from "openzeppelin-contracts/contracts/access/AccessControl.sol";

/// @title MetafillerStorage
/// @notice This contract is used to emit events containing L3 chain block and transaction data
/// @dev This contract uses AccessControl for managing permissions
contract MetafillerStorage is AccessControl {
    uint256 public immutable l3ChainId;

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    event Batch(uint256 indexed epochNumber, bytes32 indexed epochHash, bytes batch);

    /// @notice Constructor that sets up the default admin and manager roles
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role
    /// @param l3ChainId_ The L3 chain ID
    constructor(address admin, address manager, uint256 l3ChainId_) {
        require(admin != address(0), "Admin address cannot be 0");
        require(manager != address(0), "Manager address cannot be 0");
        // chain id zero has no replay protection : https://eips.ethereum.org/EIPS/eip-3788
        require(l3ChainId_ != 0, "L3 chain ID cannot be 0");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, manager);
        l3ChainId = l3ChainId_;
    }

    /// @notice Emits a Batch
    /// @param batch: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format
    function save(uint256 epochNumber, bytes32 epochHash, bytes calldata batch) external onlyRole(MANAGER_ROLE) {
        emit Batch(epochNumber, epochHash, batch);
    }

    /// @notice Emits many Batches
    /// @param batches: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format
    function saveForMany(uint256[] calldata epochNumbers, bytes32[] calldata epochHashes, bytes[] calldata batches)
        external
        onlyRole(MANAGER_ROLE)
    {
        require(
            epochNumbers.length == epochHashes.length && epochHashes.length == batches.length,
            "Array lengths must be equal"
        );
        uint256 length = epochNumbers.length;
        for (uint256 i = 0; i < length; i++) {
            emit Batch(epochNumbers[i], epochHashes[i], batches[i]);
        }
    }
}
