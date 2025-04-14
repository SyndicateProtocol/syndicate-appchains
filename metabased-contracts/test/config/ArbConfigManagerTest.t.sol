// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {ArbConfigManager} from "src/config/ArbConfigManager.sol";
import {ArbChainConfig} from "src/config/ArbChainConfig.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IBeacon} from "@openzeppelin/contracts/proxy/beacon/IBeacon.sol";
import {BeaconProxy} from "@openzeppelin/contracts/proxy/beacon/BeaconProxy.sol";

contract ArbConfigManagerTest is Test {
    ArbConfigManager public configManager;
    address public owner = address(1);
    address public rollupOwner = address(2);

    uint256 public constant CHAIN_ID = 123456;
    address public constant ARBITRUM_BRIDGE_ADDRESS = address(0x1234);
    address public constant ARBITRUM_INBOX_ADDRESS = address(0x5678);
    bool public constant ARBITRUM_IGNORE_DELAYED_MESSAGES = false;
    uint256 public constant SETTLEMENT_DELAY = 10;
    uint256 public constant SETTLEMENT_START_BLOCK = 100;
    address public constant SEQUENCING_CONTRACT_ADDRESS = address(0x9ABC);
    uint256 public constant SEQUENCING_START_BLOCK = 200;
    string public constant DEFAULT_RPC_URL = "https://example.com/rpc";

    function setUp() public {
        vm.startPrank(owner);
        configManager = new ArbConfigManager(owner);
        vm.stopPrank();
    }

    function testDeployment() public view {
        assertEq(configManager.owner(), owner);

        // Check that the beacon was created and has an implementation
        address beaconAddress = address(configManager.beacon());
        assertTrue(beaconAddress != address(0));

        // Check that the beacon has an implementation set
        address implementation = IBeacon(beaconAddress).implementation();
        assertTrue(implementation != address(0));

        // Check the implementation is a contract
        uint256 codeSize;
        assembly {
            codeSize := extcodesize(implementation)
        }
        assertTrue(codeSize > 0, "Implementation should be a contract");
    }

    function testCreateArbChainConfig() public {
        vm.startPrank(owner);

        // Deploy the config without expecting event
        address deployedAddress = configManager.createArbChainConfig(
            CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL
        );

        // Verify the deployed address is stored correctly
        assertEq(configManager.deployedConfigs(CHAIN_ID), deployedAddress);

        // Verify it's a BeaconProxy
        uint256 codeSize;
        assembly {
            codeSize := extcodesize(deployedAddress)
        }
        assertTrue(codeSize > 0, "BeaconProxy should be a contract");

        // Verify the ArbChainConfig values have been properly initialized
        // We need to cast address to ArbChainConfig to access its interface
        ArbChainConfig chainConfig = ArbChainConfig(deployedAddress);

        // Verify the values
        assertEq(chainConfig.CHAIN_ID(), CHAIN_ID);
        assertEq(chainConfig.ARBITRUM_BRIDGE_ADDRESS(), ARBITRUM_BRIDGE_ADDRESS);
        assertEq(chainConfig.ARBITRUM_INBOX_ADDRESS(), ARBITRUM_INBOX_ADDRESS);
        assertEq(chainConfig.ARBITRUM_IGNORE_DELAYED_MESSAGES(), ARBITRUM_IGNORE_DELAYED_MESSAGES);
        assertEq(chainConfig.SETTLEMENT_DELAY(), SETTLEMENT_DELAY);
        assertEq(chainConfig.SETTLEMENT_START_BLOCK(), SETTLEMENT_START_BLOCK);
        assertEq(chainConfig.SEQUENCING_CONTRACT_ADDRESS(), SEQUENCING_CONTRACT_ADDRESS);
        assertEq(chainConfig.SEQUENCING_START_BLOCK(), SEQUENCING_START_BLOCK);
        assertEq(chainConfig.ROLLUP_OWNER(), rollupOwner);
        assertEq(chainConfig.DEFAULT_SEQUENCING_CHAIN_RPC_URL(), DEFAULT_RPC_URL);

        vm.stopPrank();
    }

    function testCannotCreateDuplicateArbChainConfig() public {
        vm.startPrank(owner);

        // Deploy the first config
        configManager.createArbChainConfig(
            CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL
        );

        // Attempt to deploy a duplicate config
        vm.expectRevert("Config already exists for this chain ID");
        configManager.createArbChainConfig(
            CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL
        );

        vm.stopPrank();
    }

    function testCreateArbChainConfigRevertOnZeroChainId() public {
        vm.startPrank(owner);

        vm.expectRevert("Chain ID cannot be zero");
        configManager.createArbChainConfig(
            0, // Zero chain ID
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL
        );

        vm.stopPrank();
    }

    function testCreateArbChainConfigRevertOnNonOwner() public {
        vm.prank(address(999)); // Non-owner address

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(999)));
        configManager.createArbChainConfig(
            CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL
        );
    }

    function testGetArbChainConfigAddress() public {
        // Deploy the config first
        vm.prank(owner);
        address deployedAddress = configManager.createArbChainConfig(
            CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL
        );

        // Get the stored address
        address storedAddress = configManager.deployedConfigs(CHAIN_ID);
        address returnedAddress = configManager.getArbChainConfigAddress(CHAIN_ID);

        // Verify all addresses match
        assertEq(returnedAddress, deployedAddress, "Returned address should match deployed address");
        assertEq(storedAddress, deployedAddress, "Stored address should match deployed address");
        assertEq(returnedAddress, storedAddress, "Returned address should match stored address");
    }

    function testRollupWnerIsArbConfigChainOwner() public {
        // Deploy the config first
        vm.prank(owner);
        address deployedAddress = configManager.createArbChainConfig(
            CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL
        );

        // Now that it's deployed, the getArbChainConfigAddress should return the deployed address
        ArbChainConfig chainConfig = ArbChainConfig(deployedAddress);

        // Verify the rollup owner is set correctly
        assertEq(chainConfig.ROLLUP_OWNER(), rollupOwner);
        assertEq(chainConfig.owner(), rollupOwner);
    }

    function testGetArbChainConfigAddressRevertOnZeroChainId() public {
        vm.expectRevert("Chain ID cannot be zero");
        configManager.getArbChainConfigAddress(0);
    }

    function testUpgradeImplementation() public {
        vm.startPrank(owner);

        // Deploy a config first
        address deployedAddress = configManager.createArbChainConfig(
            CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL
        );

        // Get the current implementation
        address currentImpl = IBeacon(address(configManager.beacon())).implementation();

        // Deploy a new implementation
        ArbChainConfig newImplementation = new ArbChainConfig();
        // No need to initialize the implementation

        // Upgrade to the new implementation without expecting event
        configManager.upgradeImplementation(address(newImplementation));

        // Check the implementation was updated
        address newImpl = IBeacon(address(configManager.beacon())).implementation();
        assertEq(newImpl, address(newImplementation));
        assertNotEq(newImpl, currentImpl);

        // Check that the existing proxy still works and has the same data
        ArbChainConfig proxyConfig = ArbChainConfig(deployedAddress);
        assertEq(proxyConfig.CHAIN_ID(), CHAIN_ID);

        vm.stopPrank();
    }

    function testUpgradeImplementationRevertOnZeroAddress() public {
        vm.startPrank(owner);

        vm.expectRevert("New implementation cannot be zero address");
        configManager.upgradeImplementation(address(0));

        vm.stopPrank();
    }

    function testUpgradeImplementationRevertOnNonOwner() public {
        vm.startPrank(address(999)); // Non-owner address

        ArbChainConfig newImplementation = new ArbChainConfig();

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(999)));
        configManager.upgradeImplementation(address(newImplementation));

        vm.stopPrank();
    }

    function testChangeImplementation() public {
        vm.startPrank(owner);

        // Get the current implementation
        address currentImpl = IBeacon(address(configManager.beacon())).implementation();

        // Deploy a new implementation
        ArbChainConfig newImplementation = new ArbChainConfig();

        // Upgrade to the new implementation
        configManager.upgradeImplementation(address(newImplementation));

        // Check the implementation was updated
        address newImpl = IBeacon(address(configManager.beacon())).implementation();
        assertEq(newImpl, address(newImplementation));
        assertNotEq(newImpl, currentImpl);

        vm.stopPrank();
    }

    function testAddressCalculation() public {
        vm.startPrank(owner);

        // Calculate the expected address before deployment
        address expectedAddress = configManager.getArbChainConfigAddress(CHAIN_ID);

        // Deploy the config
        address deployedAddress = configManager.createArbChainConfig(
            CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL
        );

        // Verify the addresses match
        assertEq(deployedAddress, expectedAddress, "Deployed address should match calculated address");

        vm.stopPrank();
    }
}
