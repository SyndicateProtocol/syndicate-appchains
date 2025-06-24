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
        assertEq(emissionScheduler.EMISSION_BUFFER_TIME(), 1 hours);
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

    // ============ EMISSIONS CONTROL TESTS ============

    function test_StartEmissions_Success() public {
        _setupBridgeConfiguration();

        vm.expectEmit(false, false, false, true);
        emit EmissionsStarted(block.timestamp);

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        assertTrue(emissionScheduler.emissionsStarted());
        assertEq(emissionScheduler.emissionsStartTime(), block.timestamp);
    }

    function test_RevertWhen_StartEmissions_NotEmissionsManager() public {
        _setupBridgeConfiguration();

        vm.expectRevert();
        vm.prank(user);
        emissionScheduler.startEmissions();
    }

    function test_RevertWhen_StartEmissions_AlreadyStarted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.EmissionsAlreadyStarted.selector);
        emissionScheduler.startEmissions();
    }

    function test_RevertWhen_StartEmissions_BridgeNotConfigured() public {
        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.BridgeNotConfigured.selector);
        emissionScheduler.startEmissions();
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
        newScheduler.startEmissions();
    }

    // ============ MINT EMISSION TESTS ============

    function test_MintEmission_FirstEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 30 days + 1);

        uint256 expectedAmount = emissionsCalculator.previewCurrentEmission();
        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(false, false, false, true);
        emit EmissionMinted(0, expectedAmount, address(mockBridge));

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 1);
        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount);
        assertEq(token.totalSupply(), initialSupply + expectedAmount);

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

        for (uint256 i = 0; i < expectedEpochs; i++) {
            vm.warp(block.timestamp + 30 days + 1);
            uint256 expectedAmount = emissionsCalculator.previewCurrentEmission();

            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();

            totalMinted += expectedAmount;
        }

        assertEq(emissionScheduler.currentEpoch(), expectedEpochs);
        assertEq(emissionScheduler.totalEmissionsMinted(), totalMinted);
    }

    function test_RevertWhen_MintEmission_NotStarted() public {
        _setupBridgeConfiguration();

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.EmissionsNotStarted.selector);
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_Paused() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(pauser);
        emissionScheduler.pause();

        vm.warp(block.timestamp + 30 days + 1);

        vm.prank(emissionsManager);
        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_EmissionTooEarly() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 30 days - 2 hours); // Before buffer time

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.EmissionTooEarly.selector);
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_AllEmissionsCompleted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Complete all 48 epochs
        for (uint256 i = 0; i < 48; i++) {
            vm.warp(block.timestamp + 30 days + 1);
            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();
        }

        assertTrue(emissionScheduler.emissionsEnded());

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.AllEmissionsCompleted.selector);
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_NotEmissionsManager() public {
        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        vm.expectRevert();
        vm.prank(user);
        emissionScheduler.mintEmission();
    }

    // ============ VIEW FUNCTION TESTS ============

    function test_GetCurrentEpochInfo_BeforeStart() public view {
        (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMint) =
            emissionScheduler.getCurrentEpochInfo();

        assertEq(epoch, 0);
        assertEq(nextEmissionTime, 0);
        assertEq(nextEmissionAmount, 0);
        assertFalse(canMint);
    }

    function test_GetCurrentEpochInfo_AfterStart() public {
        _setupBridgeConfiguration();
        _startEmissions();

        (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMint) =
            emissionScheduler.getCurrentEpochInfo();

        assertEq(epoch, 0);
        assertEq(nextEmissionTime, block.timestamp + 30 days);
        assertTrue(nextEmissionAmount > 0);
        assertFalse(canMint); // Too early
    }

    function test_GetCurrentEpochInfo_CanMint() public {
        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + 30 days - 1 hours + 1); // Right at buffer time

        (,, uint256 nextEmissionAmount, bool canMint) = emissionScheduler.getCurrentEpochInfo();

        assertTrue(nextEmissionAmount > 0);
        assertTrue(canMint);
    }

    function test_GetCurrentEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertEq(emissionScheduler.getCurrentEpoch(), 0);

        vm.warp(block.timestamp + 30 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 1);

        vm.warp(block.timestamp + 60 days);
        assertEq(emissionScheduler.getCurrentEpoch(), 3);
    }

    function test_GetCurrentEpoch_BeforeStart() public view {
        assertEq(emissionScheduler.getCurrentEpoch(), 0);
    }

    function test_EmissionsEnded() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertFalse(emissionScheduler.emissionsEnded());

        // Complete all epochs
        for (uint256 i = 0; i < 48; i++) {
            vm.warp(block.timestamp + 30 days + 1);
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

    function test_GetNextEmissionTime() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 nextTime = emissionScheduler.getNextEmissionTime();
        assertEq(nextTime, block.timestamp + 30 days);

        // After first emission
        vm.warp(block.timestamp + 30 days + 1);
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        nextTime = emissionScheduler.getNextEmissionTime();
        assertEq(nextTime, emissionScheduler.emissionsStartTime() + 60 days);
    }

    function test_GetNextEmissionTime_BeforeStart() public view {
        assertEq(emissionScheduler.getNextEmissionTime(), 0);
    }

    function test_TotalEmissionsMinted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertEq(emissionScheduler.totalEmissionsMinted(), 0);

        vm.warp(block.timestamp + 30 days + 1);
        uint256 expectedAmount = emissionsCalculator.previewCurrentEmission();

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount);
    }

    function test_CurrentEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertEq(emissionScheduler.currentEpoch(), 0);

        vm.warp(block.timestamp + 30 days + 1);
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

    // ============ INTEGRATION TESTS ============

    function test_Integration_FullEmissionCycle() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Mint several epochs
        uint256 totalMinted = 0;
        for (uint256 i = 0; i < 5; i++) {
            vm.warp(block.timestamp + 30 days + 1);
            uint256 expectedAmount = emissionsCalculator.previewCurrentEmission();

            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();

            totalMinted += expectedAmount;
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

        vm.warp(block.timestamp + 30 days + 1);

        vm.prank(emissionsManager);
        vm.expectRevert("Bridge failed");
        emissionScheduler.mintEmission();
    }

    function test_Integration_PauseUnpauseCycle() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 30 days + 1);

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

    // ============ SECURITY TESTS ============

    function test_Security_ReentrancyProtection() public {
        // The nonReentrant modifier should prevent reentrancy
        // This is more of a documentation test since we can't easily test reentrancy with mock contracts
        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify state updated correctly (reentrancy would cause inconsistent state)
        assertEq(emissionScheduler.currentEpoch(), 1);
    }

    function test_Security_OnlyAuthorizedRolesCanCallCriticalFunctions() public {
        // Test all critical functions require proper roles
        vm.prank(user);
        vm.expectRevert();
        emissionScheduler.startEmissions();

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

    // ============ FUZZ TESTS ============

    function testFuzz_GetCurrentEpoch_TimeProgression(uint256 timeElapsed) public {
        timeElapsed = bound(timeElapsed, 0, 47 * 30 days + 29 days); // Just under 48 full epochs

        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + timeElapsed);

        uint256 expectedEpoch = timeElapsed / 30 days;
        // getCurrentEpoch caps at 47 when timeElapsed would put us at epoch 48+
        if (expectedEpoch > 47) expectedEpoch = 47;

        assertEq(emissionScheduler.getCurrentEpoch(), expectedEpoch);
    }

    function testFuzz_MintEmission_ValidTimeWindows(uint256 timeOffset) public {
        timeOffset = bound(timeOffset, 30 days - 1 hours + 1, 30 days + 7 days);

        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + timeOffset);

        // Should be able to mint in valid time window
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 1);
    }

    function testFuzz_BridgeData_ArbitraryData(bytes calldata testData) public {
        vm.assume(testData.length < 10000); // Reasonable size limit

        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeData(testData);

        assertEq(emissionScheduler.bridgeData(), testData);
    }

    // ============ EDGE CASE TESTS ============

    function test_EdgeCase_EmissionAtExactBufferTime() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Warp to exactly the buffer time
        vm.warp(block.timestamp + 30 days - 1 hours);

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission(); // Should succeed

        assertEq(emissionScheduler.currentEpoch(), 1);
    }

    function test_EdgeCase_EmissionJustBeforeBufferTime() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Warp to just before the buffer time
        vm.warp(block.timestamp + 30 days - 1 hours - 1);

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionSchedulerV2.EmissionTooEarly.selector);
        emissionScheduler.mintEmission();
    }

    function test_EdgeCase_LastEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Complete 47 epochs
        for (uint256 i = 0; i < 47; i++) {
            vm.warp(block.timestamp + 30 days + 1);
            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();
        }

        assertEq(emissionScheduler.currentEpoch(), 47);
        assertFalse(emissionScheduler.emissionsEnded());

        // Final epoch
        vm.warp(block.timestamp + 30 days + 1);
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 48);
        assertTrue(emissionScheduler.emissionsEnded());
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
        emissionScheduler.startEmissions();
    }
}
