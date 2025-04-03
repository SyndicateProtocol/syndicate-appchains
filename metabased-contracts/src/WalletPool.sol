// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title WalletPool
 * @dev This contract implements a wallet pool mechanism to manage allowlists across multiple
 * sequencing modules. It uses OpenZeppelin's AccessControl for flexible permission management.
 */
contract WalletPool is AccessControl {
    /// @notice Role identifier for the manager role
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    /// @notice Mapping to store addresses in the wallet pool
    mapping(address wallet => bool isInPool) public walletPool;

    event WalletAddedToPool(address indexed wallet);
    event WalletRemovedFromPool(address indexed wallet);
    event AllowlistModuleUpdated(address indexed module, address indexed wallet, bool isAllowed);

    error AddressNotAllowed();
    error AllowlistUpdateFailed();

    /**
     * @dev Sets the deployer as the initial admin with both the DEFAULT_ADMIN_ROLE and MANAGER_ROLE.
     * The DEFAULT_ADMIN_ROLE is necessary for role management (granting/revoking roles)
     * while the MANAGER_ROLE is used for day-to-day operations on the wallet pool.
     */
    constructor(address _admin) {
        if (_admin == address(0)) {
            revert AddressNotAllowed();
        }

        _grantRole(DEFAULT_ADMIN_ROLE, _admin);
        _grantRole(MANAGER_ROLE, _admin);
    }

    /**
     * @notice Adds an address to the wallet pool and updates all allowlist modules.
     * @param wallet The address to be added to the wallet pool.
     * @param allowlistModules Array of allowlist modules to update.
     */
    function addToWalletPool(address wallet, address[] calldata allowlistModules) external onlyRole(MANAGER_ROLE) {
        walletPool[wallet] = true;
        emit WalletAddedToPool(wallet);

        // Update all allowlist modules
        _updateAllowlistModules(wallet, true, allowlistModules);
    }

    /**
     * @notice Removes an address from the wallet pool and updates all allowlist modules.
     * @param wallet The address to be removed from the wallet pool.
     * @param allowlistModules Array of allowlist modules to update.
     */
    function removeFromWalletPool(address wallet, address[] calldata allowlistModules)
        external
        onlyRole(MANAGER_ROLE)
    {
        walletPool[wallet] = false;
        emit WalletRemovedFromPool(wallet);

        // Update all allowlist modules
        _updateAllowlistModules(wallet, false, allowlistModules);
    }

    /**
     * @notice Gets whether an address is in the wallet pool.
     * @param wallet The address to check.
     * @return bool indicating if the wallet is in the wallet pool.
     */
    function isInWalletPool(address wallet) external view returns (bool) {
        return walletPool[wallet];
    }

    /**
     * @notice Updates a single allowlist module.
     * @param allowlistModule The allowlist module address to update.
     * @param wallet The wallet address to update.
     * @param isAllowed Whether the wallet should be allowed or not.
     */
    function updateAllowlistModule(address allowlistModule, address wallet, bool isAllowed)
        external
        onlyRole(MANAGER_ROLE)
    {
        _updateSingleAllowlist(allowlistModule, wallet, isAllowed);
    }

    /**
     * @notice Batch updates a single wallet across multiple allowlist modules.
     * @param wallet The wallet address to update.
     * @param isAllowed Whether the wallet should be allowed or not.
     * @param allowlistModules Array of allowlist modules to update.
     */
    function updateAllowlistModules(address wallet, bool isAllowed, address[] calldata allowlistModules)
        external
        onlyRole(MANAGER_ROLE)
    {
        _updateAllowlistModules(wallet, isAllowed, allowlistModules);
    }

    /**
     * @dev Internal function to update all provided allowlist modules.
     * @param wallet The wallet address to update.
     * @param isAllowed Whether the wallet should be allowed or not.
     * @param allowlistModules Array of allowlist modules to update.
     */
    function _updateAllowlistModules(address wallet, bool isAllowed, address[] calldata allowlistModules) internal {
        for (uint256 i = 0; i < allowlistModules.length; i++) {
            _updateSingleAllowlist(allowlistModules[i], wallet, isAllowed);
        }
    }

    /**
     * @dev Internal function to update a single allowlist module.
     * @param allowlistModule The allowlist module address to update.
     * @param wallet The wallet address to update.
     * @param isAllowed Whether the wallet should be allowed or not.
     */
    function _updateSingleAllowlist(address allowlistModule, address wallet, bool isAllowed) internal {
        // Check if the address has code (is a contract)
        uint256 codeSize;
        assembly {
            codeSize := extcodesize(allowlistModule)
        }

        // Revert if the address is not a contract
        if (codeSize == 0) {
            revert AllowlistUpdateFailed();
        }

        // Call appropriate function based on whether we're adding or removing
        (bool success,) = allowlistModule.call(
            abi.encodeWithSignature(isAllowed ? "addToAllowlist(address)" : "removeFromAllowlist(address)", wallet)
        );

        // If the call failed, revert
        if (!success) {
            revert AllowlistUpdateFailed();
        }

        emit AllowlistModuleUpdated(allowlistModule, wallet, isAllowed);
    }
}
