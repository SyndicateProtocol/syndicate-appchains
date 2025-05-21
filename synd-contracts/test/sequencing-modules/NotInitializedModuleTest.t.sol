// SPDX-License-Identifier: MIT
pragma solidity 0.8.29;

import {Test} from "forge-std/Test.sol";
import {NotInitializedModule} from "src/sequencing-modules/NotInitializedModule.sol";

contract NotInitializedModuleTest is Test {
    NotInitializedModule public notInitializedModule;

    function setUp() public {
        notInitializedModule = new NotInitializedModule();
    }

    function testIsAllowedReturnsFalse() public view {
        address operator = address(0x1);
        bytes memory emptyData = new bytes(0);

        assertFalse(notInitializedModule.isAllowed(operator, address(0), emptyData));
    }
}
