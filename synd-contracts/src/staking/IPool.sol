// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

interface IPool {
    function deposit(uint256 epochIndex) external payable;
}
