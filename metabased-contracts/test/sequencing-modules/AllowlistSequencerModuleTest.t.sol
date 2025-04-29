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
        vm.expectEmit(true, false, false, false);
        emit AllowlistSequencingModule.UserAdded(user1);
        allowlistSequencer.addToAllowlist(user1);
        assertTrue(allowlistSequencer.allowlist(user1));
        vm.stopPrank();
    }

    function testAdminCanRemoveFromAllowlist() public {
        vm.startPrank(admin);
        allowlistSequencer.addToAllowlist(user1);
        vm.expectEmit(true, false, false, false);
        emit AllowlistSequencingModule.UserRemoved(user1);
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

        bytes memory emptyData = new bytes(0);

        vm.prank(user1);
        assertTrue(allowlistSequencer.isAllowed(user1, address(0), emptyData));
    }

    function testIsNotAllowedForNonAllowedAddress() public {
        bytes memory emptyData = new bytes(0);

        assertFalse(allowlistSequencer.isAllowed(makeAddr("non-allowed"), address(0), emptyData));
    }

    function testRevertsOnZeroAddressConstructor() public {
        vm.expectRevert(AllowlistSequencingModule.AddressNotAllowed.selector);
        new AllowlistSequencingModule(address(0));
    }

    function testRevertsOnZeroAddressTransferAdmin() public {
        vm.startPrank(admin);
        vm.expectRevert(AllowlistSequencingModule.AddressNotAllowed.selector);
        allowlistSequencer.transferAdmin(address(0));
        vm.stopPrank();
    }

    function testTransferAdminEmitsEvent() public {
        address newAdmin = makeAddr("newAdmin");
        vm.startPrank(admin);
        vm.expectEmit(true, true, false, false);
        emit AllowlistSequencingModule.AdminTransferred(admin, newAdmin);
        allowlistSequencer.transferAdmin(newAdmin);
        vm.stopPrank();
        assertEq(allowlistSequencer.admin(), newAdmin);
    }

    function testConstructorSetsCorrectValues() public view {
        assertEq(allowlistSequencer.admin(), admin, "Admin not set correctly");
    }
}
