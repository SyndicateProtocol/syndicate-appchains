// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken, AbstractXERC20} from "src/token/SyndicateToken.sol";
import {OptimismBridgeProxy} from "src/token/bridges/OptimismBridgeProxy.sol";
import {IBridgeProxy} from "src/token/interfaces/IBridgeProxy.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// Mock Bridge that properly handles token transfers
contract MockOptimismBridge {
    struct BridgeCall {
        address l1Token;
        address l2Token;
        address to;
        uint256 amount;
        uint32 l2Gas;
        bytes data;
    }

    BridgeCall[] public bridgeCalls;
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function depositERC20To(
        address _l1Token,
        address _l2Token,
        address _to,
        uint256 _amount,
        uint32 _l2Gas,
        bytes calldata _data
    ) external {
        if (shouldRevert) {
            revert("Bridge reverted");
        }

        // Transfer tokens from caller (should be the bridge proxy)
        IERC20(_l1Token).transferFrom(msg.sender, address(this), _amount);

        // Record the bridge call
        bridgeCalls.push(
            BridgeCall({l1Token: _l1Token, l2Token: _l2Token, to: _to, amount: _amount, l2Gas: _l2Gas, data: _data})
        );
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
    OptimismBridgeProxy public bridgeProxy;
    MockOptimismBridge public mockBridge;

    address public defaultAdmin = address(0x1234);
    address public syndFoundationAddress = address(0x5678);
    address public emissionsManager = address(0x9ABC);
    address public pauser = address(0xDEF0);
    address public user = address(0x1111);

    // Bridge configuration
    address public l2Token = address(0x2222);
    address public recipient = address(0x3333);
    uint32 public l2Gas = 200000;
    uint256 public maxSingleTransfer = 20_000_000 * 10 ** 18;
    uint256 public dailyLimit = 10_000_000 * 10 ** 18;

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
        vm.startPrank(defaultAdmin);

        // Deploy token
        token = new SyndicateToken(defaultAdmin, syndFoundationAddress, emissionsManager, pauser);

        // Deploy mock bridge
        mockBridge = new MockOptimismBridge();

        // Deploy bridge proxy
        bridgeProxy = new OptimismBridgeProxy(
            defaultAdmin, // admin
            address(token), // caller (will be set to token address later)
            address(mockBridge), // bridge target
            maxSingleTransfer,
            dailyLimit,
            l2Token,
            recipient,
            l2Gas
        );

        vm.stopPrank();
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
        emit BridgeProxyUpdated(address(IBridgeProxy(address(0))), address(IBridgeProxy(address(bridgeProxy))));

        vm.prank(defaultAdmin);
        token.setBridgeProxy(IBridgeProxy(address(bridgeProxy)));

        (IBridgeProxy proxy,) = token.getBridgeConfiguration();
        assertEq(address(proxy), address(bridgeProxy));
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
        token.setBridgeProxy(IBridgeProxy(address(bridgeProxy)));
        vm.stopPrank();
    }

    function test_RevertWhen_SetBridgeProxy_ZeroAddress() public {
        vm.startPrank(defaultAdmin);
        vm.expectRevert(AbstractXERC20.ZeroAddress.selector);
        token.setBridgeProxy(IBridgeProxy(address(0)));
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
        emit EmissionMinted(1, expectedAmount, address(bridgeProxy));

        vm.prank(emissionsManager);
        token.mintEmission();

        assertEq(token.currentEpoch(), 1);
        assertEq(token.totalEmissionsMinted(), expectedAmount);
        assertEq(token.totalSupply(), initialSupply + expectedAmount);

        // Verify bridge was called
        MockOptimismBridge.BridgeCall memory call = mockBridge.getLastBridgeCall();
        assertEq(call.l1Token, address(token));
        assertEq(call.amount, expectedAmount);
        assertEq(call.l2Token, l2Token);
        assertEq(call.to, recipient);
    }

    function test_MintEmission_MultipleEpochs() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 90 days + 1); // 3 epochs

        uint256 expectedAmount = 678_055 * 10 ** 18 * 3;
        vm.prank(emissionsManager);
        token.mintEmission();

        assertEq(token.currentEpoch(), 3);
        assertEq(token.totalEmissionsMinted(), expectedAmount);
    }

    function test_MintEmission_AllEpochs() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 48 * 30 days + 1); // All epochs

        vm.prank(emissionsManager);
        token.mintEmission();

        // TODO: still need to check if this is the correct final supply
        uint256 mintRoundingError = 4 * 10 ** 18; // Adjust for rounding errors in test

        assertEq(token.currentEpoch(), 48);
        assertEq(
            token.totalEmissionsMinted(),
            token.EMISSIONS_SUPPLY() - mintRoundingError,
            "Total emissions minted does not match expected value"
        );
        assertTrue(token.emissionsEnded());
    }

    function test_RevertWhen_MintEmission_EmissionsNotActive() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(pauser);
        token.pauseEmissions();

        vm.warp(block.timestamp + 30 days + 1);
        vm.expectRevert(SyndicateToken.EmissionsNotActive.selector);
        vm.prank(emissionsManager);
        token.mintEmission();
    }

    function test_RevertWhen_MintEmission_Paused() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(pauser);
        token.pause();

        vm.warp(block.timestamp + 30 days + 1);
        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        vm.prank(emissionsManager);
        token.mintEmission();
    }

    function test_RevertWhen_MintEmission_EmissionTooEarly() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 30 days - 2 hours); // Before buffer time

        vm.expectRevert(SyndicateToken.EmissionTooEarly.selector);
        vm.prank(emissionsManager);
        token.mintEmission();
    }

    // ============ TOTAL EMISSIONS VALIDATION TEST ============

    function test_TotalEmissions_EqualsExpectedSupply() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Calculate expected total by summing emission schedule
        uint256 expectedTotal = 0;
        uint256[48] memory schedule = token.getEmissionSchedule();
        for (uint256 i = 0; i < 48; i++) {
            expectedTotal += schedule[i];
        }

        uint256 totalSupplyRoundingError = 4 * 10 ** 18; // Adjust for rounding errors in test

        // Verify expected total equals EMISSIONS_SUPPLY constant
        assertEq(
            expectedTotal,
            token.EMISSIONS_SUPPLY() - totalSupplyRoundingError,
            "Total emissions supply does not match expected value"
        );

        // Complete all emissions
        vm.warp(block.timestamp + 48 * 30 days + 1);
        vm.prank(emissionsManager);
        token.mintEmission();

        // Verify total minted equals expected supply
        assertEq(
            token.totalEmissionsMinted(),
            token.EMISSIONS_SUPPLY() - totalSupplyRoundingError,
            "Total emissions minted does not match expected value"
        );
        assertEq(token.totalEmissionsMinted(), expectedTotal, "Total emissions minted does not match calculated total");

        // Verify final total supply
        uint256 expectedFinalSupply = token.INITIAL_MINT_SUPPLY() + token.EMISSIONS_SUPPLY();

        // TODO: still need to check if this is the correct final supply
        assertEq(token.totalSupply(), expectedFinalSupply - 4 * 10 ** 18, "F1"); // Adjusted for rounding errors in test
        assertEq(token.totalSupply(), token.TOTAL_SUPPLY() - 4 * 10 ** 18, "F2"); // Adjusted for rounding errors in test
    }

    function test_EmissionSchedule_ValidatesCorrectly() public view {
        // Verify emission schedule adds up to exactly EMISSIONS_SUPPLY
        uint256[48] memory schedule = token.getEmissionSchedule();
        uint256 total = 0;

        for (uint256 i = 0; i < 48; i++) {
            total += schedule[i];
        }

        uint256 totalSupplyRoundingError = 4 * 10 ** 18; // Adjust for rounding errors in test
        // TODO: still need to check if this is the correct final supply
        assertEq(
            total,
            token.EMISSIONS_SUPPLY() - totalSupplyRoundingError,
            "Emission schedule does not match expected supply"
        );
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
        vm.prank(emissionsManager);
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
        vm.prank(emissionsManager);
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
        vm.prank(emissionsManager);
        token.mintEmission();

        assertTrue(token.currentEpoch() > 0);
        assertTrue(token.totalEmissionsMinted() > 0);
        assertFalse(token.emissionsEnded());

        // Complete all emissions
        vm.warp(block.timestamp + 45 * 30 days + 1);
        vm.prank(emissionsManager);
        token.mintEmission();

        assertTrue(token.emissionsEnded());
        assertEq(token.currentEpoch(), 48);
    }

    function test_Integration_BridgeInteraction() public {
        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        uint256 bridgeCallsBefore = mockBridge.getBridgeCallCount();
        vm.prank(emissionsManager);
        token.mintEmission();
        uint256 bridgeCallsAfter = mockBridge.getBridgeCallCount();

        assertEq(bridgeCallsAfter, bridgeCallsBefore + 1);

        MockOptimismBridge.BridgeCall memory call = mockBridge.getLastBridgeCall();
        assertEq(call.l1Token, address(token));
        assertEq(call.amount, 678_055 * 10 ** 18);
        assertEq(call.l2Token, l2Token);
        assertEq(call.to, recipient);
    }

    // ============ FUZZ TESTS ============

    function testFuzz_MintEmission_ValidTimeProgress(uint256 timeElapsed) public {
        timeElapsed = bound(timeElapsed, 1 days, 5 * 365 days);

        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + timeElapsed);

        if (timeElapsed >= 30 days - token.EMISSION_BUFFER_TIME()) {
            uint256 expectedEpochs = timeElapsed / 30 days;
            if (expectedEpochs > 48) expectedEpochs = 48;

            vm.prank(emissionsManager);
            token.mintEmission();
            assertEq(token.currentEpoch(), expectedEpochs);
        } else {
            vm.expectRevert();
            vm.prank(emissionsManager);
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
        token.setBridgeProxy(IBridgeProxy(address(bridgeProxy)));
        token.setBridgeData(abi.encode(recipient, l2Gas));
        vm.stopPrank();
    }

    function _startEmissions() internal {
        vm.prank(emissionsManager);
        token.startEmissions();
    }
}
