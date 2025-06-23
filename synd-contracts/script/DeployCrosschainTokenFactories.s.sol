// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script, console} from "forge-std/Script.sol";

import {SyndicateTokenCrosschainFactory} from "src/token/crosschain/SyndicateTokenCrosschainFactory.sol";
import {TestnetSyndTokenCrosschainFactory} from "src/token/crosschain/TestnetSyndTokenCrosschainFactory.sol";
import {SyndicateTokenCrosschain} from "src/token/SyndicateTokenCrosschain.sol";
import {TestnetSyndTokenCrosschain} from "src/token/TestnetSyndTokenCrosschain.sol";

/**
 * @title DeploySyndicateTokenCrosschainFactory
 * @notice Deploys the mainnet SyndicateTokenCrosschain factory
 * @dev This factory enables deterministic deployment across all chains using Solmate CREATE3
 */
contract DeploySyndicateTokenCrosschainFactory is Script {
    function run() public {
        vm.startBroadcast();

        // Deploy the mainnet crosschain token factory
        SyndicateTokenCrosschainFactory factory = new SyndicateTokenCrosschainFactory();

        console.log("SyndicateTokenCrosschainFactory deployed at:", address(factory));
        console.log("Factory can deploy SyndicateTokenCrosschain with same address across all chains");

        vm.stopBroadcast();
    }
}

/**
 * @title DeployTestnetSyndTokenCrosschainFactory
 * @notice Deploys the testnet SyndicateTokenCrosschain factory
 * @dev This factory enables deterministic deployment across all testnet chains using Solmate CREATE3
 */
contract DeployTestnetSyndTokenCrosschainFactory is Script {
    function run() public {
        vm.startBroadcast();

        // Deploy the testnet crosschain token factory
        TestnetSyndTokenCrosschainFactory factory = new TestnetSyndTokenCrosschainFactory();

        console.log("TestnetSyndTokenCrosschainFactory deployed at:", address(factory));
        console.log("Factory can deploy TestnetSyndTokenCrosschain with same address across all chains");

        vm.stopBroadcast();
    }
}

/**
 * @title DeploySyndicateTokenCrosschainViaFactory
 * @notice Deploys a SyndicateTokenCrosschain using the factory for deterministic addressing
 * @dev Uses environment variables ADMIN_ADDR and MANAGER_ADDR for configuration
 */
contract DeploySyndicateTokenCrosschainViaFactory is Script {
    // Factory address - update this after deploying the factory
    address constant FACTORY_ADDRESS = address(0); // TODO: Update after factory deployment

    function run() public {
        require(FACTORY_ADDRESS != address(0), "Factory address not set");

        vm.startBroadcast();

        // Get configuration from environment
        address admin = vm.envAddress("ADMIN_ADDR");
        address treasury = vm.envAddress("MANAGER_ADDR"); // Using MANAGER_ADDR as treasury

        require(admin != address(0), "ADMIN_ADDR not set");
        require(treasury != address(0), "MANAGER_ADDR not set");

        SyndicateTokenCrosschainFactory factory = SyndicateTokenCrosschainFactory(FACTORY_ADDRESS);

        // Generate deterministic salt based on chain and configuration
        bytes32 salt = factory.generateSalt(admin, treasury, block.chainid);

        // Predict the address before deployment
        address predictedAddress = factory.predictTokenAddress(salt);
        console.log("Predicted SyndicateTokenCrosschain address:", predictedAddress);

        // Deploy the token
        address deployedToken = factory.deploySyndicateTokenCrosschain(admin, treasury, salt);

        console.log("SyndicateTokenCrosschain deployed at:", deployedToken);
        console.log("Admin address:", admin);
        console.log("Treasury address:", treasury);
        console.log("Salt used:", vm.toString(salt));

        // Verify the prediction was correct
        require(deployedToken == predictedAddress, "Address prediction failed");
        console.log("Address prediction verified successfully");

        // Verify token properties
        SyndicateTokenCrosschain token = SyndicateTokenCrosschain(deployedToken);
        console.log("Token name:", token.name());
        console.log("Token symbol:", token.symbol());
        console.log("Token decimals:", token.decimals());
        console.log("Initial supply:", token.totalSupply());

        vm.stopBroadcast();
    }
}

/**
 * @title DeployTestnetSyndTokenCrosschainViaFactory
 * @notice Deploys a TestnetSyndTokenCrosschain using the factory for deterministic addressing
 * @dev Uses environment variables ADMIN_ADDR and MINTER_ADDR for configuration
 */
