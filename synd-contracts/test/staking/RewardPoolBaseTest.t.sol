// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {SyndStaking} from "src/staking/SyndStaking.sol";
import {RewardPoolBase} from "src/staking/RewardPoolBase.sol";
import {UD60x18, ud, convert} from "@prb/math/src/UD60x18.sol";
import {IGasDataProvider} from "src/staking/interfaces/IGasDataProvider.sol";
import {MockGasProvider} from "./MockGasProvider.t.sol";

contract MockRewardPoolBase is RewardPoolBase {
    constructor(address _defaultAdmin, address _staking, address _gas) RewardPoolBase(_defaultAdmin, _staking, _gas) {}
}

contract RewardPoolBaseTest is Test {
    SyndStaking public staking;
    MockRewardPoolBase public rewardPoolBase;
    MockGasProvider public gasProvider;

    address public user1;
    address public user2;
    address public user3;

    // fixed IDs we use throughout the suite
    uint256 public appchainId1 = 111;
    uint256 public appchainId2 = 222;
    uint256 public appchainId3 = 333;

    event ClaimSuccess(
        uint256 indexed epochIndex, uint256 indexed appchainId, address indexed destination, uint256 amount
    );

    function setUp() public {
        // fund this test contract for deposits
        vm.deal(address(this), 10_000 ether);

        staking = new SyndStaking(msg.sender);
        gasProvider = new MockGasProvider();

        // pool takes admin, staking + gas provider
        rewardPoolBase = new MockRewardPoolBase(msg.sender, address(staking), address(gasProvider));

        user1 = makeAddr("user1");
        user2 = makeAddr("user2");
        user3 = makeAddr("user3");

        vm.deal(user1, 100 ether);
        vm.deal(user2, 100 ether);
        vm.deal(user3, 100 ether);

        vm.warp(staking.START_TIMESTAMP());
    }

    /* ---------- Helpers ---------- */

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
        // advance beyond stake lock windows if your staking requires it
        vm.warp(block.timestamp + 60 days);
    }

    function setGasShares(uint256 epoch, uint256 g1, uint256 g2, uint256 g3) internal {
        uint256[] memory feesLocal = new uint256[](3);
        uint256[] memory idsLocal = new uint256[](3);

        idsLocal[0] = appchainId1;
        feesLocal[0] = g1;
        idsLocal[1] = appchainId2;
        feesLocal[1] = g2;
        idsLocal[2] = appchainId3;
        feesLocal[2] = g3;
        gasProvider.setFees(epoch, idsLocal, feesLocal);
    }

    /// Returns a finalized epoch index (< current). Warps if needed.
    function _settledEpoch() internal returns (uint256) {
        uint256 cur = staking.getCurrentEpoch();
        if (cur == 0) {
            // push time forward to ensure at least one epoch has passed
            vm.warp(block.timestamp + 30 days);
            cur = staking.getCurrentEpoch();
        }
        // ensure strictly less than current
        if (cur == 0) {
            // extremely defensive: if still 0, push further
            vm.warp(block.timestamp + 365 days);
            cur = staking.getCurrentEpoch();
        }
        return cur - 1;
    }

    /* ---------- Basic Claim Tests ------------- */

    function test_getAppchainTotalReward() public {
        setupStake(30 ether, 20 ether, 10 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 60 ether, 50 ether, 40 ether);

        rewardPoolBase.deposit{value: 100 ether}(epoch);
        assertTrue(rewardPoolBase.computeDiminishingFactors(epoch, 0));
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId1), 41785679991199430718);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId2), 33578634284580271148);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId3), 24635685724220298132);
    }

    function test_computeDiminishingFactors() public {
        setupStake(30 ether, 20 ether, 10 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 60 ether, 50 ether, 40 ether);

        assertFalse(rewardPoolBase.computeDiminishingFactors(epoch, 1));
        assertFalse(rewardPoolBase.computeDiminishingFactors(epoch, 1));
        assertTrue(rewardPoolBase.computeDiminishingFactors(epoch, 1));

        rewardPoolBase.deposit{value: 100 ether}(epoch);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId1), 41785679991199430718);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId2), 33578634284580271148);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId3), 24635685724220298132);
    }

    function test_computePartial() public {
        setupStake(30 ether, 20 ether, 10 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 60 ether, 50 ether, 40 ether);
        rewardPoolBase.deposit{value: 100 ether}(epoch);

        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId1), 41785679991199430718);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId2), 33578634284580271148);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId3), 24635685724220298132);

        assertFalse(rewardPoolBase.computeDiminishingFactors(epoch, 1));
        assertEq(convert(rewardPoolBase.diminishingFactor(epoch, appchainId1)), 0);
        assertEq(convert(rewardPoolBase.diminishingFactor(epoch, appchainId2)), 0);
        assertEq(convert(rewardPoolBase.diminishingFactor(epoch, appchainId3)), 0);

        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId1), 41785679991199430718);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId2), 33578634284580271148);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId3), 24635685724220298132);
    }

    function test_computeLargeBatch() public {
        setupStake(30 ether, 20 ether, 10 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 60 ether, 50 ether, 40 ether);

        assertTrue(rewardPoolBase.computeDiminishingFactors(epoch, 100));

        rewardPoolBase.deposit{value: 100 ether}(epoch);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId1), 41785679991199430718);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId2), 33578634284580271148);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId3), 24635685724220298132);
    }
}
