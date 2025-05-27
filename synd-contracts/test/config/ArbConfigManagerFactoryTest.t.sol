// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {ArbConfigManagerFactory} from "src/config/ArbConfigManagerFactory.sol";
import {ArbConfigManager, Ownable} from "src/config/ArbConfigManager.sol";
import {ArbChainConfig} from "src/config/ArbChainConfig.sol";

contract ArbConfigManagerFactoryTest is Test {
    ArbConfigManagerFactory public factory;
    address public owner;
    address public nonOwner;
    bytes32 public salt;

    // Event to test against
    event ArbConfigManagerDeployed(address deployedAddress, address owner);

    function setUp() public {
        // Set up addresses
        owner = address(1);
        nonOwner = address(2);

        // Deploy the factory
        factory = new ArbConfigManagerFactory();

        // Set a default salt
        salt = bytes32(uint256(1));

        // Label addresses for better trace output
        vm.label(address(factory), "Factory");
        vm.label(owner, "Owner");
        vm.label(nonOwner, "NonOwner");
    }

    // ======== Address Prediction Tests ========

    function testPredictDeploymentAddress() public view {
        // Test that the prediction function returns a non-zero address
        address predictedAddress = factory.predictDeploymentAddress(owner, salt);
        assertTrue(predictedAddress != address(0), "Prediction should not be zero address");

        // Test that different salts produce different addresses
        bytes32 differentSalt = bytes32(uint256(2));
        address differentPredictedAddress = factory.predictDeploymentAddress(owner, differentSalt);
        assertTrue(predictedAddress != differentPredictedAddress, "Different salts should produce different addresses");

        // Test that different owners produce different addresses
        address differentOwnerPredictedAddress = factory.predictDeploymentAddress(nonOwner, salt);
        assertTrue(
            predictedAddress != differentOwnerPredictedAddress, "Different owners should produce different addresses"
        );
    }

    // ======== Deployment Tests ========

    function testDeployArbConfigManager() public {
        // Predict the address
        address predictedAddress = factory.predictDeploymentAddress(owner, salt);

        // Deploy the ArbConfigManager
        vm.expectEmit(true, true, false, true);
        emit ArbConfigManagerDeployed(predictedAddress, owner);

        address deployedAddress = factory.deployArbConfigManager(owner, salt);

        // Verify the deployed address matches the prediction
        assertEq(deployedAddress, predictedAddress, "Deployed address should match prediction");

        // Verify that code exists at the address
        uint256 codeSize;
        assembly {
            codeSize := extcodesize(deployedAddress)
        }
        assertTrue(codeSize > 0, "No code at deployed address");

        // Verify that the deployed contract is an ArbConfigManager
        ArbConfigManager manager = ArbConfigManager(deployedAddress);

        // Check that the owner was set correctly
        assertEq(manager.owner(), owner, "Owner not set correctly");

        // Verify that the beacon was set up
        address beaconAddress = address(manager.beacon());
        assertTrue(beaconAddress != address(0), "Beacon not initialized");
    }

    function testCannotRedeployWithSameSalt() public {
        // First deployment
        factory.deployArbConfigManager(owner, salt);

        // Second deployment with same salt should fail
        vm.expectRevert();
        factory.deployArbConfigManager(owner, salt);
    }

    function testDeployWithDifferentSalts() public {
        // First deployment
        address firstDeployment = factory.deployArbConfigManager(owner, salt);

        // Second deployment with different salt
        bytes32 differentSalt = bytes32(uint256(2));
        address secondDeployment = factory.deployArbConfigManager(owner, differentSalt);

        // Addresses should be different
        assertTrue(firstDeployment != secondDeployment, "Different salts should produce different addresses");
    }

    // ======== Helper Function Tests ========

    function testGetBytecode() public view {
        // Test that the bytecode contains information about the owner
        bytes memory bytecode = factory.getBytecode(owner);
        assertTrue(bytecode.length > 0, "Bytecode should not be empty");

        // Test that the bytecode is different for different owners
        bytes memory bytecode2 = factory.getBytecode(nonOwner);
        bool bytecodesDiffer = keccak256(bytecode) != keccak256(bytecode2);
        assertTrue(bytecodesDiffer, "Bytecodes should be different for different owners");
    }

    function testGetAddress() public view {
        bytes memory bytecode = factory.getBytecode(owner);

        // Test that the getAddress function returns a non-zero address
        address calculatedAddress = factory.getAddress(bytecode, salt);
        assertTrue(calculatedAddress != address(0), "Calculated address should not be zero");

        // Test that the address matches the prediction
        address predictedAddress = factory.predictDeploymentAddress(owner, salt);
        assertEq(calculatedAddress, predictedAddress, "Calculated address should match prediction");
    }

    // ======== Deployed Manager Functionality Tests ========

    function testDeployedManagerCanCreateConfig() public {
        // Deploy the manager
        address deployedAddress = factory.deployArbConfigManager(owner, salt);
        ArbConfigManager manager = ArbConfigManager(deployedAddress);

        // Setup parameters for creating a chain config
        uint256 chainId = 1234;
        uint256 sequencingChainId = 5678;
        address arbitrumBridgeAddress = address(10);
        address arbitrumInboxAddress = address(11);
        uint256 settlementDelay = 1000;
        uint256 settlementStartBlock = 2000;
        address sequencingContractAddress = address(12);
        uint256 sequencingStartBlock = 3000;
        address initialAppchainOwner = address(13);
        string memory sequencingChainRpcUrl = "https://example.com/rpc";
        string memory appchainBlockExplorerUrl = "https://example.com/explorer";

        // Non-owner call should revert
        bytes memory errorMessage = abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonOwner);
        vm.prank(nonOwner);
        vm.expectRevert(errorMessage);
        manager.createArbChainConfig(
            owner,
            chainId,
            sequencingChainId,
            arbitrumBridgeAddress,
            arbitrumInboxAddress,
            settlementDelay,
            settlementStartBlock,
            sequencingContractAddress,
            sequencingStartBlock,
            initialAppchainOwner,
            sequencingChainRpcUrl,
            appchainBlockExplorerUrl
        );

        // Owner call should succeed
        vm.prank(owner);
        address configAddress = manager.createArbChainConfig(
            owner,
            chainId,
            sequencingChainId,
            arbitrumBridgeAddress,
            arbitrumInboxAddress,
            settlementDelay,
            settlementStartBlock,
            sequencingContractAddress,
            sequencingStartBlock,
            initialAppchainOwner,
            sequencingChainRpcUrl,
            appchainBlockExplorerUrl
        );

        // Verify the config was created
        assertEq(manager.deployedConfigs(chainId), configAddress, "Config address not registered correctly");

        // Verify the config address matches the deterministic address
        address expectedConfigAddress = manager.getArbChainConfigAddress(chainId);
        assertEq(configAddress, expectedConfigAddress, "Config address does not match deterministic address");

        // Verify the config was initialized correctly by checking some parameters
        ArbChainConfig config = ArbChainConfig(configAddress);
        assertEq(config.owner(), owner, "Config owner not set correctly");
        assertEq(config.CHAIN_ID(), chainId, "Chain ID not set correctly");
        assertEq(config.SEQUENCING_CHAIN_ID(), sequencingChainId, "Sequencing Chain ID not set correctly");
        assertEq(config.ARBITRUM_BRIDGE_ADDRESS(), arbitrumBridgeAddress, "Arbitrum Bridge address not set correctly");
    }

    function testDeployedManagerCanUpgradeImplementation() public {
        // Deploy the manager
        address deployedAddress = factory.deployArbConfigManager(owner, salt);
        ArbConfigManager manager = ArbConfigManager(deployedAddress);

        // Create a new implementation
        ArbChainConfig newImplementation = new ArbChainConfig();

        // Non-owner cannot upgrade
        // Expect revert with OwnableUnauthorizedAccount error
        bytes memory errorMessage = abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, nonOwner);
        vm.prank(nonOwner);
        vm.expectRevert(errorMessage);
        manager.upgradeImplementation(address(newImplementation));

        // Owner can upgrade
        vm.prank(owner);
        manager.upgradeImplementation(address(newImplementation));

        // Verify the upgrade was successful by checking the beacon
        address beaconImplementation = manager.beacon().implementation();
        assertEq(beaconImplementation, address(newImplementation), "Implementation not upgraded");
    }

    function testGetArbChainConfigAddress() public {
        // Deploy the manager
        address deployedAddress = factory.deployArbConfigManager(owner, salt);
        ArbConfigManager manager = ArbConfigManager(deployedAddress);

        uint256 chainId = 1234;

        // Get the deterministic address before deployment
        address deterministicAddress = manager.getArbChainConfigAddress(chainId);
        assertTrue(deterministicAddress != address(0), "Deterministic address should not be zero");

        // Deploy the config
        vm.prank(owner);
        address deployedConfig = manager.createArbChainConfig(
            owner,
            chainId,
            5678, // sequencingChainId
            address(10), // arbitrumBridgeAddress
            address(11), // arbitrumInboxAddress
            1000, // settlementDelay
            2000, // settlementStartBlock
            address(12), // sequencingContractAddress
            3000, // sequencingStartBlock
            address(13), // initialAppchainOwner
            "https://example.com/rpc", // sequencingChainRpcUrl
            "https://example.com/explorer" // appchainBlockExplorerUrl
        );

        // Verify that the deployed address matches the deterministic address
        assertEq(deployedConfig, deterministicAddress, "Deployed config address does not match deterministic address");

        // Verify that calling getArbChainConfigAddress after deployment returns the deployed address
        address retrievedAddress = manager.getArbChainConfigAddress(chainId);
        assertEq(retrievedAddress, deployedConfig, "Retrieved address does not match deployed address");
    }

    function testInvalidChainId() public {
        // Deploy the manager
        address deployedAddress = factory.deployArbConfigManager(owner, salt);
        ArbConfigManager manager = ArbConfigManager(deployedAddress);

        // Try to get the address for chain ID 0
        vm.expectRevert("Chain ID cannot be zero");
        manager.getArbChainConfigAddress(0);

        // Try to create a config with chain ID 0
        vm.prank(owner);
        vm.expectRevert("Chain ID cannot be zero");
        manager.createArbChainConfig(
            owner,
            0, // chainId - this should fail
            5678, // sequencingChainId
            address(10), // arbitrumBridgeAddress
            address(11), // arbitrumInboxAddress
            1000, // settlementDelay
            2000, // settlementStartBlock
            address(12), // sequencingContractAddress
            3000, // sequencingStartBlock
            address(13), // initialAppchainOwner
            "https://example.com/rpc", // sequencingChainRpcUrl
            "https://example.com/explorer" // appchainBlockExplorerUrl
        );
    }

    function testCannotCreateDuplicateConfig() public {
        // Deploy the manager
        address deployedAddress = factory.deployArbConfigManager(owner, salt);
        ArbConfigManager manager = ArbConfigManager(deployedAddress);

        uint256 chainId = 1234;
        // First deployment
        vm.prank(owner);
        manager.createArbChainConfig(
            owner,
            chainId,
            5678, // sequencingChainId
            address(10), // arbitrumBridgeAddress
            address(11), // arbitrumInboxAddress
            1000, // settlementDelay
            2000, // settlementStartBlock
            address(12), // sequencingContractAddress
            3000, // sequencingStartBlock
            address(13), // initialAppchainOwner
            "https://example.com/rpc", // sequencingChainRpcUrl
            "https://example.com/explorer" // appchainBlockExplorerUrl
        );

        // Second deployment with same chainId should fail
        vm.prank(owner);
        vm.expectRevert("Config already exists for this chain ID");
        manager.createArbChainConfig(
            owner,
            chainId, // Same chainId
            5679, // Different sequencingChainId
            address(10),
            address(11),
            1000,
            2000,
            address(12),
            3000,
            address(13),
            "https://example.com/rpc",
            "https://example.com/explorer"
        );
    }

    function testCannotUpgradeToZeroImplementation() public {
        // Deploy the manager
        address deployedAddress = factory.deployArbConfigManager(owner, salt);
        ArbConfigManager manager = ArbConfigManager(deployedAddress);

        // Try to upgrade to zero address
        vm.prank(owner);
        vm.expectRevert("New implementation cannot be zero address");
        manager.upgradeImplementation(address(0));
    }
}
