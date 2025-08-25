// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndStaking} from "src/staking/SyndStaking.sol";
import {BasePool} from "src/staking/BasePool.sol";
import {EpochTracker} from "src/staking/EpochTracker.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract BasePoolTest is Test {
    SyndStaking public staking;
    BasePool public basePool;

    address public user1;
    address public user2;
    address public user3;

    function setUp() public {
        staking = new SyndStaking();
        basePool = new BasePool(address(staking));

        user1 = makeAddr("user1");
        user2 = makeAddr("user2");
        user3 = makeAddr("user3");

        vm.deal(user1, 100 ether);
        vm.deal(user2, 100 ether);
        vm.deal(user3, 100 ether);

        // Warp to exactly the epoch start timestamp (beginning of epoch 1)
        vm.warp(staking.START_TIMESTAMP());
    }

    function setupStake(uint256 user1Stake, uint256 user2Stake, uint256 user3Stake) public {
        if (user1Stake > 0) {
            vm.startPrank(user1);
            staking.stakeSynd{value: user1Stake}(111);
            vm.stopPrank();
        }

        if (user2Stake > 0) {
            vm.startPrank(user2);
            staking.stakeSynd{value: user2Stake}(111);
            vm.stopPrank();
        }

        if (user3Stake > 0) {
            vm.startPrank(user3);
            staking.stakeSynd{value: user3Stake}(111);
            vm.stopPrank();
        }

        vm.warp(block.timestamp + 60 days);
    }

    function test_claim() public {
        setupStake(100 ether, 0 ether, 0 ether);

        basePool.deposit{value: 100 ether}(2);

        assertEq(basePool.getClaimableAmount(2, user1), 100 ether);

        vm.startPrank(user1);
        basePool.claim(2, user1);
        vm.stopPrank();

        assertEq(basePool.getClaimableAmount(2, user1), 0 ether);
    }

    function test_claim_2_user_stake() public {
        setupStake(100 ether, 100 ether, 0 ether);

        basePool.deposit{value: 100 ether}(2);

        assertEq(basePool.getClaimableAmount(2, user1), 50 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 50 ether);

        vm.startPrank(user1);
        basePool.claim(2, user1);
        vm.stopPrank();

        assertEq(basePool.getClaimableAmount(2, user1), 0 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 50 ether);

        vm.startPrank(user2);
        basePool.claim(2, user2);
        vm.stopPrank();

        assertEq(basePool.getClaimableAmount(2, user1), 0 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 0 ether);
    }

    function test_claim_3_user_stake() public {
        setupStake(100 ether, 100 ether, 100 ether);

        basePool.deposit{value: 90 ether}(2);

        assertEq(basePool.getClaimableAmount(2, user1), 30 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 30 ether);
        assertEq(basePool.getClaimableAmount(2, user3), 30 ether);

        vm.startPrank(user1);
        basePool.claim(2, user1);
        vm.stopPrank();

        vm.startPrank(user2);
        basePool.claim(2, user2);
        vm.stopPrank();

        vm.startPrank(user3);
        basePool.claim(2, user3);
        vm.stopPrank();

        assertEq(basePool.getClaimableAmount(2, user1), 0 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 0 ether);
        assertEq(basePool.getClaimableAmount(2, user3), 0 ether);
    }

    function test_claim_multi_deposit() public {
        setupStake(100 ether, 100 ether, 100 ether);
        basePool.deposit{value: 90 ether}(2);

        assertEq(basePool.getClaimableAmount(2, user1), 30 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 30 ether);
        assertEq(basePool.getClaimableAmount(2, user3), 30 ether);

        basePool.deposit{value: 9 ether}(2);

        assertEq(basePool.getClaimableAmount(2, user1), 33 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 33 ether);
        assertEq(basePool.getClaimableAmount(2, user3), 33 ether);
    }

    function test_claim_multi_deposit_claim_between() public {
        setupStake(100 ether, 100 ether, 100 ether);
        basePool.deposit{value: 90 ether}(2);

        assertEq(basePool.getClaimableAmount(2, user1), 30 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 30 ether);
        assertEq(basePool.getClaimableAmount(2, user3), 30 ether);

        vm.startPrank(user1);
        basePool.claim(2, user1);
        vm.stopPrank();

        assertEq(basePool.getClaimableAmount(2, user1), 0 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 30 ether);
        assertEq(basePool.getClaimableAmount(2, user3), 30 ether);

        basePool.deposit{value: 9 ether}(2);

        assertEq(basePool.getClaimableAmount(2, user1), 3 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 33 ether);
        assertEq(basePool.getClaimableAmount(2, user3), 33 ether);

        vm.startPrank(user2);
        basePool.claim(2, user2);
        vm.stopPrank();

        assertEq(basePool.getClaimableAmount(2, user1), 3 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 0 ether);
        assertEq(basePool.getClaimableAmount(2, user3), 33 ether);
    }

    function test_claim_to_other_user() public {
        setupStake(100 ether, 0 ether, 0 ether);
        basePool.deposit{value: 100 ether}(2);

        assertEq(basePool.getClaimableAmount(2, user1), 100 ether);
        assertEq(basePool.getClaimableAmount(2, user2), 0 ether);
        assertEq(basePool.getClaimableAmount(2, user3), 0 ether);

        uint256 user2Balance = address(user2).balance;

        vm.startPrank(user1);
        basePool.claim(2, user2);
        vm.stopPrank();

        assertEq(basePool.getClaimableAmount(2, user1), 0 ether);

        assertEq(address(user2).balance, user2Balance + 100 ether);
    }

    function test_claim_current_epoch() public {
        setupStake(100 ether, 0 ether, 0 ether);
        basePool.deposit{value: 100 ether}(3);

        uint256 currentEpoch = staking.getCurrentEpoch();

        vm.startPrank(user1);
        vm.expectRevert(BasePool.ClaimNotAvailable.selector);
        basePool.claim(currentEpoch, user1);
        vm.stopPrank();
    }

    function test_claim_future_epoch() public {
        setupStake(100 ether, 0 ether, 0 ether);
        basePool.deposit{value: 100 ether}(3);

        uint256 currentEpoch = staking.getCurrentEpoch();

        vm.startPrank(user1);
        vm.expectRevert(BasePool.ClaimNotAvailable.selector);
        basePool.claim(currentEpoch + 1, user1);
        vm.stopPrank();
    }

    function test_claim_not_claimable() public {
        setupStake(100 ether, 0 ether, 0 ether);
        basePool.deposit{value: 100 ether}(2);

        assertEq(basePool.getClaimableAmount(2, user2), 0 ether);

        vm.startPrank(user2);
        vm.expectRevert(BasePool.ClaimNotAvailable.selector);
        basePool.claim(2, user2);
        vm.stopPrank();
    }

    function test_claim_bulk() public {
        setupStake(100 ether, 100 ether, 100 ether);
        vm.warp(block.timestamp + 60 days);
        basePool.deposit{value: 90 ether}(2);
        basePool.deposit{value: 90 ether}(3);
        basePool.deposit{value: 90 ether}(4);

        uint256[] memory epochIndices = new uint256[](3);
        epochIndices[0] = 2;
        epochIndices[1] = 3;
        epochIndices[2] = 4;

        vm.startPrank(user1);
        basePool.claimBulk(epochIndices, user1);
        vm.stopPrank();

        // 30 + 30 + 30
        assertEq(address(user1).balance, 90 ether);
    }

    function test_claim_bulk_current_epoch() public {
        setupStake(100 ether, 100 ether, 100 ether);
        vm.warp(block.timestamp + 30 days);
        basePool.deposit{value: 90 ether}(2);
        basePool.deposit{value: 90 ether}(3);
        basePool.deposit{value: 90 ether}(4);

        uint256[] memory epochIndices = new uint256[](3);
        epochIndices[0] = 2;
        epochIndices[1] = 3;
        epochIndices[2] = 4;

        vm.startPrank(user1);
        vm.expectRevert(BasePool.ClaimNotAvailable.selector);
        basePool.claimBulk(epochIndices, user1);
        vm.stopPrank();
    }

    function test_claim_bulk_already_claimed() public {
        setupStake(100 ether, 100 ether, 100 ether);
        vm.warp(block.timestamp + 60 days);
        basePool.deposit{value: 90 ether}(2);
        basePool.deposit{value: 90 ether}(3);
        basePool.deposit{value: 90 ether}(4);

        vm.startPrank(user1);
        basePool.claim(3, user1);
        vm.stopPrank();

        uint256[] memory epochIndices = new uint256[](3);
        epochIndices[0] = 2;
        epochIndices[1] = 3;
        epochIndices[2] = 4;

        vm.startPrank(user1);
        vm.expectRevert(BasePool.ClaimNotAvailable.selector);
        basePool.claimBulk(epochIndices, user1);
        vm.stopPrank();

        uint256[] memory epochIndicesGap = new uint256[](2);
        epochIndicesGap[0] = 2;
        epochIndicesGap[1] = 4;

        vm.startPrank(user1);
        basePool.claimBulk(epochIndicesGap, user1);
        vm.stopPrank();

        assertEq(address(user1).balance, 90 ether);
    }
}
