// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

// NOTE: this contract is used in E2E tests, please do not remove it
contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
    }
}