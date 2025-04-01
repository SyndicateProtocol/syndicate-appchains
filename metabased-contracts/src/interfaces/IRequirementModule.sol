// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {ProposerPermissionModule} from "./ProposerPermissionModule.sol";
import {CalldataPermissionModule} from "./CalldataPermissionModule.sol";

/**
 * @title IRequirementModule
 * @notice Interface for requirement modules with both proposer and calldata validation
 * @dev Combines both proposer and calldata permission check capabilities
 */
interface IRequirementModule is ProposerPermissionModule, CalldataPermissionModule {
    /**
     * @notice Adds a proposer permission check address to the list
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addProposerCheck(address _address, bool addToHead) external;

    /**
     * @notice Removes a proposer permission check address from the list
     * @param _address The address of the check to remove
     */
    function removeProposerCheck(address _address) external;

    /**
     * @notice Gets all proposer permission check addresses
     * @return An array of all proposer check addresses
     */
    function getAllProposerChecks() external view returns (address[] memory);

    /**
     * @notice Adds a calldata permission check address to the list
     * @param _address The address of the check to add
     * @param addToHead True to add to the head of the list, false to add to the tail
     */
    function addCalldataCheck(address _address, bool addToHead) external;

    /**
     * @notice Removes a calldata permission check address from the list
     * @param _address The address of the check to remove
     */
    function removeCalldataCheck(address _address) external;

    /**
     * @notice Gets all calldata permission check addresses
     * @return An array of all calldata check addresses
     */
    function getAllCalldataChecks() external view returns (address[] memory);
}
