// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule} from "../interfaces/PermissionModule.sol";

/**
 * @title AllowlistSequencingModule
 * @dev This contract implements an allowlist mechanism to control access to sequencing.
 */
contract AllowlistSequencingModule is PermissionModule {
    /// @notice The address of the admin who can modify the allowlist.
    address public admin;
    /// @notice Mapping to store allowed addresses.
    mapping(address user => bool isAllowed) public allowlist;

    event UserAdded(address indexed user);
    event UserRemoved(address indexed user);
    event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);

    /**
     * @dev Sets the deployer as the initial admin.
     */
    constructor(address _admin) {
        if (_admin == address(0)) {
            revert AddressNotAllowed();
        }

        admin = _admin;
    }

    error NotAdmin();
    error AddressNotAllowed();

    /**
     * @dev Modifier to check if the caller is the admin.
     */
    modifier onlyAdmin() {
        if (msg.sender != admin) {
            revert NotAdmin();
        }
        _;
    }

    /**
     * @notice Adds an address to the allowlist.
     * @param user The address to be added to the allowlist.
     */
    function addToAllowlist(address user) external onlyAdmin {
        allowlist[user] = true;

        emit UserAdded(user);
    }

    /**
     * @notice Removes an address from the allowlist.
     * @param user The address to be removed from the allowlist.
     */
    function removeFromAllowlist(address user) external onlyAdmin {
        allowlist[user] = false;

        emit UserRemoved(user);
    }

    /**
     * @notice Transfers the admin role to a new address.
     * @param newAdmin The address of the new admin. Cannot be address(0).
     */
    function transferAdmin(address newAdmin) external onlyAdmin {
        if (newAdmin == address(0)) {
            revert AddressNotAllowed();
        }

        admin = newAdmin;

        emit AdminTransferred(msg.sender, newAdmin);
    }

    /**
     * @notice Checks if the caller is allowed.
     * @return bool indicating if the caller is allowed.
     */
    function isAllowed(address proposer) external view override returns (bool) {
        if (!allowlist[proposer]) {
            revert AddressNotAllowed();
        }
        return true;
    }
}
