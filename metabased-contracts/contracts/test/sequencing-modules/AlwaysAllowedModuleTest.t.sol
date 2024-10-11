// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";

contract AlwaysAllowedModuleTest is Test {
    AlwaysAllowedModule public alwaysAllowedModule;

    function setUp() public {
        alwaysAllowedModule = new AlwaysAllowedModule();
    }

    function testIsAllowedReturnsAlwaysTrue() public view {
        address operator = address(0x1);
        assertTrue(alwaysAllowedModule.isAllowed(operator));
    }
}
