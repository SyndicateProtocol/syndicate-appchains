// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {ArbitrumBridgeProxy} from "src/token/bridges/ArbitrumBridgeProxy.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

// Mock Arbitrum Bridge contract
contract MockArbitrumBridge {
    struct TransferCall {
        address token;
        address refundTo;
        address to;
        uint256 amount;
        uint256 maxGas;
        uint256 gasPriceBid;
        bytes data;
        uint256 ethValue;
    }

    TransferCall[] public transferCalls;
    bool public shouldRevert;
    address public mockGateway;

    constructor() {
        // Set mock gateway to this contract for simplicity
        mockGateway = address(this);
    }

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    function getGateway(address) external view returns (address) {
        return mockGateway;
    }

    function outboundTransferCustomRefund(
        address _token,
        address _refundTo,
        address _to,
        uint256 _amount,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        bytes calldata _data
    ) external payable returns (bytes memory) {
        if (shouldRevert) {
            revert("Arbitrum bridge failed");
        }

        // Transfer tokens to simulate bridge behavior
        IERC20(_token).transferFrom(msg.sender, address(this), _amount);

        transferCalls.push(
            TransferCall({
                token: _token,
                refundTo: _refundTo,
                to: _to,
                amount: _amount,
                maxGas: _maxGas,
                gasPriceBid: _gasPriceBid,
                data: _data,
                ethValue: msg.value
            })
        );

        return abi.encode("success");
    }

    function getLastTransferCall() external view returns (TransferCall memory) {
        require(transferCalls.length > 0, "No transfers");
        return transferCalls[transferCalls.length - 1];
    }

    function getTransferCallCount() external view returns (uint256) {
        return transferCalls.length;
    }

    // Allow contract to receive ETH
    receive() external payable {}
}

