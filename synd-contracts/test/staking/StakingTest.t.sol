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
    uint256 public appchainId2;

    function setUp() public {
        staking = new SyndStaking(block.timestamp);
        pool = new BasePool(address(staking));

        user1 = makeAddr("user1");
        user2 = makeAddr("user2");

        vm.deal(user1, 100 ether);
        vm.deal(user2, 100 ether);

        appchainId1 = 111;
        appchainId2 = 222;
    }

    function stepEpoch(uint256 epochsToStep) public {
        vm.warp(block.timestamp + epochsToStep * 30 days);
    }

    function stepDays(uint256 daysToStep) public {
        vm.warp(block.timestamp + daysToStep * 1 days);
    }

    function checkStake(uint256 epochIndex, address user, uint256 amount, uint256 appchainId) public view {
        assertEq(staking.getUserStake(epochIndex, user), amount);
        assertEq(staking.getTotalStake(epochIndex), amount);
        assertEq(staking.getAppchainStake(epochIndex, appchainId), amount);
        assertEq(staking.getUserAppchainStake(epochIndex, user, appchainId), amount);
    }

    function test_stake() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(appchainId1);
        vm.stopPrank();

        checkStake(1, user1, 0 ether, appchainId1);

        stepEpoch(1);

        checkStake(2, user1, 50 ether, appchainId1);
    }

    function test_stake_and_finalize() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(appchainId1);
        vm.stopPrank();

        checkStake(1, user1, 0 ether, appchainId1);

        stepEpoch(1);

        checkStake(2, user1, 50 ether, appchainId1);

        vm.startPrank(user1);
        staking.stakeSynd{value: 10 ether}(appchainId1);
        vm.stopPrank();

        stepEpoch(1);

        checkStake(1, user1, 0 ether, appchainId1);
        checkStake(2, user1, 50 ether, appchainId1);
        checkStake(3, user1, 60 ether, appchainId1);
    }

    function test_withdraw() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 100 ether}(appchainId1);
        vm.stopPrank();

        stepEpoch(1);

        checkStake(2, user1, 100 ether, appchainId1);

        vm.startPrank(user1);
        staking.initializeWithdrawal(appchainId1);
        vm.stopPrank();

        checkStake(2, user1, 100 ether, appchainId1);

        stepEpoch(1);

        checkStake(3, user1, 0 ether, appchainId1);

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

        checkStake(2, user1, 100 ether, appchainId1);

        vm.startPrank(user1);
        staking.initializeWithdrawal(appchainId1);
        vm.stopPrank();

        checkStake(2, user1, 100 ether, appchainId1);

        stepEpoch(1);

        checkStake(3, user1, 0 ether, appchainId1);

        assertEq(address(user1).balance, 0 ether);

        vm.startPrank(user1);
        staking.withdraw(appchainId1, user1);
        vm.stopPrank();

        assertEq(address(user1).balance, 100 ether);

        vm.startPrank(user2);
        staking.stakeSynd{value: 100 ether}(appchainId1);
        vm.stopPrank();

        stepEpoch(1);

        checkStake(3, user2, 0 ether, appchainId1);
        checkStake(4, user2, 100 ether, appchainId1);
    }

    function test_stakeShare() public {
        stepDays(15);

        // Stake 20 ether for 15 days
        // Weighted stake is 20 * (15/30) = 10
        vm.startPrank(user1);
        staking.stakeSynd{value: 20 ether}(appchainId1);
        vm.stopPrank();

        stepDays(5);

        assertEq(staking.getUserStakeShare(1, user1), 10 ether);
        assertEq(staking.getTotalStakeShare(1), 10 ether);

        // Stake 30 ether for 10 days
        // Weighted stake is 30 * (10/30) = 10
        vm.startPrank(user1);
        staking.stakeSynd{value: 30 ether}(appchainId1);
        vm.stopPrank();

        assertEq(staking.getUserStakeShare(1, user1), 20 ether);
        assertEq(staking.getTotalStakeShare(1), 20 ether);

        stepEpoch(1);

        checkStake(1, user1, 0 ether, appchainId1);
        assertEq(staking.getUserStakeShare(1, user1), 20 ether);
        assertEq(staking.getTotalStakeShare(1), 20 ether);

        checkStake(2, user1, 50 ether, appchainId1);
    }

    function test_stake_and_withdraw_same_epoch() public {
        stepDays(15);

        vm.startPrank(user1);
        staking.stakeSynd{value: 100 ether}(appchainId1);
        vm.stopPrank();

        vm.startPrank(user1);
        staking.initializeWithdrawal(appchainId1);
        vm.stopPrank();

        stepEpoch(1);

        checkStake(1, user1, 0 ether, appchainId1);
        checkStake(2, user1, 0 ether, appchainId1);
        assertEq(staking.getUserStakeShare(1, user1), 50 ether);
        assertEq(staking.getTotalStakeShare(1), 50 ether);
        assertEq(staking.getUserStakeShare(2, user1), 0 ether);
        assertEq(staking.getTotalStakeShare(2), 0 ether);

        vm.startPrank(user1);
        staking.withdraw(appchainId1, user1);
        vm.stopPrank();

        checkStake(1, user1, 0 ether, appchainId1);
        checkStake(2, user1, 0 ether, appchainId1);
        assertEq(staking.getUserStakeShare(1, user1), 50 ether);
        assertEq(staking.getTotalStakeShare(1), 50 ether);
        assertEq(staking.getUserStakeShare(2, user1), 0 ether);
        assertEq(staking.getTotalStakeShare(2), 0 ether);

        assertEq(address(user1).balance, 100 ether);
    }

    function test_initialize_withdrawal_and_stake() public {
        stepDays(15);

        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(appchainId1);
        vm.stopPrank();

        vm.startPrank(user1);
        staking.initializeWithdrawal(appchainId1);
        vm.stopPrank();

        stepEpoch(2);

        checkStake(1, user1, 0 ether, appchainId1);
        checkStake(2, user1, 0 ether, appchainId1);
        checkStake(3, user1, 0 ether, appchainId1);
        assertEq(staking.getUserStakeShare(1, user1), 25 ether);
        assertEq(staking.getTotalStakeShare(1), 25 ether);
        assertEq(staking.getUserStakeShare(2, user1), 0 ether);
        assertEq(staking.getTotalStakeShare(2), 0 ether);

        vm.startPrank(user1);
        vm.expectRevert(abi.encodeWithSelector(SyndStaking.WithdrawalAlreadyInitialized.selector));
        staking.stakeSynd{value: 50 ether}(appchainId1);
        vm.stopPrank();
    }

    function test_stake_multiple_epochs() public {
        stepDays(15);

        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(appchainId1);
        vm.stopPrank();

        checkStake(1, user1, 0 ether, appchainId1);
        assertEq(staking.getUserStakeShare(1, user1), 25 ether);
        assertEq(staking.getTotalStakeShare(1), 25 ether);

        stepEpoch(1);

        checkStake(2, user1, 50 ether, appchainId1);

        vm.startPrank(user1);
        staking.stakeSynd{value: 30 ether}(appchainId1);
        vm.stopPrank();

        checkStake(2, user1, 50 ether, appchainId1);
        assertEq(staking.getUserStakeShare(2, user1), 65 ether);
        assertEq(staking.getTotalStakeShare(2), 65 ether);

        stepEpoch(1);

        checkStake(3, user1, 80 ether, appchainId1);

        vm.startPrank(user1);
        staking.stakeSynd{value: 20 ether}(appchainId1);
        vm.stopPrank();

        checkStake(3, user1, 80 ether, appchainId1);
        assertEq(staking.getTotalStakeShare(3), 90 ether);

        stepEpoch(1);

        checkStake(4, user1, 100 ether, appchainId1);
    }

    function test_multiple_users_same_appchain() public {
        stepDays(10);

        vm.startPrank(user1);
        staking.stakeSynd{value: 30 ether}(appchainId1);
        vm.stopPrank();

        stepDays(5);

        vm.startPrank(user2);
        staking.stakeSynd{value: 40 ether}(appchainId1);
        vm.stopPrank();

        checkStake(1, user1, 0 ether, appchainId1);
        checkStake(1, user2, 0 ether, appchainId1);

        stepEpoch(1);

        assertEq(staking.getUserStakeShare(1, user1), 20 ether);
        assertEq(staking.getUserStakeShare(1, user2), 20 ether);
        assertEq(staking.getTotalStakeShare(1), 40 ether);

        assertEq(staking.getUserAppchainStake(2, user1, appchainId1), 30 ether);
        assertEq(staking.getUserAppchainStake(2, user2, appchainId1), 40 ether);
        assertEq(staking.getAppchainStake(2, appchainId1), 70 ether);
        assertEq(staking.getTotalStake(2), 70 ether);
    }

    function test_multiple_users_multiple_appchains() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 30 ether}(appchainId1);
        vm.stopPrank();

        vm.startPrank(user2);
        staking.stakeSynd{value: 40 ether}(appchainId2);
        vm.stopPrank();

        checkStake(1, user1, 0 ether, appchainId1);
        checkStake(1, user2, 0 ether, appchainId2);

        stepEpoch(1);

        assertEq(staking.getUserAppchainStake(2, user1, appchainId1), 30 ether);
        assertEq(staking.getUserAppchainStake(2, user2, appchainId2), 40 ether);
        assertEq(staking.getAppchainStake(2, appchainId1), 30 ether);
        assertEq(staking.getAppchainStake(2, appchainId2), 40 ether);
        assertEq(staking.getTotalStake(2), 70 ether);
    }

    function test_single_user_multiple_appchains_then_withdraw() public {
        vm.startPrank(user1);
        staking.stakeSynd{value: 30 ether}(appchainId1);
        vm.stopPrank();

        vm.startPrank(user1);
        staking.stakeSynd{value: 40 ether}(appchainId2);
        vm.stopPrank();

        checkStake(1, user1, 0 ether, appchainId1);
        checkStake(1, user1, 0 ether, appchainId2);

        stepEpoch(1);

        assertEq(staking.getUserStake(2, user1), 70 ether);
        assertEq(staking.getTotalStake(2), 70 ether);

        assertEq(staking.getAppchainStake(2, appchainId1), 30 ether);
        assertEq(staking.getUserAppchainStake(2, user1, appchainId1), 30 ether);

        assertEq(staking.getAppchainStake(2, appchainId2), 40 ether);
        assertEq(staking.getUserAppchainStake(2, user1, appchainId2), 40 ether);

        vm.startPrank(user1);
        staking.initializeWithdrawal(appchainId1);
        vm.stopPrank();

        assertEq(staking.getUserStake(2, user1), 70 ether);
        assertEq(staking.getTotalStake(2), 70 ether);

        assertEq(staking.getAppchainStake(2, appchainId1), 30 ether);
        assertEq(staking.getUserAppchainStake(2, user1, appchainId1), 30 ether);

        assertEq(staking.getAppchainStake(2, appchainId2), 40 ether);
        assertEq(staking.getUserAppchainStake(2, user1, appchainId2), 40 ether);

        stepEpoch(1);

        checkStake(3, user1, 40 ether, appchainId2);

        assertEq(staking.getAppchainStake(3, appchainId1), 0 ether);
        assertEq(staking.getUserAppchainStake(3, user1, appchainId1), 0 ether);
    }

    function test_stake_last_second_of_epoch() public {
        vm.warp(block.timestamp + 30 days - 1);

        vm.startPrank(user1);
        staking.stakeSynd{value: 30 days}(appchainId1);
        vm.stopPrank();

        assertEq(staking.getUserStakeShare(1, user1), 1);
    }
}
