// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {PermissionModule_ReleaseCandidate} from "./PermissionModule_ReleaseCandidate.sol";

/**
 * @title IRequirementModule
 * @notice Interface for requirement modules
 */
interface IRequirementModule_ReleaseCandidate is PermissionModule_ReleaseCandidate {
    function addCheck(address _address, bool addToHead) external;
    function removeCheck(address _address) external;
    function getAllChecks() external view returns (address[] memory);
}
