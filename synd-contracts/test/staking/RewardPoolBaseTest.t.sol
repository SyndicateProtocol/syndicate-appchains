// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {SyndStaking} from "src/staking/SyndStaking.sol";
import {RewardPoolBase} from "src/staking/RewardPoolBase.sol";
import {UD60x18, ud, convert} from "@prb/math/src/UD60x18.sol";
import {IGasDataProvider} from "src/staking/interfaces/IGasDataProvider.sol";

/// @notice Mock gas provider: programmable per-epoch fees + active IDs
contract MockGasProvider is IGasDataProvider {
    // epoch => total fees
    mapping(uint256 => uint256) public totals;
    // epoch => appchainId => fees
    mapping(uint256 => mapping(uint256 => uint256)) public fee;
    // epoch => list of appchainIds (we keep exactly what tests set)
    mapping(uint256 => uint256[]) private idsByEpoch;

    function setFees(uint256 epoch, uint256[] memory appchainIds, uint256[] memory amounts) external {
        require(appchainIds.length == amounts.length, "length mismatch");

        // reset ids list
        delete idsByEpoch[epoch];

        uint256 t;
        for (uint256 i = 0; i < appchainIds.length; i++) {
            uint256 id = appchainIds[i];
            uint256 amt = amounts[i];
            fee[epoch][id] = amt;
            idsByEpoch[epoch].push(id);
            t += amt;
        }
        totals[epoch] = t;
    }

    function setFee(uint256 epoch, uint256 appchainId, uint256 amount) external {
        // if appchainId not in ids list, push it
        bool present = false;
        uint256[] storage ids = idsByEpoch[epoch];
        for (uint256 i = 0; i < ids.length; i++) {
            if (ids[i] == appchainId) {
                present = true;
                break;
            }
        }
        if (!present) ids.push(appchainId);

        uint256 prev = fee[epoch][appchainId];
        fee[epoch][appchainId] = amount;
        totals[epoch] = totals[epoch] + amount - prev;
    }

    function getTotalGasFees(uint256 epochIndex) external view returns (uint256) {
        return totals[epochIndex];
    }

    function getAppchainGasFees(uint256 epochIndex, uint256 appchainId) external view returns (uint256) {
        return fee[epochIndex][appchainId];
    }

    function getAppchainIds(uint256 epochIndex) external view returns (uint256[] memory out) {
        uint256[] storage ids = idsByEpoch[epochIndex];
        out = new uint256[](ids.length);
        for (uint256 i = 0; i < ids.length; i++) {
            out[i] = ids[i];
        }
    }

    function getAppchainIds(uint256 epochIndex, uint256 startIndex, uint256 pageSize)
        external
        view
        returns (uint256[] memory)
    {
        if (startIndex >= idsByEpoch[epochIndex].length) {
            return new uint256[](0);
        }

        uint256 endIndex = startIndex + pageSize;
        if (pageSize == 0 || endIndex > idsByEpoch[epochIndex].length) {
            endIndex = idsByEpoch[epochIndex].length;
        }
        uint256 actualSize = endIndex - startIndex;

        uint256[] memory result = new uint256[](actualSize);

        // Copy the relevant slice from the full array
        for (uint256 i = 0; i < actualSize; i++) {
            result[i] = idsByEpoch[epochIndex][startIndex + i];
        }

        return result;
    }
}

contract MockRewardPoolBase is RewardPoolBase {
    constructor(address _defaultAdmin, address _staking, address _gas) RewardPoolBase(_defaultAdmin, _staking, _gas) {}

    function getAppchainTotalReward(uint256 epochIndex, uint256 appchainId) external returns (uint256) {
        return _computeAppchainTotalReward(epochIndex, appchainId);
    }
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
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId1), 41785679991199430718);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId2), 33578634284580271148);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId3), 24635685724220298132);
    }

    function test_preComputeDiminishingFactors() public {
        setupStake(30 ether, 20 ether, 10 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 60 ether, 50 ether, 40 ether);

        assertFalse(rewardPoolBase.preComputeDiminishingFactors(epoch, 1));
        assertFalse(rewardPoolBase.preComputeDiminishingFactors(epoch, 1));
        assertTrue(rewardPoolBase.preComputeDiminishingFactors(epoch, 1));

        rewardPoolBase.deposit{value: 100 ether}(epoch);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId1), 41785679991199430718);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId2), 33578634284580271148);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId3), 24635685724220298132);
    }

    function test_preComputePartial() public {
        setupStake(30 ether, 20 ether, 10 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 60 ether, 50 ether, 40 ether);

        assertFalse(rewardPoolBase.preComputeDiminishingFactors(epoch, 1));

        rewardPoolBase.deposit{value: 100 ether}(epoch);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId1), 41785679991199430718);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId2), 33578634284580271148);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId3), 24635685724220298132);
    }

    function test_preComputeLargeBatch() public {
        setupStake(30 ether, 20 ether, 10 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 60 ether, 50 ether, 40 ether);

        assertTrue(rewardPoolBase.preComputeDiminishingFactors(epoch, 100));

        rewardPoolBase.deposit{value: 100 ether}(epoch);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId1), 41785679991199430718);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId2), 33578634284580271148);
        assertEq(rewardPoolBase.getAppchainTotalReward(epoch, appchainId3), 24635685724220298132);
    }
}
