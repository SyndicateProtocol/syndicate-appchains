// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test, console2} from "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// Core contracts
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {GasAggregator} from "src/staking/GasAggregator.sol";
import {EpochTracker} from "src/staking/EpochTracker.sol";

// Test upgrades
import {SyndicateSequencingChainUpgradeV2} from "./helpers/SyndicateSequencingChainUpgradeV2.sol";
import {SyndicateFactoryUpgradeV2} from "./helpers/SyndicateFactoryUpgradeV2.sol";

// Interfaces and modules
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";

/// @title UpgradeFlowIntegrationTest
/// @notice Comprehensive integration test for the full upgrade flow
/// @dev Tests the complete upgrade process:
///      1. Deploy factory via proxy pattern
///      2. Create sequencing chains
///      3. Upgrade factory to V2
///      4. Upgrade sequencing chain to V2
///      5. Verify all storage preserved and new functionality works
contract UpgradeFlowIntegrationTest is Test, EpochTracker {
    /*//////////////////////////////////////////////////////////////
                            TEST CONSTANTS
    //////////////////////////////////////////////////////////////*/

    address constant ADMIN = address(0x1111);
    address constant USER = address(0x2222);
    uint256 constant TEST_NONCE_1 = 1;
    uint256 constant TEST_NONCE_2 = 2;

    /*//////////////////////////////////////////////////////////////
                        CONTRACT INSTANCES
    //////////////////////////////////////////////////////////////*/

    // V1 Contracts
    SyndicateFactory factoryV1;
    SyndicateFactory factoryProxy;
    GasAggregator gasAggregator;
    AlwaysAllowedModule permissionModule;

    // Created chains
    SyndicateSequencingChain chain1;
    uint256 chain1Id;

    // V2 Upgrades
    SyndicateFactoryUpgradeV2 factoryV2;
    SyndicateSequencingChainUpgradeV2 chainV2;

    /*//////////////////////////////////////////////////////////////
                              EVENTS
    //////////////////////////////////////////////////////////////*/

    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );
    event TransactionProcessed(address indexed sender, bytes data);
    event ProcessingFeeUpdated(uint256 newFee);
    event ChainCreationToggled(bool enabled);

    /*//////////////////////////////////////////////////////////////
                              SETUP
    //////////////////////////////////////////////////////////////*/

    function setUp() public {
        // Set timestamp to start of epoch 1 + 1 day to ensure we're in a valid epoch
        vm.warp(getEpochStart(1) + 1 days);

        // Deploy as ADMIN (simulating the deterministic deployment process)
        vm.startPrank(ADMIN);

        // 1. Deploy SyndicateFactory implementation
        factoryV1 = new SyndicateFactory();
        console2.log("Factory implementation deployed:", address(factoryV1));

        // 2. Deploy SyndicateFactory proxy with initialization
        bytes memory factoryInitData = abi.encodeCall(SyndicateFactory.initialize, (ADMIN));
        ERC1967Proxy factoryProxyContract = new ERC1967Proxy(address(factoryV1), factoryInitData);
        factoryProxy = SyndicateFactory(address(factoryProxyContract));
        console2.log("Factory proxy deployed:", address(factoryProxy));

        // 3. Deploy GasAggregator (non-upgradeable)
        gasAggregator = new GasAggregator(
            1, // start epoch
            5 ether, // addChainFee
            100 // maxAppchainsToQuery
        );
        console2.log("GasAggregator deployed:", address(gasAggregator));

        // 4. Deploy permission module
        permissionModule = new AlwaysAllowedModule();
        console2.log("Permission module deployed:", address(permissionModule));

        vm.stopPrank();

        // Verify deployment
        assertTrue(
            factoryProxy.hasRole(factoryProxy.DEFAULT_ADMIN_ROLE(), ADMIN), "Admin should have DEFAULT_ADMIN_ROLE"
        );
    }

    /*//////////////////////////////////////////////////////////////
                    TEST: INITIAL DEPLOYMENT
    //////////////////////////////////////////////////////////////*/

    function test_InitialDeployment() public view {
        // Verify factory
        assertTrue(address(factoryProxy) != address(0), "Factory should be deployed");
        assertEq(factoryProxy.version(), 1_000_000, "Factory version should be 1.0.0");

        // Verify gas aggregator
        assertTrue(address(gasAggregator) != address(0), "GasAggregator should be deployed");
        assertEq(gasAggregator.VERSION(), 1_000_000, "GasAggregator VERSION should be 1.0.0");

        // Verify admin/owner
        assertTrue(
            factoryProxy.hasRole(factoryProxy.DEFAULT_ADMIN_ROLE(), ADMIN),
            "Admin should have DEFAULT_ADMIN_ROLE on factory"
        );
        assertEq(gasAggregator.owner(), ADMIN, "Admin should be owner of gas aggregator");
    }

    /*//////////////////////////////////////////////////////////////
                TEST: CREATE SEQUENCING CHAIN
    //////////////////////////////////////////////////////////////*/

    function test_CreateSequencingChain() public {
        vm.startPrank(ADMIN);

        // Create chain
        (address chainAddress, uint256 chainId) = factoryProxy.createSyndicateSequencingChain(
            TEST_NONCE_1, ADMIN, IRequirementModule(address(permissionModule))
        );

        chain1 = SyndicateSequencingChain(chainAddress);
        chain1Id = chainId;

        console2.log("Chain created:", chainAddress);
        console2.log("Chain ID:", chainId);

        // Verify chain
        assertEq(chain1.appchainId(), chainId, "Chain ID should match");
        assertEq(chain1.owner(), ADMIN, "Chain owner should be admin");
        assertEq(
            address(chain1.permissionRequirementModule()), address(permissionModule), "Permission module should match"
        );
        assertTrue(chain1.gasTrackingEnabled(), "Gas tracking should be enabled");

        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
            TEST: PROCESS TRANSACTIONS ON V1 CHAIN
    //////////////////////////////////////////////////////////////*/

    function test_ProcessTransactionV1() public {
        // Create chain first
        vm.prank(ADMIN);
        (address chainAddress,) = factoryProxy.createSyndicateSequencingChain(
            TEST_NONCE_1, ADMIN, IRequirementModule(address(permissionModule))
        );
        chain1 = SyndicateSequencingChain(chainAddress);

        // Process transaction
        bytes memory txData = hex"1234567890";

        vm.prank(USER);
        chain1.processTransaction(txData);

        // Verify the transaction was processed by checking gas tracking
        assertTrue(chain1.gasTrackingEnabled(), "Gas tracking should still be enabled");
    }

    /*//////////////////////////////////////////////////////////////
            TEST: UPGRADE FACTORY TO V2
    //////////////////////////////////////////////////////////////*/

    function test_UpgradeFactoryToV2() public {
        // Store pre-upgrade state
        uint256 preUpgradeVersion = factoryProxy.version();

        vm.startPrank(ADMIN);

        // Deploy V2 implementation
        factoryV2 = new SyndicateFactoryUpgradeV2();
        console2.log("Factory V2 implementation deployed:", address(factoryV2));

        // Upgrade
        factoryProxy.upgradeToAndCall(address(factoryV2), "");
        console2.log("Factory upgraded to V2");

        // Cast to V2 interface
        SyndicateFactoryUpgradeV2 factoryProxyV2 = SyndicateFactoryUpgradeV2(address(factoryProxy));

        vm.stopPrank();

        // Verify storage preserved
        assertEq(factoryProxy.version(), preUpgradeVersion, "Version should be preserved");
        assertTrue(factoryProxy.hasRole(factoryProxy.DEFAULT_ADMIN_ROLE(), ADMIN), "Admin role should be preserved");

        // Verify new V2 functionality
        assertEq(factoryProxyV2.factoryVersion(), "2.0.0", "Factory V2 version should be 2.0.0");
        assertEq(factoryProxyV2.totalChainsCreated(), 0, "Initial totalChainsCreated should be 0");
        assertFalse(factoryProxyV2.chainCreationEnabled(), "Chain creation should be disabled by default");
    }

    /*//////////////////////////////////////////////////////////////
        TEST: UPGRADE SEQUENCING CHAIN TO V2
    //////////////////////////////////////////////////////////////*/

    function test_UpgradeSequencingChainToV2() public {
        // Create chain first
        vm.prank(ADMIN);
        (address chainAddress, uint256 chainId) = factoryProxy.createSyndicateSequencingChain(
            TEST_NONCE_1, ADMIN, IRequirementModule(address(permissionModule))
        );
        chain1 = SyndicateSequencingChain(chainAddress);

        // Store pre-upgrade state
        uint256 preAppchainId = chain1.appchainId();
        address preOwner = chain1.owner();
        address prePermissionModule = address(chain1.permissionRequirementModule());

        vm.startPrank(ADMIN);

        // Deploy V2 implementation
        chainV2 = new SyndicateSequencingChainUpgradeV2();
        console2.log("Chain V2 implementation deployed:", address(chainV2));

        // Set as allowed implementation in factory
        factoryProxy.setSyndicateSequencingChainImplementation(address(chainV2));
        console2.log("V2 implementation set in factory");

        // Upgrade chain
        chain1.upgradeToAndCall(address(chainV2), "");
        console2.log("Chain upgraded to V2");

        // Cast to V2 interface
        SyndicateSequencingChainUpgradeV2 chainProxyV2 = SyndicateSequencingChainUpgradeV2(chainAddress);

        vm.stopPrank();

        // Verify storage preserved
        assertEq(chainProxyV2.appchainId(), preAppchainId, "AppchainId should be preserved");
        assertEq(chainProxyV2.owner(), preOwner, "Owner should be preserved");
        assertEq(
            address(chainProxyV2.permissionRequirementModule()),
            prePermissionModule,
            "Permission module should be preserved"
        );

        // Verify new V2 functionality
        assertEq(chainProxyV2.contractVersion(), "2.0.0", "Chain V2 version should be 2.0.0");
        assertEq(chainProxyV2.processingFee(), 0, "Initial processing fee should be 0");
        assertFalse(chainProxyV2.feeCollectionEnabled(), "Fee collection should be disabled by default");
        assertEq(chainProxyV2.totalFeesCollected(), 0, "Initial total fees should be 0");
    }

    /*//////////////////////////////////////////////////////////////
        TEST: FULL UPGRADE FLOW WITH FUNCTIONALITY
    //////////////////////////////////////////////////////////////*/

    function test_FullUpgradeFlowWithFunctionality() public {
        // 1. Create chain on V1
        vm.prank(ADMIN);
        (address chainAddress, uint256 chainId) = factoryProxy.createSyndicateSequencingChain(
            TEST_NONCE_1, ADMIN, IRequirementModule(address(permissionModule))
        );
        chain1 = SyndicateSequencingChain(chainAddress);
        console2.log("=== Step 1: Chain created on V1 ===");

        // 2. Process transaction on V1
        bytes memory txData = hex"1234567890";
        vm.prank(USER);
        chain1.processTransaction(txData);
        console2.log("=== Step 2: Transaction processed on V1 ===");

        // 3. Upgrade Factory to V2
        vm.startPrank(ADMIN);
        factoryV2 = new SyndicateFactoryUpgradeV2();
        factoryProxy.upgradeToAndCall(address(factoryV2), "");
        SyndicateFactoryUpgradeV2 factoryProxyV2 = SyndicateFactoryUpgradeV2(address(factoryProxy));
        console2.log("=== Step 3: Factory upgraded to V2 ===");

        // 4. Test new Factory V2 functionality
        vm.expectEmit(true, true, true, true);
        emit ChainCreationToggled(true);
        factoryProxyV2.toggleChainCreation();
        assertTrue(factoryProxyV2.chainCreationEnabled(), "Chain creation should be enabled");
        console2.log("=== Step 4: Factory V2 functionality tested ===");

        // 5. Upgrade Chain to V2
        chainV2 = new SyndicateSequencingChainUpgradeV2();
        factoryProxy.setSyndicateSequencingChainImplementation(address(chainV2));
        chain1.upgradeToAndCall(address(chainV2), "");
        SyndicateSequencingChainUpgradeV2 chainProxyV2 = SyndicateSequencingChainUpgradeV2(chainAddress);
        console2.log("=== Step 5: Chain upgraded to V2 ===");

        // 6. Test new Chain V2 functionality
        uint256 testFee = 0.01 ether;
        vm.expectEmit(true, true, true, true);
        emit ProcessingFeeUpdated(testFee);
        chainProxyV2.setProcessingFee(testFee);
        assertEq(chainProxyV2.processingFee(), testFee, "Processing fee should be set");

        chainProxyV2.toggleFeeCollection();
        assertTrue(chainProxyV2.feeCollectionEnabled(), "Fee collection should be enabled");
        console2.log("=== Step 6: Chain V2 functionality configured ===");

        vm.stopPrank();

        // 7. Process transaction with fee on V2
        vm.deal(USER, 1 ether);
        vm.prank(USER);
        chainProxyV2.processTransactionWithFee{value: testFee}(txData);
        assertEq(chainProxyV2.totalFeesCollected(), testFee, "Fee should be collected");
        console2.log("=== Step 7: Transaction with fee processed on V2 ===");

        // 8. Verify old functionality still works
        vm.prank(USER);
        chainProxyV2.processTransaction(hex"abcdef");
        console2.log("=== Step 8: Old functionality verified ===");

        // 9. Verify all storage preserved
        assertEq(chainProxyV2.appchainId(), chainId, "AppchainId should be preserved");
        assertEq(chainProxyV2.owner(), ADMIN, "Owner should be preserved");
        assertTrue(
            factoryProxy.hasRole(factoryProxy.DEFAULT_ADMIN_ROLE(), ADMIN), "Factory admin role should be preserved"
        );
        console2.log("=== Step 9: All storage verified ===");

        console2.log("\n=== FULL UPGRADE FLOW COMPLETED SUCCESSFULLY ===");
    }

    /*//////////////////////////////////////////////////////////////
        TEST: MULTIPLE CHAINS UPGRADE
    //////////////////////////////////////////////////////////////*/

    function test_MultipleSequencingChainsUpgrade() public {
        vm.startPrank(ADMIN);

        // Create multiple chains on V1
        (address chain1Addr,) = factoryProxy.createSyndicateSequencingChain(
            TEST_NONCE_1, ADMIN, IRequirementModule(address(permissionModule))
        );
        (address chain2Addr,) = factoryProxy.createSyndicateSequencingChain(
            TEST_NONCE_2, ADMIN, IRequirementModule(address(permissionModule))
        );

        console2.log("Two chains created");

        // Deploy V2 implementation
        chainV2 = new SyndicateSequencingChainUpgradeV2();
        factoryProxy.setSyndicateSequencingChainImplementation(address(chainV2));

        // Upgrade both chains
        SyndicateSequencingChain(chain1Addr).upgradeToAndCall(address(chainV2), "");
        SyndicateSequencingChain(chain2Addr).upgradeToAndCall(address(chainV2), "");

        console2.log("Both chains upgraded to V2");

        vm.stopPrank();

        // Verify both chains
        SyndicateSequencingChainUpgradeV2 chain1V2 = SyndicateSequencingChainUpgradeV2(chain1Addr);
        SyndicateSequencingChainUpgradeV2 chain2V2 = SyndicateSequencingChainUpgradeV2(chain2Addr);

        assertEq(chain1V2.contractVersion(), "2.0.0", "Chain 1 should be V2");
        assertEq(chain2V2.contractVersion(), "2.0.0", "Chain 2 should be V2");
        assertEq(chain1V2.owner(), ADMIN, "Chain 1 owner preserved");
        assertEq(chain2V2.owner(), ADMIN, "Chain 2 owner preserved");

        console2.log("Multiple chains upgrade verified");
    }

    /*//////////////////////////////////////////////////////////////
            TEST: UPGRADE AUTHORIZATION
    //////////////////////////////////////////////////////////////*/

    function test_RevertWhen_UnauthorizedUpgradeAttempt() public {
        vm.prank(ADMIN);
        (address chainAddress,) = factoryProxy.createSyndicateSequencingChain(
            TEST_NONCE_1, ADMIN, IRequirementModule(address(permissionModule))
        );
        chain1 = SyndicateSequencingChain(chainAddress);

        // Deploy V2
        vm.prank(ADMIN);
        chainV2 = new SyndicateSequencingChainUpgradeV2();

        // Try to upgrade as non-owner
        vm.prank(USER);
        vm.expectRevert();
        chain1.upgradeToAndCall(address(chainV2), "");
    }

    /*//////////////////////////////////////////////////////////////
            TEST: STORAGE LAYOUT VALIDATION
    //////////////////////////////////////////////////////////////*/

    function test_StorageLayoutPreservation() public {
        // Create chain and set various storage values
        vm.startPrank(ADMIN);
        (address chainAddress,) = factoryProxy.createSyndicateSequencingChain(
            TEST_NONCE_1, ADMIN, IRequirementModule(address(permissionModule))
        );
        chain1 = SyndicateSequencingChain(chainAddress);

        // Capture all storage
        uint256 preAppchainId = chain1.appchainId();
        address preOwner = chain1.owner();
        bool preGasTrackingEnabled = chain1.gasTrackingEnabled();

        // Upgrade
        chainV2 = new SyndicateSequencingChainUpgradeV2();
        factoryProxy.setSyndicateSequencingChainImplementation(address(chainV2));
        chain1.upgradeToAndCall(address(chainV2), "");
        SyndicateSequencingChainUpgradeV2 chainProxyV2 = SyndicateSequencingChainUpgradeV2(chainAddress);

        vm.stopPrank();

        // Verify ALL storage preserved
        assertEq(chainProxyV2.appchainId(), preAppchainId, "AppchainId must be preserved");
        assertEq(chainProxyV2.owner(), preOwner, "Owner must be preserved");
        assertEq(chainProxyV2.gasTrackingEnabled(), preGasTrackingEnabled, "GasTrackingEnabled must be preserved");
    }
}
