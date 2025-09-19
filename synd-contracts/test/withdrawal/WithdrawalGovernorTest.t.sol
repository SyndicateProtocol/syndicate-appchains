// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {WithdrawalGovernor, IRollupAdmin} from "src/withdrawal/WithdrawalGovernor.sol";


contract MockRollup is IRollupAdmin {
    address public owner;

    constructor() {
        owner = msg.sender;
    }

    function pause() external override {
        assert(msg.sender == owner);
    }

    function unpause() external override {
        assert(msg.sender == owner);
    }

    function setOwner(address newOwner) external override {
        assert(msg.sender == owner);
        owner = newOwner;
    }
}

contract WithdrawalGovernorTest is Test {
    MockRollup public rollup;
    WithdrawalGovernor public withdrawalGovernor;

    address public owner = address(0x1);
    address public pauser1 = address(0x2);
    address public pauser2 = address(0x3);

    function setUp() public {
        rollup = new MockRollup();
        withdrawalGovernor = new WithdrawalGovernor(owner, address(rollup));
        rollup.setOwner(address(withdrawalGovernor));
    }

    function testAddAndRemovePauser() public {
        assertEq(withdrawalGovernor.isPauser(pauser1), false);

        vm.prank(owner);
        withdrawalGovernor.addPauser(pauser1);
        assertEq(withdrawalGovernor.isPauser(pauser1), true);

        vm.prank(owner);
        withdrawalGovernor.removePauser(pauser1);
        assertEq(withdrawalGovernor.isPauser(pauser1), false);
    }
    
    function testPauseWithdrawal() public {
        vm.prank(owner);
        withdrawalGovernor.addPauser(pauser1);

        vm.prank(pauser1);
        withdrawalGovernor.pauseWithdrawal();
    }

    function testOwnerPauseWithdrawal() public {
        vm.prank(owner);
        withdrawalGovernor.pauseWithdrawal();
    }

    function testOwnerUnpauseWithdrawal() public {
        vm.prank(owner);
        withdrawalGovernor.pauseWithdrawal();

        vm.prank(owner);
        withdrawalGovernor.unpauseWithdrawal();
    }

    function testPauseAndOwnerUnpauseWithdrawal() public {
        vm.prank(owner);
        withdrawalGovernor.addPauser(pauser1);

        vm.prank(pauser1);
        withdrawalGovernor.pauseWithdrawal();

        vm.prank(owner);
        withdrawalGovernor.unpauseWithdrawal();
    }

    function testPauseNotAllowed() public {
        vm.prank(pauser1);
        vm.expectRevert();
        withdrawalGovernor.pauseWithdrawal();
    }

    function testUnpauseNotAllowed() public {
        vm.prank(owner);
        withdrawalGovernor.addPauser(pauser1);

        vm.prank(pauser1);
        withdrawalGovernor.pauseWithdrawal();


        vm.prank(pauser1);
        vm.expectRevert();
        withdrawalGovernor.unpauseWithdrawal();
    }

    function testAddPauserNotAllowed() public {
        vm.prank(pauser1);
        vm.expectRevert();
        withdrawalGovernor.addPauser(pauser1);
    }

    function testRemovePauserNotAllowed() public {
        vm.prank(pauser1);
        vm.expectRevert();
        withdrawalGovernor.removePauser(pauser1);
    }

    function testTransferRollupOwner() public {
        vm.prank(owner);
        withdrawalGovernor.transferRollupOwner(pauser1);
        assertEq(rollup.owner(), pauser1);
    }

    function testTransferRollupOwnerNotAllowed() public {
        vm.prank(pauser1);
        vm.expectRevert();
        withdrawalGovernor.transferRollupOwner(pauser1);
    }
}
