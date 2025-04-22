// SPDX-License-Identifier: UNLICENSED
pragma solidity >= 0.8.0;

import {CalldataPermissionModule} from "src/interfaces/CalldataPermissionModule.sol";

contract CalldataPermissionModuleBasic is CalldataPermissionModule {
    // Use a simple bool flag instead of complex mapping for Certora
    bool public allCalldataAllowed = true;

    function isCalldataAllowed(bytes calldata) external view override returns (bool) {
        // Always return true for Certora verification purposes
        return allCalldataAllowed;
    }

    function setAllowedAll(bool status) external {
        allCalldataAllowed = status;
    }
}
