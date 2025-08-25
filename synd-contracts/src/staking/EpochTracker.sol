// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/**
 * @title EpochTracker
 * @notice Abstract contract for tracking epochs based on a configurable start timestamp
 * @dev Provides epoch calculation utilities for staking and reward distribution systems
 *
 * This contract defines epochs as 30-day periods starting from a specified timestamp.
 * Epochs are 1-indexed to ensure proper initialization and finalization tracking.
 */
abstract contract EpochTracker {
    /// @notice The timestamp when epoch counting begins
    uint256 public constant startTimestamp = 0; // 1751497200; // 1st October 2025 1759273200 - (90 days) 3888000

    /// @notice Duration of each epoch in seconds (30 days)
    uint256 public constant EPOCH_DURATION = 30 days;

    /**
     * @notice Get the current epoch index based on the current block timestamp
     * @dev Epochs are 1-indexed to ensure proper initialization of finalization counts
     * @return The current epoch index (1-based)
     */
    function getCurrentEpoch() public view returns (uint256) {
        // Since all the epoch finalization counts are initialized to 0,
        // we start the epochs at 1 to make sure we will finalize the first epoch.
        return ((block.timestamp - startTimestamp) / EPOCH_DURATION) + 1;
    }

    /**
     * @notice Get the start timestamp of a specific epoch
     * @param epochIndex The epoch index to get the start time for
     * @return The timestamp when the specified epoch begins
     */
    function getEpochStart(uint256 epochIndex) public pure returns (uint256) {
        return startTimestamp + (epochIndex - 1) * EPOCH_DURATION;
    }

    /**
     * @notice Get the end timestamp of a specific epoch
     * @param epochIndex The epoch index to get the end time for
     * @return The timestamp when the specified epoch ends
     */
    function getEpochEnd(uint256 epochIndex) public pure returns (uint256) {
        return startTimestamp + epochIndex * EPOCH_DURATION;
    }
}
