// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {OptimismBridgeProxy} from "src/token/bridges/OptimismBridgeProxy.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

// Mock Optimism Bridge contract
contract MockOptimismBridge {
    struct DepositCall {
        address l1Token;
        address l2Token;
        address to;
        uint256 amount;
        uint32 l2Gas;
        bytes data;
    }

    DepositCall[] public depositCalls;
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
            revert("Optimism bridge failed");
        }

        // Transfer tokens to simulate bridge behavior
        IERC20(_l1Token).transferFrom(msg.sender, address(this), _amount);

        depositCalls.push(
            DepositCall({l1Token: _l1Token, l2Token: _l2Token, to: _to, amount: _amount, l2Gas: _l2Gas, data: _data})
        );
    }

    function getLastDepositCall() external view returns (DepositCall memory) {
        require(depositCalls.length > 0, "No deposits");
        return depositCalls[depositCalls.length - 1];
    }

    function getDepositCallCount() external view returns (uint256) {
        return depositCalls.length;
    }
}

contract OptimismBridgeProxyTest is Test {
    OptimismBridgeProxy public bridgeProxy;
    MockOptimismBridge public optimismBridge;
    ERC20Mock public token;

    address public admin = address(0x1234);
    address public caller = address(0x5678);
    address public user = address(0x9ABC);

    address public l2Token = address(0xDEF0);
    address public recipient = address(0x1111);
    uint32 public l2Gas = 200000;

    uint256 public constant MAX_SINGLE_TRANSFER = 1_000_000 * 10 ** 18;
    uint256 public constant DAILY_LIMIT = 5_000_000 * 10 ** 18;

    // Events
    event OptimismConfigUpdated(address l2Token, address recipient, uint32 l2Gas);
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target);

    function setUp() public {
        token = new ERC20Mock();
        optimismBridge = new MockOptimismBridge();

        bridgeProxy = new OptimismBridgeProxy(
            admin, caller, address(optimismBridge), MAX_SINGLE_TRANSFER, DAILY_LIMIT, l2Token, recipient, l2Gas
        );

        // Mint tokens to caller for testing
        token.mint(caller, 10_000_000 * 10 ** 18);
    }

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR TESTS
    //////////////////////////////////////////////////////////////*/

    function test_Constructor_Success() public view {
        assertEq(bridgeProxy.bridgeTarget(), address(optimismBridge));
        assertEq(bridgeProxy.l2Token(), l2Token);
        assertEq(bridgeProxy.recipient(), recipient);
        assertEq(bridgeProxy.l2Gas(), l2Gas);
        assertEq(bridgeProxy.maxSingleTransfer(), MAX_SINGLE_TRANSFER);
        assertEq(bridgeProxy.dailyLimit(), DAILY_LIMIT);
        assertTrue(bridgeProxy.bridgeActive());
    }

    function test_Constructor_BridgeInfo() public view {
        (string memory name, address target, bool active) = bridgeProxy.getBridgeInfo();

        assertEq(name, "Optimism Bridge");
        assertEq(target, address(optimismBridge));
        assertTrue(active);
    }

    function test_Constructor_RoleAssignment() public view {
        assertTrue(bridgeProxy.hasRole(bridgeProxy.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(bridgeProxy.hasRole(bridgeProxy.BRIDGE_ADMIN_ROLE(), admin));
        assertTrue(bridgeProxy.hasRole(bridgeProxy.BRIDGE_CALLER_ROLE(), caller));
    }

    /*//////////////////////////////////////////////////////////////
                        BRIDGE EXECUTION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_ExecuteBridge_Success_DefaultParams() public {
        uint256 amount = 500_000 * 10 ** 18;
        bytes memory dynamicData = "";

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.expectEmit(true, false, true, true);
        emit BridgeExecuted(address(token), amount, address(optimismBridge));

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        // Verify Optimism bridge was called correctly
        MockOptimismBridge.DepositCall memory call = optimismBridge.getLastDepositCall();
        assertEq(call.l1Token, address(token));
        assertEq(call.l2Token, l2Token);
        assertEq(call.to, recipient);
        assertEq(call.amount, amount);
        assertEq(call.l2Gas, l2Gas);
        assertEq(call.data, "");
    }

    function test_ExecuteBridge_Success_CustomParams() public {
        uint256 amount = 300_000 * 10 ** 18;
        address customRecipient = address(0x2222);
        uint32 customGas = 300000;
        bytes memory dynamicData = abi.encode(customRecipient, customGas);

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        // Verify custom parameters were used
        MockOptimismBridge.DepositCall memory call = optimismBridge.getLastDepositCall();
        assertEq(call.l1Token, address(token));
        assertEq(call.l2Token, l2Token);
        assertEq(call.to, customRecipient);
        assertEq(call.amount, amount);
        assertEq(call.l2Gas, customGas);
    }

    function test_ExecuteBridge_Success_MultipleTransfers() public {
        uint256 amount1 = 200_000 * 10 ** 18;
        uint256 amount2 = 300_000 * 10 ** 18;

        vm.startPrank(caller);
        token.approve(address(bridgeProxy), amount1 + amount2);

        bridgeProxy.executeBridge(address(token), amount1, "");
        bridgeProxy.executeBridge(address(token), amount2, "");
        vm.stopPrank();

        assertEq(optimismBridge.getDepositCallCount(), 2);
        assertEq(bridgeProxy.dailyUsed(), amount1 + amount2);
    }

    function test_RevertWhen_ExecuteBridge_OptimismBridgeFails() public {
        uint256 amount = 100_000 * 10 ** 18;

        optimismBridge.setShouldRevert(true);

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        vm.expectRevert("Optimism bridge failed");
        bridgeProxy.executeBridge(address(token), amount, "");
    }

    function test_ExecuteBridge_TokenApprovalHandling() public {
        uint256 amount = 100_000 * 10 ** 18;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        uint256 initialApproval = token.allowance(address(bridgeProxy), address(optimismBridge));
        assertEq(initialApproval, 0);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, "");

        // After successful execution, tokens should be transferred to bridge
        assertEq(token.balanceOf(address(optimismBridge)), amount);
        assertEq(token.balanceOf(address(bridgeProxy)), 0);
    }

    /*//////////////////////////////////////////////////////////////
                        CONFIGURATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_SetOptimismConfig_Success() public {
        address newL2Token = address(0x3333);
        address newRecipient = address(0x4444);
        uint32 newL2Gas = 250000;

        vm.expectEmit(false, false, false, true);
        emit OptimismConfigUpdated(newL2Token, newRecipient, newL2Gas);

        vm.prank(admin);
        bridgeProxy.setOptimismConfig(newL2Token, newRecipient, newL2Gas);

        assertEq(bridgeProxy.l2Token(), newL2Token);
        assertEq(bridgeProxy.recipient(), newRecipient);
        assertEq(bridgeProxy.l2Gas(), newL2Gas);
    }

    function test_RevertWhen_SetOptimismConfig_NotAdmin() public {
        vm.startPrank(user);
        bytes32 bridgeAdminRole = bridgeProxy.BRIDGE_ADMIN_ROLE();
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, bridgeAdminRole)
        );
        bridgeProxy.setOptimismConfig(l2Token, recipient, l2Gas);
        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                              VIEW FUNCTION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_GetOptimismConfig() public view {
        (address l2TokenAddr, address recipientAddr, uint32 gasLimit) = bridgeProxy.getOptimismConfig();

        assertEq(l2TokenAddr, l2Token);
        assertEq(recipientAddr, recipient);
        assertEq(gasLimit, l2Gas);
    }

    /*//////////////////////////////////////////////////////////////
                        INTEGRATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_Integration_FullBridgeFlow() public {
        uint256 amount = 750_000 * 10 ** 18;
        address customRecipient = address(0x5555);
        uint32 customGas = 180000;
        bytes memory dynamicData = abi.encode(customRecipient, customGas);

        uint256 initialBalance = token.balanceOf(caller);
        uint256 initialBridgeBalance = token.balanceOf(address(optimismBridge));

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        // Verify token transfer
        assertEq(token.balanceOf(caller), initialBalance - amount);
        assertEq(token.balanceOf(address(optimismBridge)), initialBridgeBalance + amount);

        // Verify bridge call
        MockOptimismBridge.DepositCall memory call = optimismBridge.getLastDepositCall();
        assertEq(call.l1Token, address(token));
        assertEq(call.l2Token, l2Token);
        assertEq(call.to, customRecipient);
        assertEq(call.amount, amount);
        assertEq(call.l2Gas, customGas);

        // Verify daily usage tracking
        assertEq(bridgeProxy.dailyUsed(), amount);
    }

    function test_Integration_DailyLimitReset() public {
        uint256 amount = 1_000_000 * 10 ** 18;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount * 2);

        // First transfer
        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, "");
        assertEq(bridgeProxy.dailyUsed(), amount);

        // Warp to next day
        vm.warp(block.timestamp + 1 days);

        // Second transfer should reset daily usage
        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, "");
        assertEq(bridgeProxy.dailyUsed(), amount); // Reset to just this transfer
        assertEq(optimismBridge.getDepositCallCount(), 2);
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

        MockOptimismBridge.DepositCall memory call = optimismBridge.getLastDepositCall();
        assertEq(call.amount, amount);
        assertEq(bridgeProxy.dailyUsed(), amount);
    }

    function testFuzz_SetOptimismConfig_ValidGas(uint256 gasLimit) public {
        gasLimit = bound(gasLimit, 21000, 10_000_000); // Reasonable gas limits

        vm.prank(admin);
        bridgeProxy.setOptimismConfig(l2Token, recipient, uint32(gasLimit));

        assertEq(bridgeProxy.l2Gas(), uint32(gasLimit));
    }

    function testFuzz_ExecuteBridge_CustomRecipient(address customRecipient) public {
        vm.assume(customRecipient != address(0));

        uint256 amount = 100_000 * 10 ** 18;
        bytes memory dynamicData = abi.encode(customRecipient, l2Gas);

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        MockOptimismBridge.DepositCall memory call = optimismBridge.getLastDepositCall();
        assertEq(call.to, customRecipient);
    }
}
