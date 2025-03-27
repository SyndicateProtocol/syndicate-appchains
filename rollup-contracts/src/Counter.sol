// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    function increment() external {
        number++;
    }

    function decrement() external {
        number--;
    }

    function reset() external {
        number = 0;
    }
}
