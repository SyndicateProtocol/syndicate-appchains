// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {TokenDistributor} from "src/staking/TokenDistributor.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract TokenDistributorTest is Test {
    TokenDistributor public distributor;
    ERC20Mock public token;

    address admin = makeAddr("admin");
    address sender = makeAddr("sender");
    address recipient = makeAddr("recipient");

    function setUp() public {
        token = new ERC20Mock();
        distributor = new TokenDistributor(address(token), admin);

        // Fund the distributor contract with tokens
        token.mint(address(distributor), 1_000_000 ether);

        // Allow the sender
        vm.prank(admin);
        distributor.updateSender(sender, true);
    }

    // --- Constructor ---

    function test_constructor_setsToken() public view {
        assertEq(address(distributor.token()), address(token));
    }

    function test_constructor_setsOwner() public view {
        assertEq(distributor.owner(), admin);
    }

    // --- updateSender ---

    function test_updateSender_allowsSender() public {
        address newSender = makeAddr("newSender");

        vm.prank(admin);
        distributor.updateSender(newSender, true);

        assertTrue(distributor.isAllowedSender(newSender));
    }

    function test_updateSender_disallowsSender() public {
        vm.prank(admin);
        distributor.updateSender(sender, false);

        assertFalse(distributor.isAllowedSender(sender));
    }

    function test_updateSender_revertsIfNotOwner() public {
        vm.prank(sender);
        vm.expectRevert();
        distributor.updateSender(sender, true);
    }

    // --- transfer ---

    function test_transfer_sendsTokens() public {
        vm.prank(sender);
        bool success = distributor.transfer(recipient, 100 ether);

        assertTrue(success);
        assertEq(token.balanceOf(recipient), 100 ether);
    }

    function test_transfer_revertsIfNotAllowedSender() public {
        address notAllowed = makeAddr("notAllowed");

        vm.prank(notAllowed);
        vm.expectRevert("Must be an allowed sender");
        distributor.transfer(recipient, 100 ether);
    }

    function test_transfer_revertsWhenPaused() public {
        vm.prank(admin);
        distributor.pause();

        vm.prank(sender);
        vm.expectRevert();
        distributor.transfer(recipient, 100 ether);
    }

    function test_transfer_revertsIfInsufficientBalance() public {
        vm.prank(sender);
        vm.expectRevert();
        distributor.transfer(recipient, 2_000_000 ether);
    }

    // --- pause / unpause ---

    function test_pause_pausesContract() public {
        vm.prank(admin);
        distributor.pause();

        assertTrue(distributor.paused());
    }

    function test_pause_revertsIfNotOwner() public {
        vm.prank(sender);
        vm.expectRevert();
        distributor.pause();
    }

    function test_unpause_unpausesContract() public {
        vm.prank(admin);
        distributor.pause();

        vm.prank(admin);
        distributor.unpause();

        assertFalse(distributor.paused());
    }

    function test_unpause_revertsIfNotOwner() public {
        vm.prank(admin);
        distributor.pause();

        vm.prank(sender);
        vm.expectRevert();
        distributor.unpause();
    }

    function test_transfer_worksAfterUnpause() public {
        vm.prank(admin);
        distributor.pause();

        vm.prank(admin);
        distributor.unpause();

        vm.prank(sender);
        bool success = distributor.transfer(recipient, 50 ether);

        assertTrue(success);
        assertEq(token.balanceOf(recipient), 50 ether);
    }
}
