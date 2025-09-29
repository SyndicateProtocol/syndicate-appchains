// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import {GasAggregator, IAppchainFactory} from "src/staking/GasAggregator.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {IGasAggregator} from "src/interfaces/IGasAggregator.sol";
import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";
import {IRequirementModule} from "src/interfaces/IRequirementModule.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

contract GasAggregatorIntegrationTest is Test {
    GasAggregator public gasAggregator;
    SyndicateFactory public factory;
    SyndicateSequencingChain public sequencingChainImpl;
    AlwaysAllowedModule public permissionModule;

    address public admin = address(0x1);
    address public user = address(0x2);

    uint256 public constant CHAIN_ID_1 = 1001;
    uint256 public constant CHAIN_ID_2 = 1002;
    uint256 public constant CHAIN_ID_3 = 1003;

    uint256 public constant EPOCH_DURATION = 30 days;
    uint256 public constant CHALLENGE_WINDOW = 24 hours;
    uint256 public constant ADD_CHAIN_FEE = 5 ether;

    function setUp() public {
        // Set timestamp to after epoch 1' START_TIMESTAMP
        vm.warp(1754089200 + 1 days);

        permissionModule = new AlwaysAllowedModule();

        // deploy factory
        SyndicateFactory factoryImpl = new SyndicateFactory();
        bytes memory factoryInitData = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy factoryProxy = new ERC1967Proxy(address(factoryImpl), factoryInitData);
        factory = SyndicateFactory(address(factoryProxy));

        gasAggregator = GasAggregator(address(factory.gasAggregator()));

        vm.deal(user, 100 ether);
        vm.deal(admin, 100 ether);
    }

    function test_Integration_AddChainWithRealFactory() public {
        // Deploy a real sequencing chain
        vm.prank(admin);
        (address chainAddress, uint256 chainId) =
            factory.createSyndicateSequencingChain(1, admin, IRequirementModule(address(permissionModule)));

        // Set emissions receiver
        vm.prank(admin);
        SyndicateSequencingChain(chainAddress).setEmissionsReceiver(address(0x3001));

        // Add the chain to the aggregator
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId);

        assertTrue(gasAggregator.isChainTracked(chainId));
        assertEq(gasAggregator.getTotalTrackedChains(), 1);
    }

    function test_Integration_AddChainWithoutDeployedContract() public {
        uint256 nonExistentChainId = 999;

        vm.prank(user);
        vm.expectRevert(); // Should fail because no contract at computed address
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(nonExistentChainId);
    }

    function test_Integration_AutomaticAggregationWithRealGasUsage() public {
        // Create multiple real sequencing chains
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(1, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(2, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain3Address, uint256 chainId3) =
            factory.createSyndicateSequencingChain(3, admin, IRequirementModule(address(permissionModule)));

        // Set emissions receivers
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).setEmissionsReceiver(address(0x3001));
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).setEmissionsReceiver(address(0x3002));
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).setEmissionsReceiver(address(0x3003));

        // Add all chains to aggregator
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId2);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId3);

        // Generate gas usage on all chains
        bytes memory txData = hex"1234567890abcdef";
        vm.txGasPrice(20 gwei);

        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).processTransaction(txData);
        bytes[] memory batchData = new bytes[](2);
        batchData[0] = hex"abcdef1234567890";
        batchData[1] = hex"fedcba0987654321";
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).processTransactionsBulk(batchData);
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(hex"fedcba0987654321");

        // Verify gas usage recorded
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        uint256 chain1Gas = SyndicateSequencingChain(chain1Address).getTokensForEpoch(currentEpoch);
        uint256 chain2Gas = SyndicateSequencingChain(chain2Address).getTokensForEpoch(currentEpoch);
        uint256 chain3Gas = SyndicateSequencingChain(chain3Address).getTokensForEpoch(currentEpoch);

        assertTrue(chain1Gas > 0, "Chain 1 should have recorded gas usage");
        assertTrue(chain2Gas > 0, "Chain 2 should have recorded gas usage");
        assertTrue(chain3Gas > 0, "Chain 3 should have recorded gas usage");

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Test automatic aggregation
        gasAggregator.aggregateTokensUsed();

        // Verify aggregation completed
        assertTrue(gasAggregator.pendingEpoch() > currentEpoch);
    }

    function test_Integration_AutomaticAggregationWithInvalidImplementations() public {
        // Create multiple real sequencing chains with valid implementation
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(1, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(2, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain3Address, uint256 chainId3) =
            factory.createSyndicateSequencingChain(3, admin, IRequirementModule(address(permissionModule)));

        // Set emissions receivers
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).setEmissionsReceiver(address(0x4001));
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).setEmissionsReceiver(address(0x4002));
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).setEmissionsReceiver(address(0x4003));

        // Add all chains to aggregator
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId2);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId3);

        // Generate gas usage on all chains
        bytes memory txData = hex"1122334455";
        vm.txGasPrice(15 gwei);

        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).processTransaction(txData);

        // Verify initial gas usage
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        uint256 chain1Gas = SyndicateSequencingChain(chain1Address).getTokensForEpoch(currentEpoch);
        uint256 chain2Gas = SyndicateSequencingChain(chain2Address).getTokensForEpoch(currentEpoch);
        uint256 chain3Gas = SyndicateSequencingChain(chain3Address).getTokensForEpoch(currentEpoch);

        assertTrue(chain1Gas > 0);
        assertTrue(chain2Gas > 0);
        assertTrue(chain3Gas > 0);

        console.log("Before invalidation - Chain 1 gas:", chain1Gas);
        console.log("Before invalidation - Chain 2 gas:", chain2Gas);
        console.log("Before invalidation - Chain 3 gas:", chain3Gas);

        // Verify all chains are tracked
        assertEq(gasAggregator.getTotalTrackedChains(), 3);
        assertTrue(gasAggregator.isChainTracked(chainId1));
        assertTrue(gasAggregator.isChainTracked(chainId2));
        assertTrue(gasAggregator.isChainTracked(chainId3));

        SyndicateSequencingChain badImpl = new SyndicateSequencingChain();

        // upgrade chain2 to use the bad implementation
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).setAllowGasTrackingBanOnUpgrade(true);
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).upgradeToAndCall(address(badImpl), bytes(""));

        //verify chain2 has been banned
        assertFalse(gasAggregator.isChainTracked(chainId2));
        assertTrue(gasAggregator.bannedAppchains(chainId2));

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Aggregate should process existing chains normally
        gasAggregator.aggregateTokensUsed();

        assertEq(gasAggregator.getTotalTrackedChains(), 2);
        assertTrue(gasAggregator.isChainTracked(chainId1));
        assertTrue(gasAggregator.isChainTracked(chainId3));

        // move to next epoch (ensure enough time has passed for the new pendingEpoch)
        // After aggregation, pendingEpoch advances, so we need to move forward enough for that epoch to complete
        vm.warp(block.timestamp + 2 * EPOCH_DURATION + 1);

        gasAggregator.aggregateTokensUsed();

        // after 2 epochs, banned chain should still be banned
        assertEq(gasAggregator.getTotalTrackedChains(), 2);
        assertTrue(gasAggregator.bannedAppchains(chainId2));
    }

    function test_Integration_MixedValidInvalidChainAggregation() public {
        // Deploy a second implementation that we'll make the default later
        SyndicateSequencingChain altImpl = new SyndicateSequencingChain();

        // Create chains with the default implementation first
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(1, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(2, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain3Address, uint256 chainId3) =
            factory.createSyndicateSequencingChain(3, admin, IRequirementModule(address(permissionModule)));

        // Set emissions receivers
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).setEmissionsReceiver(address(0x5001));
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).setEmissionsReceiver(address(0x5002));
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).setEmissionsReceiver(address(0x5003));

        // Add all chains to aggregator
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId2);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId3);

        // Generate gas usage
        bytes memory txData = hex"aabbccddee";
        vm.txGasPrice(25 gwei);

        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).processTransaction(txData);

        uint256 currentEpoch = gasAggregator.pendingEpoch();
        uint256 chain1Gas = SyndicateSequencingChain(chain1Address).getTokensForEpoch(currentEpoch);
        uint256 chain2Gas = SyndicateSequencingChain(chain2Address).getTokensForEpoch(currentEpoch);
        uint256 chain3Gas = SyndicateSequencingChain(chain3Address).getTokensForEpoch(currentEpoch);

        // Save the original implementation that the chains were created with
        address originalImpl = factory.syndicateChainImpl();

        // Add the alt implementation as the new default (this notifies gasAggregator)
        vm.prank(admin);
        factory.setSyndicateSequencingChainImplementation(address(altImpl));

        // Remove the original implementation from gasAggregator
        vm.prank(admin);
        gasAggregator.removeAllowedImplementation(originalImpl);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // The current GasAggregator implementation doesn't automatically remove chains
        // during aggregation based on implementation validity - chains only get banned
        // when they explicitly call notifyChainUpgrade with an invalid implementation

        // Aggregate should process existing chains normally
        gasAggregator.aggregateTokensUsed();

        // All chains should remain tracked despite using the now-invalid implementation
        assertEq(gasAggregator.getTotalTrackedChains(), 3);
        assertTrue(gasAggregator.isChainTracked(chainId1));
        assertTrue(gasAggregator.isChainTracked(chainId2));
        assertTrue(gasAggregator.isChainTracked(chainId3));

        // Verify the aggregated data contains all chains with their gas usage
        bytes32 aggregatedHash = gasAggregator.aggregatedEpochDataHash(currentEpoch);
        uint256[] memory expectedChainIDs = new uint256[](3);
        expectedChainIDs[0] = chainId1;
        expectedChainIDs[1] = chainId2;
        expectedChainIDs[2] = chainId3;
        uint256[] memory expectedTokens = new uint256[](3);
        expectedTokens[0] = chain1Gas;
        expectedTokens[1] = chain2Gas;
        expectedTokens[2] = chain3Gas;
        address[] memory expectedEmissionsReceivers = new address[](3);
        expectedEmissionsReceivers[0] = address(0x5001);
        expectedEmissionsReceivers[1] = address(0x5002);
        expectedEmissionsReceivers[2] = address(0x5003);
        bytes32 expectedHash = keccak256(abi.encode(expectedChainIDs, expectedTokens, expectedEmissionsReceivers));
        assertEq(aggregatedHash, expectedHash, "Should aggregate data from all chains");
    }

    function test_Integration_OffchainAggregationWithRealChains() public {
        // Create real sequencing chains
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(1, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(2, admin, IRequirementModule(address(permissionModule)));

        // Set emissions receivers
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).setEmissionsReceiver(address(0x5001));
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).setEmissionsReceiver(address(0x5002));

        // Add chains to aggregator
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId2);

        // Force fallback to offchain aggregation by setting maxAppchainsToQuery to 0
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(0);

        // Generate gas usage
        bytes memory txData = hex"deadbeef";
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).processTransaction(txData);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit offchain aggregation
        uint256[] memory chainIds = new uint256[](2);
        chainIds[0] = chainId1;
        chainIds[1] = chainId2;

        gasAggregator.submitOffchainTopChains(chainIds);

        // Wait for challenge window
        vm.warp(block.timestamp + CHALLENGE_WINDOW + 1);

        // Seal the epoch
        gasAggregator.sealPendingEpoch();

        // Verify aggregation completed
        assertTrue(gasAggregator.pendingEpoch() > 1);
    }

    function test_Integration_OffchainAggregationChallengeWindow() public {
        // Create real sequencing chains
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(1, admin, IRequirementModule(address(permissionModule)));

        // Set emissions receiver
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).setEmissionsReceiver(address(0x6001));

        // Add chain to aggregator
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId1);

        // Force fallback to offchain aggregation
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(0);

        // Generate gas usage
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(hex"cafebabe");

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit offchain aggregation
        uint256[] memory chainIds = new uint256[](1);
        chainIds[0] = chainId1;

        gasAggregator.submitOffchainTopChains(chainIds);

        // Try to seal before challenge window ends (should fail)
        vm.expectRevert();
        gasAggregator.sealPendingEpoch();

        // Wait for challenge window to pass
        vm.warp(block.timestamp + CHALLENGE_WINDOW + 1);

        // Now sealing should work
        gasAggregator.sealPendingEpoch();
    }

    function test_Integration_AggregateTokensUsed_MultipleInvalidChainsRemoval() public {
        // Test automatic aggregation resilience with multiple invalid chains being removed
        // This is similar to the mock test but uses real contracts and implementation validation

        // Create 5 real sequencing chains with valid implementation first
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(1, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(2, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain3Address, uint256 chainId3) =
            factory.createSyndicateSequencingChain(3, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain4Address, uint256 chainId4) =
            factory.createSyndicateSequencingChain(4, admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain5Address, uint256 chainId5) =
            factory.createSyndicateSequencingChain(5, admin, IRequirementModule(address(permissionModule)));

        // Set emissions receivers
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).setEmissionsReceiver(address(0x8001));
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).setEmissionsReceiver(address(0x8002));
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).setEmissionsReceiver(address(0x8003));
        vm.prank(admin);
        SyndicateSequencingChain(chain4Address).setEmissionsReceiver(address(0x8004));
        vm.prank(admin);
        SyndicateSequencingChain(chain5Address).setEmissionsReceiver(address(0x8005));

        // Add all chains to aggregator (below threshold for automatic aggregation)
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId2);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId3);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId4);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId5);

        // Generate gas usage for all chains
        bytes memory txData = hex"8888888888";
        vm.txGasPrice(35 gwei);

        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain4Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain5Address).processTransaction(txData);

        uint256 currentEpoch = gasAggregator.pendingEpoch();

        // Verify all chains are tracked
        assertEq(gasAggregator.getTotalTrackedChains(), 5);

        // Deploy a new implementation and make it the only allowed one
        // This will invalidate chains 2, 4, and any others using the old implementation
        SyndicateSequencingChain newImpl = new SyndicateSequencingChain();

        // Save the original implementation that the chains were created with
        address originalImpl = factory.syndicateChainImpl();

        // Add new implementation as the default (this notifies gasAggregator)
        vm.prank(admin);
        factory.setSyndicateSequencingChainImplementation(address(newImpl));

        // Remove the old implementation from gasAggregator (invalidating all existing chains)
        vm.prank(admin);
        gasAggregator.removeAllowedImplementation(originalImpl);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // The current GasAggregator implementation doesn't automatically remove chains
        // during aggregation based on implementation validity - chains only get banned
        // when they explicitly call notifyChainUpgrade with an invalid implementation

        // Aggregate should process all existing chains normally
        gasAggregator.aggregateTokensUsed();

        // All chains should remain tracked despite using the now-invalid implementation
        assertEq(gasAggregator.getTotalTrackedChains(), 5);
        assertTrue(gasAggregator.isChainTracked(chainId1));
        assertTrue(gasAggregator.isChainTracked(chainId2));
        assertTrue(gasAggregator.isChainTracked(chainId3));
        assertTrue(gasAggregator.isChainTracked(chainId4));
        assertTrue(gasAggregator.isChainTracked(chainId5));

        // Epoch should still increment successfully
        assertEq(gasAggregator.pendingEpoch(), currentEpoch + 1);

        // The aggregated data should contain all chains
        bytes32 aggregatedHash = gasAggregator.aggregatedEpochDataHash(currentEpoch);
        // Note: We don't assert on the exact hash here since we didn't measure the gas values
        // but we know it should not be empty since all chains are still tracked
        assertTrue(aggregatedHash != bytes32(0), "Aggregated hash should not be empty with tracked chains");
    }

    // ================== VERSION TRACKING TESTS ==================

    function testInitialVersionInGasAggregator() public view {
        assertEq(gasAggregator.version(), "1.0.0", "Initial version should be 1.0.0");
    }

    function testUpdateVersionInGasAggregator() public {
        vm.prank(admin);
        gasAggregator.updateVersion("1.3.0");

        assertEq(gasAggregator.version(), "1.3.0", "Version should be updated to 1.3.0");
    }

    function testUpdateVersionOnlyAdmin() public {
        address nonAdmin = address(999);

        vm.prank(nonAdmin);
        vm.expectRevert(); // AccessControl error
        gasAggregator.updateVersion("1.1.0");
    }

    function testVersionPersistsAfterAggregatorOperations() public {
        // Update version
        vm.prank(admin);
        gasAggregator.updateVersion("2.5.0");

        // Perform aggregator operations
        vm.prank(admin);
        gasAggregator.setChallengeWindow(7200); // 2 hours

        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(50);

        // Version should still be the same
        assertEq(gasAggregator.version(), "2.5.0", "Version should persist after aggregator operations");
    }

    function testVersionWithDifferentAdminRoles() public {
        bytes32 defaultAdminRole = gasAggregator.DEFAULT_ADMIN_ROLE();

        // Admin should be able to update version
        assertTrue(gasAggregator.hasRole(defaultAdminRole, admin));

        vm.prank(admin);
        gasAggregator.updateVersion("3.0.0");
        assertEq(gasAggregator.version(), "3.0.0", "Admin should be able to update version");

        // Grant role to another address
        address newAdmin = address(888);
        vm.prank(admin);
        gasAggregator.grantRole(defaultAdminRole, newAdmin);

        // New admin should also be able to update version
        vm.prank(newAdmin);
        gasAggregator.updateVersion("3.1.0");
        assertEq(gasAggregator.version(), "3.1.0", "New admin should be able to update version");
    }
}
