// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule} from "./PermissionModule.sol";

/**
 * @title IRequirementModule
 * @notice Interface for requirement modules
 */
interface IRequirementModule is PermissionModule {
    function addCheck(address _address, bool addToHead) external;
    function removeCheck(address _address) external;
    function getAllChecks() external view returns (address[] memory);
}
