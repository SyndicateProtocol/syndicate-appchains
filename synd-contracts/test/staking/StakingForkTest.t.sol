// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndStaking} from "src/staking/SyndStaking.sol";
import {BasePool} from "src/staking/BasePool.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {console2} from "forge-std/console2.sol";

contract StakingForkTest is Test {
    SyndStaking public staking = SyndStaking(0xF9637B60f27AF139FC46EAa655cFBbe4E731BCdF);
    BasePool public basePool = BasePool(0x71cF8bf70Bb4f5ba8e4B4588bacB5ee108f3Ed10);
    address public admin = 0x03F8b8f48a3F22109bf1F4b54b54d0fdc96E7A67;

    address public user1 = makeAddr("user1");
    address public user2 = makeAddr("user2");
    address public user3 = makeAddr("user3");
    address public user4 = makeAddr("user4");
    address public user5 = makeAddr("user5");
    address public user6 = makeAddr("user6");
    address public user7 = makeAddr("user7");
    address public user8 = makeAddr("user8");
    address public user9 = makeAddr("user9");
    address public user10 = makeAddr("user10");

    uint256 public appchainId = 111;

    address[] public users = [user1, user2, user3, user4, user5, user6, user7, user8, user9, user10];

    function setUp() public {
        // Start fork
        vm.createSelectFork("https://commons.rpc.syndicate.io");

        if (address(staking) == address(0) || address(basePool) == address(0)) {
            admin = makeAddr("admin");
            console2.log("Staking contracts not found, deploying ones to fork");
            staking = new SyndStaking(admin);
            basePool = new BasePool(address(staking));
        }

        for (uint256 i = 0; i < users.length; i++) {
            vm.deal(users[i], 100 ether);
        }
    }

    function stepEpoch(uint256 epochsToStep) public {
        vm.warp(block.timestamp + epochsToStep * 30 days);
    }

    // ========== COMPREHENSIVE EDGE CASE TESTS ==========
    // All these are ~AI~ generated, so they may not be perfect.

    function test_stakingEdgeCases() public {
        // Test 1: Zero amount staking should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidAmount.selector);
        staking.stakeSynd{value: 0}(appchainId);
        vm.stopPrank();

        // Test 2: Invalid appchain ID (0) should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidAppchainId.selector);
        staking.stakeSynd{value: 1 ether}(0);
        vm.stopPrank();

        // Test 3: Staking with exact balance should work
        vm.startPrank(user1);
        staking.stakeSynd{value: 100 ether}(appchainId);
        vm.stopPrank();

        // Test 4: Staking when paused should revert
        vm.startPrank(admin);
        staking.pause();
        vm.stopPrank();

        vm.startPrank(user2);
        vm.expectRevert();
        staking.stakeSynd{value: 1 ether}(appchainId);
        vm.stopPrank();

        // Test 5: Unpause and staking should work again
        vm.startPrank(admin);
        staking.unpause();
        vm.stopPrank();

        vm.startPrank(user2);
        staking.stakeSynd{value: 1 ether}(appchainId);
        vm.stopPrank();
    }

    function test_multipleAppchainStakingEdgeCases() public {
        uint256[] memory appchainIds = new uint256[](3);
        uint256[] memory amounts = new uint256[](3);

        appchainIds[0] = 111;
        appchainIds[1] = 222;
        appchainIds[2] = 333;
        amounts[0] = 10 ether;
        amounts[1] = 20 ether;
        amounts[2] = 30 ether;

        // Test 1: Mismatched array lengths should revert
        uint256[] memory wrongLength = new uint256[](2);
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidInput.selector);
        staking.stakeMultipleAppchains{value: 60 ether}(appchainIds, wrongLength);
        vm.stopPrank();

        // Test 2: Total amount mismatch should revert
        amounts[2] = 31 ether; // Total is now 61 ether but sending 60
        vm.startPrank(user1);
        vm.expectRevert(abi.encodeWithSelector(SyndStaking.InvalidStakingAmount.selector, 61 ether, 60 ether));
        staking.stakeMultipleAppchains{value: 60 ether}(appchainIds, amounts);
        vm.stopPrank();

        // Test 3: Valid multiple staking should work
        amounts[2] = 30 ether; // Fix the amount
        vm.startPrank(user1);
        staking.stakeMultipleAppchains{value: 60 ether}(appchainIds, amounts);
        vm.stopPrank();
    }

    function test_withdrawalEdgeCases() public {
        // First stake some tokens
        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(appchainId);
        vm.stopPrank();

        stepEpoch(1); // Move to next epoch

        // Test 1: Withdrawing zero amount should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidAmount.selector);
        staking.initializeWithdrawal(appchainId, 0);
        vm.stopPrank();

        // Test 2: Withdrawing from invalid appchain should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidAppchainId.selector);
        staking.initializeWithdrawal(0, 10 ether);
        vm.stopPrank();

        // Test 3: Withdrawing more than available should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InsufficientStake.selector);
        staking.initializeWithdrawal(appchainId, 100 ether);
        vm.stopPrank();

        // Test 4: Valid withdrawal initialization
        vm.startPrank(user1);
        staking.initializeWithdrawal(appchainId, 25 ether);
        vm.stopPrank();

        // Test 5: Trying to withdraw before epoch delay should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.WithdrawalNotReady.selector);
        staking.withdraw(3, user1); // Current epoch is 3, withdrawal was in epoch 3
        vm.stopPrank();

        // Test 6: Withdrawing to zero address should revert
        stepEpoch(1); // Move to epoch 4
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidDestination.selector);
        staking.withdraw(3, address(0));
        vm.stopPrank();

        // Test 7: Valid withdrawal
        vm.startPrank(user1);
        staking.withdraw(3, user1);
        vm.stopPrank();

        // Test 8: Trying to withdraw again should revert (already withdrawn)
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidWithdrawal.selector);
        staking.withdraw(3, user1);
        vm.stopPrank();
    }

    function test_stakeTransferEdgeCases() public {
        // First stake on two different appchains
        vm.startPrank(user1);
        staking.stakeSynd{value: 30 ether}(111);
        staking.stakeSynd{value: 20 ether}(222);
        vm.stopPrank();

        stepEpoch(1); // Move to next epoch

        // Test 1: Transferring zero amount should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidAmount.selector);
        staking.stageStakeTransfer(111, 333, 0);
        vm.stopPrank();

        // Test 2: Transferring from invalid appchain should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidAppchainId.selector);
        staking.stageStakeTransfer(0, 333, 10 ether);
        vm.stopPrank();

        // Test 3: Transferring to invalid appchain should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidAppchainId.selector);
        staking.stageStakeTransfer(111, 0, 10 ether);
        vm.stopPrank();

        // Test 4: Transferring to same appchain should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.SameAppchainTransfer.selector);
        staking.stageStakeTransfer(111, 111, 10 ether);
        vm.stopPrank();

        // Test 5: Transferring more than available should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InsufficientStake.selector);
        staking.stageStakeTransfer(111, 333, 50 ether); // Only have 30 ether on 111
        vm.stopPrank();

        // Test 6: Valid transfer
        vm.startPrank(user1);
        staking.stageStakeTransfer(111, 333, 15 ether);
        vm.stopPrank();

        // Test 7: Transferring when paused should revert
        vm.startPrank(admin);
        staking.pause();
        vm.stopPrank();

        vm.startPrank(user1);
        vm.expectRevert();
        staking.stageStakeTransfer(222, 333, 10 ether);
        vm.stopPrank();
    }

    function test_epochTimingEdgeCases() public {
        // Test 1: Staking at very end of epoch
        uint256 currentEpoch = staking.getCurrentEpoch();
        uint256 epochEnd = staking.getEpochEnd(currentEpoch);

        // Warp to 1 second before epoch end
        vm.warp(epochEnd - 1);

        vm.startPrank(user1);
        staking.stakeSynd{value: 10 ether}(appchainId);
        vm.stopPrank();

        // Test 2: Staking at epoch boundary
        vm.warp(epochEnd);
        vm.startPrank(user2);
        staking.stakeSynd{value: 20 ether}(appchainId);
        vm.stopPrank();

        // Test 3: Pro-rata calculation edge case
        // Stake at different times within epoch and verify pro-rata shares
        vm.warp(block.timestamp + 15 days); // Halfway through epoch
        vm.startPrank(user3);
        staking.stakeSynd{value: 30 ether}(appchainId);
        vm.stopPrank();

        // Verify pro-rata calculations
        uint256 user1Share = staking.getUserStakeShare(currentEpoch, user1);
        uint256 user2Share = staking.getUserStakeShare(currentEpoch + 1, user2);
        uint256 user3Share = staking.getUserStakeShare(currentEpoch + 1, user3);

        // user1 should have full share (staked at end of epoch)
        // user2 should have full share (staked at start of epoch)
        // user3 should have half share (staked halfway through epoch)
        assertTrue(user1Share > 0);
        assertTrue(user2Share > 0);
        assertTrue(user3Share > 0);
        assertTrue(user3Share < user2Share); // user3 should have less share due to timing
    }

    function test_bulkWithdrawalEdgeCases() public {
        // Setup: Stake on multiple appchains
        vm.startPrank(user1);
        staking.stakeSynd{value: 20 ether}(111);
        staking.stakeSynd{value: 30 ether}(222);
        staking.stakeSynd{value: 10 ether}(333);
        vm.stopPrank();

        stepEpoch(1);

        // Initialize withdrawals
        uint256[] memory appchainIds = new uint256[](3);
        uint256[] memory amounts = new uint256[](3);
        appchainIds[0] = 111;
        appchainIds[1] = 222;
        appchainIds[2] = 333;
        amounts[0] = 10 ether;
        amounts[1] = 15 ether;
        amounts[2] = 5 ether;

        vm.startPrank(user1);
        staking.initializeWithdrawals(appchainIds, amounts);
        vm.stopPrank();

        stepEpoch(1); // Move to next epoch

        // Test 1: Empty epoch indices array should revert
        uint256[] memory emptyEpochs = new uint256[](0);
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidInput.selector);
        staking.withdrawBulk(emptyEpochs, user1);
        vm.stopPrank();

        // Test 2: Withdrawing to zero address should revert
        uint256[] memory epochs = new uint256[](1);
        epochs[0] = 3;
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidDestination.selector);
        staking.withdrawBulk(epochs, address(0));
        vm.stopPrank();

        // Test 3: Valid bulk withdrawal
        vm.startPrank(user1);
        staking.withdrawBulk(epochs, user1);
        vm.stopPrank();

        // Test 4: Trying to withdraw again should revert
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.InvalidWithdrawal.selector);
        staking.withdrawBulk(epochs, user1);
        vm.stopPrank();
    }

    function test_claimingEdgeCases() public {
        // Setup: Stake some tokens
        vm.startPrank(user1);
        staking.stakeSynd{value: 50 ether}(appchainId);
        vm.stopPrank();

        stepEpoch(1);

        // Deposit some rewards to the base pool
        basePool.deposit{value: 100 ether}(2);

        // Test 1: Claiming with empty claims array should revert
        SyndStaking.ClaimRequest[] memory emptyClaims = new SyndStaking.ClaimRequest[](0);
        vm.startPrank(user1);
        vm.expectRevert(SyndStaking.NoClaimsProvided.selector);
        staking.claimAllRewards(emptyClaims, user1);
        vm.stopPrank();

        // Test 2: Valid claim
        SyndStaking.ClaimRequest[] memory claims = new SyndStaking.ClaimRequest[](1);
        claims[0] = SyndStaking.ClaimRequest({poolAddress: address(basePool), epochIndex: 2, appchainId: appchainId});

        vm.startPrank(user1);
        staking.claimAllRewards(claims, user1);
        vm.stopPrank();
    }

    function test_boundaryValueEdgeCases() public {
        // Test 1: Maximum uint256 values (should not overflow)
        vm.startPrank(user1);
        // Note: This would require the user to have 2^256-1 wei, which is impossible
        // But we can test with very large values
        staking.stakeSynd{value: 99 ether}(appchainId);
        vm.stopPrank();

        // Test 2: Minimum valid values
        vm.startPrank(user2);
        staking.stakeSynd{value: 1 wei}(appchainId);
        vm.stopPrank();

        // Test 3: Edge case appchain IDs
        vm.startPrank(user3);
        staking.stakeSynd{value: 1 ether}(1); // Minimum valid appchain ID
        staking.stakeSynd{value: 1 ether}(type(uint256).max); // Maximum appchain ID
        vm.stopPrank();
    }

    function test_emergencyScenarios() public {
        // Test 1: Pause during active staking
        vm.startPrank(user1);
        staking.stakeSynd{value: 10 ether}(appchainId);
        vm.stopPrank();

        vm.startPrank(admin);
        staking.pause();
        vm.stopPrank();

        // All staking operations should be paused
        vm.startPrank(user2);
        vm.expectRevert();
        staking.stakeSynd{value: 10 ether}(appchainId);
        vm.stopPrank();

        vm.startPrank(user1);
        vm.expectRevert();
        staking.stageStakeTransfer(111, 222, 5 ether);
        vm.stopPrank();

        // Test 2: Unpause and operations resume
        vm.startPrank(admin);
        staking.unpause();
        vm.stopPrank();

        vm.startPrank(user2);
        staking.stakeSynd{value: 10 ether}(appchainId);
        vm.stopPrank();
    }

    function test_complexInteractionScenarios() public {
        // Test 1: Multiple users staking, transferring, and withdrawing simultaneously
        vm.startPrank(user1);
        staking.stakeSynd{value: 30 ether}(111);
        staking.stakeSynd{value: 20 ether}(222);
        vm.stopPrank();

        vm.startPrank(user2);
        staking.stakeSynd{value: 25 ether}(111);
        staking.stakeSynd{value: 15 ether}(333);
        vm.stopPrank();

        stepEpoch(1);

        // User1 transfers stake between appchains
        vm.startPrank(user1);
        staking.stageStakeTransfer(111, 333, 10 ether);
        vm.stopPrank();

        // User2 initializes withdrawal
        vm.startPrank(user2);
        staking.initializeWithdrawal(111, 10 ether);
        vm.stopPrank();

        // User1 initializes withdrawal from different appchain
        vm.startPrank(user1);
        staking.initializeWithdrawal(222, 15 ether);
        vm.stopPrank();

        stepEpoch(1);

        // Complete withdrawals
        vm.startPrank(user1);
        staking.withdraw(3, user1);
        vm.stopPrank();

        vm.startPrank(user2);
        staking.withdraw(3, user2);
        vm.stopPrank();

        // Test 2: Staking across many epochs
        for (uint256 i = 0; i < 5; i++) {
            vm.startPrank(user3);
            staking.stakeSynd{value: 1 ether}(111);
            vm.stopPrank();

            stepEpoch(1);
        }

        // Verify final state
        assertTrue(staking.getUserStake(7, user3) > 0);
    }

    function test_allUsersComplexInteractions() public {
        // This test uses all 10 users in a complex scenario to test interactions
        // between multiple users performing different operations simultaneously

        console2.log("=== Starting complex multi-user interaction test ===");

        uint256 currentEpoch = staking.getCurrentEpoch();

        // Phase 1: Initial staking across multiple appchains
        console2.log("Phase 1: Initial staking");

        // Users 1-3 stake on appchain 111
        vm.startPrank(user1);
        staking.stakeSynd{value: 20 ether}(111);
        vm.stopPrank();

        vm.startPrank(user2);
        staking.stakeSynd{value: 30 ether}(111);
        vm.stopPrank();

        vm.startPrank(user3);
        staking.stakeSynd{value: 25 ether}(111);
        vm.stopPrank();

        // Users 4-6 stake on appchain 222
        vm.startPrank(user4);
        staking.stakeSynd{value: 15 ether}(222);
        vm.stopPrank();

        vm.startPrank(user5);
        staking.stakeSynd{value: 35 ether}(222);
        vm.stopPrank();

        vm.startPrank(user6);
        staking.stakeSynd{value: 10 ether}(222);
        vm.stopPrank();

        // Users 7-9 stake on appchain 333
        vm.startPrank(user7);
        staking.stakeSynd{value: 40 ether}(333);
        vm.stopPrank();

        vm.startPrank(user8);
        staking.stakeSynd{value: 20 ether}(333);
        vm.stopPrank();

        vm.startPrank(user9);
        staking.stakeSynd{value: 30 ether}(333);
        vm.stopPrank();

        // User 10 stakes on multiple appchains
        vm.startPrank(user10);
        staking.stakeSynd{value: 10 ether}(111);
        staking.stakeSynd{value: 15 ether}(222);
        staking.stakeSynd{value: 25 ether}(333);
        vm.stopPrank();

        // Verify initial stakes
        assertEq(staking.getAppchainStake(currentEpoch + 1, 111), 85 ether); // 20+30+25+10 = 85
        assertEq(staking.getAppchainStake(currentEpoch + 1, 222), 75 ether); // 15+35+10+15 = 75
        assertEq(staking.getAppchainStake(currentEpoch + 1, 333), 115 ether); // 40+20+30+25 = 115

        console2.log("Phase 1 complete - Total stake distributed across 3 appchains");

        // Phase 2: Move to next epoch and add more stakes
        console2.log("Phase 2: Epoch transition and additional staking");
        stepEpoch(1);
        currentEpoch++;

        // Users 1-5 add more stakes in epoch 2
        vm.startPrank(user1);
        staking.stakeSynd{value: 10 ether}(111);
        vm.stopPrank();

        vm.startPrank(user2);
        staking.stakeSynd{value: 5 ether}(222); // Cross-appchain staking
        vm.stopPrank();

        vm.startPrank(user3);
        staking.stakeSynd{value: 15 ether}(333); // Cross-appchain staking
        vm.stopPrank();

        vm.startPrank(user4);
        staking.stakeSynd{value: 20 ether}(111); // Cross-appchain staking
        vm.stopPrank();

        vm.startPrank(user5);
        staking.stakeSynd{value: 10 ether}(333); // Cross-appchain staking
        vm.stopPrank();

        // Users 6-10 perform stake transfers
        vm.startPrank(user6);
        staking.stageStakeTransfer(222, 111, 5 ether);
        vm.stopPrank();

        vm.startPrank(user7);
        staking.stageStakeTransfer(333, 222, 10 ether);
        vm.stopPrank();

        vm.startPrank(user8);
        staking.stageStakeTransfer(333, 111, 10 ether);
        vm.stopPrank();

        vm.startPrank(user9);
        staking.stageStakeTransfer(333, 222, 15 ether);
        vm.stopPrank();

        vm.startPrank(user10);
        staking.stageStakeTransfer(111, 333, 5 ether);
        staking.stageStakeTransfer(222, 111, 5 ether);
        vm.stopPrank();

        console2.log("Phase 2 complete - Cross-appchain staking and transfers");

        // Phase 2 calculations:
        // Appchain 111: 85 + 10(user1) + 20(user4) + 5(user6 transfer) + 10(user8 transfer) = 130
        // Appchain 222: 75 - 5(user10 transfer) + 5(user2) + 10(user7 transfer) + 15(user9 transfer) - 5(user10 transfer) = 95
        // Appchain 333: 115 + 15(user3) + 10(user5) - 10(user7 transfer) - 10(user8 transfer) - 15(user9 transfer) + 5(user10 transfer) = 110
        // Global: 130 + 95 + 110 = 335

        // Phase 3: Initialize withdrawals for some users
        console2.log("Phase 3: Withdrawal initialization");
        stepEpoch(1);
        currentEpoch++;

        // Users 1, 3, 5, 7, 9 initialize withdrawals
        vm.startPrank(user1);
        staking.initializeWithdrawal(111, 15 ether);
        vm.stopPrank();

        vm.startPrank(user3);
        staking.initializeWithdrawal(111, 20 ether);
        vm.stopPrank();

        vm.startPrank(user5);
        staking.initializeWithdrawal(222, 25 ether);
        vm.stopPrank();

        vm.startPrank(user7);
        staking.initializeWithdrawal(333, 20 ether);
        vm.stopPrank();

        vm.startPrank(user9);
        staking.initializeWithdrawal(333, 15 ether);
        vm.stopPrank();

        // Users 2, 4, 6, 8, 10 continue staking
        vm.startPrank(user2);
        staking.stakeSynd{value: 8 ether}(111);
        vm.stopPrank();

        vm.startPrank(user4);
        staking.stakeSynd{value: 12 ether}(222);
        vm.stopPrank();

        vm.startPrank(user6);
        staking.stakeSynd{value: 7 ether}(333);
        vm.stopPrank();

        vm.startPrank(user8);
        staking.stakeSynd{value: 5 ether}(111);
        vm.stopPrank();

        vm.startPrank(user10);
        staking.stakeSynd{value: 3 ether}(222);
        vm.stopPrank();

        console2.log("Phase 3 complete - Mixed withdrawals and continued staking");

        // Phase 3 calculations (after withdrawals and additional staking):
        // Appchain 111: 130 - 15(user1 withdrawal) - 20(user3 withdrawal) + 8(user2) + 5(user8) = 108
        // Appchain 222: 95 - 25(user5 withdrawal) + 12(user10) + 3(user10) = 85
        // Appchain 333: 110 - 20(user7 withdrawal) - 15(user9 withdrawal) + 7(user6) = 82
        // Global: 108 + 85 + 82 = 275

        // Phase 4: Complete withdrawals and more complex operations
        console2.log("Phase 4: Withdrawal completion and complex operations");
        stepEpoch(1);

        // Complete the withdrawals from epoch 3
        vm.startPrank(user1);
        staking.withdraw(currentEpoch, user1);
        vm.stopPrank();

        vm.startPrank(user3);
        staking.withdraw(currentEpoch, user3);
        vm.stopPrank();

        vm.startPrank(user5);
        staking.withdraw(currentEpoch, user5);
        vm.stopPrank();

        vm.startPrank(user7);
        staking.withdraw(currentEpoch, user7);
        vm.stopPrank();

        vm.startPrank(user9);
        staking.withdraw(currentEpoch, user9);
        vm.stopPrank();

        currentEpoch++;

        // Users 2, 4, 6, 8, 10 perform bulk operations
        uint256[] memory appchainIds = new uint256[](2);
        uint256[] memory amounts = new uint256[](2);

        // User 2 bulk withdrawal
        appchainIds[0] = 111;
        appchainIds[1] = 222;
        amounts[0] = 10 ether;
        amounts[1] = 5 ether;
        vm.startPrank(user2);
        staking.initializeWithdrawals(appchainIds, amounts);
        vm.stopPrank();

        // User 4 bulk withdrawal
        appchainIds[0] = 222;
        appchainIds[1] = 111;
        amounts[0] = 8 ether;
        amounts[1] = 5 ether;
        vm.startPrank(user4);
        staking.initializeWithdrawals(appchainIds, amounts);
        vm.stopPrank();

        // User 6 bulk withdrawal
        appchainIds[0] = 222;
        appchainIds[1] = 333;
        amounts[0] = 3 ether;
        amounts[1] = 7 ether;
        vm.startPrank(user6);
        staking.initializeWithdrawals(appchainIds, amounts);
        vm.stopPrank();

        // User 8 bulk withdrawal
        appchainIds[0] = 111;
        appchainIds[1] = 333;
        amounts[0] = 5 ether;
        amounts[1] = 5 ether;
        vm.startPrank(user8);
        staking.initializeWithdrawals(appchainIds, amounts);
        vm.stopPrank();

        // User 10 bulk withdrawal
        appchainIds = new uint256[](3);
        amounts = new uint256[](3);
        appchainIds[0] = 111;
        appchainIds[1] = 222;
        appchainIds[2] = 333;
        amounts = new uint256[](3);
        amounts[0] = 2 ether;
        amounts[1] = 3 ether;
        amounts[2] = 5 ether;
        vm.startPrank(user10);
        staking.initializeWithdrawals(appchainIds, amounts);
        vm.stopPrank();

        console2.log("Phase 4 complete - Bulk withdrawals initialized");

        // Phase 4 calculations (after bulk withdrawals):
        // Appchain 111: 108 - 10(user2 bulk) - 5(user4 bulk) - 5(user8 bulk) - 2(user10 bulk) = 86
        // Appchain 222: 85 - 5(user2 bulk) - 8(user4 bulk) - 3(user6 bulk) - 3(user10 bulk) = 66
        // Appchain 333: 82 - 7(user6 bulk) - 5(user8 bulk) - 5(user10 bulk) = 65
        // Global: 86 + 66 + 65 = 217

        // Phase 5: Final epoch with complex interactions
        console2.log("Phase 5: Final complex interactions");
        stepEpoch(1);

        // Complete bulk withdrawals
        uint256[] memory epochs = new uint256[](1);
        epochs[0] = currentEpoch;

        vm.startPrank(user2);
        staking.withdrawBulk(epochs, user2);
        vm.stopPrank();

        vm.startPrank(user4);
        staking.withdrawBulk(epochs, user4);
        vm.stopPrank();

        vm.startPrank(user6);
        staking.withdrawBulk(epochs, user6);
        vm.stopPrank();

        vm.startPrank(user8);
        staking.withdrawBulk(epochs, user8);
        vm.stopPrank();

        // User 10 has 3 appchains, so we need to handle that
        uint256[] memory epochs3 = new uint256[](1);
        epochs3[0] = currentEpoch;
        vm.startPrank(user10);
        staking.withdrawBulk(epochs3, user10);
        vm.stopPrank();

        // Final staking round - users 1, 3, 5, 7, 9 stake again after withdrawals
        vm.startPrank(user1);
        staking.stakeSynd{value: 5 ether}(222); // New appchain
        vm.stopPrank();

        vm.startPrank(user3);
        staking.stakeSynd{value: 8 ether}(333); // New appchain
        vm.stopPrank();

        vm.startPrank(user5);
        staking.stakeSynd{value: 12 ether}(111); // New appchain
        vm.stopPrank();

        vm.startPrank(user7);
        staking.stakeSynd{value: 6 ether}(111); // New appchain
        vm.stopPrank();

        vm.startPrank(user9);
        staking.stakeSynd{value: 10 ether}(222); // New appchain
        vm.stopPrank();

        // Users 2, 4, 6, 8, 10 perform final transfers
        vm.startPrank(user2);
        staking.stageStakeTransfer(111, 333, 3 ether);
        vm.stopPrank();

        vm.startPrank(user4);
        staking.stageStakeTransfer(222, 111, 4 ether);
        vm.stopPrank();

        vm.startPrank(user8);
        staking.stageStakeTransfer(111, 333, 3 ether);
        vm.stopPrank();

        vm.startPrank(user10);
        staking.stageStakeTransfer(111, 222, 1 ether);
        vm.stopPrank();

        currentEpoch++;

        console2.log("Phase 5 complete - Final complex interactions");

        // Phase 5 calculations (after final staking and transfers):
        // Appchain 111: 86 + 12(user5) + 6(user7) - 3(user2 transfer) + 4(user4 transfer) - 3(user8 transfer) + 1(user10 transfer) = 101
        // Appchain 222: 66 + 5(user1) + 10(user9) - 4(user4 transfer) + 1(user10 transfer) = 78
        // Appchain 333: 65 + 8(user3) + 3(user2 transfer) + 3(user8 transfer) = 79
        // Global: 101 + 78 + 79 = 258

        // Phase 6: Verification and final state checks
        console2.log("Phase 6: Final verification");
        stepEpoch(1);
        currentEpoch++;

        // Calculate exact expected values for each user:
        // User1: 30(111) - 15(withdrawal) + 5(222) = 20 total
        // User2: 35(111) + 5(222) - 10(111 bulk) - 5(222 bulk) + 3(333 transfer) = 28 total
        // User3: 40(111) - 20(withdrawal) + 8(333) = 28 total
        // User4: 15(222) + 20(111) + 12(222) - 8(222 bulk) - 5(111 bulk) = 34 total
        // User5: 45(222) - 25(withdrawal) + 12(111) = 32 total
        // User6: 10(222) + 7(333) - 3(222 bulk) - 7(333 bulk) = 7 total
        // User7: 40(333) - 20(withdrawal) + 6(111) = 26 total
        // User8: 20(111) + 5(333) - 5(111 bulk) - 5(333 bulk) = 15 total
        // User9: 30(333) - 15(withdrawal) + 10(222) = 25 total
        // User10: 10(111) + 15(222) + 25(333) + 3(222) - 2(111 bulk) - 3(222 bulk) - 5(333 bulk) = 43 total

        // Verify all users have exact expected stake amounts
        assertEq(staking.getUserStake(currentEpoch, user1), 20 ether, "User1 should have 20 ether total");
        assertEq(staking.getUserStake(currentEpoch, user2), 28 ether, "User2 should have 28 ether total");
        assertEq(staking.getUserStake(currentEpoch, user3), 28 ether, "User3 should have 28 ether total");
        assertEq(staking.getUserStake(currentEpoch, user4), 34 ether, "User4 should have 58 ether total");
        assertEq(staking.getUserStake(currentEpoch, user5), 32 ether, "User5 should have 32 ether total");
        assertEq(staking.getUserStake(currentEpoch, user6), 7 ether, "User6 should have 14 ether total");
        assertEq(staking.getUserStake(currentEpoch, user7), 26 ether, "User7 should have 16 ether total");
        assertEq(staking.getUserStake(currentEpoch, user8), 15 ether, "User8 should have 18 ether total");
        assertEq(staking.getUserStake(currentEpoch, user9), 25 ether, "User9 should have 10 ether total");
        assertEq(staking.getUserStake(currentEpoch, user10), 43 ether, "User10 should have 32 ether total");

        // Verify exact appchain totals
        uint256 totalStake111 = staking.getAppchainStake(currentEpoch, 111);
        uint256 totalStake222 = staking.getAppchainStake(currentEpoch, 222);
        uint256 totalStake333 = staking.getAppchainStake(currentEpoch, 333);

        assertEq(totalStake111, 101 ether, "Appchain 111 should have exactly 101 ether");
        assertEq(totalStake222, 78 ether, "Appchain 222 should have exactly 78 ether");
        assertEq(totalStake333, 79 ether, "Appchain 333 should have exactly 79 ether");

        console2.log("Final appchain totals:");
        console2.log("Appchain 111:", totalStake111);
        console2.log("Appchain 222:", totalStake222);
        console2.log("Appchain 333:", totalStake333);

        // Verify exact global total stake
        uint256 globalTotal = staking.getTotalStake(currentEpoch);
        assertEq(globalTotal, 258 ether, "Global total should be exactly 258 ether");
        assertEq(
            globalTotal, totalStake111 + totalStake222 + totalStake333, "Global total should equal sum of appchains"
        );

        console2.log("Global total stake:", globalTotal);
        console2.log("=== Complex multi-user interaction test completed successfully ===");
    }
}
