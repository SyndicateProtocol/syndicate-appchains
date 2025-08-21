// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndStaking} from "src/staking/SyndStaking.sol";
import {AppchainPool} from "src/staking/AppchainPool.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract AppchainPoolTest is Test {
    SyndStaking public staking;
    AppchainPool public appchainPool;

    address public user1;
    address public user2;
    address public user3;

    uint256 public appchainId1 = 111;
    uint256 public appchainId2 = 222;
    uint256 public appchainId3 = 333;

    address public destinationAddress;

    function setUp() public {
        staking = new SyndStaking(block.timestamp);
        appchainPool = new AppchainPool(address(staking));

        user1 = makeAddr("user1");
        user2 = makeAddr("user2");
        user3 = makeAddr("user3");

        vm.deal(user1, 100 ether);
        vm.deal(user2, 100 ether);
        vm.deal(user3, 100 ether);
    }

    function setupStake(uint256 user1Stake, uint256 user2Stake, uint256 user3Stake) public {
        if (user1Stake > 0) {
            vm.startPrank(user1);
            staking.stakeSynd{value: user1Stake}(appchainId1);
            vm.stopPrank();
        }

        if (user2Stake > 0) {
            vm.startPrank(user2);
            staking.stakeSynd{value: user2Stake}(appchainId2);
            vm.stopPrank();
        }

        if (user3Stake > 0) {
            vm.startPrank(user3);
            staking.stakeSynd{value: user3Stake}(appchainId3);
            vm.stopPrank();
        }

        vm.warp(block.timestamp + 60 days);
    }

    function test_claim() public {
        setupStake(100 ether, 0 ether, 0 ether);

        appchainPool.deposit{value: 100 ether}(2);

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 100 ether);

        vm.startPrank(user1);
        appchainPool.claim(2, destinationAddress, appchainId1);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 0 ether);
    }

    function test_claim_2_user_stake() public {
        setupStake(100 ether, 100 ether, 0 ether);

        appchainPool.deposit{value: 100 ether}(2);

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 50 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 50 ether);

        vm.startPrank(user1);
        appchainPool.claim(2, destinationAddress, appchainId1);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 0 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 50 ether);

        vm.startPrank(user2);
        appchainPool.claim(2, destinationAddress, appchainId2);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 0 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 0 ether);
    }

    function test_claim_3_user_stake() public {
        setupStake(100 ether, 100 ether, 100 ether);

        appchainPool.deposit{value: 90 ether}(2);

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 30 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 30 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId3), 30 ether);

        vm.startPrank(user1);
        appchainPool.claim(2, destinationAddress, appchainId1);
        vm.stopPrank();

        vm.startPrank(user2);
        appchainPool.claim(2, destinationAddress, appchainId2);
        vm.stopPrank();

        vm.startPrank(user3);
        appchainPool.claim(2, destinationAddress, appchainId3);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 0 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 0 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId3), 0 ether);
    }

    function test_claim_multi_deposit() public {
        setupStake(100 ether, 100 ether, 100 ether);
        appchainPool.deposit{value: 90 ether}(2);

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 30 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 30 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId3), 30 ether);

        appchainPool.deposit{value: 9 ether}(2);

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 33 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 33 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId3), 33 ether);
    }

    function test_claim_multi_deposit_claim_between() public {
        setupStake(100 ether, 100 ether, 100 ether);
        appchainPool.deposit{value: 90 ether}(2);

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 30 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 30 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId3), 30 ether);

        vm.startPrank(user1);
        appchainPool.claim(2, destinationAddress, appchainId1);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 0 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 30 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId3), 30 ether);

        appchainPool.deposit{value: 9 ether}(2);

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 3 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 33 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId3), 33 ether);

        vm.startPrank(user2);
        appchainPool.claim(2, destinationAddress, appchainId2);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 3 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId2), 0 ether);
        assertEq(appchainPool.getClaimableAmount(2, appchainId3), 33 ether);
    }

    function test_claim_current_epoch() public {
        setupStake(100 ether, 0 ether, 0 ether);
        appchainPool.deposit{value: 100 ether}(3);

        uint256 currentEpoch = staking.getCurrentEpoch();

        vm.startPrank(user1);
        vm.expectRevert(AppchainPool.ClaimNotAvailable.selector);
        appchainPool.claim(currentEpoch, user1, appchainId1);
        vm.stopPrank();
    }

    function test_claim_future_epoch() public {
        setupStake(100 ether, 0 ether, 0 ether);
        appchainPool.deposit{value: 100 ether}(3);

        uint256 currentEpoch = staking.getCurrentEpoch();

        vm.startPrank(user1);
        vm.expectRevert(AppchainPool.ClaimNotAvailable.selector);
        appchainPool.claim(currentEpoch + 1, user1, appchainId1);
        vm.stopPrank();
    }

    function test_claim_not_claimable() public {
        setupStake(100 ether, 0 ether, 0 ether);
        appchainPool.deposit{value: 100 ether}(2);

        assertEq(appchainPool.getClaimableAmount(2, appchainId1), 0 ether);

        vm.startPrank(user2);
        vm.expectRevert(AppchainPool.ClaimNotAvailable.selector);
        appchainPool.claim(2, destinationAddress, appchainId1);
        vm.stopPrank();
    }
}
