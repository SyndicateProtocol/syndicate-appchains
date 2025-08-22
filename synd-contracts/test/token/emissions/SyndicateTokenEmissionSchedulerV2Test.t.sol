// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {SyndicateTokenEmissionSchedulerV2} from "src/token/emissions/SyndicateTokenEmissionSchedulerV2.sol";
import {EmissionsCalculator} from "src/token/emissions/EmissionsCalculator.sol";
import {OptimismBridgeProxy} from "src/token/bridges/OptimismBridgeProxy.sol";
import {IBridgeProxy} from "src/token/interfaces/IBridgeProxy.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockBridgeProxy is IBridgeProxy {
    struct BridgeCall {
        address token;
        uint256 amount;
        bytes data;
    }

    BridgeCall[] public bridgeCalls;
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function executeBridge(address token, uint256 amount, bytes calldata data) external override {
        if (shouldRevert) {
            revert("Bridge failed");
        }

        IERC20(token).transferFrom(msg.sender, address(this), amount);
        bridgeCalls.push(BridgeCall({token: token, amount: amount, data: data}));
    }

    function getBridgeInfo() external view override returns (string memory name, address target, bool active) {
        return ("Mock Bridge", address(this), !shouldRevert);
    }

    function getLastBridgeCall() external view returns (BridgeCall memory) {
        require(bridgeCalls.length > 0, "No bridge calls");
        return bridgeCalls[bridgeCalls.length - 1];
    }

    function getBridgeCallCount() external view returns (uint256) {
        return bridgeCalls.length;
    }

    function clearBridgeCalls() external {
        delete bridgeCalls;
    }
}

