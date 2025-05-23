// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

contract InitializableHarness {
    uint8 private _initialized;

    function _getInitializedVersion() public view returns (uint8) {
        return _initialized;
    }
}
