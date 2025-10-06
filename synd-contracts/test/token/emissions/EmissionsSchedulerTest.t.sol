// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {EmissionsCalculator} from "src/token/emissions/EmissionsCalculator.sol";
import {EmissionsScheduler} from "src/token/emissions/EmissionsScheduler.sol";
import {IRelayer} from "src/token/emissions/EmissionsScheduler.sol";

contract Relayer is IRelayer {
    function relay(address destinationL3, uint256 epochIndex) external override {}
}

contract EmissionsSchedulerTest is Test {
    SyndicateToken public token;
    EmissionsCalculator public emissionsCalculator;
    EmissionsScheduler public emissionScheduler;
    Relayer public relayer;

    address public defaultAdmin = address(0x1234);
    address public relayDestinationL3 = address(0x5678);
    address public pauser = address(0xDEF0);
    address public user = address(0x1111);

    uint256 public constant DEFAULT_DECAY_FACTOR = 0.95e18; // 95% decay
    uint256 public constant START_EPOCH = 2;

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

        // Deploy emission scheduler
        emissionScheduler = new EmissionsScheduler(
            START_EPOCH, address(emissionsCalculator), address(relayer), relayDestinationL3, defaultAdmin, pauser
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
        assertEq(emissionScheduler.relayDestinationL3(), relayDestinationL3);
        assertEq(emissionScheduler.epochStartIndex(), START_EPOCH);
        assertEq(emissionScheduler.getCurrentEpoch(), 1);
        assertEq(emissionScheduler.totalEmissionsMinted(), 0);
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(emissionScheduler.hasRole(emissionScheduler.DEFAULT_ADMIN_ROLE(), defaultAdmin));
        assertTrue(emissionScheduler.hasRole(emissionScheduler.PAUSER_ROLE(), pauser));
    }

    function test_Constructor_RevertWhen_InvalidEpoch() public {
        vm.expectRevert(EmissionsScheduler.InvalidEpoch.selector);
        new EmissionsScheduler(
            0, address(emissionsCalculator), address(relayer), relayDestinationL3, defaultAdmin, pauser
        );
    }

    function test_RevertWhen_Constructor_ZeroEmissionsCalculator() public {
        vm.expectRevert(EmissionsScheduler.ZeroAddress.selector);
        new EmissionsScheduler(1, address(0), address(relayer), relayDestinationL3, defaultAdmin, pauser);
    }

    function test_RevertWhen_Constructor_ZeroAdmin() public {
        vm.expectRevert(EmissionsScheduler.ZeroAddress.selector);
        new EmissionsScheduler(
            1, address(emissionsCalculator), address(relayer), relayDestinationL3, address(0), pauser
        );
    }

    function test_RevertWhen_Constructor_ZeroPauser() public {
        vm.expectRevert(EmissionsScheduler.ZeroAddress.selector);
        new EmissionsScheduler(
            1, address(emissionsCalculator), address(relayer), relayDestinationL3, defaultAdmin, address(0)
        );
    }

    // ============ MINT EMISSION TESTS ============

    function test_MintEmission_FirstEpoch() public {
        vm.warp(emissionScheduler.getEpochStart(2));

        uint256 expectedAmount = emissionsCalculator.getNextEmission();
        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(false, false, false, true);
        emit EmissionMinted(0, expectedAmount);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.getCurrentEpoch(), 2);
        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount);
        assertEq(token.totalSupply(), initialSupply + expectedAmount);
    }

    function test_MintEmission_MultipleEpochs() public {
        uint256 totalMinted = 0;
        uint256 expectedEpochs = START_EPOCH + 5;

        for (uint256 i = START_EPOCH; i <= expectedEpochs; i++) {
            // Move to epoch i
            vm.warp(emissionScheduler.getEpochStart(i));

            uint256 expectedAmount = emissionsCalculator.getNextEmission();

            emissionScheduler.mintEmission();

            totalMinted += expectedAmount;
        }

        assertEq(emissionScheduler.getCurrentEpoch(), expectedEpochs);
        assertEq(emissionScheduler.totalEmissionsMinted(), totalMinted);
    }

    function test_MintEmission_BehindSchedule() public {
        // Skip to 3rd epoch since start
        vm.warp(emissionScheduler.getEpochStart(START_EPOCH + 3));

        // Make sure catch up works
        emissionScheduler.mintEmission(); // mint epoch 0
        emissionScheduler.mintEmission(); // mint epoch 1
        emissionScheduler.mintEmission(); // mint epoch 2
        emissionScheduler.mintEmission(); // mint epoch 3

        // But not too far
        vm.expectRevert(EmissionsScheduler.NoEmissionsToMint.selector);
        emissionScheduler.mintEmission(); // mint epoch 4
    }

    function test_RevertWhen_EpochBeforeStart() public {
        vm.prank(defaultAdmin);
        vm.expectRevert(EmissionsScheduler.NoEmissionsToMint.selector);
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_NoEmissionsToMint() public {
        vm.warp(emissionScheduler.getEpochStart(START_EPOCH));

        // Mint epoch 0
        emissionScheduler.mintEmission();

        // Try to mint epoch 0 again
        vm.expectRevert(EmissionsScheduler.NoEmissionsToMint.selector);
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_Paused() public {
        vm.warp(emissionScheduler.getEpochStart(2));

        vm.prank(pauser);
        emissionScheduler.pause();

        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_AllEmissionsCompleted() public {
        // Complete all 48 epochs
        for (uint256 i = START_EPOCH; i < START_EPOCH + 48; i++) {
            // Move to epoch i
            vm.warp(emissionScheduler.getEpochStart(i));

            emissionScheduler.mintEmission();
        }

        assertTrue(emissionScheduler.emissionsEnded());

        vm.expectRevert(EmissionsScheduler.AllEmissionsCompleted.selector);
        emissionScheduler.mintEmission();
    }

    // ============ VIEW FUNCTION TESTS ============
    function test_EmissionsStarted() public {
        assertFalse(emissionScheduler.emissionsStarted());

        vm.warp(emissionScheduler.getEpochStart(START_EPOCH));

        assertTrue(emissionScheduler.emissionsStarted());
    }

    function test_TotalEmissionsMinted() public {
        vm.warp(emissionScheduler.getEpochStart(START_EPOCH));

        assertEq(emissionScheduler.totalEmissionsMinted(), 0);

        uint256 expectedAmount = emissionsCalculator.getNextEmission();

        emissionScheduler.mintEmission();

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
        vm.warp(emissionScheduler.getEpochStart(START_EPOCH));

        // Pause and verify mint fails
        vm.prank(pauser);
        emissionScheduler.pause();

        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        emissionScheduler.mintEmission();

        // Unpause and verify mint works
        vm.prank(defaultAdmin);
        emissionScheduler.unpause();

        emissionScheduler.mintEmission(); // Should succeed

        assertEq(emissionScheduler.getCurrentEpoch(), 2);
    }

    // ============ INTEGRATION TESTS ============

    function test_Integration_FullEmissionCycle() public {
        // Mint several epochs
        uint256 totalMinted = 0;
        for (uint256 i = START_EPOCH; i < START_EPOCH + 7; i++) {
            // Move to epoch i
            vm.warp(emissionScheduler.getEpochStart(i));

            uint256 expectedAmount = emissionsCalculator.getNextEmission();

            emissionScheduler.mintEmission();

            totalMinted += expectedAmount;
        }

        assertEq(emissionScheduler.getCurrentEpoch(), START_EPOCH + 6);
        assertEq(emissionScheduler.totalEmissionsMinted(), totalMinted);
        assertFalse(emissionScheduler.emissionsEnded());
    }

    function test_Integration_LastEpoch() public {
        // Complete 47 epochs
        for (uint256 i = START_EPOCH; i < START_EPOCH + 47; i++) {
            vm.warp(emissionScheduler.getEpochStart(i));
            emissionScheduler.mintEmission();
        }

        assertEq(emissionScheduler.getCurrentEpoch(), 46 + START_EPOCH);
        assertFalse(emissionScheduler.emissionsEnded());

        // Final epoch
        vm.warp(emissionScheduler.getEpochStart(47 + START_EPOCH));
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.getCurrentEpoch(), 47 + START_EPOCH);
        assertTrue(emissionScheduler.emissionsEnded());
    }

    // ============ SECURITY TESTS ============

    function test_Security_OnlyAuthorizedRolesCanCallCriticalFunctions() public {
        // Test all critical functions require proper roles
        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.setRelayDestinationL3(address(0x1234));

        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.setRelayer(address(0x1234));

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