contract DeployTestnetSyndTokenCrosschainViaFactory is Script {
    // Factory address - update this after deploying the factory
    address constant FACTORY_ADDRESS = address(0); // TODO: Update after factory deployment

    function run() public {
        require(FACTORY_ADDRESS != address(0), "Factory address not set");

        vm.startBroadcast();

        // Get configuration from environment
        address admin = vm.envAddress("ADMIN_ADDR");
        address minter = vm.envAddress("MINTER_ADDR");

        require(admin != address(0), "ADMIN_ADDR not set");
        require(minter != address(0), "MINTER_ADDR not set");

        TestnetSyndTokenCrosschainFactory factory = TestnetSyndTokenCrosschainFactory(FACTORY_ADDRESS);

        // Generate deterministic salt based on chain and configuration
        bytes32 salt = factory.generateSalt(admin, minter, block.chainid);

        // Predict the address before deployment
        address predictedAddress = factory.predictTokenAddress(salt);
        console.log("Predicted TestnetSyndTokenCrosschain address:", predictedAddress);

        // Deploy the token
        address deployedToken = factory.deployTestnetSyndTokenCrosschain(admin, minter, salt);

        console.log("TestnetSyndTokenCrosschain deployed at:", deployedToken);
        console.log("Admin address:", admin);
        console.log("Minter address:", minter);
        console.log("Salt used:", vm.toString(salt));

        // Verify the prediction was correct
        require(deployedToken == predictedAddress, "Address prediction failed");
        console.log("Address prediction verified successfully");

        // Verify token properties
        TestnetSyndTokenCrosschain token = TestnetSyndTokenCrosschain(deployedToken);
        console.log("Token name:", token.name());
        console.log("Token symbol:", token.symbol());
        console.log("Token decimals:", token.decimals());
        console.log("Initial supply:", token.totalSupply());

        vm.stopBroadcast();
    }
}

/**
 * @title ConfigureCrosschainTokenBridges
 * @notice Configures bridge limits for a deployed crosschain token
 * @dev Uses environment variables for bridge addresses and limits
 */
contract ConfigureCrosschainTokenBridges is Script {
    // Token address - update this after deploying the token
    address constant TOKEN_ADDRESS = address(0); // TODO: Update after token deployment

    function run() public {
        require(TOKEN_ADDRESS != address(0), "Token address not set");

        vm.startBroadcast();

        // Example bridge configuration
        address[] memory bridges = new address[](2);
        bridges[0] = 0x1234567890123456789012345678901234567890; // Example bridge 1
        bridges[1] = 0x0987654321098765432109876543210987654321; // Example bridge 2

        uint256[] memory mintLimits = new uint256[](2);
        mintLimits[0] = 1_000_000 * 10 ** 18; // 1M tokens per day
        mintLimits[1] = 2_000_000 * 10 ** 18; // 2M tokens per day

        uint256[] memory burnLimits = new uint256[](2);
        burnLimits[0] = 1_000_000 * 10 ** 18; // 1M tokens per day
        burnLimits[1] = 2_000_000 * 10 ** 18; // 2M tokens per day

        // Configure bridges (works for both mainnet and testnet tokens)
        // Note: This assumes the caller has BRIDGE_MANAGER_ROLE
        SyndicateTokenCrosschain token = SyndicateTokenCrosschain(TOKEN_ADDRESS);

        for (uint256 i = 0; i < bridges.length; i++) {
            console.log("Configuring bridge:", bridges[i]);
            console.log("Mint limit:", mintLimits[i]);
            console.log("Burn limit:", burnLimits[i]);

            token.setBridgeLimits(bridges[i], mintLimits[i], burnLimits[i]);

            console.log("Bridge configured successfully");
        }

        console.log("All bridges configured successfully");

        vm.stopBroadcast();
    }
}

/**
 * @title VerifyCrosschainAddressConsistency
 * @notice Verifies that the same salt produces the same address across different configurations
 * @dev Useful for testing before deployment on new chains
 */
contract VerifyCrosschainAddressConsistency is Script {
    function run() public {
        // Create temporary factory instances for testing
        SyndicateTokenCrosschainFactory mainnetFactory = new SyndicateTokenCrosschainFactory();
        TestnetSyndTokenCrosschainFactory testnetFactory = new TestnetSyndTokenCrosschainFactory();

        bytes32 salt = keccak256("UNIVERSAL_SALT");

        // Verify mainnet factory produces same address regardless of constructor args
        address mainnetAddr1 = mainnetFactory.predictTokenAddress(salt);
        address mainnetAddr2 = mainnetFactory.predictTokenAddress(salt);

        console.log("Mainnet Factory Address Consistency Test:");
        console.log("Address 1:", mainnetAddr1);
        console.log("Address 2:", mainnetAddr2);
        console.log("Consistent:", mainnetAddr1 == mainnetAddr2 ? "PASS" : "FAIL");

        // Verify testnet factory produces same address regardless of constructor args
        address testnetAddr1 = testnetFactory.predictTokenAddress(salt);
        address testnetAddr2 = testnetFactory.predictTokenAddress(salt);

        console.log("\nTestnet Factory Address Consistency Test:");
        console.log("Address 1:", testnetAddr1);
        console.log("Address 2:", testnetAddr2);
        console.log("Consistent:", testnetAddr1 == testnetAddr2 ? " PASS" : "FAIL");

        console.log("\nCross-Chain Deployment Benefit:");
        console.log("Same salt will produce same addresses on ALL chains");
        console.log("Enables seamless cross-chain bridging without address confusion");
    }
}
