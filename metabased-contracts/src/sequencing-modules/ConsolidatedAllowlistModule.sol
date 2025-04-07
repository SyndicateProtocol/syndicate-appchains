// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {ConsolidatedPermissionModule} from "../interfaces/ConsolidatedPermissionModule.sol";

/**
 * @title ConsolidatedAllowlistModule
 * @dev This contract implements an allowlist mechanism to control access to sequencing
 *      using the consolidated permission module interface.
 */
contract ConsolidatedAllowlistModule is ConsolidatedPermissionModule {
    /// @notice The address of the admin who can modify the allowlist
    address public admin;

    /// @notice Mapping to store allowed addresses
    mapping(address user => bool isAllowed) public allowlist;

    /// @notice Flag to enable/disable calldata checks
    bool public calldataChecksEnabled;

    event UserAdded(address indexed user);
    event UserRemoved(address indexed user);
    event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);
    event CalldataChecksToggled(bool enabled);

    error NotAdmin();
    error AddressNotAllowed();
    error CalldataNotAllowed();

    /**
     * @dev Sets the deployer as the initial admin and disables calldata checks by default
     */
    constructor(address _admin) {
        if (_admin == address(0)) {
            revert AddressNotAllowed();
        }

        admin = _admin;
        calldataChecksEnabled = false;
    }

    /**
     * @dev Modifier to check if the caller is the admin
     */
    modifier onlyAdmin() {
        if (msg.sender != admin) {
            revert NotAdmin();
        }
        _;
    }

    /**
     * @notice Adds an address to the allowlist
     * @param user The address to be added to the allowlist
     */
    function addToAllowlist(address user) external onlyAdmin {
        allowlist[user] = true;
        emit UserAdded(user);
    }

    /**
     * @notice Removes an address from the allowlist
     * @param user The address to be removed from the allowlist
     */
    function removeFromAllowlist(address user) external onlyAdmin {
        allowlist[user] = false;
        emit UserRemoved(user);
    }

    /**
     * @notice Transfers the admin role to a new address
     * @param newAdmin The address of the new admin. Cannot be address(0)
     */
    function transferAdmin(address newAdmin) external onlyAdmin {
        if (newAdmin == address(0)) {
            revert AddressNotAllowed();
        }

        address previousAdmin = admin;
        admin = newAdmin;

        emit AdminTransferred(previousAdmin, newAdmin);
    }

    /**
     * @notice Toggles whether calldata checks are enabled
     * @param enabled Whether calldata checks should be enabled
     */
    function toggleCalldataChecks(bool enabled) external onlyAdmin {
        calldataChecksEnabled = enabled;
        emit CalldataChecksToggled(enabled);
    }

    /**
     * @notice Consolidated function to check both proposer and calldata permissions
     * @param proposer The address of the proposer to be checked
     * @param data The calldata to be checked
     * @return bool indicating if both checks pass
     */
    function isAllowed(address proposer, bytes calldata data) external view override returns (bool) {
        // Check proposer permission if proposer is not zero address
        if (proposer != address(0) && !allowlist[proposer]) {
            revert AddressNotAllowed();
        }

        // Check calldata if enabled and data is not empty
        if (calldataChecksEnabled && data.length > 0) {
            // This is a placeholder for calldata validation
            // In a real implementation, you might check for specific patterns or restrictions

            // Example: Simple validation to check first byte is not 0xFF (just a demo restriction)
            if (data.length > 0 && data[0] == 0xFF) {
                revert CalldataNotAllowed();
            }
        }

        return true;
    }
}
