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

        splitter = new Splitter(address(this), address(pool1));
    }

    function setupPools() public {
        splitter.setPerformancePool(address(pool2));
        splitter.setAppchainPool(address(pool3));
    }

    function test_split() public {
        splitter.split{value: 100 ether}(1);

        assertEq(address(pool1).balance, 100 ether);
    }

    function test_split_with_splits() public {
        setupPools();
        splitter.updateSplits(50 ether, 25 ether, 25 ether);
        splitter.split{value: 100 ether}(1);

        assertEq(address(pool1).balance, 50 ether);
        assertEq(address(pool2).balance, 25 ether);
        assertEq(address(pool3).balance, 25 ether);
    }

    function test_invalid_splits() public {
        setupPools();
        vm.expectRevert(Splitter.InvalidSplits.selector);
        splitter.updateSplits(33 ether, 33 ether, 33 ether);
    }

    function test_not_admin() public {
        setupPools();
        vm.prank(makeAddr("user1"));
        vm.expectRevert(Splitter.NotAdmin.selector);
        splitter.updateSplits(0 ether, 0 ether, 100 ether);
    }

    function test_set_base_pool() public {
        splitter.setBasePool(address(pool2));
        assertEq(splitter.basePool(), address(pool2));
    }

    function test_set_performance_pool() public {
        splitter.setPerformancePool(address(pool2));
        assertEq(splitter.performancePool(), address(pool2));
    }

    function test_set_appchain_pool() public {
        splitter.setAppchainPool(address(pool3));
        assertEq(splitter.appchainPool(), address(pool3));
    }

    function test_set_splits_with_no_pools() public {
        vm.expectRevert(Splitter.InvalidSplits.selector);
        splitter.updateSplits(99 ether, 1 ether, 0 ether);
    }

    function test_set_address_with_non_zero_split() public {
        vm.expectRevert(Splitter.InvalidSplits.selector);
        splitter.setBasePool(address(0));
    }
}