contract ArbitrumBridgeProxyTest is Test {
    ArbitrumBridgeProxy public bridgeProxy;
    MockArbitrumBridge public arbitrumBridge;
    ERC20Mock public token;

    address public admin = address(0x1234);
    address public caller = address(0x5678);
    address public user = address(0x9ABC);

    address public recipient = address(0x1111);
    uint256 public maxGas = 2_000_000;
    uint256 public gasPriceBid = 1 gwei;

    uint256 public constant MAX_SINGLE_TRANSFER = 1_000_000 * 10 ** 18;
    uint256 public constant DAILY_LIMIT = 5_000_000 * 10 ** 18;

    // Events
    event ArbitrumConfigUpdated(address recipient, uint256 maxGas, uint256 gasPriceBid);
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target);
    event EthWithdrawn(address indexed to, uint256 amount);

    function setUp() public {
        token = new ERC20Mock();
        arbitrumBridge = new MockArbitrumBridge();

        bridgeProxy = new ArbitrumBridgeProxy(
            admin, caller, address(arbitrumBridge), MAX_SINGLE_TRANSFER, DAILY_LIMIT, recipient, maxGas, gasPriceBid
        );

        // Mint tokens to caller and give some ETH
        token.mint(caller, 10_000_000 * 10 ** 18);
        vm.deal(caller, 100 ether);
        vm.deal(address(bridgeProxy), 10 ether);
    }

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR TESTS
    //////////////////////////////////////////////////////////////*/

    function test_Constructor_Success() public view {
        assertEq(bridgeProxy.bridgeTarget(), address(arbitrumBridge));
        assertEq(bridgeProxy.recipient(), recipient);
        assertEq(bridgeProxy.maxGas(), maxGas);
        assertEq(bridgeProxy.gasPriceBid(), gasPriceBid);
        assertEq(bridgeProxy.maxSubmissionCost(), 1000000000000000); // 0.001 ETH
        assertEq(bridgeProxy.maxSingleTransfer(), MAX_SINGLE_TRANSFER);
        assertEq(bridgeProxy.dailyLimit(), DAILY_LIMIT);
        assertTrue(bridgeProxy.bridgeActive());
    }

    function test_Constructor_BridgeInfo() public view {
        (string memory name, address target, bool active) = bridgeProxy.getBridgeInfo();

        assertEq(name, "Arbitrum Bridge");
        assertEq(target, address(arbitrumBridge));
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
        emit BridgeExecuted(address(token), amount, address(arbitrumBridge));

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        // Verify Arbitrum bridge was called correctly
        MockArbitrumBridge.TransferCall memory call = arbitrumBridge.getLastTransferCall();
        assertEq(call.token, address(token));
        assertEq(call.refundTo, address(bridgeProxy));
        assertEq(call.to, recipient);
        assertEq(call.amount, amount);
        assertEq(call.maxGas, maxGas);
        assertEq(call.gasPriceBid, gasPriceBid);
        uint256 expectedMaxSubmissionCost = 1000000000000000; // 0.001 ETH
        bytes memory expectedData = abi.encode(expectedMaxSubmissionCost, "");
        assertEq(call.data, expectedData);
        uint256 maxSubmissionCost = 1000000000000000; // 0.001 ETH
        uint256 expectedEthValue = (maxGas * gasPriceBid) + maxSubmissionCost;
        assertEq(call.ethValue, expectedEthValue);
    }

    function test_ExecuteBridge_Success_CustomParams() public {
        uint256 amount = 300_000 * 10 ** 18;
        address customRecipient = address(0x2222);
        uint256 customMaxGas = 1_500_000;
        uint256 customGasPriceBid = 2 gwei;
        bytes memory dynamicData = abi.encode(customRecipient, customMaxGas, customGasPriceBid);

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        // Verify custom parameters were used
        MockArbitrumBridge.TransferCall memory call = arbitrumBridge.getLastTransferCall();
        assertEq(call.token, address(token));
        assertEq(call.refundTo, address(bridgeProxy));
        assertEq(call.to, customRecipient);
        assertEq(call.amount, amount);
        assertEq(call.maxGas, customMaxGas);
        assertEq(call.gasPriceBid, customGasPriceBid);
        uint256 maxSubmissionCost = 1000000000000000; // 0.001 ETH
        uint256 expectedEthValue = (customMaxGas * customGasPriceBid) + maxSubmissionCost;
        assertEq(call.ethValue, expectedEthValue);
    }

    function test_ExecuteBridge_Success_MultipleTransfers() public {
        uint256 amount1 = 200_000 * 10 ** 18;
        uint256 amount2 = 300_000 * 10 ** 18;

        vm.startPrank(caller);
        token.approve(address(bridgeProxy), amount1 + amount2);

        bridgeProxy.executeBridge(address(token), amount1, "");
        bridgeProxy.executeBridge(address(token), amount2, "");
        vm.stopPrank();

        assertEq(arbitrumBridge.getTransferCallCount(), 2);
        assertEq(bridgeProxy.dailyUsed(), amount1 + amount2);
    }

    function test_RevertWhen_ExecuteBridge_ArbitrumBridgeFails() public {
        uint256 amount = 100_000 * 10 ** 18;

        arbitrumBridge.setShouldRevert(true);

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        vm.expectRevert("Arbitrum bridge failed");
        bridgeProxy.executeBridge(address(token), amount, "");
    }

    function test_ExecuteBridge_EthValueCalculation() public {
        uint256 amount = 100_000 * 10 ** 18;
        uint256 customMaxGas = 3_000_000;
        uint256 customGasPriceBid = 5 gwei;
        bytes memory dynamicData = abi.encode(recipient, customMaxGas, customGasPriceBid);

        uint256 maxSubmissionCost = 1000000000000000; // 0.001 ETH
        uint256 expectedEthValue = (customMaxGas * customGasPriceBid) + maxSubmissionCost;
        uint256 initialBridgeEth = address(bridgeProxy).balance;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        MockArbitrumBridge.TransferCall memory call = arbitrumBridge.getLastTransferCall();
        assertEq(call.ethValue, expectedEthValue);

        // ETH should be transferred from bridge proxy to arbitrum bridge
        assertEq(address(bridgeProxy).balance, initialBridgeEth - expectedEthValue);
    }

    /*//////////////////////////////////////////////////////////////
                        CONFIGURATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_SetArbitrumConfig_Success() public {
        address newRecipient = address(0x3333);
        uint256 newMaxGas = 2_500_000;
        uint256 newGasPriceBid = 3 gwei;

        vm.expectEmit(false, false, false, true);
        emit ArbitrumConfigUpdated(newRecipient, newMaxGas, newGasPriceBid);

        vm.prank(admin);
        bridgeProxy.setArbitrumConfig(newRecipient, newMaxGas, newGasPriceBid);

        assertEq(bridgeProxy.recipient(), newRecipient);
        assertEq(bridgeProxy.maxGas(), newMaxGas);
        assertEq(bridgeProxy.gasPriceBid(), newGasPriceBid);
    }

    function test_RevertWhen_SetArbitrumConfig_NotAdmin() public {
        vm.startPrank(user);
        bytes32 bridgeAdminRole = bridgeProxy.BRIDGE_ADMIN_ROLE();

        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, bridgeAdminRole)
        );
        bridgeProxy.setArbitrumConfig(recipient, maxGas, gasPriceBid);
        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                        MAX SUBMISSION COST TESTS
    //////////////////////////////////////////////////////////////*/

    function test_SetMaxSubmissionCost_Success() public {
        uint256 newMaxSubmissionCost = 2000000000000000; // 0.002 ETH

        vm.prank(admin);
        bridgeProxy.setMaxSubmissionCost(newMaxSubmissionCost);

        assertEq(bridgeProxy.maxSubmissionCost(), newMaxSubmissionCost);
    }

    function test_RevertWhen_SetMaxSubmissionCost_NotAdmin() public {
        uint256 newMaxSubmissionCost = 2000000000000000; // 0.002 ETH
        bytes32 bridgeAdminRole = bridgeProxy.BRIDGE_ADMIN_ROLE();

        vm.prank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, bridgeAdminRole)
        );
        bridgeProxy.setMaxSubmissionCost(newMaxSubmissionCost);
    }

    function test_ExecuteBridge_UsesUpdatedMaxSubmissionCost() public {
        uint256 newMaxSubmissionCost = 2000000000000000; // 0.002 ETH
        uint256 amount = 100_000 * 10 ** 18;

        // Update maxSubmissionCost
        vm.prank(admin);
        bridgeProxy.setMaxSubmissionCost(newMaxSubmissionCost);

        // Execute bridge
        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, "");

        // Verify the updated maxSubmissionCost is used in the bridge call
        MockArbitrumBridge.TransferCall memory call = arbitrumBridge.getLastTransferCall();
        bytes memory expectedData = abi.encode(newMaxSubmissionCost, "");
        assertEq(call.data, expectedData);

        uint256 expectedEthValue = (maxGas * gasPriceBid) + newMaxSubmissionCost;
        assertEq(call.ethValue, expectedEthValue);
    }

    /*//////////////////////////////////////////////////////////////
                              VIEW FUNCTION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_GetArbitrumConfig() public view {
        (address recipientAddr, uint256 maxGasLimit, uint256 gasPriceBidAmount) = bridgeProxy.getArbitrumConfig();

        assertEq(recipientAddr, recipient);
        assertEq(maxGasLimit, maxGas);
        assertEq(gasPriceBidAmount, gasPriceBid);
    }

    function test_CalculateEthValue() public view {
        uint256 testMaxGas = 1_800_000;
        uint256 testGasPriceBid = 4 gwei;
        uint256 expectedValue = testMaxGas * testGasPriceBid;

        assertEq(bridgeProxy.calculateEthValue(testMaxGas, testGasPriceBid), expectedValue);
    }

    /*//////////////////////////////////////////////////////////////
                          ETH HANDLING TESTS
    //////////////////////////////////////////////////////////////*/

    function test_ReceiveEth_Success() public {
        uint256 initialBalance = address(bridgeProxy).balance;
        uint256 sendAmount = 1 ether;

        vm.prank(caller);
        (bool success,) = address(bridgeProxy).call{value: sendAmount}("");

        assertTrue(success);
        assertEq(address(bridgeProxy).balance, initialBalance + sendAmount);
    }

    function test_ExecuteBridge_InsufficientEthBalance() public {
        // Deploy new bridge proxy with no ETH
        ArbitrumBridgeProxy newBridgeProxy = new ArbitrumBridgeProxy(
            admin, caller, address(arbitrumBridge), MAX_SINGLE_TRANSFER, DAILY_LIMIT, recipient, maxGas, gasPriceBid
        );

        uint256 amount = 100_000 * 10 ** 18;

        vm.prank(caller);
        token.approve(address(newBridgeProxy), amount);

        vm.prank(caller);
        vm.expectRevert(); // Should revert due to insufficient ETH balance
        newBridgeProxy.executeBridge(address(token), amount, "");
    }

    /*//////////////////////////////////////////////////////////////
                        INTEGRATION TESTS
    //////////////////////////////////////////////////////////////*/

    function test_Integration_FullBridgeFlow() public {
        uint256 amount = 750_000 * 10 ** 18;
        address customRecipient = address(0x5555);
        uint256 customMaxGas = 1_800_000;
        uint256 customGasPriceBid = 3 gwei;
        bytes memory dynamicData = abi.encode(customRecipient, customMaxGas, customGasPriceBid);

        uint256 initialTokenBalance = token.balanceOf(caller);
        uint256 initialEthBalance = address(bridgeProxy).balance;
        uint256 maxSubmissionCost = 1000000000000000; // 0.001 ETH
        uint256 expectedEthCost = (customMaxGas * customGasPriceBid) + maxSubmissionCost;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        // Verify token transfer
        assertEq(token.balanceOf(caller), initialTokenBalance - amount);
        assertEq(token.balanceOf(address(arbitrumBridge)), amount);

        // Verify ETH transfer for gas
        assertEq(address(bridgeProxy).balance, initialEthBalance - expectedEthCost);

        // Verify bridge call
        MockArbitrumBridge.TransferCall memory call = arbitrumBridge.getLastTransferCall();
        assertEq(call.token, address(token));
        assertEq(call.refundTo, address(bridgeProxy));
        assertEq(call.to, customRecipient);
        assertEq(call.amount, amount);
        assertEq(call.maxGas, customMaxGas);
        assertEq(call.gasPriceBid, customGasPriceBid);
        assertEq(call.ethValue, expectedEthCost);

        // Verify daily usage tracking
        assertEq(bridgeProxy.dailyUsed(), amount);
    }

    function test_Integration_MultipleTransfersGasCosts() public {
        uint256 amount = 200_000 * 10 ** 18;
        uint256 transferCount = 3;
        uint256 maxSubmissionCost = 1000000000000000; // 0.001 ETH
        uint256 expectedTotalEthCost = ((maxGas * gasPriceBid) + maxSubmissionCost) * transferCount;
        uint256 initialEthBalance = address(bridgeProxy).balance;

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount * transferCount);

        vm.startPrank(caller);
        for (uint256 i = 0; i < transferCount; i++) {
            bridgeProxy.executeBridge(address(token), amount, "");
        }
        vm.stopPrank();

        assertEq(address(bridgeProxy).balance, initialEthBalance - expectedTotalEthCost);
        assertEq(arbitrumBridge.getTransferCallCount(), transferCount);
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

        MockArbitrumBridge.TransferCall memory call = arbitrumBridge.getLastTransferCall();
        assertEq(call.amount, amount);
        assertEq(bridgeProxy.dailyUsed(), amount);
    }

    function testFuzz_CalculateEthValue(uint256 gas, uint256 price) public view {
        gas = bound(gas, 21000, 10_000_000);
        price = bound(price, 1 wei, 1000 gwei);

        uint256 expected = gas * price;
        assertEq(bridgeProxy.calculateEthValue(gas, price), expected);
    }

    function testFuzz_SetArbitrumConfig_ValidGasParams(uint256 newMaxGas, uint256 newGasPriceBid) public {
        newMaxGas = bound(newMaxGas, 21000, 50_000_000);
        newGasPriceBid = bound(newGasPriceBid, 1 wei, 1000 gwei);

        vm.prank(admin);
        bridgeProxy.setArbitrumConfig(recipient, newMaxGas, newGasPriceBid);

        assertEq(bridgeProxy.maxGas(), newMaxGas);
        assertEq(bridgeProxy.gasPriceBid(), newGasPriceBid);
    }

    function testFuzz_ExecuteBridge_CustomGasParams(uint256 customMaxGas, uint256 customGasPriceBid) public {
        customMaxGas = bound(customMaxGas, 100_000, 5_000_000);
        customGasPriceBid = bound(customGasPriceBid, 1 gwei, 100 gwei);

        uint256 amount = 100_000 * 10 ** 18;
        bytes memory dynamicData = abi.encode(recipient, customMaxGas, customGasPriceBid);

        // Ensure bridge has enough ETH
        uint256 maxSubmissionCost = 1000000000000000; // 0.001 ETH
        uint256 requiredEth = (customMaxGas * customGasPriceBid) + maxSubmissionCost;
        vm.deal(address(bridgeProxy), requiredEth + 1 ether);

        vm.prank(caller);
        token.approve(address(bridgeProxy), amount);

        vm.prank(caller);
        bridgeProxy.executeBridge(address(token), amount, dynamicData);

        MockArbitrumBridge.TransferCall memory call = arbitrumBridge.getLastTransferCall();
        assertEq(call.maxGas, customMaxGas);
        assertEq(call.gasPriceBid, customGasPriceBid);
        assertEq(call.ethValue, requiredEth);
    }

    /*//////////////////////////////////////////////////////////////
                         ETH WITHDRAWAL TESTS
    //////////////////////////////////////////////////////////////*/

    function test_WithdrawEth_Success() public {
        uint256 withdrawAmount = 1 ether;
        uint256 initialBridgeBalance = address(bridgeProxy).balance;
        uint256 initialUserBalance = user.balance;

        vm.expectEmit(true, false, false, true);
        emit EthWithdrawn(user, withdrawAmount);

        vm.prank(admin);
        bridgeProxy.withdrawEth(payable(user), withdrawAmount);

        assertEq(address(bridgeProxy).balance, initialBridgeBalance - withdrawAmount);
        assertEq(user.balance, initialUserBalance + withdrawAmount);
    }

    function test_WithdrawEth_Success_PartialBalance() public {
        uint256 bridgeBalance = address(bridgeProxy).balance;
        uint256 withdrawAmount = bridgeBalance / 2;
        uint256 initialUserBalance = user.balance;

        vm.prank(admin);
        bridgeProxy.withdrawEth(payable(user), withdrawAmount);

        assertEq(address(bridgeProxy).balance, bridgeBalance - withdrawAmount);
        assertEq(user.balance, initialUserBalance + withdrawAmount);
    }

    function test_WithdrawEth_Success_FullBalance() public {
        uint256 bridgeBalance = address(bridgeProxy).balance;
        uint256 initialUserBalance = user.balance;

        vm.prank(admin);
        bridgeProxy.withdrawEth(payable(user), bridgeBalance);

        assertEq(address(bridgeProxy).balance, 0);
        assertEq(user.balance, initialUserBalance + bridgeBalance);
    }

    function test_RevertWhen_WithdrawEth_NotAdmin() public {
        uint256 withdrawAmount = 1 ether;
        bytes32 bridgeAdminRole = bridgeProxy.BRIDGE_ADMIN_ROLE();

        vm.prank(user);
        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, user, bridgeAdminRole)
        );
        bridgeProxy.withdrawEth(payable(user), withdrawAmount);
    }

    function test_RevertWhen_WithdrawEth_ZeroAddress() public {
        uint256 withdrawAmount = 1 ether;

        vm.prank(admin);
        vm.expectRevert("ArbitrumBridgeProxy: zero address");
        bridgeProxy.withdrawEth(payable(address(0)), withdrawAmount);
    }

    function test_RevertWhen_WithdrawEth_InsufficientBalance() public {
        uint256 bridgeBalance = address(bridgeProxy).balance;
        uint256 withdrawAmount = bridgeBalance + 1 ether;

        vm.prank(admin);
        vm.expectRevert("ArbitrumBridgeProxy: insufficient balance");
        bridgeProxy.withdrawEth(payable(user), withdrawAmount);
    }

    function test_RevertWhen_WithdrawEth_TransferFails() public {
        // Deploy a contract that rejects ETH transfers
        RejectEthContract rejectContract = new RejectEthContract();
        uint256 withdrawAmount = 1 ether;

        vm.prank(admin);
        vm.expectRevert("ArbitrumBridgeProxy: ETH transfer failed");
        bridgeProxy.withdrawEth(payable(address(rejectContract)), withdrawAmount);
    }

    function test_WithdrawEth_EmitsCorrectEvent() public {
        uint256 withdrawAmount = 0.5 ether;
        address recipient_ = address(0x1234);

        vm.expectEmit(true, false, false, true);
        emit EthWithdrawn(recipient_, withdrawAmount);

        vm.prank(admin);
        bridgeProxy.withdrawEth(payable(recipient_), withdrawAmount);
    }

    function test_WithdrawEth_MultipleWithdrawals() public {
        uint256 firstWithdraw = 1 ether;
        uint256 secondWithdraw = 2 ether;
        uint256 initialBalance = address(bridgeProxy).balance;
        uint256 initialUserBalance = user.balance;

        vm.startPrank(admin);
        bridgeProxy.withdrawEth(payable(user), firstWithdraw);
        bridgeProxy.withdrawEth(payable(user), secondWithdraw);
        vm.stopPrank();

        assertEq(address(bridgeProxy).balance, initialBalance - firstWithdraw - secondWithdraw);
        assertEq(user.balance, initialUserBalance + firstWithdraw + secondWithdraw);
    }

    /*//////////////////////////////////////////////////////////////
                          FUZZ TESTS FOR WITHDRAWAL
    //////////////////////////////////////////////////////////////*/

    function testFuzz_WithdrawEth_ValidAmounts(uint256 withdrawAmount) public {
        uint256 bridgeBalance = address(bridgeProxy).balance;
        withdrawAmount = bound(withdrawAmount, 1 wei, bridgeBalance);

        uint256 initialUserBalance = user.balance;

        vm.prank(admin);
        bridgeProxy.withdrawEth(payable(user), withdrawAmount);

        assertEq(address(bridgeProxy).balance, bridgeBalance - withdrawAmount);
        assertEq(user.balance, initialUserBalance + withdrawAmount);
    }

    function testFuzz_WithdrawEth_DifferentRecipients(address recipient_) public {
        vm.assume(recipient_ != address(0));
        vm.assume(recipient_.code.length == 0); // Not a contract

        vm.assume(uint160(recipient_) > 0x09); // Exclude precompiles (0x01-0x09)

        // Exclude precompiles (0x01-0x0a) - https://www.evm.codes/precompiled
        vm.assume(uint160(recipient_) > 0x0a);

        uint256 withdrawAmount = 1 ether;
        uint256 initialBridgeBalance = address(bridgeProxy).balance;
        uint256 initialRecipientBalance = recipient_.balance;
        vm.prank(admin);
        bridgeProxy.withdrawEth(payable(recipient_), withdrawAmount);

        assertEq(address(bridgeProxy).balance, initialBridgeBalance - withdrawAmount);
        assertEq(recipient_.balance, initialRecipientBalance + withdrawAmount);
    }
}

// Helper contract that rejects ETH transfers
contract RejectEthContract {
    // Fallback function that reverts
    fallback() external payable {
        revert("Cannot receive ETH");
    }

    receive() external payable {
        revert("Cannot receive ETH");
    }
}
