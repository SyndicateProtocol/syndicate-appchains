// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {SyndicateTokenEmissionScheduler} from "src/token/SyndicateTokenEmissionScheduler.sol";
import {ArbitrumBridgeProxy} from "src/token/bridges/ArbitrumBridgeProxy.sol";
import {OptimismBridgeProxy} from "src/token/bridges/OptimismBridgeProxy.sol";
import {IBridgeProxy} from "src/token/interfaces/IBridgeProxy.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title SimplifiedMainnetIntegrationTest
 * @notice Simplified integration test that verifies the end-to-end flow works
 *         up to the point of calling real bridge contracts
 * @dev This test proves that:
 *      1. EmissionScheduler can mint tokens correctly
 *      2. Bridge proxies can receive tokens and approvals
 *      3. The integration architecture is sound
 *
 *      We use mock bridge targets to avoid real bridge validation failures
 *      while still proving the core integration works.
 */
contract SimplifiedMainnetIntegrationTest is Test {
    // Core contracts
    SyndicateToken public token;
    SyndicateTokenEmissionScheduler public emissionScheduler;
    ArbitrumBridgeProxy public arbitrumBridge;
    OptimismBridgeProxy public optimismBridge;

    // Mock bridge contracts for testing
    MockBridgeTarget public mockArbitrumTarget;
    MockBridgeTarget public mockOptimismTarget;

    // Test addresses
    address public admin = address(0x1234);
    address public syndTreasury = address(0x5678);
    address public emissionsManager = address(0x9ABC);
    address public pauser = address(0xDEF0);
    address public l2Recipient = address(0x1111);

    // Bridge configuration
    address public constant OP_L2_TOKEN = address(0x3333);
    uint256 public constant MAX_SINGLE_TRANSFER = 100_000_000 * 10 ** 18; // 100M tokens
    uint256 public constant DAILY_LIMIT = 100_000_000 * 10 ** 18; // 100M tokens
    uint32 public constant OP_L2_GAS = 200000;
    uint256 public constant ARB_MAX_GAS = 300000;
    uint256 public constant ARB_GAS_PRICE_BID = 1e9; // 1 gwei

    function setUp() public {
        vm.startPrank(admin);

        // Deploy mock bridge targets
        mockArbitrumTarget = new MockBridgeTarget();
        mockOptimismTarget = new MockBridgeTarget();

        // Deploy core contracts
        token = new SyndicateToken(admin, syndTreasury);
        emissionScheduler = new SyndicateTokenEmissionScheduler(address(token), admin, emissionsManager, pauser);

        // Grant emission minter role
        token.grantRole(token.EMISSION_MINTER_ROLE(), address(emissionScheduler));

        // Deploy bridge proxies with mock targets
        arbitrumBridge = new ArbitrumBridgeProxy(
            admin,
            address(emissionScheduler),
            address(mockArbitrumTarget), // Use mock instead of real bridge
            MAX_SINGLE_TRANSFER,
            DAILY_LIMIT,
            l2Recipient,
            ARB_MAX_GAS,
            ARB_GAS_PRICE_BID
        );

        optimismBridge = new OptimismBridgeProxy(
            admin,
            address(emissionScheduler),
            address(mockOptimismTarget), // Use mock instead of real bridge
            MAX_SINGLE_TRANSFER,
            DAILY_LIMIT,
            OP_L2_TOKEN,
            l2Recipient,
            OP_L2_GAS
        );

        deal(address(arbitrumBridge), 1_000_000_000 * 10 ** 18); // Fund token with 1B tokens

        vm.stopPrank();
    }

    // ============ ARBITRUM INTEGRATION TESTS ============

    function test_Integration_ArbitrumBridge_EndToEnd_simplified() public {
        // Configure emission scheduler with Arbitrum bridge
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();

        // Start emissions
        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        // Fast forward to first epoch
        vm.warp(block.timestamp + 30 days + 1);

        // Fund the emission scheduler with ETH for gas fees
        uint256 ethNeeded = ARB_MAX_GAS * ARB_GAS_PRICE_BID;
        vm.deal(address(emissionScheduler), ethNeeded);

        // Fund the mock bridge target to accept ETH transfers
        vm.deal(address(mockArbitrumTarget), 1 ether);

        // Get expected emission amount for first epoch
        uint256 expectedAmount = 6_780_550 * 10 ** 18;
        uint256 initialTokenSupply = token.totalSupply();

        // Execute emission minting (should mint and bridge tokens)
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify tokens were minted
        assertEq(token.totalSupply(), initialTokenSupply + expectedAmount, "Tokens should be minted");

        // Verify current epoch updated
        assertEq(emissionScheduler.currentEpoch(), 1, "Current epoch should be 1");

        // Verify total emissions tracking
        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount, "Total emissions should match");

        // Verify tokens left the emission scheduler
        uint256 schedulerBalance = token.balanceOf(address(emissionScheduler));
        assertEq(schedulerBalance, 0, "Emission scheduler should have no remaining tokens");

        // Verify mock bridge received the tokens
        assertEq(
            token.balanceOf(address(mockArbitrumTarget)), expectedAmount, "Mock bridge should have received tokens"
        );

        // Verify bridge call was made with correct parameters
        MockBridgeTarget.BridgeCall memory call = mockArbitrumTarget.getLastCall();
        assertEq(call.token, address(token), "Correct token address");
        assertEq(call.amount, expectedAmount, "Correct amount");
        assertEq(call.ethValue, ethNeeded, "Correct ETH value");
        assertTrue(call.called, "Bridge call was made");
    }

    function test_Integration_ArbitrumBridge_MultipleEpochs() public {
        // Setup bridge configuration
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        // Fast forward to allow 3 epochs to be minted
        vm.warp(block.timestamp + 90 days + 1);

        // Fund with sufficient ETH for all epochs at once
        uint256 ethNeeded = ARB_MAX_GAS * ARB_GAS_PRICE_BID;
        vm.deal(address(emissionScheduler), ethNeeded);
        vm.deal(address(mockArbitrumTarget), 1 ether);

        uint256 initialSupply = token.totalSupply();
        uint256 expectedTotalAmount = 6_780_550 * 10 ** 18 * 3; // 3 epochs

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(token.totalSupply(), initialSupply + expectedTotalAmount, "Should mint 3 epochs worth");
        assertEq(emissionScheduler.currentEpoch(), 3, "Should be at epoch 3");
        assertEq(token.balanceOf(address(emissionScheduler)), 0, "Scheduler should have no tokens left");
        assertEq(
            token.balanceOf(address(mockArbitrumTarget)), expectedTotalAmount, "Mock bridge should have all tokens"
        );
    }

    // ============ OPTIMISM INTEGRATION TESTS ============

    function test_Integration_OptimismBridge_EndToEnd() public {
        // Configure emission scheduler with Optimism bridge
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(optimismBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, OP_L2_GAS));
        vm.stopPrank();

        // Start emissions
        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        // Fast forward to first epoch
        vm.warp(block.timestamp + 30 days + 1);

        uint256 expectedAmount = 6_780_550 * 10 ** 18;
        uint256 initialTokenSupply = token.totalSupply();

        // Execute emission minting
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify tokens were minted and processed
        assertEq(token.totalSupply(), initialTokenSupply + expectedAmount, "Tokens should be minted");
        assertEq(emissionScheduler.currentEpoch(), 1, "Current epoch should be 1");
        assertEq(token.balanceOf(address(emissionScheduler)), 0, "Scheduler should have no tokens left");

        // Verify mock bridge received the tokens
        assertEq(token.balanceOf(address(mockOptimismTarget)), expectedAmount, "Mock bridge should have tokens");

        // Verify bridge call was made
        MockBridgeTarget.BridgeCall memory call = mockOptimismTarget.getLastCall();
        assertEq(call.token, address(token), "Correct token");
        assertEq(call.amount, expectedAmount, "Correct amount");
        assertTrue(call.called, "Bridge call was made");
    }

    // ============ BRIDGE FAILURE SCENARIOS ============

    function test_Integration_BridgeFailure_RevertEntireOperation() public {
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();

        // Configure mock bridge to fail
        mockArbitrumTarget.setShouldRevert(true);

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        vm.warp(block.timestamp + 30 days + 1);
        vm.deal(address(emissionScheduler), 1 ether);
        vm.deal(address(mockArbitrumTarget), 1 ether);

        // Entire operation should revert if bridge fails
        vm.expectRevert("Mock bridge reverted");
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify no state changes occurred due to atomic failure
        assertEq(emissionScheduler.currentEpoch(), 0, "No epoch should be processed");
        assertEq(emissionScheduler.totalEmissionsMinted(), 0, "No emissions should be recorded");
    }

    // ============ GAS ESTIMATION TESTS ============

    function test_Integration_GasEstimation() public {
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        vm.warp(block.timestamp + 30 days + 1);
        vm.deal(address(emissionScheduler), 1 ether);
        vm.deal(address(mockArbitrumTarget), 1 ether);

        // Measure gas usage for full emission cycle
        uint256 gasBefore = gasleft();
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();
        uint256 gasUsed = gasBefore - gasleft();

        // Gas usage should be reasonable (less than 400k gas with mock bridge)
        assertTrue(gasUsed < 400000, "Gas usage should be under 400k with mock bridge");

        // Log gas usage for analysis
        emit log_named_uint("Gas used for emission + mock bridge", gasUsed);
    }

    // ============ MAXIMUM EMISSION TESTS ============

    function test_Integration_MaximumEmissionAmount() public {
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        // Fast forward to process all 48 epochs
        vm.warp(block.timestamp + 48 * 30 days + 1);
        vm.deal(address(emissionScheduler), 1 ether);
        vm.deal(address(mockArbitrumTarget), 1 ether);

        uint256 initialSupply = token.totalSupply();

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify all epochs were processed
        assertEq(emissionScheduler.currentEpoch(), 48, "All epochs should be completed");
        assertTrue(emissionScheduler.emissionsEnded(), "Emissions should be ended");

        // Verify total supply increased by approximately 100M tokens
        uint256 finalSupply = token.totalSupply();
        uint256 totalEmitted = finalSupply - initialSupply;

        // Should be close to 100M tokens (allowing for rounding)
        assertTrue(
            totalEmitted >= 99_000_000 * 10 ** 18 && totalEmitted <= 101_000_000 * 10 ** 18,
            "Total emissions should be approximately 100M tokens"
        );

        // Verify all tokens went to the bridge
        assertEq(token.balanceOf(address(mockArbitrumTarget)), totalEmitted, "All emitted tokens should be bridged");
    }

    // ============ CONFIGURATION VALIDATION ============

    function test_Integration_BridgeConfiguration() public view {
        // Verify bridge configurations are correct
        (string memory arbName, address arbTarget, bool arbActive) = arbitrumBridge.getBridgeInfo();
        assertEq(arbName, "Arbitrum Bridge");
        assertEq(arbTarget, address(mockArbitrumTarget));
        assertTrue(arbActive);

        (string memory opName, address opTarget, bool opActive) = optimismBridge.getBridgeInfo();
        assertEq(opName, "Optimism Bridge");
        assertEq(opTarget, address(mockOptimismTarget));
        assertTrue(opActive);

        // Verify Arbitrum-specific config
        (address arbRecipient, uint256 arbMaxGas, uint256 arbGasPrice) = arbitrumBridge.getArbitrumConfig();
        assertEq(arbRecipient, l2Recipient);
        assertEq(arbMaxGas, ARB_MAX_GAS);
        assertEq(arbGasPrice, ARB_GAS_PRICE_BID);

        // Verify Optimism-specific config
        (address opL2Token, address opRecipient, uint32 opGas) = optimismBridge.getOptimismConfig();
        assertEq(opL2Token, OP_L2_TOKEN);
        assertEq(opRecipient, l2Recipient);
        assertEq(opGas, OP_L2_GAS);
    }

    receive() external payable {
        // Allow test contract to receive ETH refunds
    }
}

