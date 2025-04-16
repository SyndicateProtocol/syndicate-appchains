// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {NotInitializedModule} from "src/sequencing-modules/NotInitializedModule.sol";

contract NotInitializedModuleTest is Test {
    NotInitializedModule public notInitializedModule;

    function setUp() public {
        notInitializedModule = new NotInitializedModule();
    }

    function testIsAllowedReturnsFalse() public view {
        address operator = address(0x1);
        assertFalse(notInitializedModule.isAllowed(operator));
    }
}
