// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

abstract contract EpochTracker {
    uint256 public startTimestamp;

    uint256 public constant EPOCH_DURATION = 30 days;

    constructor(uint256 _startTimestamp) {
        startTimestamp = _startTimestamp;
    }

    function getCurrentEpoch() public view returns (uint256) {
        // We start at epoch 1 so we need to add 1
        return ((block.timestamp - startTimestamp) / EPOCH_DURATION) + 1;
    }

    function getEpochStart(uint256 epochIndex) public view returns (uint256) {
        return startTimestamp + (epochIndex - 1) * EPOCH_DURATION;
    }

    function getEpochEnd(uint256 epochIndex) public view returns (uint256) {
        return startTimestamp + epochIndex * EPOCH_DURATION;
    }
}
