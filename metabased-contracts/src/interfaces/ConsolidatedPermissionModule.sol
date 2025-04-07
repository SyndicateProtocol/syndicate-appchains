// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

/**
 * @title ConsolidatedPermissionModule
 * @notice Interface for a module that checks both proposer and calldata permissions in a single call
 */
interface ConsolidatedPermissionModule {
    /**
     * @notice Checks if the proposer and calldata are allowed
     * @param proposer The address of the proposer to be checked
     * @param data The calldata to be checked
     * @return bool indicating if both the proposer and calldata are allowed
     */
    function isAllowed(address proposer, bytes calldata data) external view returns (bool);
}
