// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {AccessControl} from "openzeppelin-contracts/contracts/access/AccessControl.sol";

/// @title L3BackfillStorage
/// @notice This contract is used to emit events containing L3 chain block and transaction data
/// @dev This contract uses AccessControl for managing permissions
contract L3BackfillStorage is AccessControl {
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    event Batch(uint256 indexed l1EpochBlockNumber, uint256 indexed l3BlockNumber, bytes batch);

    /// @notice Constructor that sets up the default admin and manager roles
    /// @param admin The address that will be the default admin role
    /// @param manager The address that will be the manager role
    constructor(address admin, address manager) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, manager);
    }

    /// @notice Emits a Batch
    /// @param batch: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format
    function save(uint256 l1EpochBlockNumber, uint256 l3BlockNumber, bytes calldata batch)
        external
        onlyRole(MANAGER_ROLE)
    {
        emit Batch(l1EpochBlockNumber, l3BlockNumber, batch);
    }

    /// @notice Emits many Batches
    /// @param batches: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format
    function saveForMany(
        uint256[] calldata l1EpochBlockNumbers,
        uint256[] calldata l3BlockNumbers,
        bytes[] calldata batches
    ) external onlyRole(MANAGER_ROLE) {
        require(
            l1EpochBlockNumbers.length == l3BlockNumbers.length && l3BlockNumbers.length == batches.length,
            "Array lengths must be equal"
        );
        uint256 length = l1EpochBlockNumbers.length;
        for (uint256 i = 0; i < length; i++) {
            emit Batch(l1EpochBlockNumbers[i], l3BlockNumbers[i], batches[i]);
        }
    }
}
