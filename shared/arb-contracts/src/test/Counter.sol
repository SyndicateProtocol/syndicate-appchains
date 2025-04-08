// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Counter {
    uint256 private _value;

    constructor() {
        _value = 0;
    }

    function number() external view returns (uint256) {
        return _value;
    }

    function increment() external {
        _value += 1;
    }
}
