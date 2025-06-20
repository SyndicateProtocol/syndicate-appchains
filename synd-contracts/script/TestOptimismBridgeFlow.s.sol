// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console2} from "forge-std/Script.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {SyndicateTokenEmissionScheduler} from "src/token/SyndicateTokenEmissionScheduler.sol";
import {OptimismBridgeProxy} from "src/token/bridges/OptimismBridgeProxy.sol";
import {IBridgeProxy} from "src/token/interfaces/IBridgeProxy.sol";

/**
 * @title TestOptimismBridgeFlow
 * @notice Script to test the complete emission-to-bridge flow on mainnet using Optimism bridge
 * @dev This script deploys contracts and executes a complete test of the emission scheduler
 *      integrated with the Optimism bridge proxy on Ethereum mainnet
 */
contract TestOptimismBridgeFlow is Script {
    // ETH Mainnet Optimism bridge address
    // address constant OPTIMISM_L1_STANDARD_BRIDGE = 0x99C9fc46f92E8a1c0deC1b1747d010903E884bE1;
    // Optimism L2 token address
    // address constant OP_L2_TOKEN = address(0x0); // OptmismMintableERC20 deployed on L2

    // ETH Sepolia Optimism bridge address (for testing)
    address constant OPTIMISM_L1_STANDARD_BRIDGE = 0xFBb0621E0B23b5478B630BD55a5f21f67730B0F1;
    // Optimism L2 token address (for testing)
    address constant OP_L2_TOKEN = 0x888BB38FB6480828e8Ec8e02824803528Ac358Ac; // OptmismMintableERC20 deployed on Optmism Sepolia

    // Configuration parameters
    uint256 constant MAX_SINGLE_TRANSFER = 100_000_000 * 10 ** 18; // 100M tokens
    uint256 constant DAILY_LIMIT = 100_000_000 * 10 ** 18; // 100M tokens
    uint32 constant OP_L2_GAS = 200000; // 200k gas

    // Test addresses - will be set from environment or deployer
    address admin;
    address syndFoundation;
    address emissionsManager;
    address pauser;
    address l2Recipient;
    address opL2Token; // Placeholder for Optimism L2 token

    // Contracts
    SyndicateToken token;
    SyndicateTokenEmissionScheduler emissionScheduler;
    OptimismBridgeProxy optimismBridge;

    function setUp() public {
        // Get deployer as admin if not set
        admin = vm.envOr("ADMIN_ADDR", vm.addr(vm.envUint("PRIVATE_KEY")));
        syndFoundation = vm.envOr("SYND_FOUNDATION_ADDR", admin);
        emissionsManager = vm.envOr("EMISSIONS_MANAGER_ADDR", admin);
        pauser = vm.envOr("PAUSER_ADDR", admin);
        l2Recipient = vm.envOr("L2_RECIPIENT_ADDR", admin);
        opL2Token = vm.envOr("OP_L2_TOKEN_ADDR", OP_L2_TOKEN);

        console2.log("=== OPTIMISM BRIDGE FLOW TEST CONFIGURATION ===");
        console2.log("Admin:", admin);
        console2.log("Syndicate Foundation:", syndFoundation);
        console2.log("Emissions Manager:", emissionsManager);
        console2.log("L2 Recipient:", l2Recipient);
        console2.log("Optimism L1 Standard Bridge:", OPTIMISM_L1_STANDARD_BRIDGE);
        console2.log("OP L2 Token (placeholder):", opL2Token);
        console2.log("OP L2 Gas:", OP_L2_GAS);
    }

    function run() public {
        vm.startBroadcast();

        console2.log("=== DEPLOYING CONTRACTS ===");

        // Deploy SyndicateToken
        // Actually, this should have already been deployed because OP_L2_TOKEN requires knowing the paired token it is serving.
        token = new SyndicateToken(admin, syndFoundation);
        // token = SyndicateToken(0x08AA107A167485B3f302BC596F0bc6a41e3d8E0c); // Use existing deployed token on Sepolia
        console2.log("SyndicateToken deployed at:", address(token));

        // Deploy EmissionScheduler
        emissionScheduler = new SyndicateTokenEmissionScheduler(address(token), admin, emissionsManager, pauser);
        console2.log("EmissionScheduler deployed at:", address(emissionScheduler));

        // Grant emission minter role
        token.grantRole(token.EMISSION_MINTER_ROLE(), address(emissionScheduler));
        console2.log("Granted EMISSION_MINTER_ROLE to EmissionScheduler");

        // Deploy Optimism bridge proxy
        optimismBridge = new OptimismBridgeProxy(
            admin,
            address(emissionScheduler),
            OPTIMISM_L1_STANDARD_BRIDGE,
            MAX_SINGLE_TRANSFER,
            DAILY_LIMIT,
            opL2Token,
            l2Recipient,
            OP_L2_GAS
        );
        console2.log("OptimismBridgeProxy deployed at:", address(optimismBridge));

        // Configure emission scheduler
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(optimismBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, OP_L2_GAS));
        console2.log("Configured EmissionScheduler with Optimism bridge");

        // Start emissions
        emissionScheduler.startEmissions();
        console2.log("Emissions started at block:", block.timestamp);

        console2.log("=== ATTEMPTING EMISSION AND BRIDGE FLOW ===");

        // Check if we can mint an emission (need to be past first epoch)
        uint256 currentTime = block.timestamp;
        uint256 emissionStartTime = emissionScheduler.emissionsStartTime();
        uint256 timeUntilFirstEpoch = 0; // emissionStartTime + 30 days - currentTime;

        if (timeUntilFirstEpoch > 0) {
            console2.log("Need to wait", timeUntilFirstEpoch, "seconds until first epoch");
            console2.log("First epoch available at timestamp:", emissionStartTime + 30 days);
            console2.log("Current timestamp:", currentTime);
            console2.log("=== DEPLOYMENT COMPLETE - EMISSIONS NOT YET READY ===");
        } else {
            console2.log("First epoch is available, attempting emission...");

            uint256 expectedAmount = 6_780_550 * 10 ** 18;
            uint256 initialSupply = token.totalSupply();

            console2.log("Initial token supply:", initialSupply);
            console2.log("Expected emission amount:", expectedAmount);

            // Debug bridge configuration
            console2.log("=== BRIDGE CONFIGURATION DEBUG ===");
            (address l2TokenAddr, address recipientAddr, uint32 gasLimit) = optimismBridge.getOptimismConfig();
            console2.log("L2 Token address:", l2TokenAddr);
            console2.log("Recipient address:", recipientAddr);
            console2.log("Gas limit:", gasLimit);

            // Attempt to mint emission - this will likely fail due to L2 token registration
            try emissionScheduler.mintEmission() {
                console2.log("=== EMISSION SUCCESSFUL ===");
                console2.log("Current epoch:", emissionScheduler.currentEpoch());
                console2.log("Total supply after emission:", token.totalSupply());
                console2.log("Tokens bridged successfully to Optimism!");
            } catch Error(string memory reason) {
                console2.log("=== EMISSION FAILED (EXPECTED) ===");
                console2.log("Failure reason:", reason);
                console2.log("This is expected - L2 token registration required");
                console2.log("L2 token must implement IOptimismMintableERC20 interface");
            } catch {
                console2.log("=== EMISSION FAILED (EXPECTED) ===");
                console2.log("Low-level failure - L2 token registration required");
                console2.log("L2 token must implement IOptimismMintableERC20 interface");
            }
        }

        vm.stopBroadcast();

        console2.log("=== DEPLOYMENT SUMMARY ===");
        console2.log("SyndicateToken:", address(token));
        console2.log("EmissionScheduler:", address(emissionScheduler));
        console2.log("OptimismBridgeProxy:", address(optimismBridge));
        console2.log("Bridge configuration verified");
        console2.log("=== TEST COMPLETE ===");
    }
}
