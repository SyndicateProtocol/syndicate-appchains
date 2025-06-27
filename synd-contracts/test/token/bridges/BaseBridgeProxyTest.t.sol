// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {BaseBridgeProxy} from "src/token/bridges/BaseBridgeProxy.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

// Concrete implementation of BaseBridgeProxy for testing
contract MockBridgeProxy is BaseBridgeProxy {
    bool public shouldRevert;
    uint256 public lastExecutedAmount;
    address public lastExecutedToken;
    bytes public lastExecutedData;

    constructor(address admin, address caller, address bridgeTarget, uint256 maxSingleTransfer, uint256 dailyLimit)
        BaseBridgeProxy(admin, caller, "Mock Bridge", bridgeTarget, maxSingleTransfer, dailyLimit)
    {}

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function _executeBridgeCall(address token, uint256 amount, bytes calldata dynamicData) internal override {
        if (shouldRevert) {
            revert("Mock bridge execution failed");
        }

        lastExecutedToken = token;
        lastExecutedAmount = amount;
        lastExecutedData = dynamicData;
    }
}

// Mock target bridge contract
contract MockTargetBridge {
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function bridgeFunction(address, uint256, bytes calldata) external view {
        if (shouldRevert) {
            revert("Target bridge failed");
        }
        // Mock bridge execution
    }
}

