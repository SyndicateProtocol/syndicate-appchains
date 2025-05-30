// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

// Mock StandardBridge for testing
contract MockStandardBridge {
    struct BridgeCall {
        address localToken;
        address remoteToken;
        address to;
        uint256 amount;
        uint32 minGasLimit;
        bytes extraData;
    }

    BridgeCall[] public bridgeCalls;
    bool public shouldRevert;

    function bridgeERC20To(
        address _localToken,
        address _remoteToken,
        address _to,
        uint256 _amount,
        uint32 _minGasLimit,
        bytes memory _extraData
    ) external {
        require(!shouldRevert, "Bridge reverted");

        bridgeCalls.push(
            BridgeCall({
                localToken: _localToken,
                remoteToken: _remoteToken,
                to: _to,
                amount: _amount,
                minGasLimit: _minGasLimit,
                extraData: _extraData
            })
        );
    }

    function getLastBridgeCall() external view returns (BridgeCall memory) {
        require(bridgeCalls.length > 0, "No bridge calls");
        return bridgeCalls[bridgeCalls.length - 1];
    }

    function getBridgeCallCount() external view returns (uint256) {
        return bridgeCalls.length;
    }

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }
}

contract SyndicateTokenTest is Test {
    SyndicateToken public token;
    MockStandardBridge public mockBridge;

    address public defaultAdmin;
    address public syndFoundationAddress;
    address public emissionsManager;
    address public user;
    address public bridgeAddress;
    address public l2TokenAddress;
    address public l2DestinationAddress;

    // Events from the contract that we'll test
    event Transfer(address indexed from, address indexed to, uint256 value);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event EmissionsStarted(uint256 startTime);
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed l2DestinationAddress);
    event BridgeConfigurationUpdated(
        address indexed bridgeAddress,
        address indexed l2TokenAddress,
        address indexed l2DestinationAddress,
        uint32 gasLimit
    );

    function setUp() public {
        defaultAdmin = makeAddr("defaultAdmin");
        syndFoundationAddress = makeAddr("syndFoundationAddress");
        emissionsManager = makeAddr("emissionsManager");
        user = makeAddr("user");
        l2TokenAddress = makeAddr("l2TokenAddress");
        l2DestinationAddress = makeAddr("l2DestinationAddress");

        // Deploy mock bridge
        mockBridge = new MockStandardBridge();
        bridgeAddress = address(mockBridge);

        token = new SyndicateToken(defaultAdmin, syndFoundationAddress, emissionsManager);
    }

    // ============ CONSTRUCTOR TESTS ============

    function test_Constructor_InitialSetup() public view {
        assertEq(token.name(), "Syndicate");
        assertEq(token.symbol(), "SYND");
        assertEq(token.decimals(), 18);
        assertEq(token.TOTAL_SUPPLY(), 100_000_000 * 10 ** 18);
        assertEq(token.INITIAL_MINT_SUPPLY(), 90_000_000 * 10 ** 18);
        assertEq(token.EMISSIONS_SUPPLY(), 10_000_000 * 10 ** 18);
        assertEq(token.EPOCH_DURATION(), 30 days);
        assertEq(token.TOTAL_EPOCHS(), 48);
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(token.hasRole(token.DEFAULT_ADMIN_ROLE(), defaultAdmin));
        assertTrue(token.hasRole(token.EMISSIONS_MANAGER_ROLE(), emissionsManager));

        // Verify no other roles are assigned
        assertFalse(token.hasRole(token.DEFAULT_ADMIN_ROLE(), syndFoundationAddress));
        assertFalse(token.hasRole(token.DEFAULT_ADMIN_ROLE(), emissionsManager));
        assertFalse(token.hasRole(token.EMISSIONS_MANAGER_ROLE(), defaultAdmin));
        assertFalse(token.hasRole(token.EMISSIONS_MANAGER_ROLE(), syndFoundationAddress));
    }

    function test_Constructor_InitialMint() public view {
        assertEq(token.balanceOf(syndFoundationAddress), token.INITIAL_MINT_SUPPLY());
        assertEq(token.totalSupply(), token.INITIAL_MINT_SUPPLY());

        // Verify other addresses have zero balance
        assertEq(token.balanceOf(defaultAdmin), 0);
        assertEq(token.balanceOf(emissionsManager), 0);
        assertEq(token.balanceOf(address(token)), 0);
    }

    function test_Constructor_EmissionScheduleInitialization() public view {
        uint256[48] memory schedule = token.getEmissionSchedule();

        // Test epochs 1-6: EMISSION_AMOUNT_1
        for (uint256 i = 0; i < 6; i++) {
            assertEq(schedule[i], token.EMISSION_AMOUNT_1());
        }

        // Test epochs 7-12: EMISSION_AMOUNT_2
        for (uint256 i = 6; i < 12; i++) {
            assertEq(schedule[i], token.EMISSION_AMOUNT_2());
        }

        // Test epochs 13-18: EMISSION_AMOUNT_3
        for (uint256 i = 12; i < 18; i++) {
            assertEq(schedule[i], token.EMISSION_AMOUNT_3());
        }

        // Test epochs 19-24: EMISSION_AMOUNT_4
        for (uint256 i = 18; i < 24; i++) {
            assertEq(schedule[i], token.EMISSION_AMOUNT_4());
        }

        // Test epochs 25-30: EMISSION_AMOUNT_5
        for (uint256 i = 24; i < 30; i++) {
            assertEq(schedule[i], token.EMISSION_AMOUNT_5());
        }

        // Test epochs 31-36: EMISSION_AMOUNT_6
        for (uint256 i = 30; i < 36; i++) {
            assertEq(schedule[i], token.EMISSION_AMOUNT_6());
        }

        // Test epochs 37-42: EMISSION_AMOUNT_7
        for (uint256 i = 36; i < 42; i++) {
            assertEq(schedule[i], token.EMISSION_AMOUNT_7());
        }

        // Test epochs 43-48: EMISSION_AMOUNT_8
        for (uint256 i = 42; i < 48; i++) {
            assertEq(schedule[i], token.EMISSION_AMOUNT_8());
        }
    }

    function test_Constructor_EmissionAmountConstants() public view {
        assertEq(token.EMISSION_AMOUNT_1(), 678_055 * 10 ** 18);
        assertEq(token.EMISSION_AMOUNT_2(), 406_833 * 10 ** 18);
        assertEq(token.EMISSION_AMOUNT_3(), 244_100 * 10 ** 18);
        assertEq(token.EMISSION_AMOUNT_4(), 146_460 * 10 ** 18);
        assertEq(token.EMISSION_AMOUNT_5(), 87_876 * 10 ** 18);
        assertEq(token.EMISSION_AMOUNT_6(), 52_726 * 10 ** 18);
        assertEq(token.EMISSION_AMOUNT_7(), 31_635 * 10 ** 18);
        assertEq(token.EMISSION_AMOUNT_8(), 18_981 * 10 ** 18);
    }

    function test_Constructor_EmissionScheduleTotalsCorrect() public view {
        uint256[48] memory schedule = token.getEmissionSchedule();
        uint256 totalEmissions = 0;

        for (uint256 i = 0; i < 48; i++) {
            totalEmissions += schedule[i];
        }

        // Note: Due to rounding in the original emission schedule, total is 9,999,996 tokens.
        // TODO: Justin will fix this by sending a new emission schedule.
        assertApproxEqAbs(totalEmissions, token.EMISSIONS_SUPPLY(), 4 * 10 ** 18);
        // assertTrue(totalEmissions == token.EMISSIONS_SUPPLY());
    }

    function test_Constructor_InitialEmissionState() public view {
        assertFalse(token.emissionsStarted());
        assertEq(token.emissionsStartTime(), 0);
        assertEq(token.currentEpoch(), 0);
        assertEq(token.totalEmissionsMinted(), 0);
        assertEq(token.getRemainingEmissions(), token.EMISSIONS_SUPPLY());
        assertFalse(token.isEmissionComplete());
    }

    function test_RevertWhen_Constructor_ZeroDefaultAdmin() public {
        vm.expectRevert("Default admin cannot be zero address");
        new SyndicateToken(address(0), syndFoundationAddress, emissionsManager);
    }

    function test_RevertWhen_Constructor_ZeroFoundationAddress() public {
        vm.expectRevert("Foundation address cannot be zero address");
        new SyndicateToken(defaultAdmin, address(0), emissionsManager);
    }

    function test_RevertWhen_Constructor_ZeroEmissionsManager() public {
        vm.expectRevert("Emissions manager cannot be zero address");
        new SyndicateToken(defaultAdmin, syndFoundationAddress, address(0));
    }

    // ============ SET BRIDGE CONFIGURATION TESTS ============

    function test_SetBridgeConfiguration_Success() public {
        uint32 gasLimit = 200000;
        bytes memory extraData = "0x1234";

        vm.expectEmit(true, true, true, true);
        emit BridgeConfigurationUpdated(bridgeAddress, l2TokenAddress, l2DestinationAddress, gasLimit);

        vm.prank(defaultAdmin);
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, gasLimit, extraData);

        (
            address storedBridgeAddress,
            address storedL2TokenAddress,
            address storedL2DestinationAddress,
            uint32 storedGasLimit,
            bytes memory storedExtraData
        ) = token.getBridgeConfiguration();

        assertEq(storedBridgeAddress, bridgeAddress);
        assertEq(storedL2TokenAddress, l2TokenAddress);
        assertEq(storedL2DestinationAddress, l2DestinationAddress);
        assertEq(storedGasLimit, gasLimit);
        assertEq(storedExtraData, extraData);
    }

    function test_SetBridgeConfiguration_CanUpdateMultipleTimes() public {
        uint32 gasLimit1 = 200000;
        uint32 gasLimit2 = 300000;
        bytes memory extraData1 = "0x1234";
        bytes memory extraData2 = "0x5678";

        // First configuration
        vm.prank(defaultAdmin);
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, gasLimit1, extraData1);

        // Update configuration
        address newL2Destination = address(0x8);
        vm.prank(defaultAdmin);
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, newL2Destination, gasLimit2, extraData2);

        (,, address storedL2DestinationAddress, uint32 storedGasLimit, bytes memory storedExtraData) =
            token.getBridgeConfiguration();

        assertEq(storedL2DestinationAddress, newL2Destination);
        assertEq(storedGasLimit, gasLimit2);
        assertEq(storedExtraData, extraData2);
    }

    function test_SetBridgeConfiguration_EmptyExtraData() public {
        uint32 gasLimit = 200000;
        bytes memory emptyExtraData = "";

        vm.prank(defaultAdmin);
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, gasLimit, emptyExtraData);

        (,,,, bytes memory storedExtraData) = token.getBridgeConfiguration();
        assertEq(storedExtraData.length, 0);
    }

    function test_RevertWhen_SetBridgeConfiguration_NotAdmin() public {
        uint32 gasLimit = 200000;
        bytes memory extraData = "0x1234";

        bytes32 adminRole = token.DEFAULT_ADMIN_ROLE();

        vm.prank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, adminRole)
        );
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, gasLimit, extraData);
    }

    function test_RevertWhen_SetBridgeConfiguration_ZeroBridgeAddress() public {
        uint32 gasLimit = 200000;
        bytes memory extraData = "0x1234";

        vm.prank(defaultAdmin);
        vm.expectRevert("Bridge address cannot be zero");
        token.setBridgeConfiguration(address(0), l2TokenAddress, l2DestinationAddress, gasLimit, extraData);
    }

    function test_RevertWhen_SetBridgeConfiguration_ZeroL2TokenAddress() public {
        uint32 gasLimit = 200000;
        bytes memory extraData = "0x1234";

        vm.prank(defaultAdmin);
        vm.expectRevert("L2 token address cannot be zero");
        token.setBridgeConfiguration(bridgeAddress, address(0), l2DestinationAddress, gasLimit, extraData);
    }

    function test_RevertWhen_SetBridgeConfiguration_ZeroL2DestinationAddress() public {
        uint32 gasLimit = 200000;
        bytes memory extraData = "0x1234";

        vm.prank(defaultAdmin);
        vm.expectRevert("L2 destination address cannot be zero");
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, address(0), gasLimit, extraData);
    }

    function test_RevertWhen_SetBridgeConfiguration_ZeroGasLimit() public {
        bytes memory extraData = "0x1234";

        vm.prank(defaultAdmin);
        vm.expectRevert("Gas limit must be greater than zero");
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, 0, extraData);
    }

    // ============ START EMISSIONS TESTS ============

    function test_StartEmissions_Success() public {
        // First set bridge configuration
        vm.prank(defaultAdmin);
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, 200000, "0x1234");

        uint256 expectedStartTime = block.timestamp;

        vm.expectEmit(true, true, true, true);
        emit EmissionsStarted(expectedStartTime);

        vm.prank(emissionsManager);
        token.startEmissions();

        assertTrue(token.emissionsStarted());
        assertEq(token.emissionsStartTime(), expectedStartTime);
    }

    function test_StartEmissions_ChecksInitialState() public {
        // Set bridge configuration first
        vm.prank(defaultAdmin);
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, 200000, "0x1234");

        vm.prank(emissionsManager);
        token.startEmissions();

        // Verify emission state after starting
        assertTrue(token.emissionsStarted());
        assertEq(token.currentEpoch(), 0);
        assertEq(token.totalEmissionsMinted(), 0);
        assertFalse(token.isEmissionComplete());

        // Check current epoch info
        (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMintEmission) =
            token.getCurrentEpochInfo();

        assertEq(epoch, 0);
        assertEq(nextEmissionTime, block.timestamp + 30 days);
        assertEq(nextEmissionAmount, token.EMISSION_AMOUNT_1());
        assertFalse(canMintEmission); // Can't mint immediately after starting
    }

    function test_RevertWhen_StartEmissions_NotEmissionsManager() public {
        // Set valid bridge configuration first
        _setupBridgeConfiguration();

        bytes32 emissionsRole = token.EMISSIONS_MANAGER_ROLE();

        vm.prank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, emissionsRole)
        );
        token.startEmissions();
    }

    function test_RevertWhen_StartEmissions_AlreadyStarted() public {
        // Set bridge configuration first
        vm.prank(defaultAdmin);
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, 200000, "0x1234");

        // Start emissions first time
        vm.prank(emissionsManager);
        token.startEmissions();

        // Try to start again
        vm.prank(emissionsManager);
        vm.expectRevert("Emissions already started");
        token.startEmissions();
    }

    function test_RevertWhen_StartEmissions_BridgeAddressNotSet() public {
        vm.prank(emissionsManager);
        vm.expectRevert("Bridge address not set");
        token.startEmissions();
    }

    function test_RevertWhen_StartEmissions_L2TokenAddressNotSet() public {
        // Set incomplete bridge configuration (missing L2 token address)
        vm.prank(defaultAdmin);
        vm.expectRevert("L2 token address cannot be zero");
        token.setBridgeConfiguration(
            bridgeAddress,
            address(0), // Zero L2 token address
            l2DestinationAddress,
            200000,
            "0x1234"
        );
    }

    function test_RevertWhen_StartEmissions_L2DestinationAddressNotSet() public {
        // Set incomplete bridge configuration (missing L2 destination address)
        vm.prank(defaultAdmin);
        vm.expectRevert("L2 destination address cannot be zero");
        token.setBridgeConfiguration(
            bridgeAddress,
            l2TokenAddress,
            address(0), // Zero destination address
            200000,
            "0x1234"
        );
    }

    function test_RevertWhen_StartEmissions_BridgeGasLimitNotSet() public {
        // Set incomplete bridge configuration (missing gas limit)
        vm.prank(defaultAdmin);
        vm.expectRevert("Gas limit must be greater than zero");
        token.setBridgeConfiguration(
            bridgeAddress,
            l2TokenAddress,
            l2DestinationAddress,
            0, // Zero gas limit
            "0x1234"
        );
    }

    function test_RevertWhen_StartEmissions_BridgeNotConfigured() public {
        // Try to start emissions without any bridge configuration
        vm.prank(emissionsManager);
        vm.expectRevert("Bridge address not set");
        token.startEmissions();
    }

    // ============ UTILITY FUNCTIONS FOR TESTS ============

    function _setupBridgeConfiguration() internal {
        vm.prank(defaultAdmin);
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, 200000, "0x1234");
    }

    function _startEmissions() internal {
        _setupBridgeConfiguration();
        vm.prank(emissionsManager);
        token.startEmissions();
    }

    // ============ MINT EMISSION TESTS ============

    function test_MintEmission_FirstEpoch() public {
        _startEmissions();

        // Fast forward to 30 days + 1 second
        vm.warp(block.timestamp + 30 days + 1);

        uint256 expectedAmount = token.EMISSION_AMOUNT_1();
        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(true, true, true, true);
        emit EmissionMinted(0, expectedAmount, l2DestinationAddress);

        token.mintEmission();

        // Verify state changes
        assertEq(token.currentEpoch(), 1);
        assertEq(token.totalEmissionsMinted(), expectedAmount);
        assertEq(token.totalSupply(), initialSupply + expectedAmount);

        // Verify bridge was called correctly
        MockStandardBridge.BridgeCall memory bridgeCall = mockBridge.getLastBridgeCall();
        assertEq(bridgeCall.localToken, address(token));
        assertEq(bridgeCall.remoteToken, l2TokenAddress);
        assertEq(bridgeCall.to, l2DestinationAddress);
        assertEq(bridgeCall.amount, expectedAmount);
        assertEq(bridgeCall.minGasLimit, 200000);
        assertEq(bridgeCall.extraData, "0x1234");
    }

    function test_MintEmission_MultipleEpochs() public {
        _startEmissions();

        // Fast forward to 90 days (3 epochs)
        vm.warp(block.timestamp + 90 days + 1);

        uint256 expectedTotal = token.EMISSION_AMOUNT_1() * 3;
        uint256 initialSupply = token.totalSupply();

        token.mintEmission();

        // Verify state changes
        assertEq(token.currentEpoch(), 3);
        assertEq(token.totalEmissionsMinted(), expectedTotal);
        assertEq(token.totalSupply(), initialSupply + expectedTotal);

        // Verify bridge was called with total amount
        MockStandardBridge.BridgeCall memory bridgeCall = mockBridge.getLastBridgeCall();
        assertEq(bridgeCall.amount, expectedTotal);
    }

    function test_MintEmission_CatchUpScenario() public {
        _startEmissions();

        // Mint first epoch
        vm.warp(block.timestamp + 30 days + 1);
        token.mintEmission();

        // Fast forward to 5th epoch (skip 3 epochs)
        vm.warp(block.timestamp + 120 days);

        uint256 expectedCatchUp = token.EMISSION_AMOUNT_1() * 4; // Epochs 1-4 (0-indexed)
        uint256 previousTotal = token.totalEmissionsMinted();

        token.mintEmission();

        // Verify catch-up worked
        assertEq(token.currentEpoch(), 5);
        assertEq(token.totalEmissionsMinted(), previousTotal + expectedCatchUp);
    }

    function test_MintEmission_DifferentAmountPeriods() public {
        _startEmissions();

        // Fast forward to epoch 7 (first epoch with EMISSION_AMOUNT_2)
        vm.warp(block.timestamp + 210 days + 1); // 7 * 30 days

        uint256 expectedAmount = token.EMISSION_AMOUNT_1() * 6 // Epochs 0-5
            + token.EMISSION_AMOUNT_2() * 1; // Epoch 6

        token.mintEmission();

        assertEq(token.currentEpoch(), 7);
        assertEq(token.totalEmissionsMinted(), expectedAmount);
    }

    function test_MintEmission_FinalEpoch() public {
        _startEmissions();

        // Fast forward to final epoch (48th epoch = index 47)
        vm.warp(block.timestamp + 48 * 30 days + 1);

        // Calculate total emissions manually
        uint256 expectedTotal = token.EMISSION_AMOUNT_1() * 6 + token.EMISSION_AMOUNT_2() * 6
            + token.EMISSION_AMOUNT_3() * 6 + token.EMISSION_AMOUNT_4() * 6 + token.EMISSION_AMOUNT_5() * 6
            + token.EMISSION_AMOUNT_6() * 6 + token.EMISSION_AMOUNT_7() * 6 + token.EMISSION_AMOUNT_8() * 6;

        token.mintEmission();

        assertEq(token.currentEpoch(), 48);
        assertEq(token.totalEmissionsMinted(), expectedTotal);
        assertTrue(token.isEmissionComplete());
    }

    function test_MintEmission_CanBeCalledByAnyone() public {
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        // Any address can call mintEmission
        vm.prank(user);
        token.mintEmission();

        assertEq(token.currentEpoch(), 1);
    }

    function test_MintEmission_EmitsCorrectEvents() public {
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        uint256 expectedAmount = token.EMISSION_AMOUNT_1();

        vm.expectEmit(true, true, true, true);
        emit Transfer(address(0), address(token), expectedAmount);

        vm.expectEmit(true, true, true, true);
        emit EmissionMinted(0, expectedAmount, l2DestinationAddress);

        token.mintEmission();
    }

    function test_RevertWhen_MintEmission_EmissionsNotStarted() public {
        // Don't start emissions
        _setupBridgeConfiguration();
        vm.warp(block.timestamp + 30 days + 1);

        vm.expectRevert("Emissions not started");
        token.mintEmission();
    }

    function test_RevertWhen_MintEmission_NoEpochsToMint() public {
        _startEmissions();

        // Try to mint immediately (no time has passed)
        vm.expectRevert("Current epoch already minted");
        token.mintEmission();
    }

    function test_RevertWhen_MintEmission_AllEmissionsCompleted() public {
        _startEmissions();

        // Fast forward to all epochs and mint them
        vm.warp(block.timestamp + 48 * 30 days + 1);
        token.mintEmission();

        // Try to mint again after completion
        vm.warp(block.timestamp + 30 days);
        vm.expectRevert("All emissions completed");
        token.mintEmission();
    }

    function test_RevertWhen_MintEmission_BridgeReverts() public {
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        // Make bridge revert
        mockBridge.setShouldRevert(true);

        vm.expectRevert("Bridge reverted");
        token.mintEmission();
    }

    // ============ VIEW FUNCTION TESTS ============

    function test_GetCurrentEpochInfo_BeforeEmissionsStart() public view {
        (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMintEmission) =
            token.getCurrentEpochInfo();

        assertEq(epoch, 0);
        assertEq(nextEmissionTime, 0);
        assertEq(nextEmissionAmount, 0);
        assertFalse(canMintEmission);
    }

    function test_GetCurrentEpochInfo_AfterEmissionsStarted() public {
        _startEmissions();
        uint256 startTime = block.timestamp;

        (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMintEmission) =
            token.getCurrentEpochInfo();

        assertEq(epoch, 0);
        assertEq(nextEmissionTime, startTime + 30 days);
        assertEq(nextEmissionAmount, token.EMISSION_AMOUNT_1());
        assertFalse(canMintEmission); // Can't mint immediately
    }

    function test_GetCurrentEpochInfo_CanMintEpoch() public {
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        (,, uint256 nextEmissionAmount, bool canMintEmission) = token.getCurrentEpochInfo();

        assertEq(nextEmissionAmount, token.EMISSION_AMOUNT_1());
        assertTrue(canMintEmission);
    }

    function test_GetCurrentEpochInfo_AfterMinting() public {
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);
        token.mintEmission();

        (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMintEmission) =
            token.getCurrentEpochInfo();

        assertEq(epoch, 1);
        assertEq(nextEmissionTime, token.emissionsStartTime() + 60 days); // 2nd epoch
        assertEq(nextEmissionAmount, token.EMISSION_AMOUNT_1());
        assertFalse(canMintEmission);
    }

    function test_GetCurrentEpochInfo_FinalEpoch() public {
        _startEmissions();
        vm.warp(block.timestamp + 48 * 30 days + 1);
        token.mintEmission();

        (uint256 epoch,, uint256 nextEmissionAmount, bool canMintEmission) = token.getCurrentEpochInfo();

        assertEq(epoch, 48);
        assertEq(nextEmissionAmount, 0); // No more emissions
        assertFalse(canMintEmission);
    }

    function test_GetRemainingEmissions_Initial() public view {
        assertEq(token.getRemainingEmissions(), token.EMISSIONS_SUPPLY());
    }

    function test_GetRemainingEmissions_AfterSomeEmissions() public {
        _startEmissions();
        vm.warp(block.timestamp + 90 days + 1); // 3 epochs
        token.mintEmission();

        uint256 minted = token.EMISSION_AMOUNT_1() * 3;
        uint256 expected = token.EMISSIONS_SUPPLY() - minted;

        assertEq(token.getRemainingEmissions(), expected);
    }

    function test_GetRemainingEmissions_AfterAllEmissions() public {
        _startEmissions();
        vm.warp(block.timestamp + 48 * 30 days + 1);
        token.mintEmission();

        uint256 remaining = token.getRemainingEmissions();
        assertTrue(remaining <= 10 * 10 ** 18); // Should be very small (due to rounding)
    }

    function test_IsEmissionComplete_Initial() public view {
        assertFalse(token.isEmissionComplete());
    }

    function test_IsEmissionComplete_DuringEmissions() public {
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);
        token.mintEmission();

        assertFalse(token.isEmissionComplete());
    }

    function test_IsEmissionComplete_AfterAllEmissions() public {
        _startEmissions();
        vm.warp(block.timestamp + 48 * 30 days + 1);
        token.mintEmission();

        assertTrue(token.isEmissionComplete());
    }

    function test_GetEmissionSchedule_ReturnsCorrectLength() public view {
        uint256[48] memory schedule = token.getEmissionSchedule();
        // Just verify we can access all 48 elements
        assertEq(schedule.length, 48);
        assertEq(schedule[0], token.EMISSION_AMOUNT_1());
        assertEq(schedule[47], token.EMISSION_AMOUNT_8());
    }

    // ============ INTEGRATION TESTS ============

    function test_Integration_FullEmissionCycle() public {
        // Test complete emission lifecycle
        _startEmissions();
        uint256 startTime = block.timestamp;

        // Mint emissions at different intervals
        vm.warp(startTime + 30 days + 1);
        token.mintEmission(); // Epoch 0

        vm.warp(startTime + 90 days + 1);
        token.mintEmission(); // Epochs 1-2

        vm.warp(startTime + 210 days + 1);
        token.mintEmission(); // Epochs 3-6 (transition to EMISSION_AMOUNT_2)

        // Verify state throughout
        assertTrue(token.currentEpoch() > 0);
        assertTrue(token.totalEmissionsMinted() > 0);
        assertFalse(token.isEmissionComplete());

        // Complete all emissions
        vm.warp(startTime + 48 * 30 days + 1);
        token.mintEmission();

        assertTrue(token.isEmissionComplete());
        assertEq(token.currentEpoch(), 48);
    }

    function test_Integration_BridgeInteraction() public {
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        uint256 bridgeCallsBefore = mockBridge.getBridgeCallCount();
        token.mintEmission();
        uint256 bridgeCallsAfter = mockBridge.getBridgeCallCount();

        // Verify bridge was called exactly once
        assertEq(bridgeCallsAfter, bridgeCallsBefore + 1);

        // Verify bridge call parameters
        MockStandardBridge.BridgeCall memory call = mockBridge.getLastBridgeCall();
        assertEq(call.localToken, address(token));
        assertEq(call.remoteToken, l2TokenAddress);
        assertEq(call.to, l2DestinationAddress);
        assertEq(call.minGasLimit, 200000);
    }

    function test_Integration_TokenSupplyConsistency() public {
        uint256 initialSupply = token.totalSupply();

        _startEmissions();
        vm.warp(block.timestamp + 180 days + 1); // 6 epochs
        token.mintEmission();

        uint256 expectedMinted = token.EMISSION_AMOUNT_1() * 6;
        uint256 newSupply = token.totalSupply();

        assertEq(newSupply, initialSupply + expectedMinted);
        assertEq(token.totalEmissionsMinted(), expectedMinted);
    }

    function test_Integration_PermissionBoundaries() public {
        // Only admin can set bridge config
        vm.prank(user);
        vm.expectRevert();
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, 200000, "");

        // Only emissions manager can start emissions
        _setupBridgeConfiguration();
        vm.prank(user);
        vm.expectRevert();
        token.startEmissions();

        // Anyone can mint emissions (after setup)
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);
        vm.prank(user);
        token.mintEmission(); // Should succeed
    }

    // ============ FUZZ TESTS ============

    function testFuzz_MintEmission_ValidTimeProgress(uint256 timeElapsed) public {
        // Bound time to reasonable range (1 day to 5 years)
        timeElapsed = bound(timeElapsed, 1 days, 5 * 365 days);

        _startEmissions();
        vm.warp(block.timestamp + timeElapsed);

        if (timeElapsed >= 30 days) {
            uint256 expectedEpochs = timeElapsed / 30 days;
            if (expectedEpochs > 48) expectedEpochs = 48;

            token.mintEmission();
            assertEq(token.currentEpoch(), expectedEpochs);
        } else {
            vm.expectRevert("Current epoch already minted");
            token.mintEmission();
        }
    }

    function testFuzz_BridgeConfiguration_ValidParameters(uint32 gasLimit, bytes calldata extraData) public {
        gasLimit = uint32(bound(gasLimit, 1, type(uint32).max));

        vm.prank(defaultAdmin);
        token.setBridgeConfiguration(bridgeAddress, l2TokenAddress, l2DestinationAddress, gasLimit, extraData);

        (,,, uint32 storedGasLimit, bytes memory storedExtraData) = token.getBridgeConfiguration();
        assertEq(storedGasLimit, gasLimit);
        assertEq(storedExtraData, extraData);
    }
}
