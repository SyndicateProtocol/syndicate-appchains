// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {AllowlistSequencingModule} from "src/sequencing-modules/AllowlistSequencingModule.sol";

contract AllowlistSequencingModuleTest is Test {
    AllowlistSequencingModule allowlistSequencer;
    address admin = address(0x1);
    address user1 = address(0x2);
    address user2 = address(0x3);

    function setUp() public {
        vm.startPrank(admin);
        allowlistSequencer = new AllowlistSequencingModule(admin);
        vm.stopPrank();
    }

    function testAdminCanAddToAllowlist() public {
        vm.startPrank(admin);
        allowlistSequencer.addToAllowlist(user1);
        assertTrue(allowlistSequencer.allowlist(user1));
        vm.stopPrank();
    }

    function testAdminCanRemoveFromAllowlist() public {
        vm.startPrank(admin);
        allowlistSequencer.addToAllowlist(user1);
        allowlistSequencer.removeFromAllowlist(user1);
        assertFalse(allowlistSequencer.allowlist(user1));
        vm.stopPrank();
    }

    function testNonAdminCannotAddToAllowlist() public {
        vm.expectRevert(AllowlistSequencingModule.NotAdmin.selector);
        allowlistSequencer.addToAllowlist(user1);
    }

    function testNonAdminCannotRemoveFromAllowlist() public {
        vm.startPrank(admin);
        allowlistSequencer.addToAllowlist(user1);
        vm.stopPrank();

        vm.expectRevert(AllowlistSequencingModule.NotAdmin.selector);
        allowlistSequencer.removeFromAllowlist(user1);
    }

    function testIsAllowedForAllowedAddress() public {
        vm.startPrank(admin);
        allowlistSequencer.addToAllowlist(user1);
        vm.stopPrank();

        vm.prank(user1);
        assertTrue(allowlistSequencer.isAllowed(user1));
    }

    function testIsNotAllowedForNonAllowedAddress() public {
        assertFalse(allowlistSequencer.isAllowed(makeAddr("non-allowed")));
    }
}
