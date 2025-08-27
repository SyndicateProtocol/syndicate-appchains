// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {EmissionsCalculator} from "src/token/emissions/EmissionsCalculator.sol";
import {EmissionsScheduler} from "src/token/emissions/EmissionsScheduler.sol";
import {IRelayer} from "src/token/emissions/EmissionsScheduler.sol";

contract Relayer is IRelayer {
    function relay(uint256 amount, address destination, uint256 epochIndex) external override {}
}

contract EmissionsSchedulerTest is Test {
    SyndicateToken public token;
    EmissionsCalculator public emissionsCalculator;
    EmissionsScheduler public emissionScheduler;
    Relayer public relayer;

    address public defaultAdmin = address(0x1234);
    address public relayDestination = address(0x5678);
    address public pauser = address(0xDEF0);
    address public user = address(0x1111);

    uint256 public constant DEFAULT_DECAY_FACTOR = 0.95e18; // 95% decay

    event EmissionMinted(uint256 epoch, uint256 amount);
    event Paused(address account);
    event Unpaused(address account);

    function setUp() public {
        vm.startPrank(defaultAdmin);

        // Deploy token
        token = new SyndicateToken(defaultAdmin, defaultAdmin);

        // Deploy emissions calculator
        emissionsCalculator = new EmissionsCalculator(address(token), defaultAdmin, defaultAdmin);

        // Deploy relayer
        relayer = new Relayer();

        // Deploy emission scheduler V2
        emissionScheduler = new EmissionsScheduler(
            address(emissionsCalculator), address(relayer), relayDestination, defaultAdmin, pauser
        );
        vm.warp(emissionScheduler.START_TIMESTAMP());

        // Grant necessary roles
        token.grantRole(token.EMISSION_MINTER_ROLE(), address(emissionsCalculator));
        emissionsCalculator.grantRole(emissionsCalculator.EMISSIONS_ROLE(), address(emissionScheduler));

        // Initialize emissions calculator
        emissionsCalculator.initializeEmissions(DEFAULT_DECAY_FACTOR);

        vm.stopPrank();
    }

    // ============ CONSTRUCTOR TESTS ============

    function test_Constructor_InitialSetup() public view {
        assertEq(address(emissionScheduler.emissionsCalculator()), address(emissionsCalculator));
        assertEq(address(emissionScheduler.relayer()), address(relayer));
        assertEq(emissionScheduler.relayDestination(), relayDestination);
        assertEq(emissionScheduler.epochStartIndex(), 2);
        assertEq(emissionScheduler.getCurrentEpoch(), 1);
        assertEq(emissionScheduler.totalEmissionsMinted(), 0);
        assertFalse(emissionScheduler.epochMinted(1));
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(emissionScheduler.hasRole(emissionScheduler.DEFAULT_ADMIN_ROLE(), defaultAdmin));
        assertTrue(emissionScheduler.hasRole(emissionScheduler.PAUSER_ROLE(), pauser));
    }

    function test_RevertWhen_Constructor_ZeroEmissionsCalculator() public {
        vm.expectRevert(EmissionsScheduler.ZeroAddress.selector);
        new EmissionsScheduler(address(0), address(relayer), relayDestination, defaultAdmin, pauser);
    }

    function test_RevertWhen_Constructor_ZeroAdmin() public {
        vm.expectRevert(EmissionsScheduler.ZeroAddress.selector);
        new EmissionsScheduler(address(emissionsCalculator), address(relayer), relayDestination, address(0), pauser);
    }

    function test_RevertWhen_Constructor_ZeroPauser() public {
        vm.expectRevert(EmissionsScheduler.ZeroAddress.selector);
        new EmissionsScheduler(
            address(emissionsCalculator), address(relayer), relayDestination, defaultAdmin, address(0)
        );
    }

    // ============ MINT EMISSION TESTS ============

    function test_MintEmission_FirstEpoch() public {
        vm.warp(emissionScheduler.getEpochStart(2));

        uint256 expectedAmount = emissionsCalculator.getNextEmission();
        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(false, false, false, true);
        emit EmissionMinted(2, expectedAmount);
        emissionScheduler.mintEmission(2);

        assertEq(emissionScheduler.getCurrentEpoch(), 2);
        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount);
        assertEq(token.totalSupply(), initialSupply + expectedAmount);
        assertTrue(emissionScheduler.epochMinted(2));
    }

    function test_MintEmission_MultipleEpochs() public {
        uint256 totalMinted = 0;
        uint256 expectedEpochs = 5;

        for (uint256 i = 2; i <= expectedEpochs; i++) {
            // Move to epoch i
            vm.warp(emissionScheduler.getEpochStart(i));

            uint256 expectedAmount = emissionsCalculator.getNextEmission();

            emissionScheduler.mintEmission(i);

            totalMinted += expectedAmount;
            assertTrue(emissionScheduler.epochMinted(i));
        }

        assertEq(emissionScheduler.getCurrentEpoch(), expectedEpochs);
        assertEq(emissionScheduler.totalEmissionsMinted(), totalMinted);
    }

    function test_MintEmission_NonSequentialEpochs() public {
        // Skip to epoch 5 - this should fail because calculator epoch and time epoch don't match
        vm.warp(emissionScheduler.getEpochStart(5));

        // This should fail because currentTimeEpoch (5) != calculatorEpoch (0)
        vm.expectRevert(EmissionsScheduler.InvalidEpoch.selector);
        emissionScheduler.mintEmission(5);

        // Verify epochs haven't been minted
        assertFalse(emissionScheduler.epochMinted(0));
        assertFalse(emissionScheduler.epochMinted(4));
        assertFalse(emissionScheduler.epochMinted(5));

        // Make sure catch up works
        emissionScheduler.mintEmission(2);
        emissionScheduler.mintEmission(3);
        emissionScheduler.mintEmission(4);
        emissionScheduler.mintEmission(5);
    }

    function test_RevertWhen_EpochBeforeStart() public {
        vm.prank(defaultAdmin);
        vm.expectRevert(EmissionsScheduler.InvalidEpoch.selector);
        emissionScheduler.mintEmission(1);
    }

    function test_RevertWhen_MintEmission_EpochAlreadyMinted() public {
        vm.warp(emissionScheduler.getEpochStart(2));

        // Mint epoch 2
        emissionScheduler.mintEmission(2);
        assertTrue(emissionScheduler.epochMinted(2));

        // Try to mint epoch 2 again
        vm.expectRevert(EmissionsScheduler.EpochAlreadyMinted.selector);
        emissionScheduler.mintEmission(2);
    }

    function test_RevertWhen_MintEmission_Paused() public {
        vm.warp(emissionScheduler.getEpochStart(2));

        vm.prank(pauser);
        emissionScheduler.pause();

        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        emissionScheduler.mintEmission(2);
    }

    function test_RevertWhen_MintEmission_AllEmissionsCompleted() public {
        // Complete all 48 epochs
        for (uint256 i = 2; i < 50; i++) {
            // Move to epoch i
            vm.warp(emissionScheduler.getEpochStart(i));

            emissionScheduler.mintEmission(i);
        }

        assertTrue(emissionScheduler.emissionsEnded());

        vm.expectRevert(EmissionsScheduler.AllEmissionsCompleted.selector);
        emissionScheduler.mintEmission(50);
    }

    // ============ VIEW FUNCTION TESTS ============
    function test_EmissionsStarted() public {
        assertFalse(emissionScheduler.emissionsStarted());

        vm.warp(emissionScheduler.getEpochStart(2));

        assertTrue(emissionScheduler.emissionsStarted());
    }

    function test_TotalEmissionsMinted() public {
        vm.warp(emissionScheduler.getEpochStart(2));

        assertEq(emissionScheduler.totalEmissionsMinted(), 0);

        uint256 expectedAmount = emissionsCalculator.getNextEmission();

        emissionScheduler.mintEmission(2);

        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount);
    }

    // ============ PAUSE FUNCTIONALITY TESTS ============

    function test_Pause_Success() public {
        vm.expectEmit(false, false, false, true);
        emit Paused(pauser);

        vm.prank(pauser);
        emissionScheduler.pause();

        assertTrue(emissionScheduler.paused());
    }

    function test_Unpause_Success() public {
        vm.prank(pauser);
        emissionScheduler.pause();

        vm.expectEmit(false, false, false, true);
        emit Unpaused(defaultAdmin);

        vm.prank(defaultAdmin);
        emissionScheduler.unpause();

        assertFalse(emissionScheduler.paused());
    }

    function test_RevertWhen_Pause_NotPauser() public {
        vm.expectRevert();
        vm.prank(user);
        emissionScheduler.pause();
    }

    function test_RevertWhen_Unpause_NotAdmin() public {
        vm.prank(pauser);
        emissionScheduler.pause();

        vm.expectRevert();
        vm.prank(user);
        emissionScheduler.unpause();
    }

    function test_PauseUnpause_Integration() public {
        vm.warp(emissionScheduler.getEpochStart(2));

        // Pause and verify mint fails
        vm.prank(pauser);
        emissionScheduler.pause();

        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        emissionScheduler.mintEmission(2);

        // Unpause and verify mint works
        vm.prank(defaultAdmin);
        emissionScheduler.unpause();

        emissionScheduler.mintEmission(2); // Should succeed

        assertEq(emissionScheduler.getCurrentEpoch(), 2);
    }

    // ============ INTEGRATION TESTS ============

    function test_Integration_FullEmissionCycle() public {
        vm.warp(emissionScheduler.getEpochStart(2));

        // Mint several epochs
        uint256 totalMinted = 0;
        for (uint256 i = 2; i <= 7; i++) {
            // Move to epoch i
            vm.warp(emissionScheduler.getEpochStart(i));

            uint256 expectedAmount = emissionsCalculator.getNextEmission();

            emissionScheduler.mintEmission(i);

            totalMinted += expectedAmount;
            assertTrue(emissionScheduler.epochMinted(i));
        }

        assertEq(emissionScheduler.getCurrentEpoch(), 7);
        assertEq(emissionScheduler.totalEmissionsMinted(), totalMinted);
        assertFalse(emissionScheduler.emissionsEnded());
    }

    function test_Integration_LastEpoch() public {
        vm.warp(emissionScheduler.getEpochStart(2));

        // Complete 47 epochs
        for (uint256 i = 2; i <= 48; i++) {
            vm.warp(emissionScheduler.getEpochStart(i));
            emissionScheduler.mintEmission(i);
        }

        assertEq(emissionScheduler.getCurrentEpoch(), 48);
        assertFalse(emissionScheduler.emissionsEnded());

        // Final epoch
        vm.warp(emissionScheduler.getEpochStart(49));
        emissionScheduler.mintEmission(49);

        assertEq(emissionScheduler.getCurrentEpoch(), 49);
        assertTrue(emissionScheduler.emissionsEnded());
        assertTrue(emissionScheduler.epochMinted(49));
    }

    // ============ SECURITY TESTS ============

    function test_Security_OnlyAuthorizedRolesCanCallCriticalFunctions() public {
        // Test all critical functions require proper roles
        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.setRelayDestination(address(0x1234));

        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.pause();

        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.unpause();
    }

    // ============ EDGE CASE TESTS ============

    function test_EdgeCase_EpochBoundary() public {
        // Right at epoch boundary
        vm.warp(emissionScheduler.getEpochStart(2));
        assertEq(emissionScheduler.getCurrentEpoch(), 2);

        // One second before epoch boundary
        vm.warp(emissionScheduler.getEpochStart(2) + 30 days - 1);
        assertEq(emissionScheduler.getCurrentEpoch(), 2);
    }
}
