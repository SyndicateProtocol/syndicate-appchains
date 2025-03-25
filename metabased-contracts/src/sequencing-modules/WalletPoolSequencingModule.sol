// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {ProposerPermissionModule} from "../interfaces/ProposerPermissionModule.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title WalletPoolSequencingModule
 * @dev This contract implements a wallet pool mechanism to control access to sequencing.
 */
contract WalletPoolSequencingModule is ProposerPermissionModule, AccessControl {
    /// @notice Role identifier for the manager role
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    /// @notice Mapping to store addresses in the wallet pool
    mapping(address user => bool isInPool) public walletPool;

    event UserAddedToPool(address indexed user);
    event UserRemovedFromPool(address indexed user);

    error AddressNotAllowed();

    /**
     * @dev Sets the deployer as the initial admin with the ADMIN_ROLE.
     */
    constructor(address _admin) {
        if (_admin == address(0)) {
            revert AddressNotAllowed();
        }

        _grantRole(DEFAULT_ADMIN_ROLE, _admin);
        _grantRole(MANAGER_ROLE, _admin);
    }

    /**
     * @notice Adds an address to the wallet pool.
     * @param user The address to be added to the wallet pool.
     */
    function addToWalletPool(address user) external onlyRole(MANAGER_ROLE) {
        walletPool[user] = true;

        emit UserAddedToPool(user);
    }

    /**
     * @notice Removes an address from the wallet pool.
     * @param user The address to be removed from the wallet pool.
     */
    function removeFromWalletPool(address user) external onlyRole(MANAGER_ROLE) {
        walletPool[user] = false;

        emit UserRemovedFromPool(user);
    }

    /**
     * @notice Checks if the address is allowed in the wallet pool.
     * @return bool indicating if the proposer is in the wallet pool.
     */
    function isAllowed(address proposer) external view override returns (bool) {
        return walletPool[proposer];
    }
}
