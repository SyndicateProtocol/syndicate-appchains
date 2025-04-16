// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {CalldataAlwaysAllowedModule} from "src/sequencing-modules/calldata-modules/CalldataAlwaysAllowedModule.sol";

contract CalldataAlwaysAllowedModuleTest is Test {
    CalldataAlwaysAllowedModule public module;

    function setUp() public {
        module = new CalldataAlwaysAllowedModule();
    }

    function testIsCalldataAllowed() public view {
        // Test with empty calldata
        bytes memory emptyCalldata = new bytes(0);
        assertTrue(module.isCalldataAllowed(emptyCalldata), "Empty calldata should be allowed");

        // Test with some non-empty calldata
        bytes memory nonEmptyCalldata = abi.encode("test data");
        assertTrue(module.isCalldataAllowed(nonEmptyCalldata), "Non-empty calldata should be allowed");
    }
}
