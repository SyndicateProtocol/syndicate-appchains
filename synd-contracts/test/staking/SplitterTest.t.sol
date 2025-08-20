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

        splitter = new Splitter(address(pool1), address(pool2), address(pool3));
    }

    function test_split() public {
        splitter.deposit{value: 100 ether}(1);

        assertEq(address(pool1).balance, 30 ether);
        assertEq(address(pool2).balance, 30 ether);
        assertEq(address(pool3).balance, 40 ether);
    }

    function test_split_with_no_value() public {
        vm.expectRevert(abi.encodeWithSelector(Splitter.NoValueSent.selector));
        splitter.deposit(1);
    }

    function test_split_no_dust() public {
        splitter.deposit{value: 111111111111111111111}(1);

        assertEq(address(pool1).balance, 33333333333333333334);
        assertEq(address(pool2).balance, 33333333333333333333);
        assertEq(address(pool3).balance, 44444444444444444444);
    }
}
