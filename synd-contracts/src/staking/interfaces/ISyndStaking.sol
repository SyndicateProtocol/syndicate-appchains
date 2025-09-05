// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {IEpochTracker} from "./IEpochTracker.sol";

/**
 * @title ISyndStaking
 * @notice Interface for the SyndStaking contract providing stake query functionality
 * @dev Defines the core view functions for accessing stake information across different dimensions
 */
interface ISyndStaking is IEpochTracker {
    /**
     * @notice Get the total stake amount for a specific user in a specific epoch
     * @param epochIndex The epoch index to query
     * @param user The address of the user
     * @return The user's total stake amount for the specified epoch
     */
    function getUserStake(uint256 epochIndex, address user) external view returns (uint256);

    /**
     * @notice Get the pro-rata stake share for a specific user in a specific epoch
     * @param epochIndex The epoch index to query
     * @param user The address of the user
     * @return The user's pro-rata stake share for the specified epoch
     */
    function getUserStakeShare(uint256 epochIndex, address user) external view returns (uint256);

    /**
     * @notice Get the total stake amount across all users in a specific epoch
     * @param epochIndex The epoch index to query
     * @return The total stake amount for the specified epoch
     */
    function getTotalStake(uint256 epochIndex) external view returns (uint256);

    /**
     * @notice Get the total pro-rata stake share across all users in a specific epoch
     * @param epochIndex The epoch index to query
     * @return The total pro-rata stake share for the specified epoch
     */
    function getTotalStakeShare(uint256 epochIndex) external view returns (uint256);

    /**
     * @notice Get the total stake amount for a specific appchain in a specific epoch
     * @param epochIndex The epoch index to query
     * @param appchainId The ID of the appchain
     * @return The total stake amount for the specified appchain and epoch
     */
    function getAppchainStake(uint256 epochIndex, uint256 appchainId) external view returns (uint256);

    /**
     * @notice Get the stake amount for a specific user on a specific appchain in a specific epoch
     * @param epochIndex The epoch index to query
     * @param user The address of the user
     * @param appchainId The ID of the appchain
     * @return The user's stake amount for the specified appchain and epoch
     */
    function getUserAppchainStake(uint256 epochIndex, address user, uint256 appchainId)
        external
        view
        returns (uint256);

    /**
     * @notice Get the current epoch index
     * @return The current epoch index
     */
    function getCurrentEpoch() external view returns (uint256);
}
