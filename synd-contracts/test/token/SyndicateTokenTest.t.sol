// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken, AbstractXERC20} from "src/token/SyndicateToken.sol";
import {BridgeProxy} from "src/token/BridgeProxy.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

// Mock Bridge that properly handles token transfers
contract MockBridge {
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

    // This function will be called by the bridge proxy
    function executeBridge(address token, uint256 amount, bytes calldata data) external {
        if (shouldRevert) {
            revert("Bridge reverted");
        }

        // Record the bridge call
        bridgeCalls.push(BridgeCall({token: token, amount: amount, data: data}));
    }

    function getLastBridgeCall() external view returns (BridgeCall memory) {
        require(bridgeCalls.length > 0, "No bridge calls");
        return bridgeCalls[bridgeCalls.length - 1];
    }

    function getBridgeCallCount() external view returns (uint256) {
        return bridgeCalls.length;
    }
}

contract SyndicateTokenTest is Test {
    SyndicateToken public token;
    BridgeProxy public bridgeProxy;
    MockBridge public mockBridge;

    address public defaultAdmin = address(0x1234);
    address public syndFoundationAddress = address(0x5678);
    address public emissionsManager = address(0x9ABC);
    address public pauser = address(0xDEF0);
    address public user = address(0x1111);

    // Events
    event EmissionsStarted(uint256 startTime);
    event EmissionsPaused();
    event EmissionsResumed();
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed destination);
    event BridgeProxyUpdated(address indexed oldProxy, address indexed newProxy);
    event BridgeDataUpdated(bytes oldData, bytes newData);
    event Paused(address account);
    event Unpaused(address account);

    function setUp() public {
        // Deploy mock bridge and bridge proxy
        mockBridge = new MockBridge();
        bridgeProxy = new BridgeProxy(defaultAdmin);

        // Deploy token
        token = new SyndicateToken(defaultAdmin, syndFoundationAddress, emissionsManager, pauser);
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
        assertTrue(token.hasRole(token.BRIDGE_MANAGER_ROLE(), defaultAdmin));
        assertTrue(token.hasRole(token.PAUSER_ROLE(), pauser));
    }

    function test_Constructor_InitialMint() public view {
        assertEq(token.balanceOf(syndFoundationAddress), token.INITIAL_MINT_SUPPLY());
        assertEq(token.totalSupply(), token.INITIAL_MINT_SUPPLY());
    }

    // ============ BRIDGE PROXY CONFIGURATION TESTS ============

    function test_SetBridgeProxy_Success() public {
        vm.expectEmit(true, true, false, true);
        emit BridgeProxyUpdated(address(0), address(bridgeProxy));

        vm.prank(defaultAdmin);
        token.setBridgeProxy(address(bridgeProxy));

        (address proxy,) = token.getBridgeConfiguration();
        assertEq(proxy, address(bridgeProxy));
    }

    function test_SetBridgeData_Success() public {
        bytes memory testData = abi.encode("test", 123);

        vm.expectEmit(false, false, false, true);
        emit BridgeDataUpdated("", testData);

        vm.prank(defaultAdmin);
        token.setBridgeData(testData);

        (, bytes memory data) = token.getBridgeConfiguration();
        assertEq(data, testData);
    }

    function test_RevertWhen_SetBridgeProxy_NotBridgeManager() public {
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, user, token.BRIDGE_MANAGER_ROLE()
            )
        );
        token.setBridgeProxy(address(bridgeProxy));
        vm.stopPrank();
    }

    function test_RevertWhen_SetBridgeProxy_ZeroAddress() public {
        vm.startPrank(defaultAdmin);
        vm.expectRevert(AbstractXERC20.ZeroAddress.selector);
        token.setBridgeProxy(address(0));
        vm.stopPrank();
    }

    // ============ EMISSIONS CONTROL TESTS ============

    function test_StartEmissions_Success() public {
        _setupBridgeConfiguration();

        vm.expectEmit(false, false, false, true);
        emit EmissionsStarted(block.timestamp);

        vm.prank(emissionsManager);
        token.startEmissions();

        assertTrue(token.emissionsStarted());
        assertTrue(token.emissionsActive());
        assertEq(token.emissionsStartTime(), block.timestamp);
    }

    function test_PauseEmissions_Success() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.expectEmit(false, false, false, false);
        emit EmissionsPaused();

        vm.prank(pauser);
        token.pauseEmissions();

        assertFalse(token.emissionsActive());
    }

    function test_ResumeEmissions_Success() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(pauser);
        token.pauseEmissions();

        vm.expectEmit(false, false, false, false);
        emit EmissionsResumed();

        vm.prank(emissionsManager);
        token.resumeEmissions();

        assertTrue(token.emissionsActive());
    }

    function test_RevertWhen_StartEmissions_NotEmissionsManager() public {
        _setupBridgeConfiguration();

        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, user, token.EMISSIONS_MANAGER_ROLE()
            )
        );
        token.startEmissions();
        vm.stopPrank();
    }

    function test_RevertWhen_StartEmissions_AlreadyStarted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateToken.EmissionsAlreadyStarted.selector);
        token.startEmissions();
    }

    function test_RevertWhen_StartEmissions_BridgeNotConfigured() public {
        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateToken.BridgeNotConfigured.selector);
        token.startEmissions();
    }

    // ============ MINT EMISSION TESTS ============

    function test_MintEmission_FirstEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 30 days + 1);

        uint256 expectedAmount = 678_055 * 10 ** 18;
        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(false, false, false, true);
        emit EmissionMinted(1, expectedAmount, address(0));

        token.mintEmission();

        assertEq(token.currentEpoch(), 1);
        assertEq(token.totalEmissionsMinted(), expectedAmount);
        assertEq(token.totalSupply(), initialSupply + expectedAmount);

        // Verify bridge was called
        MockBridge.BridgeCall memory call = mockBridge.getLastBridgeCall();
        assertEq(call.token, address(token));
        assertEq(call.amount, expectedAmount);
    }

    function test_MintEmission_MultipleEpochs() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 90 days + 1); // 3 epochs

        uint256 expectedAmount = 678_055 * 10 ** 18 * 3;
        token.mintEmission();

        assertEq(token.currentEpoch(), 3);
        assertEq(token.totalEmissionsMinted(), expectedAmount);
    }

    function test_RevertWhen_MintEmission_EmissionsNotActive() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(pauser);
        token.pauseEmissions();

        vm.warp(block.timestamp + 30 days + 1);
        vm.expectRevert(SyndicateToken.EmissionsNotActive.selector);
        token.mintEmission();
    }

    function test_RevertWhen_MintEmission_EpochAlreadyMinted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.expectRevert(SyndicateToken.EpochAlreadyMinted.selector);
        token.mintEmission();
    }

    function test_RevertWhen_MintEmission_Paused() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(pauser);
        token.pause();

        vm.warp(block.timestamp + 30 days + 1);
        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        token.mintEmission();
    }

    // ============ VIEW FUNCTION TESTS ============

    function test_GetCurrentEpochInfo_BeforeStart() public view {
        (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMint) =
            token.getCurrentEpochInfo();

        assertEq(epoch, 0);
        assertEq(nextEmissionTime, 0);
        assertEq(nextEmissionAmount, 0);
        assertFalse(canMint);
    }

    function test_GetCurrentEpochInfo_AfterStart() public {
        _setupBridgeConfiguration();
        _startEmissions();

        (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMint) =
            token.getCurrentEpochInfo();

        assertEq(epoch, 0);
        assertEq(nextEmissionTime, block.timestamp + 30 days);
        assertEq(nextEmissionAmount, 678_055 * 10 ** 18);
        assertFalse(canMint);
    }

    function test_GetCurrentEpochInfo_CanMint() public {
        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        (,, uint256 nextEmissionAmount, bool canMint) = token.getCurrentEpochInfo();

        assertEq(nextEmissionAmount, 678_055 * 10 ** 18);
        assertTrue(canMint);
    }

    function test_GetRemainingEmissions() public {
        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + 60 days + 1);
        token.mintEmission();

        uint256 minted = 678_055 * 10 ** 18 * 2;
        uint256 expected = token.EMISSIONS_SUPPLY() - minted;

        assertEq(token.getRemainingEmissions(), expected);
    }

    function test_EmissionsEnded() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertFalse(token.emissionsEnded());

        vm.warp(block.timestamp + 48 * 30 days + 1);
        token.mintEmission();

        assertTrue(token.emissionsEnded());
    }

    // ============ PAUSE FUNCTIONALITY TESTS ============

    function test_Pause_Success() public {
        vm.expectEmit(false, false, false, true);
        emit Paused(pauser);

        vm.prank(pauser);
        token.pause();

        assertTrue(token.paused());
    }

    function test_Unpause_Success() public {
        vm.prank(pauser);
        token.pause();

        vm.expectEmit(false, false, false, true);
        emit Unpaused(defaultAdmin);

        vm.prank(defaultAdmin);
        token.unpause();

        assertFalse(token.paused());
    }

    function test_RevertWhen_Pause_NotPauser() public {
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, token.PAUSER_ROLE())
        );
        token.pause();
        vm.stopPrank();
    }

    function test_RevertWhen_Unpause_NotAdmin() public {
        vm.prank(pauser);
        token.pause();

        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, user, token.DEFAULT_ADMIN_ROLE()
            )
        );
        token.unpause();
        vm.stopPrank();
    }

    // ============ XERC20 FUNCTIONALITY TESTS ============

    function test_SetLimits_Success() public {
        address bridge = address(0x123);
        uint256 mintingLimit = 1000 * 10 ** 18;
        uint256 burningLimit = 500 * 10 ** 18;

        vm.prank(defaultAdmin);
        token.setLimits(bridge, mintingLimit, burningLimit);

        assertEq(token.mintingMaxLimitOf(bridge), mintingLimit);
        assertEq(token.burningMaxLimitOf(bridge), burningLimit);
        assertTrue(token.isBridge(bridge));
    }

    function test_Mint_AsBridge() public {
        address bridge = address(0x123);
        vm.prank(defaultAdmin);
        token.setLimits(bridge, 1000 * 10 ** 18, 500 * 10 ** 18);

        vm.prank(bridge);
        token.mint(user, 100 * 10 ** 18);

        assertEq(token.balanceOf(user), 100 * 10 ** 18);
    }

    function test_Burn_AsBridge() public {
        address bridge = address(0x123);
        vm.prank(defaultAdmin);
        token.setLimits(bridge, 1000 * 10 ** 18, 500 * 10 ** 18);

        // First mint some tokens
        vm.prank(bridge);
        token.mint(user, 200 * 10 ** 18);

        // Then burn some
        vm.prank(bridge);
        token.burn(user, 50 * 10 ** 18);

        assertEq(token.balanceOf(user), 150 * 10 ** 18);
    }

    // ============ INTEGRATION TESTS ============

    function test_Integration_FullEmissionCycle() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Mint first few epochs
        vm.warp(block.timestamp + 90 days + 1); // 3 epochs
        token.mintEmission();

        assertTrue(token.currentEpoch() > 0);
        assertTrue(token.totalEmissionsMinted() > 0);
        assertFalse(token.emissionsEnded());

        // Complete all emissions
        vm.warp(block.timestamp + 45 * 30 days + 1);
        token.mintEmission();

        assertTrue(token.emissionsEnded());
        assertEq(token.currentEpoch(), 48);
    }

    function test_Integration_BridgeInteraction() public {
        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        uint256 bridgeCallsBefore = mockBridge.getBridgeCallCount();
        token.mintEmission();
        uint256 bridgeCallsAfter = mockBridge.getBridgeCallCount();

        assertEq(bridgeCallsAfter, bridgeCallsBefore + 1);

        MockBridge.BridgeCall memory call = mockBridge.getLastBridgeCall();
        assertEq(call.token, address(token));
        assertEq(call.amount, 678_055 * 10 ** 18);
    }

    // ============ FUZZ TESTS ============

    function testFuzz_MintEmission_ValidTimeProgress(uint256 timeElapsed) public {
        timeElapsed = bound(timeElapsed, 1 days, 5 * 365 days);

        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + timeElapsed);

        if (timeElapsed >= 30 days) {
            uint256 expectedEpochs = timeElapsed / 30 days;
            if (expectedEpochs > 48) expectedEpochs = 48;

            token.mintEmission();
            assertEq(token.currentEpoch(), expectedEpochs);
        } else {
            vm.expectRevert(SyndicateToken.EpochAlreadyMinted.selector);
            token.mintEmission();
        }
    }

    function testFuzz_SetLimits_ValidParameters(uint256 mintingLimit, uint256 burningLimit) public {
        mintingLimit = bound(mintingLimit, 0, 1e30);
        burningLimit = bound(burningLimit, 0, 1e30);

        address bridge = address(0x123);

        vm.prank(defaultAdmin);
        token.setLimits(bridge, mintingLimit, burningLimit);

        assertEq(token.mintingMaxLimitOf(bridge), mintingLimit);
        assertEq(token.burningMaxLimitOf(bridge), burningLimit);
    }

    // ============ HELPER FUNCTIONS ============

    function _setupBridgeConfiguration() internal {
        vm.startPrank(defaultAdmin);
        bridgeProxy.setBridgeTarget(address(mockBridge));
        bridgeProxy.setBridgeCalldata(abi.encodeWithSignature("executeBridge(address,uint256,bytes)"));
        token.setBridgeProxy(address(bridgeProxy));
        token.setBridgeData("0x1234");
        vm.stopPrank();
    }

    function _startEmissions() internal {
        vm.prank(emissionsManager);
        token.startEmissions();
    }
}
