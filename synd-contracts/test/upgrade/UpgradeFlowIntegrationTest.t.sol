// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test, console2} from "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// Core contracts
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {GasAggregator} from "src/staking/GasAggregator.sol";
import {EpochTracker} from "src/staking/EpochTracker.sol";
import {GasMeter} from "src/staking/GasMeter.sol";

// Test upgrades
import {SyndicateSequencingChainUpgradeV2} from "./helpers/SyndicateSequencingChainUpgradeV2.sol";

// Interfaces and modules
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";

/// @title UpgradeFlowIntegrationTest
/// @notice Comprehensive integration test for the full upgrade flow
/// @dev Tests the complete upgrade process:
///      1. Create sequencing chains
///      2. Upgrade sequencing chain to V2
///      3. Verify all storage preserved and new functionality works
/// @dev This test runs as a fork test against risa_devnet
/// @dev Run with: forge test --match-contract UpgradeFlowIntegrationTest --fork-url risa_devnet -vv
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
    GasAggregator gasAggregator;
    address sequencingChainImpl;
    AlwaysAllowedModule permissionModule;
    address gasMeter;

    // Created chains
    SyndicateSequencingChain chain1;
    uint256 chain1Id;

    // V2 Upgrades
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
        // Skip test in CI since the RPC is flaky
        vm.skip(true);

        // Create fork from risa_devnet
        vm.createSelectFork("risa_devnet");

        // Fund test accounts with native currency
        vm.deal(ADMIN, 100 ether);
        vm.deal(USER, 100 ether);

        // Set timestamp to start of epoch 1 + 1 day to ensure we're in a valid epoch
        vm.warp(getEpochStart(1) + 1 days);

        // Deploy as ADMIN (simulating the deterministic deployment process)
        vm.startPrank(ADMIN);

        // 1. Deploy GasMeter
        GasMeter gasMeterImpl = new GasMeter();
        gasMeter = address(new ERC1967Proxy(address(gasMeterImpl), abi.encodeCall(GasMeter.initialize, ())));

        // 2. Deploy SyndicateSequencingChain implementation
        sequencingChainImpl = address(new SyndicateSequencingChain(gasMeter));
        console2.log("Sequencing chain implementation deployed:", address(sequencingChainImpl));

        // 3. Deploy permission module
        permissionModule = new AlwaysAllowedModule();
        console2.log("Permission module deployed:", address(permissionModule));

        vm.stopPrank();
    }

    function deployChain(address admin, address _permissionModule, uint256 nonce)
        public
        returns (SyndicateSequencingChain)
    {
        return SyndicateSequencingChain(
            address(
                new ERC1967Proxy(
                    sequencingChainImpl,
                    abi.encodeCall(SyndicateSequencingChain.initialize, (admin, _permissionModule, nonce))
                )
            )
        );
    }

    /*//////////////////////////////////////////////////////////////
                    TEST: INITIAL DEPLOYMENT
    //////////////////////////////////////////////////////////////*/

    function test_InitialDeployment() public view {
        // Verify gas meter
        assertTrue(gasMeter != address(0), "GasMeter should be deployed");
        assertEq(GasMeter(gasMeter).VERSION(), 1_000_000, "GasMeter VERSION should be 1.0.0");
    }

    /*//////////////////////////////////////////////////////////////
                TEST: CREATE SEQUENCING CHAIN
    //////////////////////////////////////////////////////////////*/

    function test_CreateSequencingChain() public {
        vm.startPrank(ADMIN);

        // Create chain
        chain1 = deployChain(ADMIN, address(permissionModule), TEST_NONCE_1);
        chain1Id = chain1.appchainId();

        console2.log("Chain created:", address(chain1));
        console2.log("Chain ID:", chain1Id);

        // Verify chain
        assertEq(chain1.appchainId(), chain1Id, "Chain ID should match");
        assertEq(chain1.owner(), ADMIN, "Chain owner should be admin");
        assertEq(
            address(chain1.permissionRequirementModule()), address(permissionModule), "Permission module should match"
        );
        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
            TEST: PROCESS TRANSACTIONS ON V1 CHAIN
    //////////////////////////////////////////////////////////////*/

    function test_ProcessTransactionV1() public {
        // Create chain first
        vm.prank(ADMIN);
        chain1 = deployChain(ADMIN, address(permissionModule), TEST_NONCE_1);

        // Process transaction
        bytes memory txData = hex"1234567890";

        vm.prank(USER);
        chain1.processTransaction(txData);
    }

    /*//////////////////////////////////////////////////////////////
        TEST: UPGRADE SEQUENCING CHAIN TO V2
    //////////////////////////////////////////////////////////////*/

    function test_UpgradeSequencingChainToV2() public {
        // Create chain first
        vm.prank(ADMIN);
        chain1 = deployChain(ADMIN, address(permissionModule), TEST_NONCE_1);

        // Store pre-upgrade state
        uint256 preAppchainId = chain1.appchainId();
        address preOwner = chain1.owner();
        address prePermissionModule = address(chain1.permissionRequirementModule());

        vm.startPrank(ADMIN);

        // Deploy V2 implementation
        chainV2 = new SyndicateSequencingChainUpgradeV2(gasMeter);
        console2.log("Chain V2 implementation deployed:", address(chainV2));

        // Upgrade chain
        chain1.upgradeToAndCall(address(chainV2), "");
        console2.log("Chain upgraded to V2");

        // Cast to V2 interface
        SyndicateSequencingChainUpgradeV2 chainProxyV2 = SyndicateSequencingChainUpgradeV2(address(chain1));

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
        chain1 = deployChain(ADMIN, address(permissionModule), TEST_NONCE_1);
        chain1Id = chain1.appchainId();
        console2.log("=== Step 1: Chain created on V1 ===");

        // 2. Process transaction on V1
        bytes memory txData = hex"1234567890";
        vm.prank(USER);
        chain1.processTransaction(txData);
        console2.log("=== Step 2: Transaction processed on V1 ===");

        // 5. Upgrade Chain to V2
        chainV2 = new SyndicateSequencingChainUpgradeV2(gasMeter);
        chain1.upgradeToAndCall(address(chainV2), "");
        SyndicateSequencingChainUpgradeV2 chainProxyV2 = SyndicateSequencingChainUpgradeV2(address(chain1));
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
        assertEq(chainProxyV2.appchainId(), chain1Id, "AppchainId should be preserved");
        assertEq(chainProxyV2.owner(), ADMIN, "Owner should be preserved");
        console2.log("=== Step 9: All storage verified ===");

        console2.log("\n=== FULL UPGRADE FLOW COMPLETED SUCCESSFULLY ===");
    }

    /*//////////////////////////////////////////////////////////////
        TEST: MULTIPLE CHAINS UPGRADE
    //////////////////////////////////////////////////////////////*/

    function test_MultipleSequencingChainsUpgrade() public {
        vm.startPrank(ADMIN);

        // Create multiple chains on V1
        address chain1Addr = address(deployChain(ADMIN, address(permissionModule), TEST_NONCE_1));
        address chain2Addr = address(deployChain(ADMIN, address(permissionModule), TEST_NONCE_2));

        console2.log("Two chains created");

        // Deploy V2 implementation
        chainV2 = new SyndicateSequencingChainUpgradeV2(gasMeter);

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
        chain1 = deployChain(ADMIN, address(permissionModule), TEST_NONCE_1);

        // Deploy V2
        vm.prank(ADMIN);
        chainV2 = new SyndicateSequencingChainUpgradeV2(gasMeter);

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
        chain1 = deployChain(ADMIN, address(permissionModule), TEST_NONCE_1);

        // Capture all storage
        uint256 preAppchainId = chain1.appchainId();
        address preOwner = chain1.owner();
        address preGasMeter = chain1.gasMeter();

        // Upgrade
        chainV2 = new SyndicateSequencingChainUpgradeV2(gasMeter);
        chain1.upgradeToAndCall(address(chainV2), "");
        SyndicateSequencingChainUpgradeV2 chainProxyV2 = SyndicateSequencingChainUpgradeV2(address(chain1));

        vm.stopPrank();

        // Verify ALL storage preserved
        assertEq(chainProxyV2.appchainId(), preAppchainId, "AppchainId must be preserved");
        assertEq(chainProxyV2.owner(), preOwner, "Owner must be preserved");
        assertEq(chainProxyV2.gasMeter(), preGasMeter, "GasMeter must be preserved");
    }
}
