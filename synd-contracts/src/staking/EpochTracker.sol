// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

abstract contract EpochTracker {
    uint256 public startTimestamp;

    uint256 public constant EPOCH_DURATION = 30 days;

    constructor(uint256 _startTimestamp) {
        startTimestamp = _startTimestamp;
    }

    function getCurrentEpoch() public view returns (uint256) {
        // Since all the epoch finalization counts are initialized to 0,
        // we start the epochs at 1 to make sure we will finalize the first epoch.
        return ((block.timestamp - startTimestamp) / EPOCH_DURATION) + 1;
    }

    function getEpochStart(uint256 epochIndex) public view returns (uint256) {
        return startTimestamp + (epochIndex - 1) * EPOCH_DURATION;
    }

    function getEpochEnd(uint256 epochIndex) public view returns (uint256) {
        return startTimestamp + epochIndex * EPOCH_DURATION;
    }
}
