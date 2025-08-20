// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

contract DummyPool {
    event Deposit(address from, uint256 epochIndex, uint256 amount);

    function deposit(uint256 epochIndex) external payable {
        emit Deposit(msg.sender, epochIndex, msg.value);
    }
}
