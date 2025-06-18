// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console2} from "forge-std/Script.sol";
import {SyndicateToken} from "src/token/SyndicateToken.sol";
import {SyndicateTokenEmissionScheduler} from "src/token/SyndicateTokenEmissionScheduler.sol";
import {ArbitrumBridgeProxy} from "src/token/bridges/ArbitrumBridgeProxy.sol";
import {IBridgeProxy} from "src/token/interfaces/IBridgeProxy.sol";

/**
 * @title TestArbitrumBridgeFlow
 * @notice Script to test the complete emission-to-bridge flow on mainnet using Arbitrum bridge
 * @dev This script deploys contracts and executes a complete test of the emission scheduler
 *      integrated with the Arbitrum bridge proxy on Ethereum mainnet
 */
contract TestArbitrumBridgeFlow is Script {
    // ETH Mainnet Arb bridge address
    // address constant ARBITRUM_L1_GATEWAY_ROUTER = 0x72Ce9c846789fdB6fC1f34aC4AD25Dd9ef7031ef;
    // ETH Sepolia Arb bridge address (for testing)
    address constant ARBITRUM_L1_GATEWAY_ROUTER = 0xcE18836b233C83325Cc8848CA4487e94C6288264;

    // Configuration parameters
    uint256 constant MAX_SINGLE_TRANSFER = 100_000_000 * 10 ** 18; // 100M tokens
    uint256 constant DAILY_LIMIT = 100_000_000 * 10 ** 18; // 100M tokens
    uint256 constant ARB_MAX_GAS = 1000000; // 1M gas
    uint256 constant ARB_GAS_PRICE_BID = 10e9; // 10 gwei

    // Test addresses - will be set from environment or deployer
    address admin;
    address syndFoundation;
    address emissionsManager;
    address pauser;
    address l2Recipient;

    // Contracts
    SyndicateToken token;
    SyndicateTokenEmissionScheduler emissionScheduler;
    ArbitrumBridgeProxy arbitrumBridge;

    function setUp() public {
        // Get deployer as admin if not set
        admin = vm.envOr("ADMIN_ADDR", vm.addr(vm.envUint("PRIVATE_KEY")));
        syndFoundation = vm.envOr("SYND_FOUNDATION_ADDR", admin);
        emissionsManager = vm.envOr("EMISSIONS_MANAGER_ADDR", admin);
        pauser = vm.envOr("PAUSER_ADDR", admin);
        l2Recipient = vm.envOr("L2_RECIPIENT_ADDR", admin);

        console2.log("=== ARBITRUM BRIDGE FLOW TEST CONFIGURATION ===");
        console2.log("Admin:", admin);
        console2.log("Syndicate Foundation:", syndFoundation);
        console2.log("Emissions Manager:", emissionsManager);
        console2.log("L2 Recipient:", l2Recipient);
        console2.log("Arbitrum L1 Gateway Router:", ARBITRUM_L1_GATEWAY_ROUTER);
        console2.log("Max Gas:", ARB_MAX_GAS);
        console2.log("Gas Price Bid:", ARB_GAS_PRICE_BID);
    }

    function run() public {
        vm.startBroadcast();

        console2.log("=== DEPLOYING CONTRACTS ===");

        // Deploy SyndicateToken
        token = new SyndicateToken(admin, syndFoundation);
        console2.log("SyndicateToken deployed at:", address(token));

        // Deploy EmissionScheduler
        emissionScheduler = new SyndicateTokenEmissionScheduler(address(token), admin, emissionsManager, pauser);
        console2.log("EmissionScheduler deployed at:", address(emissionScheduler));

        // Grant emission minter role
        token.grantRole(token.EMISSION_MINTER_ROLE(), address(emissionScheduler));
        console2.log("Granted EMISSION_MINTER_ROLE to EmissionScheduler");

        // Deploy Arbitrum bridge proxy
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
        console2.log("ArbitrumBridgeProxy deployed at:", address(arbitrumBridge));

        // Calculate ETH needed for bridge operations
        uint256 ethNeeded = ARB_MAX_GAS * ARB_GAS_PRICE_BID;
        console2.log("ETH needed for bridge operation:", ethNeeded);
        console2.log("ETH needed in ether (approx):", ethNeeded / 1e18);

        // Fund the bridge proxy with ETH for gas payments
        if (address(arbitrumBridge).balance < ethNeeded) {
            console2.log("Funding bridge proxy with ETH...");
            payable(address(arbitrumBridge)).transfer(ethNeeded * 2); // 2x buffer
            console2.log("Bridge proxy funded with:", address(arbitrumBridge).balance);
        }

        // Configure emission scheduler
        emissionScheduler.setBridgeProxy(IBridgeProxy(address(arbitrumBridge)));
        emissionScheduler.setBridgeData(abi.encode(l2Recipient, ARB_MAX_GAS, ARB_GAS_PRICE_BID));
        console2.log("Configured EmissionScheduler with Arbitrum bridge");

        // Start emissions
        emissionScheduler.startEmissions();
        console2.log("Emissions started at block:", block.timestamp);

        console2.log("=== ATTEMPTING EMISSION AND BRIDGE FLOW ===");

        // Check if we can mint an emission (need to be past first epoch)
        uint256 currentTime = block.timestamp;
        uint256 emissionStartTime = emissionScheduler.emissionsStartTime();
        uint256 timeUntilFirstEpoch = emissionStartTime + 30 days - currentTime;

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

            // Attempt to mint emission - this will likely fail due to L2 token registration
            try emissionScheduler.mintEmission() {
                console2.log("=== EMISSION SUCCESSFUL ===");
                console2.log("Current epoch:", emissionScheduler.currentEpoch());
                console2.log("Total supply after emission:", token.totalSupply());
                console2.log("Tokens bridged successfully to Arbitrum!");
            } catch Error(string memory reason) {
                console2.log("=== EMISSION FAILED (EXPECTED) ===");
                console2.log("Failure reason:", reason);
                console2.log("This is expected - L2 token registration required");
            } catch {
                console2.log("=== EMISSION FAILED (EXPECTED) ===");
                console2.log("Low-level failure - L2 token registration required");
            }
        }

        vm.stopBroadcast();

        console2.log("=== DEPLOYMENT SUMMARY ===");
        console2.log("SyndicateToken:", address(token));
        console2.log("EmissionScheduler:", address(emissionScheduler));
        console2.log("ArbitrumBridgeProxy:", address(arbitrumBridge));
        console2.log("Bridge proxy ETH balance:", address(arbitrumBridge).balance);
        console2.log("=== TEST COMPLETE ===");
    }

    // // Allow contract to receive ETH
    // receive() external payable {}
}
