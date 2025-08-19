// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Splitter} from "src/staking/Splitter.sol";
import {IPool} from "src/staking/IPool.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract MockPool is IPool {
    function deposit(uint256 epochIndex) external payable {
        // Do nothing
    }
}

contract SplitterTest is Test {
    Splitter public splitter;

    MockPool public pool1;
    MockPool public pool2;
    MockPool public pool3;

    function setUp() public {
        pool1 = new MockPool();
        pool2 = new MockPool();
        pool3 = new MockPool();

        splitter = new Splitter(address(this), address(pool1), address(pool2), address(pool3));
    }

    function test_split() public {
        splitter.split{value: 100 ether}(1);

        assertEq(address(pool1).balance, 100 ether);
        assertEq(address(pool2).balance, 0 ether);
        assertEq(address(pool3).balance, 0 ether);
    }

    function test_split_with_splits() public {
        splitter.updateSplits(50 ether, 25 ether, 25 ether);
        splitter.split{value: 100 ether}(1);

        assertEq(address(pool1).balance, 50 ether);
        assertEq(address(pool2).balance, 25 ether);
        assertEq(address(pool3).balance, 25 ether);
    }

    function test_invalid_splits() public {
        vm.expectRevert(Splitter.InvalidSplits.selector);
        splitter.updateSplits(33 ether, 33 ether, 33 ether);
    }

    function test_not_admin() public {
        vm.prank(makeAddr("user1"));
        vm.expectRevert(Splitter.NotAdmin.selector);
        splitter.updateSplits(0 ether, 0 ether, 100 ether);
    }
}
