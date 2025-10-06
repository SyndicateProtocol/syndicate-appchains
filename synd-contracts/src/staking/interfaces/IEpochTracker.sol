pragma solidity 0.8.28;

/// @title IEpochTracker
/// @notice Interface for epoch tracking
/// @dev This interface is used to track epochs
interface IEpochTracker {
    /**
     * @notice Get the current epoch index based on the current block timestamp
     * @dev Epochs are 1-indexed to ensure proper initialization of finalization counts
     * @return The current epoch index (1-based)
     */
    function getCurrentEpoch() external view returns (uint256);

    /**
     * @notice Get the start timestamp of a specific epoch
     * @param epochIndex The epoch index to get the start time for
     * @return The timestamp when the specified epoch begins
     */
    function getEpochStart(uint256 epochIndex) external view returns (uint256);

    /**
     * @notice Get the end timestamp of a specific epoch
     * @param epochIndex The epoch index to get the end time for
     * @return The timestamp when the specified epoch ends
     */
    function getEpochEnd(uint256 epochIndex) external view returns (uint256);
}
