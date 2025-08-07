// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndStaking} from "src/staking/SyndStaking.sol";
import {BasePool} from "src/staking/BasePool.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract StakingTest is Test {
    SyndStaking public staking;
    BasePool public pool;

    address public user1;
    address public user2;

    function setUp() public {
        staking = new SyndStaking(block.timestamp);
        pool = new BasePool(address(staking));

        user1 = makeAddr("user1");
        user2 = makeAddr("user2");

        vm.deal(user1, 100 ether);
        vm.deal(user2, 100 ether);
    }

    function stepEpoch(uint256 epochsToStep) public {
        vm.warp(block.timestamp + epochsToStep * 30 days);
    }

    function stepDays(uint256 daysToStep) public {
        vm.warp(block.timestamp + daysToStep * 1 days);
    }

    function test_stake() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(111);
        vm.stopPrank();

        (uint256 amount1, uint256 total1) = staking.getStakeDetails(1, user1);
        assertEq(amount1, 0 ether);
        assertEq(total1, 0 ether);

        stepEpoch(1);

        (uint256 amount2, uint256 total2) = staking.getStakeDetails(2, user1);
        assertEq(amount2, 50 ether);
        assertEq(total2, 50 ether);
    }

    function test_stake_and_finalize() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(111);
        vm.stopPrank();

        (uint256 amount1, uint256 total1) = staking.getStakeDetails(1, user1);
        assertEq(amount1, 0 ether);
        assertEq(total1, 0 ether);

        stepEpoch(1);

        (uint256 amount2, uint256 total2) = staking.getStakeDetails(2, user1);
        assertEq(amount2, 50 ether);
        assertEq(total2, 50 ether);

        vm.startPrank(user1);
        staking.stakeSynd{value: 10 ether}(111);
        vm.stopPrank();

        stepEpoch(1);

        (uint256 amount3, uint256 total3) = staking.getStakeDetails(1, user1);
        assertEq(amount3, 0 ether);
        assertEq(total3, 0 ether);

        (uint256 amount4, uint256 total4) = staking.getStakeDetails(2, user1);
        assertEq(amount4, 50 ether);
        assertEq(total4, 50 ether);

        (uint256 amount5, uint256 total5) = staking.getStakeDetails(3, user1);
        assertEq(amount5, 60 ether);
        assertEq(total5, 60 ether);
    }

    function test_withdraw() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 100 ether}(111);
        vm.stopPrank();

        stepEpoch(1);

        (uint256 amount1, uint256 total1) = staking.getStakeDetails(2, user1);
        assertEq(amount1, 100 ether);
        assertEq(total1, 100 ether);

        vm.startPrank(user1);
        staking.initializeWithdrawal();
        vm.stopPrank();

        (uint256 amount2, uint256 total2) = staking.getStakeDetails(2, user1);
        assertEq(amount2, 100 ether);
        assertEq(total2, 100 ether);

        stepEpoch(1);

        (uint256 amount3, uint256 total3) = staking.getStakeDetails(3, user1);
        assertEq(amount3, 0 ether);
        assertEq(total3, 0 ether);

        assertEq(address(user1).balance, 0 ether);

        vm.startPrank(user1);
        staking.withdraw();
        vm.stopPrank();

        assertEq(address(user1).balance, 100 ether);
    }

    function test_weightedStake() public {
        stepDays(15);

        // Stake 20 ether for 15 days
        // Weighted stake is 20 * (15/30) = 10
        vm.startPrank(user1);
        staking.stakeSynd{value: 20 ether}(111);
        vm.stopPrank();

        stepDays(5);

        // Stake 30 ether for 10 days
        // Weighted stake is 30 * (10/30) = 10
        vm.startPrank(user1);
        staking.stakeSynd{value: 30 ether}(111);
        vm.stopPrank();

        stepEpoch(1);

        (uint256 amountWeighted1, uint256 totalWeighted1) = staking.getWeightedStakeDetails(1, user1);
        assertEq(amountWeighted1, 20 ether);
        assertEq(totalWeighted1, 20 ether);

        (uint256 amountWeighted2, uint256 totalWeighted2) = staking.getWeightedStakeDetails(2, user1);
        assertEq(amountWeighted2, 50 ether);
        assertEq(totalWeighted2, 50 ether);
    }
}