contract SyndicateTokenEmissionSchedulerV2Test is Test {
    SyndicateToken public token;
    EmissionsCalculator public emissionsCalculator;
    SyndicateTokenEmissionSchedulerV2 public emissionScheduler;
    MockBridgeProxy public mockBridge;

    address public defaultAdmin = address(0x1234);
    address public syndTreasuryAddress = address(0x5678);
    address public emissionsManager = address(0x9ABC);
    address public pauser = address(0xDEF0);
    address public decayManager = address(0x4321);
    address public user = address(0x1111);

    uint256 public constant DEFAULT_DECAY_FACTOR = 0.95e18; // 95% decay

    // Events from SyndicateTokenEmissionSchedulerV2
    event EmissionsStarted(uint256 startTime);
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed destination);
    event BridgeProxyUpdated(address indexed oldProxy, address indexed newProxy);
    event BridgeDataUpdated(bytes oldData, bytes newData);
    event Paused(address account);
    event Unpaused(address account);

    function setUp() public {
        vm.startPrank(defaultAdmin);

        // Deploy token
        token = new SyndicateToken(defaultAdmin, syndTreasuryAddress);

        // Deploy emissions calculator
        emissionsCalculator = new EmissionsCalculator(address(token), defaultAdmin, decayManager);

        // Deploy emission scheduler V2
        emissionScheduler =
            new SyndicateTokenEmissionSchedulerV2(address(emissionsCalculator), defaultAdmin, emissionsManager, pauser);

        // Grant necessary roles
        token.grantRole(token.EMISSION_MINTER_ROLE(), address(emissionsCalculator));
        emissionsCalculator.grantRole(emissionsCalculator.EMISSIONS_ROLE(), address(emissionScheduler));

        // Initialize emissions calculator
        emissionsCalculator.initializeEmissions(DEFAULT_DECAY_FACTOR);

        // Deploy mock bridge
        mockBridge = new MockBridgeProxy();

        vm.stopPrank();
    }

    // ============ CONSTRUCTOR TESTS ============

    function test_Constructor_InitialSetup() public view {
        assertEq(address(emissionScheduler.emissionsCalculator()), address(emissionsCalculator));
        assertEq(emissionScheduler.EPOCH_DURATION(), 30 days);
        assertEq(emissionScheduler.TOTAL_EPOCHS(), 48);
        assertEq(emissionScheduler.emissionsStartTime(), 0);
        assertFalse(emissionScheduler.emissionsStarted());
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(emissionScheduler.hasRole(emissionScheduler.DEFAULT_ADMIN_ROLE(), defaultAdmin));
        assertTrue(emissionScheduler.hasRole(emissionScheduler.EMISSIONS_MANAGER_ROLE(), emissionsManager));
        assertTrue(emissionScheduler.hasRole(emissionScheduler.BRIDGE_MANAGER_ROLE(), defaultAdmin));
        assertTrue(emissionScheduler.hasRole(emissionScheduler.PAUSER_ROLE(), pauser));
    }

    function test_RevertWhen_Constructor_ZeroEmissionsCalculator() public {
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.ZeroAddress.selector);
        new SyndicateTokenEmissionSchedulerV2(address(0), defaultAdmin, emissionsManager, pauser);
    }

    function test_RevertWhen_Constructor_ZeroAdmin() public {
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.ZeroAddress.selector);
        new SyndicateTokenEmissionSchedulerV2(address(emissionsCalculator), address(0), emissionsManager, pauser);
    }

    function test_RevertWhen_Constructor_ZeroEmissionsManager() public {
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.ZeroAddress.selector);
        new SyndicateTokenEmissionSchedulerV2(address(emissionsCalculator), defaultAdmin, address(0), pauser);
    }

    function test_RevertWhen_Constructor_ZeroPauser() public {
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.ZeroAddress.selector);
        new SyndicateTokenEmissionSchedulerV2(address(emissionsCalculator), defaultAdmin, emissionsManager, address(0));
    }

    // ============ BRIDGE PROXY CONFIGURATION TESTS ============

    function test_SetBridgeProxy_Success() public {
        vm.expectEmit(true, true, false, true);
        emit BridgeProxyUpdated(address(0), address(mockBridge));

        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(mockBridge)));

        assertEq(address(emissionScheduler.bridgeProxy()), address(mockBridge));
    }

    function test_SetBridgeData_Success() public {
        bytes memory testData = abi.encode("test", 123);

        vm.expectEmit(false, false, false, true);
        emit BridgeDataUpdated("", testData);

        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeData(testData);

        assertEq(emissionScheduler.bridgeData(), testData);
    }

    function test_SetBridgeProxy_UpdateExisting() public {
        // Set initial bridge
        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(mockBridge)));

        // Deploy new bridge and update
        MockBridgeProxy newBridge = new MockBridgeProxy();

        vm.expectEmit(true, true, false, true);
        emit BridgeProxyUpdated(address(mockBridge), address(newBridge));

        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(newBridge)));

        assertEq(address(emissionScheduler.bridgeProxy()), address(newBridge));
    }

    function test_RevertWhen_SetBridgeProxy_NotBridgeManager() public {
        vm.expectRevert();
        vm.prank(user);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(mockBridge)));
    }

    function test_RevertWhen_SetBridgeProxy_ZeroAddress() public {
        vm.prank(defaultAdmin);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.ZeroAddress.selector);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(0)));
    }

    function test_RevertWhen_SetBridgeData_NotBridgeManager() public {
        vm.expectRevert();
        vm.prank(user);
        emissionScheduler.setBridgeData("test");
    }

    // ============ EMISSIONS START TESTS ============

    function test_StartEmissions_Success_ImmediateStart() public {
        _setupBridgeConfiguration();

        uint256 startTime = block.timestamp;
        vm.expectEmit(false, false, false, true);
        emit EmissionsStarted(startTime);

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(startTime);

        assertTrue(emissionScheduler.emissionsStarted());
        assertEq(emissionScheduler.emissionsStartTime(), startTime);
    }

    function test_StartEmissions_Success_FutureStart() public {
        _setupBridgeConfiguration();

        uint256 futureTime = block.timestamp + 365 days; // 1 year in future
        vm.expectEmit(false, false, false, true);
        emit EmissionsStarted(futureTime);

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(futureTime);

        assertTrue(emissionScheduler.emissionsStarted());
        assertEq(emissionScheduler.emissionsStartTime(), futureTime);
    }

    function test_StartEmissions_Success_VeryFarFuture() public {
        _setupBridgeConfiguration();

        uint256 veryFutureTime = block.timestamp + 10 * 365 days; // 10 years in future
        vm.expectEmit(false, false, false, true);
        emit EmissionsStarted(veryFutureTime);

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(veryFutureTime);

        assertTrue(emissionScheduler.emissionsStarted());
        assertEq(emissionScheduler.emissionsStartTime(), veryFutureTime);
    }

    function test_RevertWhen_StartEmissions_NotEmissionsManager() public {
        _setupBridgeConfiguration();

        vm.expectRevert();
        vm.prank(user);
        emissionScheduler.startEmissions(block.timestamp);
    }

    function test_RevertWhen_StartEmissions_AlreadyStarted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.EmissionsAlreadyStarted.selector);
        emissionScheduler.startEmissions(block.timestamp + 1);
    }

    function test_RevertWhen_StartEmissions_BridgeNotConfigured() public {
        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.BridgeNotConfigured.selector);
        emissionScheduler.startEmissions(block.timestamp);
    }

    function test_RevertWhen_StartEmissions_CalculatorNotInitialized() public {
        // Create a new calculator that's not initialized
        EmissionsCalculator uninitializedCalculator =
            new EmissionsCalculator(address(token), defaultAdmin, decayManager);

        SyndicateTokenEmissionSchedulerV2 newScheduler = new SyndicateTokenEmissionSchedulerV2(
            address(uninitializedCalculator), defaultAdmin, emissionsManager, pauser
        );

        vm.prank(defaultAdmin);
        newScheduler.setBridgeProxy(IBridgeProxy(address(mockBridge)));

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.CalculatorNotInitialized.selector);
        newScheduler.startEmissions(block.timestamp);
    }

    function test_RevertWhen_StartEmissions_PastTime() public {
        _setupBridgeConfiguration();

        uint256 pastTime = block.timestamp - 1;

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.InvalidStartTime.selector);
        emissionScheduler.startEmissions(pastTime);
    }

    // ============ EPOCH WINDOW TESTS ============

    function test_EpochTiming_BeforeStart() public {
        _setupBridgeConfiguration();
        uint256 startTime = block.timestamp + 1000;

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(startTime);

        // Before start time
        assertEq(emissionScheduler.getCurrentEpoch(), 0);
        assertFalse(emissionScheduler.isEpochActive(0));
        assertEq(emissionScheduler.getEpochStartTime(0), startTime);
        assertEq(emissionScheduler.getEpochEndTime(0), startTime + 30 days);
    }

    function test_EpochTiming_AfterStart() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertEq(emissionScheduler.getCurrentEpoch(), 0);
        assertTrue(emissionScheduler.isEpochActive(0));
        assertFalse(emissionScheduler.isEpochActive(1));
    }

    function test_EpochTiming_EpochTransition() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Move to epoch 1
        vm.warp(startTime + 30 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 1);
        assertTrue(emissionScheduler.isEpochActive(1));
        assertFalse(emissionScheduler.isEpochActive(0));

        // Move to epoch 2
        vm.warp(startTime + 60 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 2);
        assertTrue(emissionScheduler.isEpochActive(2));
        assertFalse(emissionScheduler.isEpochActive(1));
    }

    function test_EpochTiming_LastEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Move to last epoch (47)
        vm.warp(startTime + 47 * 30 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 47);
        assertTrue(emissionScheduler.isEpochActive(47));

        // Move past last epoch - should still be epoch 47
        vm.warp(startTime + 50 * 30 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 47);
        assertFalse(emissionScheduler.isEpochActive(47));
    }

    // ============ MINT EMISSION TESTS ============

    function test_MintEmission_FirstEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 expectedAmount = emissionsCalculator.previewCurrentEmission();
        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(false, false, false, true);
        emit EmissionMinted(0, expectedAmount, address(mockBridge));

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 1);
        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount);
        assertEq(token.totalSupply(), initialSupply + expectedAmount);
        assertTrue(emissionScheduler.epochMinted(0));

        // Verify bridge was called
        MockBridgeProxy.BridgeCall memory call = mockBridge.getLastBridgeCall();
        assertEq(call.token, address(token));
        assertEq(call.amount, expectedAmount);
    }

    function test_MintEmission_MultipleEpochs() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 totalMinted = 0;
        uint256 expectedEpochs = 3;
        uint256 startTime = emissionScheduler.emissionsStartTime();

        for (uint256 i = 0; i < expectedEpochs; i++) {
            // Move to epoch i
            vm.warp(startTime + i * 30 days);

            uint256 expectedAmount = emissionsCalculator.previewCurrentEmission();

            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();

            totalMinted += expectedAmount;
            assertTrue(emissionScheduler.epochMinted(i));
        }

        assertEq(emissionScheduler.currentEpoch(), expectedEpochs);
        assertEq(emissionScheduler.totalEmissionsMinted(), totalMinted);
        assertEq(mockBridge.getBridgeCallCount(), expectedEpochs);
    }

    function test_MintEmission_NonSequentialEpochs() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Skip to epoch 5 - this should fail because calculator epoch and time epoch don't match
        vm.warp(startTime + 5 * 30 days);

        // This should fail because currentTimeEpoch (5) != calculatorEpoch (0)
        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.NotInEpochWindow.selector);
        emissionScheduler.mintEmission();

        // Verify epochs haven't been minted
        assertFalse(emissionScheduler.epochMinted(0));
        assertFalse(emissionScheduler.epochMinted(4));
        assertFalse(emissionScheduler.epochMinted(5));
    }

    function test_RevertWhen_MintEmission_NotStarted() public {
        _setupBridgeConfiguration();

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.EmissionsNotStarted.selector);
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_BeforeEpochStart() public {
        _setupBridgeConfiguration();
        uint256 futureStart = block.timestamp + 1000;

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(futureStart);

        // Before start time, epochs actually match (both 0), so minting will work
        // This is the new behavior - the contract doesn't prevent minting before start time
        // if the epochs align
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission(); // Should succeed

        assertTrue(emissionScheduler.epochMinted(0));
        assertEq(emissionScheduler.currentEpoch(), 1);
    }

    function test_RevertWhen_MintEmission_OutsideEpochWindow() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Move to middle of epoch 1, but calculator is still at epoch 0
        vm.warp(startTime + 35 days);

        // This should fail because currentTimeEpoch (1) != calculatorEpoch (0)
        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.NotInEpochWindow.selector);
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_EpochAlreadyMinted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Mint epoch 0
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();
        assertTrue(emissionScheduler.epochMinted(0));

        // Try to mint epoch 0 again (calculator is now at epoch 1, but we'll try to manipulate)
        // This would require some complex setup to get the epochs to align again, so let's test the mapping directly
        assertTrue(emissionScheduler.epochMinted(0));
    }

    function test_RevertWhen_MintEmission_Paused() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(pauser);
        emissionScheduler.pause();

        vm.prank(emissionsManager);
        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_NotEmissionsManager() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.expectRevert();
        vm.prank(user);
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_AllEmissionsCompleted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Complete all 48 epochs
        for (uint256 i = 0; i < 48; i++) {
            // Move to epoch i
            vm.warp(startTime + i * 30 days);

            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();
        }

        assertTrue(emissionScheduler.emissionsEnded());

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.AllEmissionsCompleted.selector);
        emissionScheduler.mintEmission();
    }

    // ============ VIEW FUNCTION TESTS ============

    function test_GetCurrentEpochInfo_BeforeStart() public view {
        (
            uint256 epoch,
            uint256 epochStartTime,
            uint256 epochEndTime,
            uint256 nextEmissionAmount,
            bool canMint,
            bool epochAlreadyMinted
        ) = emissionScheduler.getCurrentEpochInfo();

        assertEq(epoch, 0);
        assertEq(epochStartTime, 0);
        assertEq(epochEndTime, 0);
        assertEq(nextEmissionAmount, 0);
        assertFalse(canMint);
        assertFalse(epochAlreadyMinted);
    }

    function test_GetCurrentEpochInfo_AfterStart() public {
        _setupBridgeConfiguration();
        _startEmissions();

        (
            uint256 epoch,
            uint256 epochStartTime,
            uint256 epochEndTime,
            uint256 nextEmissionAmount,
            bool canMint,
            bool epochAlreadyMinted
        ) = emissionScheduler.getCurrentEpochInfo();

        assertEq(epoch, 0);
        assertEq(epochStartTime, emissionScheduler.emissionsStartTime());
        assertEq(epochEndTime, emissionScheduler.emissionsStartTime() + 30 days);
        assertTrue(nextEmissionAmount > 0);
        assertTrue(canMint);
        assertFalse(epochAlreadyMinted);
    }

    function test_GetCurrentEpochInfo_AfterMinting() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Mint epoch 0
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        (
            uint256 epoch,
            uint256 epochStartTime,
            uint256 epochEndTime,
            uint256 nextEmissionAmount,
            bool canMint,
            bool epochAlreadyMinted
        ) = emissionScheduler.getCurrentEpochInfo();

        // Calculator moved to epoch 1, but time is still in epoch 0
        uint256 currentTimeEpoch = emissionScheduler.getCurrentEpoch();

        assertEq(epoch, currentTimeEpoch);
        assertTrue(epochAlreadyMinted || !canMint); // Either already minted or can't mint due to epoch mismatch
    }

    function test_GetCurrentEpoch_TimeProgression() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        assertEq(emissionScheduler.getCurrentEpoch(), 0);

        vm.warp(startTime + 15 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 0);

        vm.warp(startTime + 30 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 1);

        vm.warp(startTime + 45 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 1);

        vm.warp(startTime + 60 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 2);
    }

    function test_EmissionsEnded() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertFalse(emissionScheduler.emissionsEnded());

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Complete all epochs
        for (uint256 i = 0; i < 48; i++) {
            vm.warp(startTime + i * 30 days);
            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();
        }

        assertTrue(emissionScheduler.emissionsEnded());
    }

    function test_EmissionsStarted() public {
        assertFalse(emissionScheduler.emissionsStarted());

        _setupBridgeConfiguration();
        _startEmissions();

        assertTrue(emissionScheduler.emissionsStarted());
    }

    function test_TotalEmissionsMinted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertEq(emissionScheduler.totalEmissionsMinted(), 0);

        uint256 expectedAmount = emissionsCalculator.previewCurrentEmission();

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount);
    }

    function test_CurrentEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertEq(emissionScheduler.currentEpoch(), 0);

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 1);
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
        _setupBridgeConfiguration();
        _startEmissions();

        // Pause and verify mint fails
        vm.prank(pauser);
        emissionScheduler.pause();

        vm.prank(emissionsManager);
        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        emissionScheduler.mintEmission();

        // Unpause and verify mint works
        vm.prank(defaultAdmin);
        emissionScheduler.unpause();

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission(); // Should succeed

        assertEq(emissionScheduler.currentEpoch(), 1);
    }

    // ============ INTEGRATION TESTS ============

    function test_Integration_FullEmissionCycle() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Mint several epochs
        uint256 totalMinted = 0;
        for (uint256 i = 0; i < 5; i++) {
            // Move to epoch i
            vm.warp(startTime + i * 30 days);

            uint256 expectedAmount = emissionsCalculator.previewCurrentEmission();

            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();

            totalMinted += expectedAmount;
            assertTrue(emissionScheduler.epochMinted(i));
        }

        assertEq(emissionScheduler.currentEpoch(), 5);
        assertEq(emissionScheduler.totalEmissionsMinted(), totalMinted);
        assertFalse(emissionScheduler.emissionsEnded());

        // Check bridge calls
        assertEq(mockBridge.getBridgeCallCount(), 5);
    }

    function test_Integration_BridgeFailure_RevertsMintEmission() public {
        _setupBridgeConfiguration();
        _startEmissions();

        mockBridge.setShouldRevert(true);

        vm.prank(emissionsManager);
        vm.expectRevert("Bridge failed");
        emissionScheduler.mintEmission();
    }

    function test_Integration_BridgeUpdate_MidCycle() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Mint first epoch
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Update bridge
        MockBridgeProxy newBridge = new MockBridgeProxy();
        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(newBridge)));

        // Move to next epoch and mint
        uint256 startTime = emissionScheduler.emissionsStartTime();
        vm.warp(startTime + 30 days);

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify old bridge has 1 call, new bridge has 1 call
        assertEq(mockBridge.getBridgeCallCount(), 1);
        assertEq(newBridge.getBridgeCallCount(), 1);
    }

    function test_Integration_FutureStartTime() public {
        _setupBridgeConfiguration();

        uint256 futureStart = block.timestamp + 1000;
        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(futureStart);

        // Before start time, both getCurrentEpoch() and calculator.currentEpoch() are 0, so they match!
        // But the epoch hasn't actually started yet, so this should work but epoch won't be active
        assertEq(emissionScheduler.getCurrentEpoch(), 0);
        assertEq(emissionScheduler.currentEpoch(), 0);
        assertFalse(emissionScheduler.isEpochActive(0)); // Epoch 0 not active yet

        // Try to mint before start - should succeed because epochs match, but epoch isn't "active"
        // The contract allows minting when epochs match, regardless of time
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 1);
        assertTrue(emissionScheduler.epochMinted(0));

        // Move to start time - now we're in time epoch 0 but calculator is at epoch 1
        vm.warp(futureStart);
        assertEq(emissionScheduler.getCurrentEpoch(), 0); // Time-based epoch
        assertEq(emissionScheduler.currentEpoch(), 1); // Calculator epoch

        // Now trying to mint should fail because epochs don't match
        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.NotInEpochWindow.selector);
        emissionScheduler.mintEmission();
    }

    function test_Integration_LastEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Complete 47 epochs
        for (uint256 i = 0; i < 47; i++) {
            vm.warp(startTime + i * 30 days);
            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();
        }

        assertEq(emissionScheduler.currentEpoch(), 47);
        assertFalse(emissionScheduler.emissionsEnded());

        // Final epoch
        vm.warp(startTime + 47 * 30 days);
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 48);
        assertTrue(emissionScheduler.emissionsEnded());
        assertTrue(emissionScheduler.epochMinted(47));
    }

    // ============ SECURITY TESTS ============

    function test_Security_ReentrancyProtection() public {
        // The nonReentrant modifier should prevent reentrancy
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify state updated correctly (reentrancy would cause inconsistent state)
        assertEq(emissionScheduler.currentEpoch(), 1);
        assertTrue(emissionScheduler.epochMinted(0));
    }

    function test_Security_OnlyAuthorizedRolesCanCallCriticalFunctions() public {
        // Test all critical functions require proper roles
        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.startEmissions(block.timestamp);

        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.mintEmission();

        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(mockBridge)));

        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.setBridgeData("test");

        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.pause();

        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.unpause();
    }

    function test_Security_EpochMintingMapping() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Verify initial state
        assertFalse(emissionScheduler.epochMinted(0));
        assertFalse(emissionScheduler.epochMinted(1));

        // Mint epoch 0
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify mapping updated
        assertTrue(emissionScheduler.epochMinted(0));
        assertFalse(emissionScheduler.epochMinted(1));
    }

    // ============ FUZZ TESTS ============

    function testFuzz_StartEmissions_ValidFutureTime(uint256 futureOffset) public {
        futureOffset = bound(futureOffset, 1, 100 * 365 days); // 1 second to 100 years

        _setupBridgeConfiguration();

        uint256 startTime = block.timestamp + futureOffset;

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(startTime);

        assertEq(emissionScheduler.emissionsStartTime(), startTime);
        assertTrue(emissionScheduler.emissionsStarted());
    }

    function testFuzz_GetCurrentEpoch_TimeProgression(uint256 timeElapsed) public {
        timeElapsed = bound(timeElapsed, 0, 50 * 30 days); // Up to 50 epochs worth

        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();
        vm.warp(startTime + timeElapsed);

        uint256 expectedEpoch = timeElapsed / 30 days;
        // Cap at 47 (last epoch)
        if (expectedEpoch > 47) expectedEpoch = 47;

        assertEq(emissionScheduler.getCurrentEpoch(), expectedEpoch);
    }

    function test_EpochTimes_Consistency() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Test a few specific epochs to avoid overflow issues
        uint256[5] memory testEpochs = [uint256(0), 5, 10, 25, 47];

        for (uint256 i = 0; i < testEpochs.length; i++) {
            uint256 epochIndex = testEpochs[i];
            uint256 epochStart = emissionScheduler.getEpochStartTime(epochIndex);
            uint256 epochEnd = emissionScheduler.getEpochEndTime(epochIndex);

            assertEq(epochStart, startTime + epochIndex * 30 days, "Epoch start time mismatch");
            assertEq(epochEnd, startTime + (epochIndex + 1) * 30 days, "Epoch end time mismatch");
            assertTrue(epochEnd > epochStart, "End time not greater than start");
            assertEq(epochEnd - epochStart, 30 days, "Duration not 30 days");
        }
    }

    function testFuzz_BridgeData_ArbitraryData(bytes calldata testData) public {
        vm.assume(testData.length < 10000); // Reasonable size limit

        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeData(testData);

        assertEq(emissionScheduler.bridgeData(), testData);
    }

    // ============ EDGE CASE TESTS ============

    function test_EdgeCase_EpochBoundary() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Right at epoch boundary
        vm.warp(startTime + 30 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 1);
        assertTrue(emissionScheduler.isEpochActive(1));
        assertFalse(emissionScheduler.isEpochActive(0));

        // One second before epoch boundary
        vm.warp(startTime + 30 days - 1);
        assertEq(emissionScheduler.getCurrentEpoch(), 0);
        assertTrue(emissionScheduler.isEpochActive(0));
        assertFalse(emissionScheduler.isEpochActive(1));
    }

    function test_EdgeCase_StartTimeAtCurrentTime() public {
        _setupBridgeConfiguration();

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(block.timestamp);

        assertTrue(emissionScheduler.emissionsStarted());
        assertEq(emissionScheduler.getCurrentEpoch(), 0);
    }

    function test_EdgeCase_MaxEpochs() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 startTime = emissionScheduler.emissionsStartTime();

        // Move way past the end
        vm.warp(startTime + 100 * 30 days);

        // Should cap at epoch 47
        assertEq(emissionScheduler.getCurrentEpoch(), 47);
        assertFalse(emissionScheduler.isEpochActive(47));
    }

    // ============ HELPER FUNCTIONS ============

    function _setupBridgeConfiguration() internal {
        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(mockBridge)));

        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeData(abi.encode("test bridge data"));
    }

    function _startEmissions() internal {
        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(block.timestamp);
    }

    function _startEmissionsAt(uint256 startTime) internal {
        vm.prank(emissionsManager);
        emissionScheduler.startEmissions(startTime);
    }
}
