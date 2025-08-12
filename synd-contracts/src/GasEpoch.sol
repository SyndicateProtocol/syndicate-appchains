// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/**
 * @title GasEpoch
 * @notice Implements the rule of epoch tracking
 * @author Syndicate Protocol
 */
abstract contract GasEpoch {
    /// @notice Duration of each epoch
    uint256 public constant epochDuration = 30 days;
    /// @notice timestamp of when the first epoch starts
    uint256 public constant epochTimestampStart = 0;

    /// @notice Calculates the current epoch number
    function getCurrentEpoch() public view returns (uint256) {
        return (block.timestamp - epochTimestampStart) / epochDuration;
    }

    function getEpochStartTime(uint256 epoch) public pure returns (uint256) {
        return epochTimestampStart + epoch * epochDuration;
    }

    function getEpochEndTime(uint256 epoch) public pure returns (uint256) {
        return getEpochStartTime(epoch) + epochDuration;
    }
}
