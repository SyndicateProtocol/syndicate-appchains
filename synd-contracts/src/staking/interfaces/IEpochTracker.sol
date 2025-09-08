pragma solidity 0.8.28;

interface IEpochTracker {
    function getCurrentEpoch() external view returns (uint256);
    function getEpochStart(uint256 epochIndex) external view returns (uint256);
    function getEpochEnd(uint256 epochIndex) external view returns (uint256);
}
