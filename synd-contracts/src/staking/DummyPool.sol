// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IPool} from "./IPool.sol";

contract DummyPool is IPool {
    event Deposit(address from, uint256 epochIndex, uint256 amount);

    function deposit(uint256 epochIndex) external payable {
        emit Deposit(msg.sender, epochIndex, msg.value);
    }
}
