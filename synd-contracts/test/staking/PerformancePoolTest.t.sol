// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {SyndStaking} from "src/staking/SyndStaking.sol";
import {PerformancePool} from "src/staking/PerformancePool.sol";
import {RewardPoolBase} from "src/staking/RewardPoolBase.sol";
import {UD60x18, ud, convert} from "@prb/math/src/UD60x18.sol";

/// @notice Interface the pool expects for gas accounting
interface IGasProvider {
    function getTotalGasFees(uint256 epochIndex) external view returns (uint256);
    function getAppchainGasFees(uint256 epochIndex, uint256 appchainId) external view returns (uint256);
    function getActiveAppchainIds(uint256 epochIndex) external view returns (uint256[] memory);
}

/// @notice Mock gas provider: programmable per-epoch fees + active IDs
contract MockGasProvider is IGasProvider {
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

    function getActiveAppchainIds(uint256 epochIndex) external view returns (uint256[] memory out) {
        uint256[] storage ids = idsByEpoch[epochIndex];
        out = new uint256[](ids.length);
        for (uint256 i = 0; i < ids.length; i++) {
            out[i] = ids[i];
        }
    }
}

contract PerformancePoolTest is Test {
    SyndStaking public staking;
    PerformancePool public performancePool;
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
        performancePool = new PerformancePool(msg.sender, address(staking), address(gasProvider));

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

    function test_claim_1_user_stake() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0); // only chain1 has fees

        performancePool.deposit{value: 100 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 100 ether);

        vm.startPrank(user1);
        performancePool.claim(epoch, user1, appchainId1);
        vm.stopPrank();

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0 ether);
    }

    function test_claim_2_user_stake() public {
        setupStake(100 ether, 100 ether, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 0); // chains 1 & 2 active

        performancePool.deposit{value: 100 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 50 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 50 ether);

        vm.startPrank(user1);
        performancePool.claim(epoch, user1, appchainId1);
        vm.stopPrank();

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 50 ether);

        vm.startPrank(user2);
        performancePool.claim(epoch, user2, appchainId2);
        vm.stopPrank();

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 0 ether);
    }

    function test_claim_3_user_stake() public {
        setupStake(100 ether, 100 ether, 100 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 1); // symmetric gas activity

        performancePool.deposit{value: 90 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 30 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 30 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user3, appchainId3), 30 ether);

        vm.startPrank(user1);
        performancePool.claim(epoch, user1, appchainId1);
        vm.stopPrank();

        vm.startPrank(user2);
        performancePool.claim(epoch, user2, appchainId2);
        vm.stopPrank();

        vm.startPrank(user3);
        performancePool.claim(epoch, user3, appchainId3);
        vm.stopPrank();

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 0 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user3, appchainId3), 0 ether);
    }

    function test_claim_multi_deposit() public {
        setupStake(100 ether, 100 ether, 100 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 1);

        performancePool.deposit{value: 90 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 30 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 30 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user3, appchainId3), 30 ether);

        performancePool.deposit{value: 9 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 33 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 33 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user3, appchainId3), 33 ether);
    }

    function test_claim_multi_deposit_claim_between() public {
        setupStake(100 ether, 100 ether, 100 ether);
        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 1);

        performancePool.deposit{value: 90 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 30 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 30 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user3, appchainId3), 30 ether);

        vm.startPrank(user1);
        performancePool.claim(epoch, user1, appchainId1);
        vm.stopPrank();

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 30 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user3, appchainId3), 30 ether);

        performancePool.deposit{value: 9 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 3 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 33 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user3, appchainId3), 33 ether);

        vm.startPrank(user2);
        performancePool.claim(epoch, user2, appchainId2);
        vm.stopPrank();

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 3 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 0 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user3, appchainId3), 33 ether);
    }

    function test_claim_to_other_user() public {
        setupStake(100 ether, 0, 0);
        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);

        performancePool.deposit{value: 100 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 100 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId1), 0 ether);
        assertEq(performancePool.getClaimableAmount(epoch, user3, appchainId1), 0 ether);

        uint256 user2Balance = address(user2).balance;

        vm.startPrank(user1);
        performancePool.claim(epoch, user2, appchainId1);
        vm.stopPrank();

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0 ether);

        assertEq(address(user2).balance, user2Balance + 100 ether);
    }

    /* ---------- Edge Case Tests ------------- */

    function test_claim_current_epoch() public {
        setupStake(100 ether, 0, 0);
        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);
        performancePool.deposit{value: 100 ether}(epoch);

        uint256 currentEpoch = staking.getCurrentEpoch();

        vm.startPrank(user1);
        vm.expectRevert(RewardPoolBase.ClaimNotAvailable.selector);
        performancePool.claim(currentEpoch, user1, appchainId1);
        vm.stopPrank();
    }

    function test_claim_future_epoch() public {
        setupStake(100 ether, 0, 0);
        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);
        performancePool.deposit{value: 100 ether}(epoch);

        uint256 currentEpoch = staking.getCurrentEpoch();

        vm.startPrank(user1);
        vm.expectRevert(RewardPoolBase.ClaimNotAvailable.selector);
        performancePool.claim(currentEpoch + 1, user1, appchainId1);
        vm.stopPrank();
    }

    function test_claim_not_claimable() public {
        // Stake exists but NO gas activity → totalGasFees = 0 → claimable = 0
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 0, 0, 0);
        performancePool.deposit{value: 100 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0);

        vm.startPrank(user1);
        vm.expectRevert(RewardPoolBase.ClaimNotAvailable.selector);
        performancePool.claim(epoch, user1, appchainId1);
        vm.stopPrank();
    }

    function test_claim_zero_user_stake() public {
        // No stakes but gas activity exists
        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);

        performancePool.deposit{value: 100 ether}(epoch);

        // claimable must be zero and claim must revert
        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0, "should be zero when userStaked==0");

        vm.startPrank(user1);
        vm.expectRevert(RewardPoolBase.ClaimNotAvailable.selector);
        performancePool.claim(epoch, user1, appchainId1);
        vm.stopPrank();
    }

    function test_claim_zero_appchain_stake() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);

        performancePool.deposit{value: 100 ether}(epoch);

        // Try to claim for appchainId2 which has no stake
        assertEq(
            performancePool.getClaimableAmount(epoch, user1, appchainId2), 0, "should be zero when appchainStaked==0"
        );

        vm.startPrank(user1);
        vm.expectRevert(RewardPoolBase.ClaimNotAvailable.selector);
        performancePool.claim(epoch, user1, appchainId2);
        vm.stopPrank();
    }

    /* ---------- claimFor Tests (onlyStakingContract modifier) ------------- */

    function test_claimFor_success() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);

        performancePool.deposit{value: 100 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 100 ether);

        uint256 user2Balance = address(user2).balance;

        // Only staking contract can call claimFor
        vm.startPrank(address(staking));
        performancePool.claimFor(epoch, user1, user2, appchainId1);
        vm.stopPrank();

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0 ether);
        assertEq(address(user2).balance, user2Balance + 100 ether);
    }

    function test_claimFor_unauthorized_caller() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);

        performancePool.deposit{value: 100 ether}(epoch);

        // Non-staking contract should not be able to call claimFor
        vm.startPrank(user1);
        vm.expectRevert(PerformancePool.UnauthorizedCaller.selector);
        performancePool.claimFor(epoch, user1, user2, appchainId1);
        vm.stopPrank();
    }

    function test_claimFor_zero_amount() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 0, 0, 0); // No gas activity

        performancePool.deposit{value: 100 ether}(epoch);

        // Should revert with ClaimNotAvailable when amount is 0
        vm.startPrank(address(staking));
        vm.expectRevert(RewardPoolBase.ClaimNotAvailable.selector);
        performancePool.claimFor(epoch, user1, user2, appchainId1);
        vm.stopPrank();
    }

    /* ---------- Diminishing Factor Tests ------------- */

    /// j = {0.60, 0.30, 0.10}, f(j)=ln(1+2j), pool=1,030,904
    /// NOTE: Expects DECAY_FACTOR=2 and multipliers FEE=0.4, STAKE=0.2 (as in the contract).
    function test_claim_diminishing_ln_example() public {
        // 60:30:10 stakes → stake shares 0.6, 0.3, 0.1
        setupStake(60 ether, 30 ether, 10 ether);

        uint256 epoch = _settledEpoch();

        // Match fee shares to the same ratio so dominance uses same shares
        setGasShares(epoch, 60, 30, 10);

        uint256 pool = 1_030_904; // raw integer to mirror the doc
        performancePool.deposit{value: pool}(epoch);

        uint256 a = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        uint256 b = performancePool.getClaimableAmount(epoch, user2, appchainId2);
        uint256 c = performancePool.getClaimableAmount(epoch, user3, appchainId3);

        // With weights 0.4/0.2 and DECAY=2, expected approx:
        // A ≈ 580,482 ; B ≈ 329,119 ; C ≈ 121,302 (sum = 1,030,904)
        assertApproxEqAbs(a, 580_482, 3, "Appchain A claimable mismatch");
        assertApproxEqAbs(b, 329_119, 3, "Appchain B claimable mismatch");
        assertApproxEqAbs(c, 121_302, 3, "Appchain C claimable mismatch");

        uint256 sum = a + b + c;
        assertApproxEqAbs(sum, pool, 2, "Sum of claimables within rounding tolerance");
    }

    /* ---------- Event and Balance Tests ------------- */

    function test_claim_pays_receiver_and_updates_pool_and_emits_event() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);

        // Fund pool
        performancePool.deposit{value: 100 ether}(epoch);

        // Snapshot balances pre-claim
        uint256 poolBefore = address(performancePool).balance;
        uint256 userBefore = user1.balance;

        // Preview expected amount
        uint256 expected = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        assertEq(expected, 100 ether);

        // Expect the ClaimSuccess event
        vm.expectEmit(true, true, true, true, address(performancePool));
        emit ClaimSuccess(epoch, appchainId1, user1, expected);

        // Claim
        vm.prank(user1);
        performancePool.claim(epoch, user1, appchainId1);

        // User got paid
        assertEq(user1.balance, userBefore + expected, "user did not receive amount");

        // Pool balance decreased by the same amount
        assertEq(address(performancePool).balance, poolBefore - expected, "pool balance not reduced");

        // Further claim should be 0
        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0);
    }

    /* ---------- Multiple Epochs Tests ------------- */

    function test_multiple_epochs_claim() public {
        setupStake(100 ether, 0, 0);

        // Epoch 1
        uint256 e1 = _settledEpoch();
        setGasShares(e1, 1, 0, 0);
        performancePool.deposit{value: 40 ether}(e1);

        uint256 userBefore1 = user1.balance;
        uint256 poolBefore1 = address(performancePool).balance;
        uint256 exp1 = performancePool.getClaimableAmount(e1, user1, appchainId1);

        vm.expectEmit(true, true, true, true, address(performancePool));
        emit ClaimSuccess(e1, appchainId1, user1, exp1);
        vm.prank(user1);
        performancePool.claim(e1, user1, appchainId1);

        assertEq(user1.balance, userBefore1 + exp1, "epoch1 user did not receive");
        assertEq(address(performancePool).balance, poolBefore1 - exp1, "pool not reduced e1");

        // Advance time so we get a new settled epoch
        vm.warp(block.timestamp + 30 days);

        // Epoch 2
        uint256 e2 = _settledEpoch();
        require(e2 != e1, "need a different epoch after warp");
        setGasShares(e2, 1, 0, 0);
        performancePool.deposit{value: 60 ether}(e2);

        uint256 userBefore2 = user1.balance;
        uint256 poolBefore2 = address(performancePool).balance;

        uint256 exp2 = performancePool.getClaimableAmount(e2, user1, appchainId1);

        vm.expectEmit(true, true, true, true, address(performancePool));
        emit ClaimSuccess(e2, appchainId1, user1, exp2);
        vm.prank(user1);
        performancePool.claim(e2, user1, appchainId1);

        assertEq(user1.balance, userBefore2 + exp2, "epoch2 user did not receive");
        assertEq(address(performancePool).balance, poolBefore2 - exp2, "pool not reduced e2");
    }

    /* ---------- Partial Claims Tests ------------- */

    function test_claim_then_deposit_then_claim_pays_incremental() public {
        setupStake(100 ether, 100 ether, 100 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 1);

        // First deposit & first claim by user1
        performancePool.deposit{value: 90 ether}(epoch);

        uint256 before1 = user1.balance;
        uint256 expected1 = performancePool.getClaimableAmount(epoch, user1, appchainId1);

        vm.prank(user1);
        performancePool.claim(epoch, user1, appchainId1);

        assertEq(user1.balance, before1 + expected1, "first claim not paid");

        // Second deposit increases claimable for all users
        performancePool.deposit{value: 9 ether}(epoch);
        uint256 before2 = user1.balance;
        uint256 expected2 = performancePool.getClaimableAmount(epoch, user1, appchainId1); // should be the delta (≈ 3 ether)

        vm.prank(user1);
        performancePool.claim(epoch, user1, appchainId1);

        assertEq(user1.balance, before2 + expected2, "incremental claim not paid");
    }

    /* ---------- GetClaimableAmount Edge Cases ------------- */

    function test_getClaimableAmount_reverts_on_current_or_future_epoch() public {
        setupStake(100 ether, 0, 0);

        // Use a fresh epoch index equal to current
        uint256 currentEpoch = staking.getCurrentEpoch();

        // Configure data & deposit into currentEpoch (math fine, but reads should revert per gating)
        setGasShares(currentEpoch, 1, 0, 0);
        performancePool.deposit{value: 10 ether}(currentEpoch);

        // Expect revert on getClaimableAmount for current
        vm.expectRevert(RewardPoolBase.ClaimNotAvailable.selector);
        performancePool.getClaimableAmount(currentEpoch, user1, appchainId1);

        // Future epoch should also revert
        vm.expectRevert(RewardPoolBase.ClaimNotAvailable.selector);
        performancePool.getClaimableAmount(currentEpoch + 1, user1, appchainId1);
    }

    /* ---------- Setter Functions Tests ------------- */

    function test_setter_functions_and_calculations() public {
        setupStake(100 ether, 100 ether, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 0);

        // Initial deposit
        performancePool.deposit{value: 100 ether}(epoch);

        // Get initial claimable amounts
        uint256 initialUser1 = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        uint256 initialUser2 = performancePool.getClaimableAmount(epoch, user2, appchainId2);
        assertGt(initialUser1, 0, "Should have claimable amount");
        assertGt(initialUser2, 0, "Should have claimable amount");

        // Test setFeeMultiplier - use the actual owner
        vm.startPrank(performancePool.owner());
        performancePool.setFeeMultiplier(0.6e18); // Change from 0.4 to 0.6
        vm.stopPrank();

        // Verify the multiplier was actually changed
        bool isEqFeeMultiplier = performancePool.feeMultiplier().eq(ud(0.6e18));
        assertTrue(isEqFeeMultiplier, "Fee multiplier should be updated to 0.6e18");

        // Test setStakeMultiplier
        vm.startPrank(performancePool.owner());
        performancePool.setStakeMultiplier(0.4e18); // Change from 0.2 to 0.4
        vm.stopPrank();

        // Verify the multiplier was actually changed
        assertTrue(performancePool.stakeMultiplier().eq(ud(0.4e18)), "Stake multiplier should be updated");

        // Test setDecayFactor
        vm.startPrank(performancePool.owner());
        performancePool.setDecayFactor(3e18); // Change from 2 to 3
        vm.stopPrank();

        // Verify the multiplier was actually changed
        assertTrue(performancePool.decayFactor().eq(ud(3e18)), "Decay factor should be updated");

        // Verify that calculations still work after all changes
        uint256 finalUser1 = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        uint256 finalUser2 = performancePool.getClaimableAmount(epoch, user2, appchainId2);

        // Both should still have claimable amounts
        assertGt(finalUser1, 0, "User1 should still have claimable amount");
        assertGt(finalUser2, 0, "User2 should still have claimable amount");

        // Sum should still equal total deposit
        assertApproxEqAbs(finalUser1 + finalUser2, 100 ether, 1, "Total claimable should equal deposit");
    }

    function test_setter_functions_only_owner() public {
        vm.startPrank(user1); // non-owner

        vm.expectRevert(); // Ownable: caller is not the owner
        performancePool.setFeeMultiplier(0.5e18);

        vm.expectRevert(); // Ownable: caller is not the owner
        performancePool.setStakeMultiplier(0.3e18);

        vm.expectRevert(); // Ownable: caller is not the owner
        performancePool.setDecayFactor(2.5e18);

        vm.stopPrank();
    }

    /* ---------- Precision and Edge Cases ------------- */

    function test_precision_handling_small_amounts() public {
        setupStake(1 ether, 1 ether, 0); // Small stakes

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 0);

        // Very small deposit
        performancePool.deposit{value: 1 wei}(epoch);

        // Should handle small amounts without reverting
        uint256 claimable1 = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        uint256 claimable2 = performancePool.getClaimableAmount(epoch, user2, appchainId2);

        // With very small amounts, one might get 0 due to rounding
        assertTrue(claimable1 >= 0, "Claimable1 should be >= 0");
        assertTrue(claimable2 >= 0, "Claimable2 should be >= 0");
        assertTrue(claimable1 + claimable2 <= 1 wei, "Total claimable should not exceed deposit");
    }

    function test_complex_scenario_fair_distribution() public {
        setupStake(60 ether, 30 ether, 10 ether); // 60:30:10 ratio

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 60, 30, 10); // Match stake ratio

        // Large deposit
        performancePool.deposit{value: 1000 ether}(epoch);

        // Get claimable amounts
        uint256 claimable1 = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        uint256 claimable2 = performancePool.getClaimableAmount(epoch, user2, appchainId2);
        uint256 claimable3 = performancePool.getClaimableAmount(epoch, user3, appchainId3);

        // All should have claimable amounts
        assertGt(claimable1, 0, "User1 should have claimable amount");
        assertGt(claimable2, 0, "User2 should have claimable amount");
        assertGt(claimable3, 0, "User3 should have claimable amount");

        // Sum should equal total deposit
        assertApproxEqAbs(claimable1 + claimable2 + claimable3, 1000 ether, 2, "Total should equal deposit");

        // User1 should get more than User2, which should get more than User3
        assertGt(claimable1, claimable2, "User1 should get more than User2");
        assertGt(claimable2, claimable3, "User2 should get more than User3");

        // Test partial claims
        vm.startPrank(user1);
        performancePool.claim(epoch, user1, appchainId1);
        vm.stopPrank();

        // User1 should now have 0 claimable
        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0, "User1 should be fully claimed");

        // User2 and 3 should still have their amounts
        assertEq(
            performancePool.getClaimableAmount(epoch, user2, appchainId2),
            claimable2,
            "User2 should still have claimable amount"
        );
        assertEq(
            performancePool.getClaimableAmount(epoch, user3, appchainId3),
            claimable3,
            "User3 should still have claimable amount"
        );
    }

    /* ---------- Multiple Deposits and Claims Tests ------------- */

    function test_multiple_deposits_claims_no_underflow() public {
        setupStake(100 ether, 100 ether, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 0);

        // Multiple deposits
        performancePool.deposit{value: 50 ether}(epoch);
        performancePool.deposit{value: 30 ether}(epoch);
        performancePool.deposit{value: 20 ether}(epoch);

        // Total deposit should be 100 ether
        assertEq(performancePool.epochTotal(epoch), 100 ether, "Total epoch deposit should be 100 ether");

        // User1 claims
        uint256 claimable1 = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        assertGt(claimable1, 0, "User1 should have claimable amount");

        vm.startPrank(user1);
        performancePool.claim(epoch, user1, appchainId1);
        vm.stopPrank();

        // User2 claims
        uint256 claimable2 = performancePool.getClaimableAmount(epoch, user2, appchainId2);
        assertGt(claimable2, 0, "User2 should have claimable amount");

        vm.startPrank(user2);
        performancePool.claim(epoch, user2, appchainId2);
        vm.stopPrank();

        // Both should now have 0 claimable
        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 0, "User1 should be fully claimed");
        assertEq(performancePool.getClaimableAmount(epoch, user2, appchainId2), 0, "User2 should be fully claimed");

        // Additional deposit should still work
        performancePool.deposit{value: 10 ether}(epoch);

        // Both should now have new claimable amounts
        uint256 newClaimable1 = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        uint256 newClaimable2 = performancePool.getClaimableAmount(epoch, user2, appchainId2);

        assertGt(newClaimable1, 0, "User1 should have new claimable amount");
        assertGt(newClaimable2, 0, "User2 should have new claimable amount");
        assertApproxEqAbs(newClaimable1 + newClaimable2, 10 ether, 1, "Total new claimable should equal new deposit");
    }

    /* ---------- Caching Tests ------------- */

    function test_diminishing_factors_caching() public {
        setupStake(50 ether, 50 ether, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 0);

        performancePool.deposit{value: 100 ether}(epoch);

        // First call should calculate and cache
        uint256 firstCall = performancePool.getClaimableAmount(epoch, user1, appchainId1);

        // Second call should use cached value
        uint256 secondCall = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        assertEq(firstCall, secondCall, "Cached calls should return same value");

        // After changing multipliers
        vm.startPrank(performancePool.owner());
        performancePool.setFeeMultiplier(0.8e18); // Double the fee multiplier
        vm.stopPrank();
        assertTrue(performancePool.feeMultiplier().eq(ud(0.8e18)), "Fee multiplier should remain updated");

        // New call should recalculate
        uint256 thirdCall = performancePool.getClaimableAmount(epoch, user1, appchainId1);
        assertEq(thirdCall, firstCall, "Setting fee multiplier should not change claimable amount until next epoch");
    }

    function test_claim_zero_destination() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);

        performancePool.deposit{value: 100 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 100 ether);

        vm.startPrank(user1);
        vm.expectRevert(RewardPoolBase.InvalidDestination.selector);
        performancePool.claim(epoch, address(0), appchainId1);
        vm.stopPrank();
    }

    function test_claimFor_zero_destination() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);

        performancePool.deposit{value: 100 ether}(epoch);

        assertEq(performancePool.getClaimableAmount(epoch, user1, appchainId1), 100 ether);

        // Only staking contract can call claimFor
        vm.startPrank(address(staking));
        vm.expectRevert(RewardPoolBase.InvalidDestination.selector);
        performancePool.claimFor(epoch, user1, address(0), appchainId1);
        vm.stopPrank();
    }
}