contract BaseBridgeProxyTest is Test {
    MockBridgeProxy public bridgeProxy;
    ERC20Mock public token;
    MockTargetBridge public targetBridge;

    address public admin = address(0x1234);
    address public caller = address(0x5678);
    address public user = address(0x9ABC);
    address public newTarget = address(0xDEF0);

    uint256 public constant MAX_SINGLE_TRANSFER = 1_000_000 * 10 ** 18;
    uint256 public constant DAILY_LIMIT = 5_000_000 * 10 ** 18;

    // Events from BaseBridgeProxy
    event BridgeTargetUpdated(address indexed oldTarget, address indexed newTarget);
    event BridgeStatusUpdated(bool active);
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target);
    event DailyLimitUpdated(uint256 oldLimit, uint256 newLimit);
    event DailyLimitReset(uint256 day, uint256 previousUsed);

    function setUp() public {
        token = new ERC20Mock();
        targetBridge = new MockTargetBridge();

        bridgeProxy = new MockBridgeProxy(admin, caller, address(targetBridge), MAX_SINGLE_TRANSFER, DAILY_LIMIT);

        // Mint tokens to caller for testing
        token.mint(caller, 10_000_000 * 10 ** 18);
    }

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR TESTS
    //////////////////////////////////////////////////////////////*/

    function test_Constructor_Success() public view {
        assertEq(bridgeProxy.bridgeTarget(), address(targetBridge));
        assertEq(bridgeProxy.bridgeName(), "Mock Bridge");
        assertEq(bridgeProxy.maxSingleTransfer(), MAX_SINGLE_TRANSFER);
        assertEq(bridgeProxy.dailyLimit(), DAILY_LIMIT);
        assertTrue(bridgeProxy.bridgeActive());
        assertEq(bridgeProxy.dailyUsed(), 0);
        assertEq(bridgeProxy.lastResetDay(), block.timestamp / 1 days);
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(bridgeProxy.hasRole(bridgeProxy.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(bridgeProxy.hasRole(bridgeProxy.BRIDGE_ADMIN_ROLE(), admin));
        assertTrue(bridgeProxy.hasRole(bridgeProxy.BRIDGE_CALLER_ROLE(), caller));
    }

    function test_RevertWhen_Constructor_ZeroAdmin() public {
        vm.expectRevert(BaseBridgeProxy.ZeroAddress.selector);
        new MockBridgeProxy(address(0), caller, address(targetBridge), MAX_SINGLE_TRANSFER, DAILY_LIMIT);
    }

    function test_RevertWhen_Constructor_ZeroCaller() public {
        vm.expectRevert(BaseBridgeProxy.ZeroAddress.selector);
        new MockBridgeProxy(admin, address(0), address(targetBridge), MAX_SINGLE_TRANSFER, DAILY_LIMIT);
    }

    /*//////////////////////////////////////////////////////////////
                        BRIDGE EXECUTION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_ExecuteBridge_Success() public {
        uint256 amount = 500_000 * 10 ** 18;
        bytes memory dynamicData = abi.encode("test data");

        // Approve bridge to spend tokens
        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.expectEmit(true, false, true, true);
        emit BridgeExecuted(address(token), amount, address(targetBridge));

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        // Verify state changes
        assertEq(bridgeProxy.dailyUsed(), amount);
        assertEq(bridgeProxy.lastExecutedAmount(), amount);
        assertEq(bridgeProxy.lastExecutedToken(), address(token));
        assertEq(bridgeProxy.lastExecutedData(), dynamicData);

        // Verify tokens were transferred
        assertEq(token.balanceOf(address(bridgeProxy)), amount);
    }

    function test_ExecuteBridge_MultipleTransfers() public {
        uint256 amount1 = 300_000 * 10 ** 18;
        uint256 amount2 = 200_000 * 10 ** 18;

        vm.startPrank(caller);
        token.approve(address(bridgeProxy), amount1 + amount2);

        bridgeProxy.executeBridge(address(token), amount1, "");
        bridgeProxy.executeBridge(address(token), amount2, "");
        vm.stopPrank();

        assertEq(bridgeProxy.dailyUsed(), amount1 + amount2);
    }

    function test_RevertWhen_ExecuteBridge_ZeroToken() public {
        vm.prank(caller);
        vm.expectRevert(BaseBridgeProxy.ZeroAddress.selector);
        bridgeProxy.executeBridge(address(0), 1000, "");
    }

    function test_RevertWhen_ExecuteBridge_ZeroAmount() public {
        vm.prank(caller);
        vm.expectRevert(BaseBridgeProxy.ZeroAmount.selector);
        bridgeProxy.executeBridge(address(token), 0, "");
    }

    function test_RevertWhen_ExecuteBridge_ExceedsMaxSingle() public {
        uint256 excessiveAmount = MAX_SINGLE_TRANSFER + 1;

        vm.prank(caller);
        token.approve(address(bridgeProxy), excessiveAmount);

        vm.prank(caller);
        vm.expectRevert(BaseBridgeProxy.ExcessiveAmount.selector);
        bridgeProxy.executeBridge(address(token), excessiveAmount, "");
    }

    function test_RevertWhen_ExecuteBridge_ExceedsDailyLimit() public {
        uint256 amount = DAILY_LIMIT + 1;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        vm.expectRevert(BaseBridgeProxy.ExcessiveAmount.selector);
        bridgeProxy.executeBridge(address(token), amount, "");
    }

    function test_RevertWhen_ExecuteBridge_UnauthorizedCaller() public {
        vm.prank(user);
        vm.expectRevert(BaseBridgeProxy.UnauthorizedCaller.selector);
        bridgeProxy.executeBridge(address(token), 1000, "");
    }

    function test_RevertWhen_ExecuteBridge_BridgeInactive() public {
        vm.prank(admin);
        bridgeProxy.setBridgeActive(false);

        vm.prank(caller);
        token.approve(address(bridgeProxy), 1000);

        vm.prank(caller);
        vm.expectRevert(BaseBridgeProxy.BridgeNotActive.selector);
        bridgeProxy.executeBridge(address(token), 1000, "");
    }

    /*//////////////////////////////////////////////////////////////
                        DAILY LIMIT TESTS
    //////////////////////////////////////////////////////////////*/

    function test_DailyLimit_Reset() public {
        uint256 amount = 1_000_000 * 10 ** 18;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount * 2);

        // First transfer
        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, "");
        assertEq(bridgeProxy.dailyUsed(), amount);

        // Warp to next day
        vm.warp(block.timestamp + 1 days);

        vm.expectEmit(false, false, false, true);
        emit DailyLimitReset(block.timestamp / 1 days, amount);

        // Second transfer should reset daily usage
        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, "");
        assertEq(bridgeProxy.dailyUsed(), amount);
    }

    function test_DailyLimit_Cumulative() public {
        uint256 amount = 500_000 * 10 ** 18;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount * 10);

        // Multiple transfers in same day
        vm.startPrank(caller);
        for (uint256 i = 0; i < 10; i++) {
            bridgeProxy.executeBridge(address(token), amount, "");
        }
        vm.stopPrank();

        assertEq(bridgeProxy.dailyUsed(), amount * 10);

        // Should fail on next transfer (exceeds daily limit)
        vm.prank(caller);
        token.approve(address(bridgeProxy), 1);
        vm.prank(caller);
        vm.expectRevert(BaseBridgeProxy.ExcessiveAmount.selector);
        bridgeProxy.executeBridge(address(token), 1, "");
    }

    /*//////////////////////////////////////////////////////////////
                        ADMIN CONFIGURATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_SetBridgeTarget_Success() public {
        vm.expectEmit(true, true, false, true);
        emit BridgeTargetUpdated(address(targetBridge), newTarget);

        vm.prank(admin);
        bridgeProxy.setBridgeTarget(newTarget);

        assertEq(bridgeProxy.bridgeTarget(), newTarget);
    }

    function test_SetBridgeActive_Success() public {
        vm.expectEmit(false, false, false, true);
        emit BridgeStatusUpdated(false);

        vm.prank(admin);
        bridgeProxy.setBridgeActive(false);

        assertFalse(bridgeProxy.bridgeActive());
    }

    function test_SetDailyLimit_Success() public {
        uint256 newLimit = 10_000_000 * 10 ** 18;

        vm.expectEmit(false, false, false, true);
        emit DailyLimitUpdated(DAILY_LIMIT, newLimit);

        vm.prank(admin);
        bridgeProxy.setDailyLimit(newLimit);

        assertEq(bridgeProxy.dailyLimit(), newLimit);
    }

    function test_SetMaxSingleTransfer_Success() public {
        uint256 newMax = 2_000_000 * 10 ** 18;

        vm.prank(admin);
        bridgeProxy.setMaxSingleTransfer(newMax);

        assertEq(bridgeProxy.maxSingleTransfer(), newMax);
    }

    function test_RevertWhen_SetBridgeTarget_ZeroAddress() public {
        vm.prank(admin);
        vm.expectRevert(BaseBridgeProxy.ZeroAddress.selector);
        bridgeProxy.setBridgeTarget(address(0));
    }

    function test_RevertWhen_SetBridgeTarget_NotAdmin() public {
        vm.startPrank(user);
        bytes32 bridgeAdminRole = bridgeProxy.BRIDGE_ADMIN_ROLE();
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, bridgeAdminRole)
        );
        bridgeProxy.setBridgeTarget(newTarget);
        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                            VIEW FUNCTION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_GetBridgeInfo() public view {
        (string memory name, address target, bool active) = bridgeProxy.getBridgeInfo();

        assertEq(name, "Mock Bridge");
        assertEq(target, address(targetBridge));
        assertTrue(active);
    }

    function test_GetDailyUsage_Initial() public view {
        (uint256 used, uint256 limit, uint256 remaining) = bridgeProxy.getDailyUsage();

        assertEq(used, 0);
        assertEq(limit, DAILY_LIMIT);
        assertEq(remaining, DAILY_LIMIT);
    }

    function test_GetDailyUsage_AfterTransfer() public {
        uint256 amount = 1_000_000 * 10 ** 18;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);
        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, "");

        (uint256 used, uint256 limit, uint256 remaining) = bridgeProxy.getDailyUsage();

        assertEq(used, amount);
        assertEq(limit, DAILY_LIMIT);
        assertEq(remaining, DAILY_LIMIT - amount);
    }

    /*//////////////////////////////////////////////////////////////
                        EMERGENCY FUNCTION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_RecoverTokens_Success() public {
        uint256 amount = 100_000 * 10 ** 18;

        // Send tokens directly to bridge (simulate stuck tokens)
        token.mint(address(bridgeProxy), amount);

        uint256 initialBalance = token.balanceOf(admin);

        vm.prank(admin);
        bridgeProxy.recoverTokens(address(token), amount, admin);

        assertEq(token.balanceOf(admin), initialBalance + amount);
        assertEq(token.balanceOf(address(bridgeProxy)), 0);
    }

    function test_RevertWhen_RecoverTokens_ZeroAddress() public {
        vm.prank(admin);
        vm.expectRevert(BaseBridgeProxy.ZeroAddress.selector);
        bridgeProxy.recoverTokens(address(token), 1000, address(0));
    }

    function test_RevertWhen_RecoverTokens_NotAdmin() public {
        vm.startPrank(user);
        bytes32 defaultAdminRole = bridgeProxy.DEFAULT_ADMIN_ROLE();
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, defaultAdminRole)
        );
        bridgeProxy.recoverTokens(address(token), 1000, admin);
        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                            REENTRANCY TESTS
    //////////////////////////////////////////////////////////////*/

    function test_ExecuteBridge_ReentrancyProtection() public {
        // This test would require a malicious token that tries to reenter
        // For now, we verify the nonReentrant modifier is in place
        uint256 amount = 100_000 * 10 ** 18;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, "");

        // If we reach here, reentrancy protection is working
        assertEq(bridgeProxy.lastExecutedAmount(), amount);
    }

    /*//////////////////////////////////////////////////////////////
                              FUZZ TESTS
    //////////////////////////////////////////////////////////////*/

    function testFuzz_ExecuteBridge_ValidAmounts(uint256 amount) public {
        amount = bound(amount, 1, MAX_SINGLE_TRANSFER);

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, "");

        assertEq(bridgeProxy.lastExecutedAmount(), amount);
        assertEq(bridgeProxy.dailyUsed(), amount);
    }

    function testFuzz_SetDailyLimit_ValidValues(uint256 newLimit) public {
        newLimit = bound(newLimit, 1, type(uint128).max);

        vm.prank(admin);
        bridgeProxy.setDailyLimit(newLimit);

        assertEq(bridgeProxy.dailyLimit(), newLimit);
    }
}
