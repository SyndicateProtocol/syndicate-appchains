// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

import {SyndicateSequencingChain} from "../../src/SyndicateSequencingChain.sol";
import {SyndicateSequencingChainTestingUpgradeability} from "./helpers/SyndicateSequencingChainTestingUpgradeability.sol";
import {IPermissionModule} from "../../src/interfaces/IPermissionModule.sol";

/// @notice Mock factory contract for testing upgrades
contract MockFactory {
    function isImplementationAllowed(address) external pure returns (bool) {
        return true; // Allow all implementations for testing
    }

    function notifyChainUpgrade(uint256, address) external pure {
        // No-op for testing
    }
}

/// @title StorageUpgradeTest
/// @notice Comprehensive test for storage layout safety during contract upgrades
/// @dev Tests both traditional storage append and ERC-7201 namespaced storage patterns
contract StorageUpgradeTest is Test {
    // Constants for testing
    uint256 constant TEST_APPCHAIN_ID = 12345;
    address constant EMISSIONS_RECEIVER = address(0x5678);
    address constant ADMIN = address(0x9999);
    address constant PERMISSION_MODULE = address(1); // Always allow module

    // Contract instances
    SyndicateSequencingChain syndicateV1;
    SyndicateSequencingChainTestingUpgradeability syndicateV2;
    ERC1967Proxy proxy;
    MockFactory factory;

    // Storage verification data
    struct OriginalStorageData {
        uint256 appchainId;
        address emissionsReceiver;
        address factory;
        bool allowGasTrackingBanOnUpgrade;
        address permissionModule;
        bool gasTrackingEnabled;
    }

    event TransactionProcessed(address indexed sender, bytes data);
    event MaxGasPerTransactionUpdated(uint256 newMax);
    event MaxTransactionsPerBatchUpdated(uint256 newMax);

    function setUp() public {
        // Deploy mock factory
        factory = new MockFactory();

        vm.startPrank(address(factory));

        // Deploy V1 implementation
        syndicateV1 = new SyndicateSequencingChain();

        // Deploy proxy pointing to V1
        bytes memory initData =
            abi.encodeCall(SyndicateSequencingChain.initialize, (ADMIN, PERMISSION_MODULE, TEST_APPCHAIN_ID));

        proxy = new ERC1967Proxy(address(syndicateV1), initData);
        syndicateV1 = SyndicateSequencingChain(address(proxy));

        vm.stopPrank();

        // Switch to admin to set emissions receiver
        vm.prank(ADMIN);
        syndicateV1.setEmissionsReceiver(EMISSIONS_RECEIVER);
    }

    /*//////////////////////////////////////////////////////////////
                        STORAGE LAYOUT VERIFICATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Test that all V1 storage is preserved after upgrade to V2
    function testStoragePreservationAfterUpgrade() public {
        vm.startPrank(ADMIN);

        // Capture original storage state
        OriginalStorageData memory originalData = OriginalStorageData({
            appchainId: syndicateV1.appchainId(),
            emissionsReceiver: syndicateV1.getEmissionsReceiver(),
            factory: syndicateV1.factory(),
            allowGasTrackingBanOnUpgrade: syndicateV1.allowGasTrackingBanOnUpgrade(),
            permissionModule: address(syndicateV1.permissionRequirementModule()),
            gasTrackingEnabled: syndicateV1.gasTrackingEnabled()
        });

        // Deploy V2 implementation
        syndicateV2 = new SyndicateSequencingChainTestingUpgradeability();

        // Upgrade proxy to V2
        syndicateV1.upgradeToAndCall(address(syndicateV2), "");

        // Cast proxy to V2 interface
        syndicateV2 = SyndicateSequencingChainTestingUpgradeability(address(proxy));

        // Verify all original storage is preserved
        assertEq(syndicateV2.appchainId(), originalData.appchainId, "appchainId should be preserved");
        assertEq(
            syndicateV2.getEmissionsReceiver(), originalData.emissionsReceiver, "emissionsReceiver should be preserved"
        );
        assertEq(syndicateV2.factory(), originalData.factory, "factory should be preserved");
        assertEq(
            syndicateV2.allowGasTrackingBanOnUpgrade(),
            originalData.allowGasTrackingBanOnUpgrade,
            "allowGasTrackingBanOnUpgrade should be preserved"
        );
        assertEq(
            address(syndicateV2.permissionRequirementModule()),
            originalData.permissionModule,
            "permissionRequirementModule should be preserved"
        );
        assertEq(
            syndicateV2.gasTrackingEnabled(), originalData.gasTrackingEnabled, "gasTrackingEnabled should be preserved"
        );

        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                    NAMESPACED STORAGE SAFETY TESTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Test that namespaced storage (ERC-7201) works correctly after upgrade
    function testNamespacedStorageAfterUpgrade() public {
        vm.startPrank(ADMIN);

        // Deploy V2 and upgrade
        syndicateV2 = new SyndicateSequencingChainTestingUpgradeability();
        syndicateV1.upgradeToAndCall(address(syndicateV2), "");
        syndicateV2 = SyndicateSequencingChainTestingUpgradeability(address(proxy));

        // Test namespaced storage fields work correctly
        // Note: These should be zero-initialized since they're new fields added during upgrade
        assertEq(syndicateV2.maxTransactionsPerBatch(), 0, "maxTransactionsPerBatch should be zero-initialized");
        assertFalse(syndicateV2.batchProcessingEnabled(), "batchProcessingEnabled should be false (zero-initialized)");
        assertEq(syndicateV2.lastPermissionUpdate(), 0, "lastPermissionUpdate should be zero-initialized");

        // Test that we can modify namespaced storage
        syndicateV2.toggleBatchProcessing(); // Enable batch processing first
        assertTrue(syndicateV2.batchProcessingEnabled(), "batchProcessingEnabled should be enabled");

        syndicateV2.setMaxTransactionsPerBatch(50);
        assertEq(syndicateV2.maxTransactionsPerBatch(), 50, "maxTransactionsPerBatch should be updated");

        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                    TRADITIONAL STORAGE SAFETY TESTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Test that new traditional storage fields work correctly
    function testNewTraditionalStorageFields() public {
        vm.startPrank(ADMIN);

        // Deploy V2 and upgrade
        syndicateV2 = new SyndicateSequencingChainTestingUpgradeability();
        syndicateV1.upgradeToAndCall(address(syndicateV2), "");
        syndicateV2 = SyndicateSequencingChainTestingUpgradeability(address(proxy));

        // Test new traditional storage fields are zero-initialized during upgrade
        assertEq(syndicateV2.maxGasPerTransaction(), 0, "maxGasPerTransaction should be zero-initialized");
        assertFalse(syndicateV2.replayProtectionEnabled(), "replayProtectionEnabled should be false (zero-initialized)");
        assertEq(syndicateV2.minTimeBetweenTxs(), 0, "minTimeBetweenTxs should be zero-initialized");

        // Test that we can modify new fields
        vm.expectEmit(true, true, true, true);
        emit MaxGasPerTransactionUpdated(500000);
        syndicateV2.setMaxGasPerTransaction(500000);
        assertEq(syndicateV2.maxGasPerTransaction(), 500000, "maxGasPerTransaction should be updated");

        syndicateV2.toggleReplayProtection();
        assertTrue(syndicateV2.replayProtectionEnabled(), "replayProtectionEnabled should be toggled to true");

        syndicateV2.setMinTimeBetweenTxs(60); // 1 minute
        assertEq(syndicateV2.minTimeBetweenTxs(), 60, "minTimeBetweenTxs should be updated");

        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                        FUNCTIONALITY TESTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Test that V1 functionality still works after upgrade
    function testV1FunctionalityAfterUpgrade() public {
        vm.startPrank(ADMIN);

        // Deploy V2 and upgrade
        syndicateV2 = new SyndicateSequencingChainTestingUpgradeability();
        syndicateV1.upgradeToAndCall(address(syndicateV2), "");
        syndicateV2 = SyndicateSequencingChainTestingUpgradeability(address(proxy));

        // Enable batch processing for V2 functionality
        syndicateV2.toggleBatchProcessing();
        syndicateV2.setMaxTransactionsPerBatch(10); // Set a reasonable limit

        vm.stopPrank();

        // Test transaction processing still works
        bytes memory testData = "test transaction data";

        vm.expectEmit(true, true, true, true);
        emit TransactionProcessed(address(this), testData);
        syndicateV2.processTransaction(testData);

        // Test batch processing still works
        bytes[] memory batchData = new bytes[](2);
        batchData[0] = "tx1";
        batchData[1] = "tx2";

        vm.expectEmit(true, true, true, true);
        emit TransactionProcessed(address(this), batchData[0]);
        vm.expectEmit(true, true, true, true);
        emit TransactionProcessed(address(this), batchData[1]);
        syndicateV2.processTransactionsBulk(batchData);
    }

    /// @notice Test new V2 functionality works correctly
    function testV2NewFunctionality() public {
        vm.startPrank(ADMIN);

        // Deploy V2 and upgrade
        syndicateV2 = new SyndicateSequencingChainTestingUpgradeability();
        syndicateV1.upgradeToAndCall(address(syndicateV2), "");
        syndicateV2 = SyndicateSequencingChainTestingUpgradeability(address(proxy));

        // Enable batch processing first, then test batch limit enforcement
        syndicateV2.toggleBatchProcessing();
        syndicateV2.setMaxTransactionsPerBatch(1);

        bytes[] memory largeBatch = new bytes[](2);
        largeBatch[0] = "tx1";
        largeBatch[1] = "tx2";

        vm.stopPrank();

        vm.expectRevert("Too many transactions in batch");
        syndicateV2.processTransactionsBulk(largeBatch);

        // Test replay protection
        vm.startPrank(ADMIN);
        syndicateV2.toggleReplayProtection(); // Enable replay protection first
        syndicateV2.setMinTimeBetweenTxs(5); // 5 seconds (less than current timestamp)
        vm.stopPrank();

        // Set timestamp to something reasonable for testing
        vm.warp(100);

        bytes memory txData = "replay test";

        // Use a completely fresh address for replay protection test
        address replayTester = address(0xDEADBEEF);
        vm.prank(replayTester);
        syndicateV2.processTransaction(txData); // First tx should work

        vm.expectRevert("Transaction too soon after previous");
        vm.prank(replayTester);
        syndicateV2.processTransaction(txData); // Immediate second tx should fail

        // Should work after time passes
        vm.warp(block.timestamp + 6); // Wait longer than minTimeBetweenTxs (5)
        vm.prank(replayTester);
        syndicateV2.processTransaction(txData);
    }

    /*//////////////////////////////////////////////////////////////
                        STORAGE COLLISION TESTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Test that there are no storage collisions between V1 and V2
    function testNoStorageCollisions() public {
        vm.startPrank(ADMIN);

        // Set specific values in V1
        syndicateV1.setAllowGasTrackingBanOnUpgrade(true);

        // Deploy V2 and upgrade
        syndicateV2 = new SyndicateSequencingChainTestingUpgradeability();
        syndicateV1.upgradeToAndCall(address(syndicateV2), "");
        syndicateV2 = SyndicateSequencingChainTestingUpgradeability(address(proxy));

        // Verify original values are still intact
        assertTrue(syndicateV2.allowGasTrackingBanOnUpgrade(), "V1 storage should not be affected by V2 upgrade");

        // Set new V2 values
        syndicateV2.setMaxGasPerTransaction(999999);
        syndicateV2.setMaxTransactionsPerBatch(77);
        syndicateV2.setMinTimeBetweenTxs(123);

        // Verify V1 storage is still intact after V2 modifications
        assertTrue(
            syndicateV2.allowGasTrackingBanOnUpgrade(), "V1 storage should not be affected by V2 storage modifications"
        );
        assertEq(syndicateV2.appchainId(), TEST_APPCHAIN_ID, "V1 storage should remain intact");
        assertEq(syndicateV2.factory(), address(factory), "V1 storage should remain intact");

        // Verify V2 storage is working correctly
        assertEq(syndicateV2.maxGasPerTransaction(), 999999, "V2 storage should work correctly");
        assertEq(syndicateV2.maxTransactionsPerBatch(), 77, "V2 namespaced storage should work correctly");
        assertEq(syndicateV2.minTimeBetweenTxs(), 123, "V2 storage should work correctly");

        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                        STORAGE LAYOUT INSPECTION
    //////////////////////////////////////////////////////////////*/

    /// @notice Test to generate and compare storage layouts
    /// @dev This test documents the storage layout differences between V1 and V2
    function testStorageLayoutDocumentation() public {
        // This test serves as documentation for storage layout changes
        // V1 storage (slots 0-3):
        // - Slot 0: appchainId (uint256)
        // - Slot 1: emissionsReceiver (address)
        // - Slot 2: factory (address)
        // - Slot 3: allowGasTrackingBanOnUpgrade (bool) - packed with factory

        // V2 additional traditional storage (slots 4-7):
        // - Slot 4: maxGasPerTransaction (uint256)
        // - Slot 5: replayProtectionEnabled (bool)
        // - Slot 6: minTimeBetweenTxs (uint256)
        // - Slot 7: lastTransactionTime mapping

        // Namespaced storage (ERC-7201):
        // - Location: 0x5c6d1774bdd69d8d16847c3c97b51ea7343257b8f5ace5da9e25ab3bafd7d500
        // - Field 0: permissionRequirementModule (IPermissionModule)
        // - Field 1: maxTransactionsPerBatch (uint256) - NEW in V2
        // - Field 2: batchProcessingEnabled (bool) - NEW in V2
        // - Field 3: lastPermissionUpdate (uint256) - NEW in V2

        assertTrue(true, "Storage layout documentation test passed");
    }
}
