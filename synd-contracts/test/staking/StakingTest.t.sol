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

    uint256 public appchainId1;

    function setUp() public {
        staking = new SyndStaking(block.timestamp);
        pool = new BasePool(address(staking));

        user1 = makeAddr("user1");
        user2 = makeAddr("user2");

        vm.deal(user1, 100 ether);
        vm.deal(user2, 100 ether);

        appchainId1 = 111;
    }

    function stepEpoch(uint256 epochsToStep) public {
        vm.warp(block.timestamp + epochsToStep * 30 days);
    }

    function stepDays(uint256 daysToStep) public {
        vm.warp(block.timestamp + daysToStep * 1 days);
    }

    function test_stake() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(appchainId1);
        vm.stopPrank();

        assertEq(staking.getUserStake(1, user1), 0 ether);
        assertEq(staking.getTotalStake(1), 0 ether);

        stepEpoch(1);

        assertEq(staking.getUserStake(2, user1), 50 ether);
        assertEq(staking.getTotalStake(2), 50 ether);
    }

    function test_stake_and_finalize() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(appchainId1);
        vm.stopPrank();

        assertEq(staking.getUserStake(1, user1), 0 ether);
        assertEq(staking.getTotalStake(1), 0 ether);

        stepEpoch(1);

        assertEq(staking.getUserStake(2, user1), 50 ether);
        assertEq(staking.getTotalStake(2), 50 ether);

        vm.startPrank(user1);
        staking.stakeSynd{value: 10 ether}(appchainId1);
        vm.stopPrank();

        stepEpoch(1);

        assertEq(staking.getUserStake(1, user1), 0 ether);
        assertEq(staking.getTotalStake(1), 0 ether);

        assertEq(staking.getUserStake(2, user1), 50 ether);
        assertEq(staking.getTotalStake(2), 50 ether);

        assertEq(staking.getUserStake(3, user1), 60 ether);
        assertEq(staking.getTotalStake(3), 60 ether);
    }

    function test_withdraw() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 100 ether}(appchainId1);
        vm.stopPrank();

        stepEpoch(1);

        assertEq(staking.getUserStake(2, user1), 100 ether);
        assertEq(staking.getTotalStake(2), 100 ether);

        vm.startPrank(user1);
        staking.initializeWithdrawal(appchainId1);
        vm.stopPrank();

        assertEq(staking.getUserStake(2, user1), 100 ether);
        assertEq(staking.getTotalStake(2), 100 ether);

        stepEpoch(1);

        assertEq(staking.getUserStake(3, user1), 0 ether);
        assertEq(staking.getTotalStake(3), 0 ether);

        assertEq(address(user1).balance, 0 ether);

        vm.startPrank(user1);
        staking.withdraw(appchainId1, user1);
        vm.stopPrank();

        assertEq(address(user1).balance, 100 ether);
    }

    function test_withdraw_before_finalization() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 100 ether}(appchainId1);
        vm.stopPrank();

        stepEpoch(1);

        assertEq(staking.getUserStake(2, user1), 100 ether);
        assertEq(staking.getTotalStake(2), 100 ether);

        vm.startPrank(user1);
        staking.initializeWithdrawal(appchainId1);
        vm.stopPrank();

        assertEq(staking.getUserStake(2, user1), 100 ether);
        assertEq(staking.getTotalStake(2), 100 ether);

        stepEpoch(1);

        assertEq(staking.getUserStake(3, user1), 0 ether);
        assertEq(staking.getTotalStake(3), 0 ether);

        assertEq(address(user1).balance, 0 ether);

        vm.startPrank(user1);
        staking.withdraw(appchainId1, user1);
        vm.stopPrank();

        assertEq(address(user1).balance, 100 ether);

        vm.startPrank(user2);
        staking.stakeSynd{value: 100 ether}(appchainId1);
        vm.stopPrank();

        stepEpoch(1);

        assertEq(staking.getUserStake(3, user2), 0 ether);
        assertEq(staking.getTotalStake(3), 0 ether);

        assertEq(staking.getUserStake(4, user2), 100 ether);
        assertEq(staking.getTotalStake(4), 100 ether);
    }

    function test_weightedStake() public {
        stepDays(15);

        // Stake 20 ether for 15 days
        // Weighted stake is 20 * (15/30) = 10
        vm.startPrank(user1);
        staking.stakeSynd{value: 20 ether}(appchainId1);
        vm.stopPrank();

        stepDays(5);

        // Stake 30 ether for 10 days
        // Weighted stake is 30 * (10/30) = 10
        vm.startPrank(user1);
        staking.stakeSynd{value: 30 ether}(appchainId1);
        vm.stopPrank();

        stepEpoch(1);

        assertEq(staking.getUserStakeShare(1, user1), 20 ether);
        assertEq(staking.getTotalStakeShare(1), 20 ether);

        assertEq(staking.getUserStakeShare(2, user1), 50 ether);
        assertEq(staking.getTotalStakeShare(2), 50 ether);
    }

    // Test cases to add
    // Add appchain stake ratio checks to all tests
    // Some tests on precision

    // User adds epoch 1 and then immiediately withdraws
    // User adds epoch 1 and then immiediately  initalizes withdrawal, never completes it, but trys to deposit more epoch 2
    // User adds epoch 1 and also adds epoch 2
    // User adds epoch 1 for chain X withdraws and then adds epoch 2 for chain Y
    // User adds epoch 1 and then checks share epoch 10
    // User adds epoch 1 and then immediately initalizes withdrawal, user 2 adds epoch 2 check all balances are correct epoch 3
}
