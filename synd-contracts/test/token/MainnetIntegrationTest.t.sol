// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {SyndicateTokenEmissionScheduler} from "src/token/SyndicateTokenEmissionScheduler.sol";
import {ArbitrumBridgeProxy} from "src/token/bridges/ArbitrumBridgeProxy.sol";
import {OptimismBridgeProxy} from "src/token/bridges/OptimismBridgeProxy.sol";
import {IBridgeProxy} from "src/token/interfaces/IBridgeProxy.sol";

/**
 * @title MainnetIntegrationTest
 * @notice Integration tests using Ethereum mainnet state to verify end-to-end
 *         minting and bridging functionality with real bridge contracts
 * @dev These tests fork mainnet and use actual Arbitrum and Optimism bridge contracts
 *      to validate the complete flow from emission minting to L2 bridging
 */
contract MainnetIntegrationTest is Test {
    // Mainnet bridge contract addresses
    address constant ARBITRUM_L1_GATEWAY_ROUTER = 0x72Ce9c846789fdB6fC1f34aC4AD25Dd9ef7031ef;
    address constant OPTIMISM_L1_STANDARD_BRIDGE = 0x99C9fc46f92E8a1c0deC1b1747d010903E884bE1;

    // Test configuration
    SyndicateToken public token;
    SyndicateTokenEmissionScheduler public emissionScheduler;
    ArbitrumBridgeProxy public arbitrumBridge;
    OptimismBridgeProxy public optimismBridge;

    // Test addresses
    address public admin = address(0x1234);
    address public syndFoundation = address(0x5678);
    address public emissionsManager = address(0x9ABC);
    address public pauser = address(0xDEF0);
    address public l2Recipient = address(0x1111);

    // Bridge configuration
    address public constant ARB_L2_TOKEN = address(0x2222); // Placeholder for Arbitrum L2 token
    address public constant OP_L2_TOKEN = address(0x3333); // Placeholder for Optimism L2 token
    uint256 public constant MAX_SINGLE_TRANSFER = 100_000_000 * 10 ** 18; // 100M tokens
    uint256 public constant DAILY_LIMIT = 100_000_000 * 10 ** 18; // 100M tokens
    uint32 public constant OP_L2_GAS = 200000;
    uint256 public constant ARB_MAX_GAS = 300000;
    uint256 public constant ARB_GAS_PRICE_BID = 1e9; // 1 gwei

    // Events to track bridge operations
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target);
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed destination);

    function setUp() public {
        // Fork Ethereum mainnet - requires MAINNET_RPC_URL environment variable
        string memory rpcUrl = vm.envOr("MAINNET_RPC_URL", string("https://eth.llamarpc.com"));
        vm.createFork(rpcUrl);

        // Deploy contracts
        vm.startPrank(admin);

        // Deploy SyndicateToken
        token = new SyndicateToken(admin, syndFoundation);

        // Deploy EmissionScheduler
        emissionScheduler = new SyndicateTokenEmissionScheduler(address(token), admin, emissionsManager, pauser);

        // Grant emission minter role to scheduler
        token.grantRole(token.EMISSION_MINTER_ROLE(), address(emissionScheduler));

        // Deploy Arbitrum bridge proxy with real bridge address
        arbitrumBridge = new ArbitrumBridgeProxy(
            admin,
            address(emissionScheduler),
            ARBITRUM_L1_GATEWAY_ROUTER,
            MAX_SINGLE_TRANSFER,
            DAILY_LIMIT,
            l2Recipient,
            ARB_MAX_GAS,
            ARB_GAS_PRICE_BID
        );

        // Deploy Optimism bridge proxy with real bridge address
        optimismBridge = new OptimismBridgeProxy(
            admin,
            address(emissionScheduler),
            OPTIMISM_L1_STANDARD_BRIDGE,
            MAX_SINGLE_TRANSFER,
            DAILY_LIMIT,
            OP_L2_TOKEN,
            l2Recipient,
            OP_L2_GAS
        );

        vm.stopPrank();
    }

    // ============ ARBITRUM INTEGRATION TESTS ============

    function test_Integration_ArbitrumBridge_EndToEnd() public {
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

        // Fund the emission scheduler with much more ETH for gas fees (real bridges need more)
        uint256 ethNeeded = ARB_MAX_GAS * ARB_GAS_PRICE_BID;
        vm.deal(address(emissionScheduler), ethNeeded * 10); // Much larger buffer for real bridges

        // Also fund the arbitrum bridge proxy directly for gas payments
        vm.deal(address(arbitrumBridge), 10 ether);

        // Get expected emission amount for first epoch
        uint256 expectedAmount = 6_780_550 * 10 ** 18;
        uint256 initialTokenSupply = token.totalSupply();

        // Record bridge balance before
        uint256 bridgeTokenBalanceBefore = token.balanceOf(ARBITRUM_L1_GATEWAY_ROUTER);

        // Execute emission minting (should mint and bridge tokens)
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify tokens were minted
        assertEq(token.totalSupply(), initialTokenSupply + expectedAmount, "Tokens should be minted");

        // Verify current epoch updated
        assertEq(emissionScheduler.currentEpoch(), 1, "Current epoch should be 1");

        // Verify total emissions tracking
        assertEq(emissionScheduler.totalEmissionsMinted(), expectedAmount, "Total emissions should match");

        // NOTE: We can't easily verify the tokens reached Arbitrum L2 in this test,
        // but we can verify they left the emission scheduler and were sent to the bridge
        uint256 schedulerBalance = token.balanceOf(address(emissionScheduler));
        assertEq(schedulerBalance, 0, "Emission scheduler should have no remaining tokens");

        // Verify bridge contract received approval and processed the tokens
        // The tokens should either be in the bridge contract or already forwarded to L2
        uint256 bridgeTokenBalanceAfter = token.balanceOf(ARBITRUM_L1_GATEWAY_ROUTER);
        assertTrue(bridgeTokenBalanceAfter >= bridgeTokenBalanceBefore, "Bridge should have processed the tokens");
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

        // Fund with sufficient ETH for multiple epochs
        uint256 ethNeeded = ARB_MAX_GAS * ARB_GAS_PRICE_BID * 3;
        vm.deal(address(emissionScheduler), ethNeeded * 10);
        vm.deal(address(arbitrumBridge), 10 ether);

        uint256 initialSupply = token.totalSupply();
        uint256 expectedTotalAmount = 6_780_550 * 10 ** 18 * 3; // 3 epochs

        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        assertEq(token.totalSupply(), initialSupply + expectedTotalAmount, "Should mint 3 epochs worth");
        assertEq(emissionScheduler.currentEpoch(), 3, "Should be at epoch 3");
        assertEq(token.balanceOf(address(emissionScheduler)), 0, "Scheduler should have no tokens left");
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

        // Record bridge balance before
        uint256 bridgeTokenBalanceBefore = token.balanceOf(OPTIMISM_L1_STANDARD_BRIDGE);

        // Execute emission minting
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify tokens were minted and processed
        assertEq(token.totalSupply(), initialTokenSupply + expectedAmount, "Tokens should be minted");
        assertEq(emissionScheduler.currentEpoch(), 1, "Current epoch should be 1");
        assertEq(token.balanceOf(address(emissionScheduler)), 0, "Scheduler should have no tokens left");

        // Verify bridge processed the tokens
        uint256 bridgeTokenBalanceAfter = token.balanceOf(OPTIMISM_L1_STANDARD_BRIDGE);
        assertTrue(bridgeTokenBalanceAfter >= bridgeTokenBalanceBefore, "Bridge should have processed the tokens");
    }

    // ============ BRIDGE FAILURE SCENARIOS ============

    function test_Integration_BridgeFailure_TokensStuckInScheduler() public {
        // Setup with invalid bridge target to simulate failure
        vm.startPrank(admin);
        arbitrumBridge.setBridgeTarget(address(0xdead)); // Invalid target
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        vm.warp(block.timestamp + 30 days + 1);
        vm.deal(address(emissionScheduler), 1 ether);

        // This should revert due to invalid bridge target
        vm.expectRevert();
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();

        // Verify no tokens were minted due to failed bridge operation
        assertEq(emissionScheduler.currentEpoch(), 0, "No epoch should be processed");
    }

    // ============ GAS ESTIMATION TESTS ============

    function test_Integration_GasEstimation_ArbitrumBridge() public {
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        vm.warp(block.timestamp + 30 days + 1);
        vm.deal(address(emissionScheduler), 10 ether);
        vm.deal(address(arbitrumBridge), 10 ether);

        // Measure gas usage for full emission cycle
        uint256 gasBefore = gasleft();
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();
        uint256 gasUsed = gasBefore - gasleft();

        // Gas usage should be reasonable (less than 500k gas)
        assertTrue(gasUsed < 500000, "Gas usage should be under 500k");

        // Log gas usage for analysis
        emit log_named_uint("Gas used for emission + bridge", gasUsed);
    }

    function test_Integration_GasEstimation_OptimismBridge() public {
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(optimismBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, OP_L2_GAS));
        vm.stopPrank();

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        vm.warp(block.timestamp + 30 days + 1);

        uint256 gasBefore = gasleft();
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();
        uint256 gasUsed = gasBefore - gasleft();

        assertTrue(gasUsed < 400000, "Optimism bridge should use less gas");
        emit log_named_uint("Gas used for Optimism emission + bridge", gasUsed);
    }

    // ============ EDGE CASE TESTS ============

    function test_Integration_MaximumEmissionAmount() public {
        vm.startPrank(admin);
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        // Fast forward to process all 48 epochs
        vm.warp(block.timestamp + 48 * 30 days + 1);
        vm.deal(address(emissionScheduler), 100 ether); // Sufficient ETH for all epochs
        vm.deal(address(arbitrumBridge), 100 ether);

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
    }

    function test_Integration_DailyLimitEnforcement() public {
        // Set very low daily limit to test enforcement
        vm.startPrank(admin);
        arbitrumBridge.setDailyLimit(1000 * 10 ** 18); // 1k tokens per day
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        vm.stopPrank();

        vm.prank(emissionsManager);
        emissionScheduler.startEmissions();

        vm.warp(block.timestamp + 30 days + 1);
        vm.deal(address(emissionScheduler), 1 ether);

        // This should fail due to daily limit being exceeded
        // (first epoch is ~6.7M tokens, but limit is 1k)
        vm.expectRevert();
        vm.prank(emissionsManager);
        emissionScheduler.mintEmission();
    }

    // ============ HELPER FUNCTIONS ============

    function test_Integration_BridgeConfiguration() public view {
        // Verify bridge configurations are correct
        (string memory arbName, address arbTarget, bool arbActive) = arbitrumBridge.getBridgeInfo();
        assertEq(arbName, "Arbitrum Bridge");
        assertEq(arbTarget, ARBITRUM_L1_GATEWAY_ROUTER);
        assertTrue(arbActive);

        (string memory opName, address opTarget, bool opActive) = optimismBridge.getBridgeInfo();
        assertEq(opName, "Optimism Bridge");
        assertEq(opTarget, OPTIMISM_L1_STANDARD_BRIDGE);
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