// ============ MOCK BRIDGE TARGET ============

/**
 * @title MockBridgeTarget
 * @notice Mock contract that simulates bridge contract behavior
 */
contract MockBridgeTarget {
    struct BridgeCall {
        address token;
        uint256 amount;
        uint256 ethValue;
        bool called;
    }

    BridgeCall public lastCall;
    bool public shouldRevert;

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }

    // Mock Arbitrum bridge function
    function outboundTransfer(
        address _token,
        address _to,
        uint256 _amount,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        bytes calldata _data
    ) external payable returns (bytes memory) {
        if (shouldRevert) {
            revert("Mock bridge reverted");
        }

        // Transfer tokens to simulate bridge behavior
        IERC20(_token).transferFrom(msg.sender, address(this), _amount);

        // Record the call
        lastCall = BridgeCall({token: _token, amount: _amount, ethValue: msg.value, called: true});

        return "";
    }

    // Mock Optimism bridge function
    function depositERC20To(
        address _l1Token,
        address _l2Token,
        address _to,
        uint256 _amount,
        uint32 _l2Gas,
        bytes calldata _data
    ) external {
        if (shouldRevert) {
            revert("Mock bridge reverted");
        }

        // Transfer tokens to simulate bridge behavior
        IERC20(_l1Token).transferFrom(msg.sender, address(this), _amount);

        // Record the call
        lastCall = BridgeCall({token: _l1Token, amount: _amount, ethValue: 0, called: true});
    }

    function getLastCall() external view returns (BridgeCall memory) {
        return lastCall;
    }

    // Allow contract to receive ETH
    receive() external payable {}
}
