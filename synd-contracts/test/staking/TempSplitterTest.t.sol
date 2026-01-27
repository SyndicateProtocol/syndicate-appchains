// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {TempSplitter} from "src/staking/TempSplitter.sol";
import {IPool} from "src/staking/interfaces/IPool.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract MockPool is IPool {
    function deposit(uint256 epochIndex) external payable {
        // Do nothing
    }

    function getClaimableAmount(uint256 epochIndex, address user) external view returns (uint256) {
        return 100 ether;
    }

    function claimFor(uint256 epochIndex, address user, address destination) external {
        // Do nothing
    }
}

contract TempSplitterTest is Test {
    TempSplitter public splitter;

    MockPool public basePool;
    MockPool public appchainPool;

    function setUp() public {
        basePool = new MockPool();
        appchainPool = new MockPool();

        splitter = new TempSplitter(address(basePool), address(appchainPool));
    }

    function test_split() public {
        splitter.deposit{value: 100 ether}(1);

        // Base pool gets 60% (base 30% + performance 30%)
        assertEq(address(basePool).balance, 60 ether);
        // Appchain pool gets 40%
        assertEq(address(appchainPool).balance, 40 ether);
    }

    function test_split_with_no_value() public {
        vm.expectRevert(abi.encodeWithSelector(TempSplitter.NoValueSent.selector));
        splitter.deposit(1);
    }

    function test_split_no_dust() public {
        splitter.deposit{value: 111111111111111111111}(1);

        // Appchain: 40% = 44444444444444444444
        // Base: remainder = 66666666666666666667
        assertEq(address(basePool).balance, 66666666666666666667);
        assertEq(address(appchainPool).balance, 44444444444444444444);
    }

    function test_constructor_reverts_on_zero_base_pool() public {
        vm.expectRevert(abi.encodeWithSelector(TempSplitter.InvalidAddress.selector));
        new TempSplitter(address(0), address(appchainPool));
    }

    function test_constructor_reverts_on_zero_appchain_pool() public {
        vm.expectRevert(abi.encodeWithSelector(TempSplitter.InvalidAddress.selector));
        new TempSplitter(address(basePool), address(0));
    }

    function test_split_emits_event() public {
        vm.expectEmit(true, true, true, true);
        emit TempSplitter.Split(1, 60 ether, 40 ether);

        splitter.deposit{value: 100 ether}(1);
    }
}
