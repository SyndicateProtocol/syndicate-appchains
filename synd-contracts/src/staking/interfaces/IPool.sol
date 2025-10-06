// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

/// @title IPool
/// @notice Interface for all pools
/// @dev This interface is used to deposit rewards for specific epochs
interface IPool {
    /**
     * @notice Deposit rewards for a specific epoch
     * @param epochIndex The epoch index to deposit to
     */
    function deposit(uint256 epochIndex) external payable;
}

/// @title IUserPool
/// @notice Interface for user pools (BasePool & PerformancePool)
/// @dev This interface is used to claim rewards for specific epochs
interface IUserPool is IPool {
    /**
     * @notice Get the claimable amount for a specific user and appchain in a given epoch
     * @param epochIndex The epoch index to query
     * @param user The user to query
     * @param appchainId The appchain ID to query
     * @return The claimable amount
     */
    function getClaimableAmount(uint256 epochIndex, address user, uint256 appchainId) external returns (uint256);

    /**
     * @notice Claim rewards for a specific user and appchain in a given epoch
     * @param epochIndex The epoch index to query
     * @param user The user to query
     * @param destination The destination address to send the rewards to
     * @param appchainId The appchain ID to query
     */
    function claimFor(uint256 epochIndex, address user, address destination, uint256 appchainId) external;
}
