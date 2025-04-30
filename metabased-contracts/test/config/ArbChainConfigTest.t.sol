// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {ArbChainConfig} from "src/config/ArbChainConfig.sol";
import {ArbConfigManager} from "src/config/ArbConfigManager.sol";

contract ArbChainConfigTestBase is Test {
    address public owner = address(1);
    address public rollupOwner = address(2);
    address public newRollupOwner = address(3);

    uint256 public constant CHAIN_ID = 123456;
    uint256 public constant SEQUENCING_CHAIN_ID = 654321;
    address public constant ARBITRUM_BRIDGE_ADDRESS = address(0x1234);
    address public constant ARBITRUM_INBOX_ADDRESS = address(0x5678);
    bool public constant ARBITRUM_IGNORE_DELAYED_MESSAGES = false;
    uint256 public constant SETTLEMENT_DELAY = 10;
    uint256 public constant SETTLEMENT_START_BLOCK = 100;
    address public constant SEQUENCING_CONTRACT_ADDRESS = address(0x9ABC);
    uint256 public constant SEQUENCING_START_BLOCK = 200;
    string public constant DEFAULT_RPC_URL = "https://example.com/rpc";
    string public constant APPCHAIN_BLOCK_EXPLORER_URL = "https://example.com/explorer";

    // Define allowed settlement addresses for testing
    address[] public ALLOWED_SETTLEMENT_ADDRESSES;

    // Events for testing
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RollupOwnerUpdated(address indexed newRollupOwner);
    event DefaultSequencingChainRpcUrlUpdated(string newRpcUrl);
    event AllowedSettlementAddressesUpdated(address[] newAllowedSettlementAddresses);
    event ArbChainConfigCreated(uint256 indexed chainId, address configAddress);
    event ImplementationUpgraded(address newImplementation);

    function setUp() public virtual {
        // Initialize allowed settlement addresses
        ALLOWED_SETTLEMENT_ADDRESSES = new address[](2);
        ALLOWED_SETTLEMENT_ADDRESSES[0] = address(0xABCD);
        ALLOWED_SETTLEMENT_ADDRESSES[1] = address(0xEF12);
    }

    function _createArbConfigManager() internal returns (ArbConfigManager) {
        vm.startPrank(owner);
        ArbConfigManager manager = new ArbConfigManager(owner);
        vm.stopPrank();
        return manager;
    }

    function _deployArbChainConfig(ArbConfigManager manager, uint256 chainId) internal returns (ArbChainConfig) {
        vm.startPrank(owner);
        address proxyAddress = manager.createArbChainConfig(
            owner,
            chainId,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        vm.stopPrank();
        return ArbChainConfig(proxyAddress);
    }
}

contract BasicTests is ArbChainConfigTestBase {
    ArbConfigManager public configManager;
    ArbChainConfig public chainConfig;

    function setUp() public override {
        super.setUp();
        configManager = _createArbConfigManager();
        chainConfig = _deployArbChainConfig(configManager, CHAIN_ID);
    }

    function testImmutableValues() public view {
        assertEq(chainConfig.CHAIN_ID(), CHAIN_ID);
        assertEq(chainConfig.SEQUENCING_CHAIN_ID(), SEQUENCING_CHAIN_ID);
        assertEq(chainConfig.ARBITRUM_BRIDGE_ADDRESS(), ARBITRUM_BRIDGE_ADDRESS);
        assertEq(chainConfig.ARBITRUM_INBOX_ADDRESS(), ARBITRUM_INBOX_ADDRESS);
        assertEq(chainConfig.ARBITRUM_IGNORE_DELAYED_MESSAGES(), ARBITRUM_IGNORE_DELAYED_MESSAGES);
        assertEq(chainConfig.SETTLEMENT_DELAY(), SETTLEMENT_DELAY);
        assertEq(chainConfig.SETTLEMENT_START_BLOCK(), SETTLEMENT_START_BLOCK);
        assertEq(chainConfig.SEQUENCING_CONTRACT_ADDRESS(), SEQUENCING_CONTRACT_ADDRESS);
        assertEq(chainConfig.SEQUENCING_START_BLOCK(), SEQUENCING_START_BLOCK);
    }

    function testInitialMutableValues() public view {
        assertEq(chainConfig.ROLLUP_OWNER(), rollupOwner);
        assertEq(chainConfig.DEFAULT_SEQUENCING_CHAIN_RPC_URL(), DEFAULT_RPC_URL);
        assertEq(chainConfig.APPCHAIN_BLOCK_EXPLORER_URL(), APPCHAIN_BLOCK_EXPLORER_URL);

        // Verify allowed settlement addresses
        assertEq(chainConfig.ALLOWED_SETTLEMENT_ADDRESSES(0), ALLOWED_SETTLEMENT_ADDRESSES[0]);
        assertEq(chainConfig.ALLOWED_SETTLEMENT_ADDRESSES(1), ALLOWED_SETTLEMENT_ADDRESSES[1]);
    }

    function testOwnership() public view {
        // Verify that owner is set correctly
        assertEq(chainConfig.owner(), owner);
    }
}

contract InvalidParameterTests is ArbChainConfigTestBase {
    function testCreateWithZeroChainId() public {
        ArbConfigManager manager = _createArbConfigManager();

        vm.startPrank(owner);
        vm.expectRevert("Chain ID cannot be zero");
        manager.createArbChainConfig(
            owner,
            0, // Invalid chain ID
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        vm.stopPrank();
    }

    function testCreateWithDuplicateChainId() public {
        ArbConfigManager manager = _createArbConfigManager();

        // First creation should succeed
        _deployArbChainConfig(manager, CHAIN_ID);

        // Second creation with same chain ID should fail
        vm.startPrank(owner);
        vm.expectRevert("Config already exists for this chain ID");
        manager.createArbChainConfig(
            owner,
            CHAIN_ID, // Duplicate chain ID
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        vm.stopPrank();
    }

    function testCreateWithZeroBridgeAddress() public {
        ArbConfigManager manager = _createArbConfigManager();

        vm.startPrank(owner);
        vm.expectRevert("Arbitrum bridge address cannot be zero");
        manager.createArbChainConfig(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            address(0), // Invalid address
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        vm.stopPrank();
    }

    function testCreateWithZeroInboxAddress() public {
        ArbConfigManager manager = _createArbConfigManager();

        vm.startPrank(owner);
        vm.expectRevert("Arbitrum inbox address cannot be zero");
        manager.createArbChainConfig(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            address(0), // Invalid address
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        vm.stopPrank();
    }

    function testCreateWithZeroSequencingContract() public {
        ArbConfigManager manager = _createArbConfigManager();

        vm.startPrank(owner);
        vm.expectRevert("Sequencing contract address cannot be zero");
        manager.createArbChainConfig(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            address(0), // Invalid address
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        vm.stopPrank();
    }

    function testCreateWithZeroRollupOwner() public {
        ArbConfigManager manager = _createArbConfigManager();

        vm.startPrank(owner);
        vm.expectRevert("Rollup owner cannot be zero address");
        manager.createArbChainConfig(
            owner,
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            address(0), // Invalid address
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        vm.stopPrank();
    }

    function testCreateWithZeroOwnerAddress() public {
        ArbConfigManager manager = _createArbConfigManager();

        vm.startPrank(owner);
        vm.expectRevert("Owner cannot be zero address");
        manager.createArbChainConfig(
            address(0), // Invalid address
            CHAIN_ID,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        vm.stopPrank();
    }
}

contract ArbChainConfigUpdateTests is ArbChainConfigTestBase {
    ArbConfigManager public configManager;
    ArbChainConfig public chainConfig;

    function setUp() public override {
        super.setUp();
        configManager = _createArbConfigManager();
        chainConfig = _deployArbChainConfig(configManager, CHAIN_ID);
    }

    function testUpdateRollupOwner() public {
        vm.startPrank(owner);

        vm.expectEmit(true, false, false, true);
        emit RollupOwnerUpdated(newRollupOwner);

        chainConfig.updateRollupOwner(newRollupOwner);
        assertEq(chainConfig.ROLLUP_OWNER(), newRollupOwner);

        vm.stopPrank();
    }

    function testUpdateRollupOwnerRevertOnZeroAddress() public {
        vm.startPrank(owner);

        vm.expectRevert("New rollup owner cannot be zero address");
        chainConfig.updateRollupOwner(address(0));

        vm.stopPrank();
    }

    function testUpdateRollupOwnerRevertOnNonOwner() public {
        vm.prank(address(999)); // Non-owner address

        vm.expectRevert("Caller is not the owner");
        chainConfig.updateRollupOwner(newRollupOwner);
    }

    function testTransferOwnership() public {
        vm.startPrank(owner);

        address newOwner = address(4);

        // Test transferring ownership
        vm.expectEmit(true, true, false, true);
        emit OwnershipTransferred(owner, newOwner);

        chainConfig.transferOwnership(newOwner);
        assertEq(chainConfig.owner(), newOwner);

        vm.stopPrank();

        // New owner should be able to call onlyOwner functions
        vm.startPrank(newOwner);
        chainConfig.updateRollupOwner(newRollupOwner);
        assertEq(chainConfig.ROLLUP_OWNER(), newRollupOwner);
        vm.stopPrank();
    }

    function testTransferOwnershipRevertOnZeroAddress() public {
        vm.startPrank(owner);

        vm.expectRevert("New owner cannot be zero address");
        chainConfig.transferOwnership(address(0));

        vm.stopPrank();
    }

    function testTransferOwnershipRevertOnNonOwner() public {
        vm.prank(address(999)); // Non-owner address

        vm.expectRevert("Caller is not the owner");
        chainConfig.transferOwnership(address(4));
    }

    function testUpdateDefaultSequencingChainRpcUrl() public {
        vm.startPrank(owner);

        string memory newRpcUrl = "https://new-example.com/rpc";

        vm.expectEmit(true, false, false, true);
        emit DefaultSequencingChainRpcUrlUpdated(newRpcUrl);

        chainConfig.updateDefaultSequencingChainRpcUrl(newRpcUrl);
        assertEq(chainConfig.DEFAULT_SEQUENCING_CHAIN_RPC_URL(), newRpcUrl);

        vm.stopPrank();
    }

    function testUpdateDefaultSequencingChainRpcUrlRevertOnNonOwner() public {
        vm.prank(address(999)); // Non-owner address

        vm.expectRevert("Caller is not the owner");
        chainConfig.updateDefaultSequencingChainRpcUrl("https://new-example.com/rpc");
    }

    function testEmptyDefaultRpcUrl() public {
        vm.startPrank(owner);

        // Test with empty RPC URL
        chainConfig.updateDefaultSequencingChainRpcUrl("");
        assertEq(chainConfig.DEFAULT_SEQUENCING_CHAIN_RPC_URL(), "");

        vm.stopPrank();
    }

    function testUpdateAppchainBlockExplorerUrl() public {
        vm.startPrank(owner);

        string memory newUrl = "https://new-example.com/explorer";

        vm.expectEmit(true, false, false, true);
        emit ArbChainConfig.AppchainBlockExplorerUrlUpdated(newUrl);

        chainConfig.updateAppchainBlockExplorerUrl(newUrl);
        assertEq(chainConfig.APPCHAIN_BLOCK_EXPLORER_URL(), newUrl);

        vm.stopPrank();
    }

    function testUpdateAppchainBlockExplorerUrlRevertOnNonOwner() public {
        vm.prank(address(999)); // Non-owner address

        vm.expectRevert("Caller is not the owner");
        chainConfig.updateAppchainBlockExplorerUrl("https://new-example.com/explorer");
    }

    function testEmptyAppchainBlockExplorerUrl() public {
        vm.startPrank(owner);

        // Test with empty URL
        chainConfig.updateAppchainBlockExplorerUrl("");
        assertEq(chainConfig.APPCHAIN_BLOCK_EXPLORER_URL(), "");

        vm.stopPrank();
    }
}

contract ArbConfigManagerTests is ArbChainConfigTestBase {
    function testUpgradeImplementation() public {
        ArbConfigManager manager = _createArbConfigManager();

        // Deploy initial config
        uint256 chainId1 = CHAIN_ID + 1000;
        ArbChainConfig config1 = _deployArbChainConfig(manager, chainId1);

        vm.startPrank(owner);

        // Create a new implementation
        ArbChainConfig newImplementation = new ArbChainConfig();

        // Upgrade the implementation via the manager
        vm.expectEmit(true, false, false, true);
        emit ImplementationUpgraded(address(newImplementation));

        manager.upgradeImplementation(address(newImplementation));

        // Deploy a new config after the upgrade
        uint256 chainId2 = CHAIN_ID + 1001;
        address config2Address = manager.createArbChainConfig(
            owner,
            chainId2,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        ArbChainConfig config2 = ArbChainConfig(config2Address);

        // Verify functionality on both configs
        config1.updateRollupOwner(newRollupOwner);
        config2.updateRollupOwner(newRollupOwner);

        assertEq(config1.ROLLUP_OWNER(), newRollupOwner);
        assertEq(config2.ROLLUP_OWNER(), newRollupOwner);

        vm.stopPrank();
    }

    function testGetArbChainConfigAddress() public {
        ArbConfigManager manager = _createArbConfigManager();

        uint256 testChainId = CHAIN_ID + 2000;

        // Get the predicted address before deployment
        vm.startPrank(owner);
        address predictedAddress = manager.getArbChainConfigAddress(testChainId);

        // Deploy the config
        address actualAddress = manager.createArbChainConfig(
            owner,
            testChainId,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );
        vm.stopPrank();

        // Verify the predicted address matches the actual address
        assertEq(predictedAddress, actualAddress);

        // Check that the address is correctly stored in the deployedConfigs mapping
        assertEq(manager.deployedConfigs(testChainId), actualAddress);

        // Get the address after deployment should return the same address
        address retrievedAddress = manager.getArbChainConfigAddress(testChainId);
        assertEq(retrievedAddress, actualAddress);
    }

    function testCannotUpgradeToZeroAddress() public {
        ArbConfigManager manager = _createArbConfigManager();

        vm.startPrank(owner);
        vm.expectRevert("New implementation cannot be zero address");
        manager.upgradeImplementation(address(0));
        vm.stopPrank();
    }

    function testManagerOnlyOwnerFunctions() public {
        ArbConfigManager manager = _createArbConfigManager();
        address nonOwner = address(999);

        // Test createArbChainConfig with non-owner
        vm.prank(nonOwner);
        vm.expectRevert(); // OpenZeppelin's Ownable error
        manager.createArbChainConfig(
            owner,
            CHAIN_ID + 3000,
            SEQUENCING_CHAIN_ID,
            ARBITRUM_BRIDGE_ADDRESS,
            ARBITRUM_INBOX_ADDRESS,
            ARBITRUM_IGNORE_DELAYED_MESSAGES,
            SETTLEMENT_DELAY,
            SETTLEMENT_START_BLOCK,
            SEQUENCING_CONTRACT_ADDRESS,
            SEQUENCING_START_BLOCK,
            rollupOwner,
            DEFAULT_RPC_URL,
            APPCHAIN_BLOCK_EXPLORER_URL,
            ALLOWED_SETTLEMENT_ADDRESSES
        );

        // Test upgradeImplementation with non-owner
        vm.prank(nonOwner);
        vm.expectRevert(); // OpenZeppelin's Ownable error
        manager.upgradeImplementation(address(0x1234));
    }
}
