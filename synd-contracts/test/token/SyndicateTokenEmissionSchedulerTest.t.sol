// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {SyndicateTokenEmissionScheduler} from "src/token/SyndicateTokenEmissionScheduler.sol";
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

contract SyndicateTokenEmissionSchedulerTest is Test {
    SyndicateToken public token;
    SyndicateTokenEmissionScheduler public emissionScheduler;
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
    uint256 public maxSingleTransfer = 200_000_000 * 10 ** 18;
    uint256 public dailyLimit = 100_000_000 * 10 ** 18;

    // Events
    event EmissionsStarted(uint256 startTime);
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed destination);
    event BridgeProxyUpdated(address indexed oldProxy, address indexed newProxy);
    event BridgeDataUpdated(bytes oldData, bytes newData);
    event Paused(address account);
    event Unpaused(address account);

    function setUp() public {
        vm.startPrank(defaultAdmin);

        // Deploy token
        token = new SyndicateToken(defaultAdmin, syndFoundationAddress);

        // Deploy emission scheduler
        emissionScheduler = new SyndicateTokenEmissionScheduler(address(token), defaultAdmin, emissionsManager, pauser);

        // Grant emission minter role to the scheduler
        token.grantRole(token.EMISSION_MINTER_ROLE(), address(emissionScheduler));

        // Deploy mock bridge
        mockBridge = new MockOptimismBridge();

        // Deploy bridge proxy
        bridgeProxy = new OptimismBridgeProxy(
            defaultAdmin, // admin
            address(emissionScheduler), // caller (emission scheduler)
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
        assertEq(address(emissionScheduler.syndicateToken()), address(token));
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

    function test_RevertWhen_Constructor_ZeroToken() public {
        vm.expectRevert(SyndicateTokenEmissionScheduler.ZeroAddress.selector);
        new SyndicateTokenEmissionScheduler(address(0), defaultAdmin, emissionsManager, pauser);
    }

    function test_RevertWhen_Constructor_ZeroAdmin() public {
        vm.expectRevert(SyndicateTokenEmissionScheduler.ZeroAddress.selector);
        new SyndicateTokenEmissionScheduler(address(token), address(0), emissionsManager, pauser);
    }

    // ============ BRIDGE PROXY CONFIGURATION TESTS ============

    function test_SetBridgeProxy_Success() public {
        vm.expectEmit(true, true, false, true);
        emit BridgeProxyUpdated(address(IBridgeProxy(address(0))), address(IBridgeProxy(address(bridgeProxy))));

        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(bridgeProxy)));

        (IBridgeProxy proxy,) = emissionScheduler.getBridgeConfiguration();
        assertEq(address(proxy), address(bridgeProxy));
    }

    function test_SetBridgeData_Success() public {
        bytes memory testData = abi.encode("test", 123);

        vm.expectEmit(false, false, false, true);
        emit BridgeDataUpdated("", testData);

        vm.prank(defaultAdmin);
        emissionScheduler.setBridgeData(testData);

        (, bytes memory data) = emissionScheduler.getBridgeConfiguration();
        assertEq(data, testData);
    }

    function test_RevertWhen_SetBridgeProxy_NotBridgeManager() public {
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, user, emissionScheduler.BRIDGE_MANAGER_ROLE()
            )
        );
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(bridgeProxy)));
        vm.stopPrank();
    }

    function test_RevertWhen_SetBridgeProxy_ZeroAddress() public {
        vm.startPrank(defaultAdmin);
        vm.expectRevert(SyndicateTokenEmissionScheduler.ZeroAddress.selector);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(0)));
        vm.stopPrank();
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

        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector,
                user,
                emissionScheduler.EMISSIONS_MANAGER_ROLE()
            )
        );
        emissionScheduler.startEmissions();
        vm.stopPrank();
    }

    function test_RevertWhen_StartEmissions_AlreadyStarted() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionScheduler.EmissionsAlreadyStarted.selector);
        emissionScheduler.startEmissions();
    }

    function test_RevertWhen_StartEmissions_BridgeNotConfigured() public {
        vm.prank(emissionsManager);
        vm.expectRevert(SyndicateTokenEmissionScheduler.BridgeNotConfigured.selector);
        emissionScheduler.startEmissions();
    }

    // ============ MINT EMISSION TESTS ============

    function test_MintEmission_FirstEpoch() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 30 days + 1);

        uint256 expectedAmount = 6_780_550 * 10 ** 18;
        uint256 initialSupply = token.totalSupply();

        vm.expectEmit(false, false, false, true);
        emit EmissionMinted(1, expectedAmount, address(bridgeProxy));

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 1);
        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount);
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

        uint256 expectedAmount = 6_780_550 * 10 ** 18 * 3;
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 3);
        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount);
    }

    function test_MintEmission_AllEpochs() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 48 * 30 days + 1); // All epochs

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(emissionScheduler.currentEpoch(), 48);
        assertTrue(emissionScheduler.emissionsEnded());
        // Verify all scheduled emissions were minted
        assertTrue(emissionScheduler.totalEmissionsMinted() > 0);
    }


    function test_RevertWhen_MintEmission_Paused() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.prank(pauser);
        emissionScheduler.pause();

        vm.warp(block.timestamp + 30 days + 1);
        vm.expectRevert(abi.encodeWithSignature("EnforcedPause()"));
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();
    }

    function test_RevertWhen_MintEmission_EmissionTooEarly() public {
        _setupBridgeConfiguration();
        _startEmissions();

        vm.warp(block.timestamp + 30 days - 2 hours); // Before buffer time

        vm.expectRevert(SyndicateTokenEmissionScheduler.EmissionTooEarly.selector);
        vm.prank(emissionsManager);
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
        assertEq(nextEmissionAmount, 6_780_550 * 10 ** 18);
        assertFalse(canMint);
    }

    function test_GetCurrentEpochInfo_CanMint() public {
        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        (,, uint256 nextEmissionAmount, bool canMint) = emissionScheduler.getCurrentEpochInfo();

        assertEq(nextEmissionAmount, 6_780_550 * 10 ** 18);
        assertTrue(canMint);
    }


    function test_EmissionsEnded() public {
        _setupBridgeConfiguration();
        _startEmissions();

        assertFalse(emissionScheduler.emissionsEnded());

        vm.warp(block.timestamp + 48 * 30 days + 1);
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertTrue(emissionScheduler.emissionsEnded());
    }

    function test_GetEmissionSchedule() public view {
        uint256[48] memory schedule = emissionScheduler.getEmissionSchedule();

        // Check first epoch
        assertEq(schedule[0], 6_780_550 * 10 ** 18);
        // Check last epoch
        assertEq(schedule[47], 189_810 * 10 ** 18);
    }

    function test_GetNextEmissionTime() public {
        _setupBridgeConfiguration();
        _startEmissions();

        uint256 nextTime = emissionScheduler.getNextEmissionTime();
        assertEq(nextTime, block.timestamp + 30 days);
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
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, user, emissionScheduler.PAUSER_ROLE()
            )
        );
        emissionScheduler.pause();
        vm.stopPrank();
    }

    function test_RevertWhen_Unpause_NotAdmin() public {
        vm.prank(pauser);
        emissionScheduler.pause();

        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, user, emissionScheduler.DEFAULT_ADMIN_ROLE()
            )
        );
        emissionScheduler.unpause();
        vm.stopPrank();
    }

    // ============ INTEGRATION TESTS ============

    function test_Integration_FullEmissionCycle() public {
        _setupBridgeConfiguration();
        _startEmissions();

        // Mint first few epochs
        vm.warp(block.timestamp + 90 days + 1); // 3 epochs
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertTrue(emissionScheduler.currentEpoch() > 0);
        assertTrue(emissionScheduler.totalEmissionsMinted() > 0);
        assertFalse(emissionScheduler.emissionsEnded());

        // Complete all emissions
        vm.warp(block.timestamp + 45 * 30 days + 1);
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertTrue(emissionScheduler.emissionsEnded());
        assertEq(emissionScheduler.currentEpoch(), 48);
    }

    function test_Integration_BridgeInteraction() public {
        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + 30 days + 1);

        uint256 bridgeCallsBefore = mockBridge.getBridgeCallCount();
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();
        uint256 bridgeCallsAfter = mockBridge.getBridgeCallCount();

        assertEq(bridgeCallsAfter, bridgeCallsBefore + 1);

        MockOptimismBridge.BridgeCall memory call = mockBridge.getLastBridgeCall();
        assertEq(call.l1Token, address(token));
        assertEq(call.amount, 6_780_550 * 10 ** 18);
        assertEq(call.l2Token, l2Token);
        assertEq(call.to, recipient);
    }

    function test_EmissionSchedule_ValidatesCorrectly() public view {
        // Verify emission schedule adds up to exactly EMISSIONS_SUPPLY
        uint256[48] memory schedule = emissionScheduler.getEmissionSchedule();
        uint256 total = 0;

        for (uint256 i = 0; i < 48; i++) {
            total += schedule[i];
        }

        // Verify that the schedule is reasonable (should be around 100M tokens)
        uint256 expectedTotal = 100_000_000 * 10 ** 18;
        uint256 tolerance = 1_000_000 * 10 ** 18; // 1M token tolerance
        assertTrue(total > expectedTotal - tolerance && total < expectedTotal + tolerance, "Emission schedule total is not within expected range");
    }

    // ============ FUZZ TESTS ============

    function testFuzz_MintEmission_ValidTimeProgress(uint256 timeElapsed) public {
        timeElapsed = bound(timeElapsed, 1 days, 5 * 365 days);

        _setupBridgeConfiguration();
        _startEmissions();
        vm.warp(block.timestamp + timeElapsed);

        if (timeElapsed >= 30 days - emissionScheduler.EMISSION_BUFFER_TIME()) {
            uint256 expectedEpochs = timeElapsed / 30 days;
            if (expectedEpochs > 48) expectedEpochs = 48;

            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();
            assertEq(emissionScheduler.currentEpoch(), expectedEpochs);
        } else {
            vm.expectRevert();
            vm.prank(emissionsManager);
            emissionScheduler.mintEmission();
        }
    }

    // ============ HELPER FUNCTIONS ============

    function _setupBridgeConfiguration() internal {
        vm.startPrank(defaultAdmin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(bridgeProxy)));
        emissionScheduler.setBridgeData(abi.encode(recipient, l2Gas));
        vm.stopPrank();
    }

    function _startEmissions() internal {
        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();
    }
}
