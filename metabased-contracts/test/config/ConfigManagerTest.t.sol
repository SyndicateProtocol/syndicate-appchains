// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {ConfigManager} from "src/config/ConfigManager.sol";
import {ChainConfig} from "src/config/ChainConfig.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IBeacon} from "@openzeppelin/contracts/proxy/beacon/IBeacon.sol";
import {BeaconProxy} from "@openzeppelin/contracts/proxy/beacon/BeaconProxy.sol";

contract ConfigManagerTest is Test {
    ConfigManager public configManager;
    address public owner = address(1);
    address public rollupOwner = address(2);

    uint256 public constant CHAIN_ID = 123456;
    bytes32 public constant TARGET_ROLLUP_TYPE = bytes32("ARBITRUM");
    bool public constant MINE_EMPTY_BLOCKS = true;
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
        configManager = new ConfigManager();
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

    function testCreateChainConfig() public {
        vm.startPrank(owner);

        // Deploy the config without expecting event
        address deployedAddress = configManager.createChainConfig(CHAIN_ID);

        // Verify the deployed address is stored correctly
        assertEq(configManager.deployedConfigs(CHAIN_ID), deployedAddress);

        // Verify it's a BeaconProxy
        uint256 codeSize;
        assembly {
            codeSize := extcodesize(deployedAddress)
        }
        assertTrue(codeSize > 0, "BeaconProxy should be a contract");

        // Verify the ChainConfig values have been properly initialized
        // We need to cast address to ChainConfig to access its interface
        ChainConfig chainConfig = ChainConfig(deployedAddress);

        // initialize the ChainConfig with the expected values
        chainConfig.initialize(
            CHAIN_ID,
            TARGET_ROLLUP_TYPE,
            MINE_EMPTY_BLOCKS,
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
        // Verify the values
        assertEq(chainConfig.CHAIN_ID(), CHAIN_ID);
        assertEq(chainConfig.TARGET_ROLLUP_TYPE(), TARGET_ROLLUP_TYPE);
        assertEq(chainConfig.MINE_EMPTY_BLOCKS(), MINE_EMPTY_BLOCKS);
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

    function testCannotCreateDuplicateChainConfig() public {
        vm.startPrank(owner);

        // Deploy the first config
        configManager.createChainConfig(CHAIN_ID);

        // Attempt to deploy a duplicate config
        vm.expectRevert("Config already exists for this chain ID");
        configManager.createChainConfig(CHAIN_ID);

        vm.stopPrank();
    }

    function testCreateChainConfigRevertOnZeroChainId() public {
        vm.startPrank(owner);

        vm.expectRevert("Chain ID cannot be zero");
        configManager.createChainConfig(
            0 // Zero chain ID
        );

        vm.stopPrank();
    }

    function testCreateChainConfigRevertOnNonOwner() public {
        vm.prank(address(999)); // Non-owner address

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(999)));
        configManager.createChainConfig(CHAIN_ID);
    }

    function testGetChainConfigAddress() public {
        // Deploy the config first
        vm.prank(owner);
        address deployedAddress = configManager.createChainConfig(CHAIN_ID);

        // Now that it's deployed, the getChainConfigAddress should return the deployed address
        address returnedAddress = configManager.getChainConfigAddress(CHAIN_ID);

        // Verify getChainConfigAddress returns the deployed address
        assertEq(returnedAddress, deployedAddress);
    }

    function testGetChainConfigAddressRevertOnZeroChainId() public {
        vm.expectRevert("Chain ID cannot be zero");
        configManager.getChainConfigAddress(0);
    }

    function testUpgradeImplementation() public {
        vm.startPrank(owner);

        // Deploy a config first
        address deployedAddress = configManager.createChainConfig(CHAIN_ID);

        // Get the current implementation
        address currentImpl = IBeacon(address(configManager.beacon())).implementation();

        // Deploy a new implementation
        ChainConfig newImplementation = new ChainConfig();
        // No need to initialize the implementation

        // Upgrade to the new implementation without expecting event
        configManager.upgradeImplementation(address(newImplementation));

        // Check the implementation was updated
        address newImpl = IBeacon(address(configManager.beacon())).implementation();
        assertEq(newImpl, address(newImplementation));
        assertNotEq(newImpl, currentImpl);

        // Check that the existing proxy still works and has the same data
        ChainConfig proxyConfig = ChainConfig(deployedAddress);

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

        ChainConfig newImplementation = new ChainConfig();

        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(999)));
        configManager.upgradeImplementation(address(newImplementation));

        vm.stopPrank();
    }

    function testMultipleConfigsShareImplementation() public {
        vm.startPrank(owner);

        // Deploy two configs with different chain IDs
        uint256 secondChainId = 789012;

        address firstConfig = configManager.createChainConfig(CHAIN_ID);

        address secondConfig = configManager.createChainConfig(secondChainId);

        ChainConfig firstProxyConfig = ChainConfig(firstConfig);
        ChainConfig secondProxyConfig = ChainConfig(secondConfig);

        // Get the current implementation
        address currentImpl = IBeacon(address(configManager.beacon())).implementation();

        // Deploy a new implementation
        ChainConfig newImplementation = new ChainConfig();

        // Upgrade to the new implementation
        configManager.upgradeImplementation(address(newImplementation));

        // Check the implementation was updated
        address newImpl = IBeacon(address(configManager.beacon())).implementation();
        assertEq(newImpl, address(newImplementation));
        assertNotEq(newImpl, currentImpl);

        vm.stopPrank();
    }
}
