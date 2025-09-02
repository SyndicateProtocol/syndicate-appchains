// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {SyndStaking} from "src/staking/SyndStaking.sol";
import {AppchainPool} from "src/staking/AppchainPool.sol";

/// @notice Interface the pool expects for gas accounting
interface IGasProvider {
    function getTotalGasFees(uint256 epochIndex) external view returns (uint256);
    function getAppchainGasFees(uint256 epochIndex, uint256 appchainId) external view returns (uint256);
    function getActiveAppchainIds(uint256 epochIndex) external view returns (uint256[] memory);
    function getAppchainRewardsReceiver(uint256 epochIndex, uint256 appchainId) external view returns (address);
}

/// @notice Mock gas provider: programmable per-epoch fees + active IDs + reward receivers
contract MockGasProvider is IGasProvider {
    // epoch => total fees
    mapping(uint256 => uint256) public totals;
    // epoch => appchainId => fees
    mapping(uint256 => mapping(uint256 => uint256)) public fee;
    // epoch => list of appchainIds (we keep exactly what tests set)
    mapping(uint256 => uint256[]) private idsByEpoch;
    // epoch => appchainId => rewards receiver
    mapping(uint256 => mapping(uint256 => address)) public receiver;

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

    function setReceiver(uint256 epoch, uint256 appchainId, address to) external {
        receiver[epoch][appchainId] = to;
    }

    function setReceivers(uint256 epoch, uint256[] memory appchainIds, address[] memory dests) external {
        require(appchainIds.length == dests.length, "length mismatch");
        for (uint256 i = 0; i < appchainIds.length; i++) {
            receiver[epoch][appchainIds[i]] = dests[i];
        }
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

    function getAppchainRewardsReceiver(uint256 epochIndex, uint256 appchainId) external view returns (address) {
        return receiver[epochIndex][appchainId];
    }
}

contract AppchainPoolTest is Test {
    SyndStaking public staking;
    AppchainPool public appchainPool;
    MockGasProvider public gasProvider;

    address public user1;
    address public user2;
    address public user3;

    // fixed IDs we use throughout the suite
    uint256 public appchainId1 = 111;
    uint256 public appchainId2 = 222;
    uint256 public appchainId3 = 333;

    // appchains destinations
    address public appchainDest1 = makeAddr("appchainDest1");
    address public appchainDest2 = makeAddr("appchainDest2");
    address public appchainDest3 = makeAddr("appchainDest3");

    event ClaimSuccess(
        uint256 indexed epochIndex, uint256 indexed appchainId, address indexed destination, uint256 amount
    );

    function setUp() public {
        // fund this test contract for deposits
        vm.deal(address(this), 10_000 ether);

        staking = new SyndStaking(msg.sender);
        gasProvider = new MockGasProvider();

        // pool takes staking + gas provider
        appchainPool = new AppchainPool(address(staking), address(gasProvider));

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

    function setDefaultReceivers(uint256 epoch) internal {
        uint256[] memory idsLocal = new uint256[](3);
        address[] memory dests = new address[](3);
        idsLocal[0] = appchainId1;
        dests[0] = appchainDest1;
        idsLocal[1] = appchainId2;
        dests[1] = appchainDest2;
        idsLocal[2] = appchainId3;
        dests[2] = appchainDest3;
        gasProvider.setReceivers(epoch, idsLocal, dests);
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

    /* ---------- Tests ------------- */

    function test_claim_1_user_stake() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0); // only chain1 has fees
        setDefaultReceivers(epoch);

        appchainPool.deposit{value: 100 ether}(epoch);

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 100 ether);

        vm.startPrank(user1);
        appchainPool.claim(epoch, appchainId1);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 0);
    }

    function test_claim_2_user_stake() public {
        setupStake(100 ether, 100 ether, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 0); // chains 1 & 2 active
        setDefaultReceivers(epoch);

        appchainPool.deposit{value: 100 ether}(epoch);

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 50 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 50 ether);

        vm.startPrank(user1);
        appchainPool.claim(epoch, appchainId1);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 0);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 50 ether);

        vm.startPrank(user2);
        appchainPool.claim(epoch, appchainId2);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 0);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 0);
    }

    function test_claim_3_user_stake() public {
        setupStake(100 ether, 100 ether, 100 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 1); // symmetric gas activity
        setDefaultReceivers(epoch);

        appchainPool.deposit{value: 90 ether}(epoch);

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 30 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 30 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId3), 30 ether);

        vm.startPrank(user1);
        appchainPool.claim(epoch, appchainId1);
        vm.stopPrank();

        vm.startPrank(user2);
        appchainPool.claim(epoch, appchainId2);
        vm.stopPrank();

        vm.startPrank(user3);
        appchainPool.claim(epoch, appchainId3);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 0);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 0);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId3), 0);
    }

    function test_claim_multi_deposit() public {
        setupStake(100 ether, 100 ether, 100 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 1);
        setDefaultReceivers(epoch);

        appchainPool.deposit{value: 90 ether}(epoch);

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 30 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 30 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId3), 30 ether);

        appchainPool.deposit{value: 9 ether}(epoch);

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 33 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 33 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId3), 33 ether);
    }

    function test_claim_multi_deposit_claim_between() public {
        setupStake(100 ether, 100 ether, 100 ether);
        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 1);
        setDefaultReceivers(epoch);

        appchainPool.deposit{value: 90 ether}(epoch);

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 30 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 30 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId3), 30 ether);

        vm.startPrank(user1);
        appchainPool.claim(epoch, appchainId1);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 0);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 30 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId3), 30 ether);

        appchainPool.deposit{value: 9 ether}(epoch);

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 3 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 33 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId3), 33 ether);

        vm.startPrank(user2);
        appchainPool.claim(epoch, appchainId2);
        vm.stopPrank();

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 3 ether);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId2), 0);
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId3), 33 ether);
    }

    function test_claim_current_epoch() public {
        setupStake(100 ether, 0, 0);
        uint256 depositEpoch = _settledEpoch();

        setGasShares(depositEpoch, 1, 0, 0);
        setDefaultReceivers(depositEpoch);
        appchainPool.deposit{value: 100 ether}(depositEpoch);

        uint256 currentEpoch = staking.getCurrentEpoch();

        vm.startPrank(user1);
        vm.expectRevert(AppchainPool.ClaimNotAvailable.selector);
        appchainPool.claim(currentEpoch, appchainId1);
        vm.stopPrank();
    }

    function test_claim_future_epoch() public {
        setupStake(100 ether, 0, 0);
        uint256 depositEpoch = _settledEpoch();

        setGasShares(depositEpoch, 1, 0, 0);
        setDefaultReceivers(depositEpoch);
        appchainPool.deposit{value: 100 ether}(depositEpoch);

        uint256 currentEpoch = staking.getCurrentEpoch();

        vm.startPrank(user1);
        vm.expectRevert(AppchainPool.ClaimNotAvailable.selector);
        appchainPool.claim(currentEpoch + 1, appchainId1);
        vm.stopPrank();
    }

    function test_claim_not_claimable() public {
        // Stake exists but NO gas activity → totalGasFees = 0 → claimable = 0
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 0, 0, 0);
        setDefaultReceivers(epoch);
        appchainPool.deposit{value: 100 ether}(epoch);

        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 0);

        vm.startPrank(user2); // random user
        vm.expectRevert(AppchainPool.ClaimNotAvailable.selector);
        appchainPool.claim(epoch, appchainId1);
        vm.stopPrank();
    }

    /// j = {0.60, 0.30, 0.10}, f(j)=ln(1+2j), pool=1,030,904
    /// NOTE: Expects DECAY_FACTOR=2 and multipliers FEE=0.4, STAKE=0.2 (as in the contract).
    function test_claim_diminishing_ln_example() public {
        // 60:30:10 stakes → stake shares 0.6, 0.3, 0.1
        setupStake(60 ether, 30 ether, 10 ether);

        uint256 epoch = _settledEpoch();

        // Match fee shares to the same ratio so dominance uses same shares
        setGasShares(epoch, 60, 30, 10);
        setDefaultReceivers(epoch);

        uint256 pool = 1_030_904; // raw integer to mirror the doc
        appchainPool.deposit{value: pool}(epoch);

        uint256 a = appchainPool.getClaimableAmount(epoch, appchainId1);
        uint256 b = appchainPool.getClaimableAmount(epoch, appchainId2);
        uint256 c = appchainPool.getClaimableAmount(epoch, appchainId3);

        // With weights 0.4/0.2 and DECAY=2, expected approx:
        // A ≈ 580,482 ; B ≈ 329,119 ; C ≈ 121,302 (sum = 1,030,904)
        assertApproxEqAbs(a, 580_482, 3, "Appchain A claimable mismatch");
        assertApproxEqAbs(b, 329_119, 3, "Appchain B claimable mismatch");
        assertApproxEqAbs(c, 121_302, 3, "Appchain C claimable mismatch");

        uint256 sum = a + b + c;
        assertApproxEqAbs(sum, pool, 2, "Sum of claimables within rounding tolerance");
    }

    /// Ensures the configured rewards receiver actually gets paid,
    /// pool balance decreases by the same amount, and the event is emitted.
    function test_claim_pays_receiver_and_updates_pool_and_emits_event() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);
        setDefaultReceivers(epoch);

        // Fund pool
        appchainPool.deposit{value: 100 ether}(epoch);

        // Snapshot balances pre-claim
        uint256 poolBefore = address(appchainPool).balance;
        uint256 destBefore = appchainDest1.balance;

        // Preview expected amount
        uint256 expected = appchainPool.getClaimableAmount(epoch, appchainId1);
        assertEq(expected, 100 ether);

        // Expect the ClaimSuccess event
        vm.expectEmit(true, true, true, true, address(appchainPool));
        emit ClaimSuccess(epoch, appchainId1, appchainDest1, expected);

        // Claim
        vm.prank(user1);
        appchainPool.claim(epoch, appchainId1);

        // Receiver got paid
        assertEq(appchainDest1.balance, destBefore + expected, "receiver did not receive amount");

        // Pool balance decreased by the same amount
        assertEq(address(appchainPool).balance, poolBefore - expected, "pool balance not reduced");

        // Further claim should be 0
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 0);
    }

    /// If a receiver isn’t configured (address(0)), claim must revert with InvalidDestination.
    function test_claim_reverts_when_receiver_not_set() public {
        setupStake(100 ether, 0, 0);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 0, 0);

        // Intentionally DO NOT set receiver for appchainId1
        // (but set one for id2 just to show the epoch has some config)
        gasProvider.setReceiver(epoch, appchainId2, appchainDest2);

        appchainPool.deposit{value: 100 ether}(epoch);

        // Reads are fine; the revert happens at claim-time because destination == address(0)
        // If getClaimableAmount reverts due to gating in your version, skip this assert.
        uint256 amt = appchainPool.getClaimableAmount(epoch, appchainId1);
        assertEq(amt, 100 ether, "math should still compute before destination check");

        vm.startPrank(user1);
        vm.expectRevert(AppchainPool.InvalidDestination.selector);
        appchainPool.claim(epoch, appchainId1);
        vm.stopPrank();
    }

    /// If there is gas activity but totalStake is 0, no one should be able to claim.
    function test_totalStakeZero_blocks_claims() public {
        // no stakes
        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 10, 0, 0); // non-zero gas fees but zero total stake
        setDefaultReceivers(epoch);

        appchainPool.deposit{value: 100 ether}(epoch);

        // claimable must be zero and claim must revert
        assertEq(appchainPool.getClaimableAmount(epoch, appchainId1), 0, "should be zero when totalStake==0");

        vm.startPrank(user1);
        vm.expectRevert(AppchainPool.ClaimNotAvailable.selector);
        appchainPool.claim(epoch, appchainId1);
        vm.stopPrank();
    }

    /// Receivers are per-epoch: different epochs can route to different addresses.
    function test_receivers_are_per_epoch_and_respected() public {
        setupStake(100 ether, 0, 0);

        // Epoch 1
        uint256 e1 = _settledEpoch();
        setGasShares(e1, 1, 0, 0);
        gasProvider.setReceiver(e1, appchainId1, appchainDest1);
        appchainPool.deposit{value: 40 ether}(e1);

        // Make sure we are strictly past e1 (already the helper returns < current, so OK)
        uint256 dest1Before = appchainDest1.balance;
        uint256 poolBefore1 = address(appchainPool).balance;
        uint256 exp1 = appchainPool.getClaimableAmount(e1, appchainId1);

        vm.expectEmit(true, true, true, true, address(appchainPool));
        emit ClaimSuccess(e1, appchainId1, appchainDest1, exp1);
        vm.prank(user1);
        appchainPool.claim(e1, appchainId1);

        assertEq(appchainDest1.balance, dest1Before + exp1, "epoch1 receiver did not receive");
        assertEq(address(appchainPool).balance, poolBefore1 - exp1, "pool not reduced e1");

        // Advance time so we get a new settled epoch
        vm.warp(block.timestamp + 30 days);

        // Epoch 2 with a different receiver
        uint256 e2 = _settledEpoch();
        require(e2 != e1, "need a different epoch after warp");
        setGasShares(e2, 1, 0, 0);
        gasProvider.setReceiver(e2, appchainId1, appchainDest2); // <— different receiver
        appchainPool.deposit{value: 60 ether}(e2);

        uint256 dest2Before = appchainDest2.balance;
        uint256 poolBefore2 = address(appchainPool).balance;
        uint256 exp2 = appchainPool.getClaimableAmount(e2, appchainId1);

        vm.expectEmit(true, true, true, true, address(appchainPool));
        emit ClaimSuccess(e2, appchainId1, appchainDest2, exp2);
        vm.prank(user1);
        appchainPool.claim(e2, appchainId1);

        assertEq(appchainDest2.balance, dest2Before + exp2, "epoch2 receiver did not receive");
        assertEq(address(appchainPool).balance, poolBefore2 - exp2, "pool not reduced e2");
    }

    /// After a partial claim and a later deposit, the receiver should get the incremental amount.
    function test_claim_then_deposit_then_claim_pays_incremental_to_receiver() public {
        setupStake(100 ether, 100 ether, 100 ether);

        uint256 epoch = _settledEpoch();
        setGasShares(epoch, 1, 1, 1);
        setDefaultReceivers(epoch);

        // First deposit & first claim by appchain 1
        appchainPool.deposit{value: 90 ether}(epoch);
        uint256 before1 = appchainDest1.balance;
        uint256 expected1 = appchainPool.getClaimableAmount(epoch, appchainId1);

        vm.prank(user1);
        appchainPool.claim(epoch, appchainId1);

        assertEq(appchainDest1.balance, before1 + expected1, "first claim not paid");

        // Second deposit increases claimable for all appchains
        appchainPool.deposit{value: 9 ether}(epoch);
        uint256 before2 = appchainDest1.balance;
        uint256 expected2 = appchainPool.getClaimableAmount(epoch, appchainId1); // should be the delta (≈ 3 ether)

        vm.prank(user1);
        appchainPool.claim(epoch, appchainId1);

        assertEq(appchainDest1.balance, before2 + expected2, "incremental claim not paid");
    }

    /// getClaimableAmount should revert for current/future epochs per contract gating.
    function test_getClaimableAmount_reverts_on_current_or_future_epoch() public {
        setupStake(100 ether, 0, 0);

        // Use a fresh epoch index equal to current
        uint256 currentEpoch = staking.getCurrentEpoch();

        // Configure data & deposit into currentEpoch (math fine, but reads should revert per gating)
        setGasShares(currentEpoch, 1, 0, 0);
        gasProvider.setReceiver(currentEpoch, appchainId1, appchainDest1);
        appchainPool.deposit{value: 10 ether}(currentEpoch);

        // Expect revert on getClaimableAmount for current
        vm.expectRevert(AppchainPool.ClaimNotAvailable.selector);
        appchainPool.getClaimableAmount(currentEpoch, appchainId1);

        // Future epoch should also revert
        vm.expectRevert(AppchainPool.ClaimNotAvailable.selector);
        appchainPool.getClaimableAmount(currentEpoch + 1, appchainId1);
    }
}
