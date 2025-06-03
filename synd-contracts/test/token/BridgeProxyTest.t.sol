// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {BridgeProxy} from "src/token/BridgeProxy.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// Mock ERC20 token for testing
contract MockERC20 is ERC20 {
    constructor() ERC20("Mock Token", "MOCK") {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

// Mock bridge target that accepts calls with the correct signature
contract MockBridgeTarget {
    struct BridgeCall {
        address token;
        uint256 amount;
        bytes data;
        uint256 value;
    }

    BridgeCall[] public bridgeCalls;
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    // This function matches what BridgeProxy actually calls: (address, uint256, bytes)
    function bridgeTokens(address token, uint256 amount, bytes calldata data) external payable {
        require(!shouldRevert, "MockBridge: forced revert");

        // Transfer tokens from the caller (BridgeProxy) to this contract
        IERC20(token).transferFrom(msg.sender, address(this), amount);

        bridgeCalls.push(BridgeCall({token: token, amount: amount, data: data, value: msg.value}));
    }

    function getBridgeCallCount() external view returns (uint256) {
        return bridgeCalls.length;
    }

    function getLastBridgeCall() external view returns (BridgeCall memory) {
        require(bridgeCalls.length > 0, "No bridge calls");
        return bridgeCalls[bridgeCalls.length - 1];
    }

    // Fallback function to accept any other call
    fallback() external payable {
        require(!shouldRevert, "MockBridge: forced revert");

        bridgeCalls.push(BridgeCall({token: address(0), amount: 0, data: msg.data, value: msg.value}));
    }

    receive() external payable {
        require(!shouldRevert, "MockBridge: forced revert");
    }
}

contract BridgeProxyTest is Test {
    BridgeProxy public bridgeProxy;
    MockERC20 public token;
    MockBridgeTarget public mockBridgeTarget;

    address public admin;
    address public user;
    address public recipient;

    // Events
    event BridgeTargetUpdated(address indexed oldTarget, address indexed newTarget);
    event BridgeCalldataUpdated(bytes oldCalldata, bytes newCalldata);
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target, bool success);

    function setUp() public {
        admin = makeAddr("admin");
        user = makeAddr("user");
        recipient = makeAddr("recipient");

        bridgeProxy = new BridgeProxy(admin);
        token = new MockERC20();
        mockBridgeTarget = new MockBridgeTarget();
    }

    // ============ CONSTRUCTOR TESTS ============

    function test_Constructor_Success() public view {
        assertTrue(bridgeProxy.hasRole(bridgeProxy.DEFAULT_ADMIN_ROLE(), admin));
        assertTrue(bridgeProxy.hasRole(bridgeProxy.BRIDGE_ADMIN_ROLE(), admin));
    }

    function test_RevertWhen_Constructor_ZeroAdmin() public {
        vm.expectRevert(BridgeProxy.ZeroAddress.selector);
        new BridgeProxy(address(0));
    }

    // ============ SET BRIDGE TARGET TESTS ============

    function test_SetBridgeTarget_Success() public {
        vm.expectEmit(true, true, false, false);
        emit BridgeTargetUpdated(address(0), address(mockBridgeTarget));

        vm.prank(admin);
        bridgeProxy.setBridgeTarget(address(mockBridgeTarget));

        (address target,) = bridgeProxy.getBridgeConfiguration();
        assertEq(target, address(mockBridgeTarget));
    }

    function test_SetBridgeTarget_UpdateExisting() public {
        MockBridgeTarget newTarget = new MockBridgeTarget();

        // Set initial target
        vm.prank(admin);
        bridgeProxy.setBridgeTarget(address(mockBridgeTarget));

        // Update to new target
        vm.expectEmit(true, true, false, false);
        emit BridgeTargetUpdated(address(mockBridgeTarget), address(newTarget));

        vm.prank(admin);
        bridgeProxy.setBridgeTarget(address(newTarget));

        (address target,) = bridgeProxy.getBridgeConfiguration();
        assertEq(target, address(newTarget));
    }

    function test_RevertWhen_SetBridgeTarget_NotAdmin() public {
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, user, bridgeProxy.BRIDGE_ADMIN_ROLE()
            )
        );
        bridgeProxy.setBridgeTarget(address(mockBridgeTarget));
        vm.stopPrank();
    }

    function test_RevertWhen_SetBridgeTarget_ZeroAddress() public {
        vm.prank(admin);
        vm.expectRevert(BridgeProxy.ZeroAddress.selector);
        bridgeProxy.setBridgeTarget(address(0));
    }

    // ============ SET BRIDGE CALLDATA TESTS ============

    function test_SetBridgeCalldata_Success() public {
        bytes memory calldata_ = abi.encodeWithSignature("bridgeTokens(address,uint256,bytes)");

        vm.expectEmit(false, false, false, true);
        emit BridgeCalldataUpdated("", calldata_);

        vm.prank(admin);
        bridgeProxy.setBridgeCalldata(calldata_);

        (, bytes memory storedCalldata) = bridgeProxy.getBridgeConfiguration();
        assertEq(storedCalldata, calldata_);
    }

    function test_SetBridgeCalldata_UpdateExisting() public {
        bytes memory calldata1 = abi.encodeWithSignature("bridgeTokens(address,uint256,bytes)");
        bytes memory calldata2 = abi.encodeWithSignature("bridge(address,uint256)");

        // Set initial calldata
        vm.prank(admin);
        bridgeProxy.setBridgeCalldata(calldata1);

        // Update calldata
        vm.expectEmit(false, false, false, true);
        emit BridgeCalldataUpdated(calldata1, calldata2);

        vm.prank(admin);
        bridgeProxy.setBridgeCalldata(calldata2);

        (, bytes memory storedCalldata) = bridgeProxy.getBridgeConfiguration();
        assertEq(storedCalldata, calldata2);
    }

    function test_SetBridgeCalldata_EmptyCalldata() public {
        bytes memory emptyCalldata = "";

        vm.prank(admin);
        bridgeProxy.setBridgeCalldata(emptyCalldata);

        (, bytes memory storedCalldata) = bridgeProxy.getBridgeConfiguration();
        assertEq(storedCalldata.length, 0);
    }

    function test_RevertWhen_SetBridgeCalldata_NotAdmin() public {
        bytes memory calldata_ = abi.encodeWithSignature("bridgeTokens(address,uint256,bytes)");

        bytes32 bridgeAdminRole = bridgeProxy.BRIDGE_ADMIN_ROLE();

        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, bridgeAdminRole)
        );
        bridgeProxy.setBridgeCalldata(calldata_);
        vm.stopPrank();
    }

    // ============ EXECUTE BRIDGE TESTS ============

    function test_ExecuteBridge_Success() public {
        uint256 amount = 1000 * 10 ** 18;
        bytes memory dynamicData = abi.encode(recipient);

        // Setup bridge configuration
        _setupBridgeConfiguration();

        // Mint tokens to user and approve bridge proxy
        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);

        vm.expectEmit(true, false, true, true);
        emit BridgeExecuted(address(token), amount, address(mockBridgeTarget), true);

        vm.prank(user);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        // Verify token transfer
        assertEq(token.balanceOf(user), 0);
        assertEq(token.balanceOf(address(bridgeProxy)), 0);
        assertEq(token.balanceOf(address(mockBridgeTarget)), amount);

        // Verify bridge was called
        assertEq(mockBridgeTarget.getBridgeCallCount(), 1);
        MockBridgeTarget.BridgeCall memory call = mockBridgeTarget.getLastBridgeCall();
        assertEq(call.token, address(token));
        assertEq(call.amount, amount);
        assertEq(call.data, dynamicData);
    }

    function test_ExecuteBridge_WithComplexCalldata() public {
        uint256 amount = 500 * 10 ** 18;
        bytes memory dynamicData = abi.encode(recipient, "extra_data");

        // Setup bridge function
        _setupBridgeConfiguration();

        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);

        vm.prank(user);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        assertEq(mockBridgeTarget.getBridgeCallCount(), 1);
        assertEq(token.balanceOf(address(mockBridgeTarget)), amount);
    }

    function test_ExecuteBridge_EmptyDynamicData() public {
        uint256 amount = 200 * 10 ** 18;
        bytes memory emptyData = "";

        _setupBridgeConfiguration();

        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);

        vm.prank(user);
        bridgeProxy.executeBridge(address(token), amount, emptyData);

        assertEq(mockBridgeTarget.getBridgeCallCount(), 1);
        assertEq(token.balanceOf(address(mockBridgeTarget)), amount);
    }

    function test_RevertWhen_ExecuteBridge_BridgeNotConfigured() public {
        uint256 amount = 100 * 10 ** 18;

        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);

        vm.prank(user);
        vm.expectRevert(BridgeProxy.ZeroAddress.selector);
        bridgeProxy.executeBridge(address(token), amount, "");
    }

    function test_RevertWhen_ExecuteBridge_ZeroAmount() public {
        _setupBridgeConfiguration();

        vm.prank(user);
        vm.expectRevert(BridgeProxy.ZeroAmount.selector);
        bridgeProxy.executeBridge(address(token), 0, "");
    }

    function test_RevertWhen_ExecuteBridge_InsufficientBalance() public {
        uint256 amount = 1000 * 10 ** 18;

        _setupBridgeConfiguration();

        // Don't mint tokens to user
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);

        vm.prank(user);
        vm.expectRevert(abi.encodeWithSignature("ERC20InsufficientBalance(address,uint256,uint256)", user, 0, amount));
        bridgeProxy.executeBridge(address(token), amount, "");
    }

    function test_RevertWhen_ExecuteBridge_InsufficientAllowance() public {
        uint256 amount = 1000 * 10 ** 18;

        _setupBridgeConfiguration();

        token.mint(user, amount);
        // Don't approve bridge proxy

        vm.prank(user);
        vm.expectRevert(
            abi.encodeWithSignature(
                "ERC20InsufficientAllowance(address,uint256,uint256)", address(bridgeProxy), 0, amount
            )
        );
        bridgeProxy.executeBridge(address(token), amount, "");
    }

    function test_RevertWhen_ExecuteBridge_BridgeCallFails() public {
        uint256 amount = 1000 * 10 ** 18;

        _setupBridgeConfiguration();
        mockBridgeTarget.setShouldRevert(true);

        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);

        vm.prank(user);
        vm.expectRevert(BridgeProxy.BridgeCallFailed.selector);
        bridgeProxy.executeBridge(address(token), amount, "");
    }

    // ============ RECOVER TOKENS TESTS ============

    function test_RecoverTokens_Success() public {
        uint256 amount = 500 * 10 ** 18;

        // Somehow tokens got stuck in the bridge proxy
        token.mint(address(bridgeProxy), amount);

        vm.prank(admin);
        bridgeProxy.recoverTokens(address(token), amount, recipient);

        assertEq(token.balanceOf(address(bridgeProxy)), 0);
        assertEq(token.balanceOf(recipient), amount);
    }

    function test_RecoverTokens_PartialAmount() public {
        uint256 totalAmount = 1000 * 10 ** 18;
        uint256 recoverAmount = 300 * 10 ** 18;

        token.mint(address(bridgeProxy), totalAmount);

        vm.prank(admin);
        bridgeProxy.recoverTokens(address(token), recoverAmount, recipient);

        assertEq(token.balanceOf(address(bridgeProxy)), totalAmount - recoverAmount);
        assertEq(token.balanceOf(recipient), recoverAmount);
    }

    function test_RevertWhen_RecoverTokens_NotAdmin() public {
        uint256 amount = 100 * 10 ** 18;

        token.mint(address(bridgeProxy), amount);
        bytes32 adminRole = bridgeProxy.DEFAULT_ADMIN_ROLE();
        vm.startPrank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, adminRole)
        );
        bridgeProxy.recoverTokens(address(token), amount, recipient);
        vm.stopPrank();
    }

    function test_RevertWhen_RecoverTokens_ZeroRecipient() public {
        uint256 amount = 100 * 10 ** 18;

        token.mint(address(bridgeProxy), amount);

        vm.prank(admin);
        vm.expectRevert(BridgeProxy.ZeroAddress.selector);
        bridgeProxy.recoverTokens(address(token), amount, address(0));
    }

    // ============ VIEW FUNCTION TESTS ============

    function test_GetBridgeConfiguration_Initial() public view {
        (address target, bytes memory calldata_) = bridgeProxy.getBridgeConfiguration();

        assertEq(target, address(0));
        assertEq(calldata_.length, 0);
    }

    function test_GetBridgeConfiguration_AfterSetup() public {
        bytes memory expectedCalldata = abi.encodeWithSignature("bridgeTokens(address,uint256,bytes)");

        vm.startPrank(admin);
        bridgeProxy.setBridgeTarget(address(mockBridgeTarget));
        bridgeProxy.setBridgeCalldata(expectedCalldata);
        vm.stopPrank();

        (address target, bytes memory calldata_) = bridgeProxy.getBridgeConfiguration();

        assertEq(target, address(mockBridgeTarget));
        assertEq(calldata_, expectedCalldata);
    }

    // ============ INTEGRATION TESTS ============

    function test_Integration_FullBridgeFlow() public {
        uint256 amount = 1000 * 10 ** 18;
        bytes memory dynamicData = abi.encode(recipient);

        // 1. Setup bridge configuration
        _setupBridgeConfiguration();

        // 2. Prepare user with tokens
        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);

        // 3. Execute bridge
        vm.prank(user);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        // 4. Verify final state
        assertEq(token.balanceOf(user), 0);
        assertEq(token.balanceOf(address(bridgeProxy)), 0);
        assertEq(token.balanceOf(address(mockBridgeTarget)), amount);
        assertEq(mockBridgeTarget.getBridgeCallCount(), 1);
    }

    function test_Integration_MultipleBridgeOperations() public {
        uint256 amount1 = 500 * 10 ** 18;
        uint256 amount2 = 300 * 10 ** 18;

        _setupBridgeConfiguration();

        // First bridge operation
        token.mint(user, amount1);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount1);
        vm.prank(user);
        bridgeProxy.executeBridge(address(token), amount1, "");

        // Second bridge operation
        token.mint(user, amount2);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount2);
        vm.prank(user);
        bridgeProxy.executeBridge(address(token), amount2, "");

        assertEq(mockBridgeTarget.getBridgeCallCount(), 2);
        assertEq(token.balanceOf(address(mockBridgeTarget)), amount1 + amount2);
    }

    function test_Integration_BridgeConfigurationUpdate() public {
        uint256 amount = 1000 * 10 ** 18;
        MockBridgeTarget newTarget = new MockBridgeTarget();

        // Setup initial configuration
        _setupBridgeConfiguration();

        // Execute with first configuration
        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);
        vm.prank(user);
        bridgeProxy.executeBridge(address(token), amount, "");

        // Update configuration
        vm.prank(admin);
        bridgeProxy.setBridgeTarget(address(newTarget));

        // Execute with new configuration
        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);
        vm.prank(user);
        bridgeProxy.executeBridge(address(token), amount, "");

        // Verify both targets were called
        assertEq(mockBridgeTarget.getBridgeCallCount(), 1);
        assertEq(newTarget.getBridgeCallCount(), 1);
    }

    // ============ FUZZ TESTS ============

    function testFuzz_ExecuteBridge_ValidAmounts(uint256 amount) public {
        amount = bound(amount, 1, type(uint128).max);

        _setupBridgeConfiguration();

        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(bridgeProxy), amount);

        vm.prank(user);
        bridgeProxy.executeBridge(address(token), amount, "");

        assertEq(token.balanceOf(address(mockBridgeTarget)), amount);
        assertEq(mockBridgeTarget.getBridgeCallCount(), 1);
    }

    function testFuzz_SetBridgeCalldata_ArbitraryData(bytes calldata arbitraryData) public {
        vm.prank(admin);
        bridgeProxy.setBridgeCalldata(arbitraryData);

        (, bytes memory storedCalldata) = bridgeProxy.getBridgeConfiguration();
        assertEq(storedCalldata, arbitraryData);
    }

    function testFuzz_RecoverTokens_ValidAmounts(uint256 stuckAmount, uint256 recoverAmount) public {
        stuckAmount = bound(stuckAmount, 1, type(uint128).max);
        recoverAmount = bound(recoverAmount, 1, stuckAmount);

        token.mint(address(bridgeProxy), stuckAmount);

        vm.prank(admin);
        bridgeProxy.recoverTokens(address(token), recoverAmount, recipient);

        assertEq(token.balanceOf(recipient), recoverAmount);
        assertEq(token.balanceOf(address(bridgeProxy)), stuckAmount - recoverAmount);
    }

    // ============ HELPER FUNCTIONS ============

    function _setupBridgeConfiguration() internal {
        vm.startPrank(admin);
        bridgeProxy.setBridgeTarget(address(mockBridgeTarget));
        bridgeProxy.setBridgeCalldata(abi.encodeWithSignature("bridgeTokens(address,uint256,bytes)"));
        vm.stopPrank();
    }
}
