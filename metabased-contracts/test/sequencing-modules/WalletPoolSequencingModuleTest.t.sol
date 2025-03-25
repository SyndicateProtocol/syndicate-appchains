// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {WalletPoolSequencingModule} from "src/sequencing-modules/WalletPoolSequencingModule.sol";

contract WalletPoolSequencingModuleTest is Test {
    WalletPoolSequencingModule walletPoolSequencer;
    address manager = address(0x1);
    address user1 = address(0x2);
    address user2 = address(0x3);

    // Role constants
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    function setUp() public {
        vm.startPrank(manager);
        walletPoolSequencer = new WalletPoolSequencingModule(manager);
        vm.stopPrank();
    }

    function testManagerCanAddToWalletPool() public {
        vm.startPrank(manager);
        vm.expectEmit(true, false, false, false);
        emit WalletPoolSequencingModule.UserAddedToPool(user1);
        walletPoolSequencer.addToWalletPool(user1);
        assertTrue(walletPoolSequencer.walletPool(user1));
        vm.stopPrank();
    }

    function testManagerCanRemoveFromWalletPool() public {
        vm.startPrank(manager);
        walletPoolSequencer.addToWalletPool(user1);
        vm.expectEmit(true, false, false, false);
        emit WalletPoolSequencingModule.UserRemovedFromPool(user1);
        walletPoolSequencer.removeFromWalletPool(user1);
        assertFalse(walletPoolSequencer.walletPool(user1));
        vm.stopPrank();
    }

    function testNonManagerCannotAddToWalletPool() public {
        bytes memory errorMsg =
            abi.encodeWithSignature("AccessControlUnauthorizedAccount(address,bytes32)", address(this), MANAGER_ROLE);
        vm.expectRevert(errorMsg);
        walletPoolSequencer.addToWalletPool(user1);
    }

    function testNonManagerCannotRemoveFromWalletPool() public {
        vm.startPrank(manager);
        walletPoolSequencer.addToWalletPool(user1);
        vm.stopPrank();

        bytes memory errorMsg =
            abi.encodeWithSignature("AccessControlUnauthorizedAccount(address,bytes32)", address(this), MANAGER_ROLE);
        vm.expectRevert(errorMsg);
        walletPoolSequencer.removeFromWalletPool(user1);
    }

    function testIsAllowedForAllowedAddress() public {
        vm.startPrank(manager);
        walletPoolSequencer.addToWalletPool(user1);
        vm.stopPrank();

        vm.prank(user1);
        assertTrue(walletPoolSequencer.isAllowed(user1));
    }

    function testIsNotAllowedForNonAllowedAddress() public {
        assertFalse(walletPoolSequencer.isAllowed(makeAddr("non-allowed")));
    }

    function testRevertsOnZeroAddressConstructor() public {
        vm.expectRevert(WalletPoolSequencingModule.AddressNotAllowed.selector);
        new WalletPoolSequencingModule(address(0));
    }

    function testRoleAssignment() public view {
        assertTrue(walletPoolSequencer.hasRole(DEFAULT_ADMIN_ROLE, manager));
        assertTrue(walletPoolSequencer.hasRole(MANAGER_ROLE, manager));
    }

    function testRoleManagement() public {
        address newManager = makeAddr("newManager");

        vm.startPrank(manager);
        walletPoolSequencer.grantRole(MANAGER_ROLE, newManager);
        vm.stopPrank();

        assertTrue(walletPoolSequencer.hasRole(MANAGER_ROLE, newManager));

        vm.prank(newManager);
        walletPoolSequencer.addToWalletPool(user2);
        assertTrue(walletPoolSequencer.walletPool(user2));
    }

    function testRevokeRole() public {
        address tempManager = makeAddr("tempManager");

        vm.prank(manager);
        walletPoolSequencer.grantRole(MANAGER_ROLE, tempManager);

        vm.startPrank(tempManager);
        walletPoolSequencer.addToWalletPool(user2);
        assertTrue(walletPoolSequencer.walletPool(user2));
        vm.stopPrank();

        vm.prank(manager);
        walletPoolSequencer.revokeRole(MANAGER_ROLE, tempManager);

        bytes memory errorMsg =
            abi.encodeWithSignature("AccessControlUnauthorizedAccount(address,bytes32)", tempManager, MANAGER_ROLE);
        vm.prank(tempManager);
        vm.expectRevert(errorMsg);
        walletPoolSequencer.addToWalletPool(user1);
    }

    function testConstructorSetsCorrectValues() public view {
        assertTrue(walletPoolSequencer.hasRole(DEFAULT_ADMIN_ROLE, manager));
        assertTrue(walletPoolSequencer.hasRole(MANAGER_ROLE, manager));
    }
}
