// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

interface IPool {
    function deposit(uint256 epochIndex) external payable;
    function getClaimableAmount(uint256 epochIndex, address user) external view returns (uint256);
    function claimFor(uint256 epochIndex, address user, address destination) external;
}
