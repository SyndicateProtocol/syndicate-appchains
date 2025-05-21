// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPermissionModule} from "./IPermissionModule.sol";

/**
 * @title IRequirementModule
 * @notice Interface for requirement modules that manage collections of permission checks
 * @dev Extends IPermissionModule with methods to add/remove individual permission modules
 */
interface IRequirementModule is IPermissionModule {
    /**
     * @notice Adds a permission check address to the list
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addPermissionCheck(address _address, bool addToHead) external;

    /**
     * @notice Removes a permission check address from the list
     * @param _address The address of the check to remove
     */
    function removePermissionCheck(address _address) external;

    /**
     * @notice Gets all permission check addresses
     * @return An array of all  check addresses
     */
    function getAllPermissionChecks() external view returns (address[] memory);
}
