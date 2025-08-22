// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {Forwarder} from "../../src/staking/Forwarder.sol";
import {BasePool} from "../../src/staking/BasePool.sol";
import {SyndStaking} from "../../src/staking/SyndStaking.sol";
import {Splitter} from "../../src/staking/Splitter.sol";

/**
 * @title ForwarderTest
 * @notice Test contract for the Forwarder functionality
 */
contract ForwarderTest is Test {
    Forwarder public forwarder;
    BasePool public basePool;
    SyndStaking public stakingContract;
    Splitter public splitter;

    address public user = address(0x1);

    uint256 public constant EPOCH_DURATION = 30 days;
    uint256 public constant START_TIMESTAMP = 1700000000;

    function setUp() public {
        // Deploy contracts
        stakingContract = new SyndStaking(START_TIMESTAMP);
        basePool = new BasePool(address(stakingContract));
        forwarder = new Forwarder(address(stakingContract));

        // Set up forwarder in other contracts
        basePool.setForwarder(address(forwarder));
        stakingContract.setForwarder(address(forwarder));

        // Fund user with ETH
        vm.deal(user, 100 ether);
    }

    function test_ClaimAllRewards() public {
        // Setup: Add rewards to pool
        uint256 epochIndex = 1;
        basePool.deposit{value: 10 ether}(epochIndex);

        // Setup: User stakes in the staking contract
        vm.startPrank(user);
        stakingContract.stakeSynd{value: 5 ether}(1); // appchainId = 1
        vm.stopPrank();

        // Move to next epoch so rewards can be claimed
        vm.warp(START_TIMESTAMP + EPOCH_DURATION + 1);

        // Create claim requests
        Forwarder.ClaimRequest[] memory claims = new Forwarder.ClaimRequest[](1);
        claims[0] = Forwarder.ClaimRequest({
            epochIndex: epochIndex,
            poolAddress: address(basePool),
            destination: user
        });

        // Call claimAllRewards directly from user
        vm.prank(user);
        (uint256 totalClaimed, uint256 claimCount) = forwarder.claimAllRewards(claims);

        assertGt(totalClaimed, 0, "Should have claimed rewards");
        assertEq(claimCount, 1, "Should have made 1 claim");
    }

    function test_RestakeAllRewards() public {
        // Setup: User has ETH to restake
        uint256 restakeAmount = 2 ether;

        // Create restake requests
        Forwarder.RestakeRequest[] memory restakes = new Forwarder.RestakeRequest[](1);
        restakes[0] = Forwarder.RestakeRequest({
            fromAppchainId: 0, // Direct ETH restake
            toAppchainId: 2,   // Restake to appchain 2
            amount: restakeAmount
        });

        // Call restakeAllRewards directly from user
        vm.prank(user);
        (uint256 totalRestaked, uint256 restakeCount) = forwarder.restakeAllRewards{value: restakeAmount}(restakes);

        assertEq(totalRestaked, restakeAmount, "Should have restaked the full amount");
        assertEq(restakeCount, 1, "Should have made 1 restake");
    }

    function test_ClaimAndRestake() public {
        // Setup: Add rewards to pool
        uint256 epochIndex = 1;
        basePool.deposit{value: 10 ether}(epochIndex);

        // Setup: User stakes in the staking contract
        vm.startPrank(user);
        stakingContract.stakeSynd{value: 5 ether}(1); // appchainId = 1
        vm.stopPrank();

        // Move to next epoch so rewards can be claimed
        vm.warp(START_TIMESTAMP + EPOCH_DURATION + 1);

        // Create claim requests
        Forwarder.ClaimRequest[] memory claims = new Forwarder.ClaimRequest[](1);
        claims[0] = Forwarder.ClaimRequest({
            epochIndex: epochIndex,
            poolAddress: address(basePool),
            destination: address(forwarder) // Send to forwarder for restaking
        });

        // Create restake requests
        Forwarder.RestakeRequest[] memory restakes = new Forwarder.RestakeRequest[](1);
        restakes[0] = Forwarder.RestakeRequest({
            fromAppchainId: 0, // Direct ETH restake
            toAppchainId: 2,   // Restake to appchain 2
            amount: 0 // Will be determined by claimed amount
        });

        // Call claimAndRestake directly from user
        vm.prank(user);
        (uint256 totalClaimed, uint256 totalRestaked) = forwarder.claimAndRestake{value: 0}(claims, restakes);

        assertGt(totalClaimed, 0, "Should have claimed rewards");
        assertGt(totalRestaked, 0, "Should have restaked rewards");
    }

    function test_RevertNoClaimsProvided() public {
        Forwarder.ClaimRequest[] memory claims = new Forwarder.ClaimRequest[](0);

        vm.prank(user);
        vm.expectRevert(Forwarder.NoClaimsProvided.selector);
        forwarder.claimAllRewards(claims);
    }

    function test_RevertNoRestakesProvided() public {
        Forwarder.RestakeRequest[] memory restakes = new Forwarder.RestakeRequest[](0);

        vm.prank(user);
        vm.expectRevert(Forwarder.NoRestakesProvided.selector);
        forwarder.restakeAllRewards(restakes);
    }

    function test_RevertUnauthorizedForwarder() public {
        // Try to call claimFor directly on BasePool (should fail)
        vm.prank(user);
        vm.expectRevert(BasePool.UnauthorizedForwarder.selector);
        basePool.claimFor(1, user);

        // Try to call stakeSyndFor directly on SyndStaking (should fail)
        vm.prank(user);
        vm.expectRevert(SyndStaking.UnauthorizedForwarder.selector);
        stakingContract.stakeSyndFor{value: 1 ether}(user, 1);
    }

    function test_MultipleClaims() public {
        // Setup: Add rewards to multiple pools
        uint256 epochIndex = 1;
        basePool.deposit{value: 10 ether}(epochIndex);

        // Setup: User stakes in the staking contract
        vm.startPrank(user);
        stakingContract.stakeSynd{value: 5 ether}(1); // appchainId = 1
        vm.stopPrank();

        // Move to next epoch so rewards can be claimed
        vm.warp(START_TIMESTAMP + EPOCH_DURATION + 1);

        // Create multiple claim requests
        Forwarder.ClaimRequest[] memory claims = new Forwarder.ClaimRequest[](2);
        claims[0] = Forwarder.ClaimRequest({
            epochIndex: epochIndex,
            poolAddress: address(basePool),
            destination: user
        });
        claims[1] = Forwarder.ClaimRequest({
            epochIndex: epochIndex,
            poolAddress: address(basePool), // Same pool, different destination
            destination: address(0x999)
        });

        // Call claimAllRewards
        vm.prank(user);
        (uint256 totalClaimed, uint256 claimCount) = forwarder.claimAllRewards(claims);

        assertGt(totalClaimed, 0, "Should have claimed rewards");
        assertEq(claimCount, 2, "Should have made 2 claims");
    }

    function test_MultipleRestakes() public {
        // Setup: User has ETH to restake
        uint256 restakeAmount1 = 1 ether;
        uint256 restakeAmount2 = 0.5 ether;

        // Create multiple restake requests
        Forwarder.RestakeRequest[] memory restakes = new Forwarder.RestakeRequest[](2);
        restakes[0] = Forwarder.RestakeRequest({
            fromAppchainId: 0, // Direct ETH restake
            toAppchainId: 1,   // Restake to appchain 1
            amount: restakeAmount1
        });
        restakes[1] = Forwarder.RestakeRequest({
            fromAppchainId: 0, // Direct ETH restake
            toAppchainId: 2,   // Restake to appchain 2
            amount: restakeAmount2
        });

        // Call restakeAllRewards
        vm.prank(user);
        (uint256 totalRestaked, uint256 restakeCount) = forwarder.restakeAllRewards{value: restakeAmount1 + restakeAmount2}(restakes);

        assertEq(totalRestaked, restakeAmount1 + restakeAmount2, "Should have restaked the full amount");
        assertEq(restakeCount, 2, "Should have made 2 restakes");
    }
}
