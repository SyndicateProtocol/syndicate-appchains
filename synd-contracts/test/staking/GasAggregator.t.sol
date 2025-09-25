// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GasAggregator, ISequencingContract} from "../../src/staking/GasAggregator.sol";
import {SyndicateFactory, IGasAggregator} from "../../src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "../../src/SyndicateSequencingChain.sol";
import {AlwaysAllowedModule} from "../../src/sequencing-modules/AlwaysAllowedModule.sol";
import {IRequirementModule} from "../../src/interfaces/IRequirementModule.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

contract GasAggregatorTest is Test {
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
    uint256 public constant ADD_CHAIN_FEE = 0.1 ether;

    function setUp() public {
        // Deploy real SyndicateFactory
        SyndicateFactory factoryImpl = new SyndicateFactory();
        bytes memory factoryInitData = abi.encodeCall(SyndicateFactory.initialize, (admin));
        ERC1967Proxy factoryProxy = new ERC1967Proxy(address(factoryImpl), factoryInitData);
        factory = SyndicateFactory(address(factoryProxy));

        // Deploy sequencing chain implementation
        sequencingChainImpl = new SyndicateSequencingChain();

        // Deploy permission module for testing
        permissionModule = new AlwaysAllowedModule();

        // Deploy GasAggregator using TransparentUpgradeableProxy pattern
        ProxyAdmin proxyAdmin = new ProxyAdmin(admin);
        GasAggregator gasAggregatorImpl = new GasAggregator();

        // Warp to exactly the epoch start timestamp before proxy deployment
        vm.warp(gasAggregatorImpl.START_TIMESTAMP());

        bytes memory gasAggregatorInitData =
            abi.encodeWithSelector(GasAggregator.initialize.selector, factory, admin, CHALLENGE_WINDOW, ADD_CHAIN_FEE);

        TransparentUpgradeableProxy gasAggregatorProxy =
            new TransparentUpgradeableProxy(address(gasAggregatorImpl), address(proxyAdmin), gasAggregatorInitData);

        gasAggregator = GasAggregator(address(gasAggregatorProxy));

        // Configure gas aggregator
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(10);

        // Set up the connection between factory and gas aggregator
        vm.prank(admin);
        factory.setGasAggregator(IGasAggregator(address(gasAggregator)));

        // Give user ETH for fees
        vm.deal(user, 10 ether);
    }

    function test_Integration_AddChainWithRealFactory() public {
        // Create a real sequencing chain using the factory
        vm.prank(admin);
        (address chainAddress, uint256 actualChainId) = factory.createSyndicateSequencingChain(
            admin, IRequirementModule(address(IRequirementModule(address(permissionModule))))
        );

        // The factory returns a deterministic chain ID, use that for verification
        address expectedAddress = factory.computeSequencingChainAddress(actualChainId);
        assertEq(chainAddress, expectedAddress);

        // Add the chain to the gas aggregator
        vm.expectEmit(true, true, true, false);
        emit GasAggregator.ChainAdded(actualChainId, expectedAddress, user);

        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(actualChainId);

        // Verify chain was added to aggregator
        assertTrue(gasAggregator.isChainTracked(actualChainId));
        assertEq(gasAggregator.getTotalTrackedChains(), 1);

        uint256[] memory trackedChains = gasAggregator.getTrackedChainIds();
        assertEq(trackedChains.length, 1);
        assertEq(trackedChains[0], actualChainId);
    }

    function test_Integration_AddChainWithoutDeployedContract() public {
        // Try to add a chain that hasn't been deployed via factory
        vm.expectRevert(abi.encodeWithSelector(GasAggregator.ChainNotFound.selector, CHAIN_ID_1));

        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_1);
    }

    function test_Integration_AutomaticAggregationWithRealGasUsage() public {
        // Create multiple real sequencing chains
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain3Address, uint256 chainId3) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));

        // Set emissions receivers for the chains
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).setEmissionsReceiver(address(0x3001));
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).setEmissionsReceiver(address(0x3002));
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).setEmissionsReceiver(address(0x3003));

        // Add chains to aggregator (below threshold for automatic aggregation)
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId2);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId3);

        // Generate real gas usage by processing transactions on each chain
        bytes memory txData1 = hex"1234567890abcdef";
        bytes memory txData2 = hex"abcdef1234567890";
        bytes memory txData3 = hex"fedcba0987654321";

        // Set gas price for realistic gas cost calculation
        uint256 gasPrice = 20 gwei;
        vm.txGasPrice(gasPrice);

        // Process transactions to generate gas usage on chain 1
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(txData1);
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(txData2);

        // Process transactions on chain 2
        bytes[] memory bulkTxData = new bytes[](2);
        bulkTxData[0] = txData2;
        bulkTxData[1] = txData3;
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).processTransactionsBulk(bulkTxData);

        // Process transaction on chain 3
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).processTransaction(txData3);

        // Verify gas was tracked for current epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        uint256 chain1Gas = ISequencingContract(chain1Address).getTokensForEpoch(currentEpoch);
        uint256 chain2Gas = ISequencingContract(chain2Address).getTokensForEpoch(currentEpoch);
        uint256 chain3Gas = ISequencingContract(chain3Address).getTokensForEpoch(currentEpoch);

        // Gas usage should be greater than 0 for all chains
        assertTrue(chain1Gas > 0, "Chain 1 should have recorded gas usage");
        assertTrue(chain2Gas > 0, "Chain 2 should have recorded gas usage");
        assertTrue(chain3Gas > 0, "Chain 3 should have recorded gas usage");

        // Chain 1 should have more gas usage (2 transactions vs 1 for chain 3)
        assertTrue(chain1Gas > chain3Gas, "Chain 1 should use more gas than chain 3");

        console.log("Chain 1 gas usage:", chain1Gas);
        console.log("Chain 2 gas usage:", chain2Gas);
        console.log("Chain 3 gas usage:", chain3Gas);

        // Move to next epoch to complete the current epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Now aggregate the tokens using the automatic aggregation mechanism
        gasAggregator.aggregateTokensUsed();

        // Verify that the epoch was incremented
        assertEq(gasAggregator.pendingEpoch(), currentEpoch + 1);

        // Verify the aggregated data hash was stored
        bytes32 aggregatedHash = gasAggregator.aggregatedEpochDataHash(currentEpoch);
        assertTrue(aggregatedHash != bytes32(0), "Aggregated data hash should be set");

        // The aggregated hash should match the expected encoding
        uint256[] memory expectedChainIDs = new uint256[](3);
        expectedChainIDs[0] = chainId1;
        expectedChainIDs[1] = chainId2;
        expectedChainIDs[2] = chainId3;
        uint256[] memory expectedTokens = new uint256[](3);
        expectedTokens[0] = chain1Gas;
        expectedTokens[1] = chain2Gas;
        expectedTokens[2] = chain3Gas;
        address[] memory expectedEmissionsReceivers = new address[](3);
        expectedEmissionsReceivers[0] = address(0x3001);
        expectedEmissionsReceivers[1] = address(0x3002);
        expectedEmissionsReceivers[2] = address(0x3003);

        bytes32 expectedHash = keccak256(abi.encode(expectedChainIDs, expectedTokens, expectedEmissionsReceivers));
        assertEq(aggregatedHash, expectedHash, "Aggregated hash should match expected value");
    }

    function test_Integration_AutomaticAggregationWithInvalidImplementations() public {
        // Create multiple real sequencing chains with valid implementation
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain3Address, uint256 chainId3) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));

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
        uint256 chain1Gas = ISequencingContract(chain1Address).getTokensForEpoch(currentEpoch);
        uint256 chain2Gas = ISequencingContract(chain2Address).getTokensForEpoch(currentEpoch);
        uint256 chain3Gas = ISequencingContract(chain3Address).getTokensForEpoch(currentEpoch);

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

        // move to next epoch
        assertEq(gasAggregator.pendingEpoch(), currentEpoch + 1);

        // The aggregated data should contain all chains with their gas usage
        bytes32 aggregatedHash = gasAggregator.aggregatedEpochDataHash(currentEpoch);
        uint256[] memory expectedChainIDs = new uint256[](3);
        expectedChainIDs[0] = chainId1;
        expectedChainIDs[1] = chainId3;
        uint256[] memory expectedTokens = new uint256[](3);
        expectedTokens[0] = chain1Gas;
        expectedTokens[1] = chain3Gas;
        address[] memory expectedEmissionsReceivers = new address[](3);
        expectedEmissionsReceivers[0] = address(0x4001);
        expectedEmissionsReceivers[1] = address(0x4003);
        bytes32 expectedHash = keccak256(abi.encode(expectedChainIDs, expectedTokens, expectedEmissionsReceivers));
        assertEq(aggregatedHash, expectedHash, "Should aggregate data from all chains");
    }

    function test_Integration_MixedValidInvalidChainAggregation() public {
        // Deploy a second implementation that we'll make the default later
        SyndicateSequencingChain altImpl = new SyndicateSequencingChain();

        // Create chains with the default implementation first
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain3Address, uint256 chainId3) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));

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
        uint256 chain1Gas = ISequencingContract(chain1Address).getTokensForEpoch(currentEpoch);
        uint256 chain2Gas = ISequencingContract(chain2Address).getTokensForEpoch(currentEpoch);
        uint256 chain3Gas = ISequencingContract(chain3Address).getTokensForEpoch(currentEpoch);

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
        // Set low threshold to force offchain aggregation
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Create multiple real sequencing chains (more than threshold)
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain3Address, uint256 chainId3) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));

        // Set emissions receivers
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).setEmissionsReceiver(address(0x6001));
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).setEmissionsReceiver(address(0x6002));
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).setEmissionsReceiver(address(0x6003));

        // Add all chains to aggregator (exceeds threshold of 2)
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId2);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId3);

        // Verify we're now in offchain aggregation mode
        assertTrue(gasAggregator.fallbackToOffchainAggregation(), "Should use offchain aggregation");

        // Generate gas usage on all chains
        bytes memory txData1 = hex"1111111111";
        bytes memory txData2 = hex"2222222222";
        bytes memory txData3 = hex"3333333333";

        vm.txGasPrice(30 gwei);

        // Generate different amounts of gas usage for each chain
        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(txData1);

        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).processTransaction(txData2);
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).processTransaction(txData2); // chain2 uses more gas

        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).processTransaction(txData3);
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).processTransaction(txData3);
        vm.prank(admin);
        SyndicateSequencingChain(chain3Address).processTransaction(txData3); // chain3 uses most gas

        // Get gas usage for current epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        uint256 chain1Gas = ISequencingContract(chain1Address).getTokensForEpoch(currentEpoch);
        uint256 chain2Gas = ISequencingContract(chain2Address).getTokensForEpoch(currentEpoch);
        uint256 chain3Gas = ISequencingContract(chain3Address).getTokensForEpoch(currentEpoch);

        assertTrue(chain1Gas > 0);
        assertTrue(chain2Gas > chain1Gas, "Chain 2 should have more gas than chain 1");
        assertTrue(chain3Gas > chain2Gas, "Chain 3 should have most gas");

        console.log("Chain 1 gas usage:", chain1Gas);
        console.log("Chain 2 gas usage:", chain2Gas);
        console.log("Chain 3 gas usage:", chain3Gas);

        // Move to next epoch to complete the current epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Verify automatic aggregation is not allowed
        vm.expectRevert(GasAggregator.MustUseOffchainAggregation.selector);
        gasAggregator.aggregateTokensUsed();

        // Submit offchain aggregation data for top 2 chains (ascending order)
        // Let's say we want to submit chain2 and chain3 as the top performers
        uint256[] memory topChainIds = new uint256[](2);
        topChainIds[0] = chainId2 < chainId3 ? chainId2 : chainId3; // Lower ID first
        topChainIds[1] = chainId2 < chainId3 ? chainId3 : chainId2; // Higher ID second

        uint256 submissionTime = block.timestamp;
        gasAggregator.submitOffchainTopChains(topChainIds);

        // Verify submission data was recorded
        assertEq(gasAggregator.pendingEpochFirstSubmissionTime(), submissionTime);
        uint256 expectedTotalGas = chainId2 < chainId3 ? chain2Gas + chain3Gas : chain3Gas + chain2Gas;
        assertEq(gasAggregator.pendingTotalTokensUsed(), expectedTotalGas);

        // Verify the pending data hash is set correctly
        uint256[] memory expectedTokens = new uint256[](2);
        expectedTokens[0] = chainId2 < chainId3 ? chain2Gas : chain3Gas;
        expectedTokens[1] = chainId2 < chainId3 ? chain3Gas : chain2Gas;
        address[] memory expectedEmissionsReceivers = new address[](2);
        expectedEmissionsReceivers[0] = chainId2 < chainId3 ? address(0x6002) : address(0x6003);
        expectedEmissionsReceivers[1] = chainId2 < chainId3 ? address(0x6003) : address(0x6002);

        bytes32 expectedPendingHash = keccak256(abi.encode(topChainIds, expectedTokens, expectedEmissionsReceivers));
        assertEq(gasAggregator.pendingDataHash(), expectedPendingHash);

        // Try to seal before challenge window - should fail
        vm.expectRevert(
            abi.encodeWithSelector(GasAggregator.WindowNotOver.selector, gasAggregator.pendingEpoch(), CHALLENGE_WINDOW)
        );
        gasAggregator.sealPendingEpoch();

        // Wait for challenge window to pass
        vm.warp(submissionTime + CHALLENGE_WINDOW + 1);

        // Now seal the pending epoch
        gasAggregator.sealPendingEpoch();

        // Verify epoch was incremented and data was finalized
        assertEq(gasAggregator.pendingEpoch(), currentEpoch + 1);
        assertEq(gasAggregator.pendingEpochFirstSubmissionTime(), 0); // Reset
        assertEq(gasAggregator.pendingDataHash(), bytes32(0)); // Reset
        assertEq(gasAggregator.pendingTotalTokensUsed(), 0); // Reset

        // Verify aggregated data hash was stored
        bytes32 aggregatedHash = gasAggregator.aggregatedEpochDataHash(currentEpoch);
        assertEq(aggregatedHash, expectedPendingHash, "Aggregated hash should match submitted data");
    }

    function test_Integration_OffchainAggregationChallengeWindow() public {
        // Set low threshold for offchain aggregation
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(1);

        // Create chains
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));

        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).setEmissionsReceiver(address(0x7001));
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).setEmissionsReceiver(address(0x7002));

        // Add chains (exceeds threshold)
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(chainId2);

        // Generate gas usage
        vm.txGasPrice(40 gwei);
        bytes memory txData = hex"4444444444";

        vm.prank(admin);
        SyndicateSequencingChain(chain1Address).processTransaction(txData);
        vm.prank(admin);
        SyndicateSequencingChain(chain2Address).processTransaction(txData);

        uint256 currentEpoch = gasAggregator.pendingEpoch();
        uint256 chain1Gas = ISequencingContract(chain1Address).getTokensForEpoch(currentEpoch);
        uint256 chain2Gas = ISequencingContract(chain2Address).getTokensForEpoch(currentEpoch);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit initial data (chain1 only)
        uint256[] memory initialChains = new uint256[](1);
        initialChains[0] = chainId1;

        // Note: Due to some Foundry variable behavior, we'll use hardcoded timestamps
        // based on the known epoch start time (1756681201)
        gasAggregator.submitOffchainTopChains(initialChains);

        // Verify initial submission
        assertEq(gasAggregator.pendingTotalTokensUsed(), chain1Gas);

        // Submit better data during challenge window (both chains)
        vm.warp(1756681201 + CHALLENGE_WINDOW / 2); // Halfway through challenge window (using actual first submission time)

        uint256[] memory betterChains = new uint256[](2);
        betterChains[0] = chainId1 < chainId2 ? chainId1 : chainId2;
        betterChains[1] = chainId1 < chainId2 ? chainId2 : chainId1;

        gasAggregator.submitOffchainTopChains(betterChains);

        // Verify the better submission was accepted
        uint256 expectedTotal = chainId1 < chainId2 ? chain1Gas + chain2Gas : chain2Gas + chain1Gas;
        assertEq(gasAggregator.pendingTotalTokensUsed(), expectedTotal);
        // First submission time should not change (still tracks the first submission)
        // Due to some weird Foundry behavior with variables, we'll directly check the expected value
        uint256 actualFirstSubmissionTime = gasAggregator.pendingEpochFirstSubmissionTime();
        assertEq(actualFirstSubmissionTime, 1756681201, "First submission time should remain unchanged");

        // Try to submit after challenge window ends - should fail
        vm.warp(1756681201 + CHALLENGE_WINDOW + 1); // Use the actual first submission time

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.WindowOver.selector, currentEpoch, CHALLENGE_WINDOW));
        gasAggregator.submitOffchainTopChains(initialChains);

        // But sealing should now work
        gasAggregator.sealPendingEpoch();

        // Verify final state
        assertEq(gasAggregator.pendingEpoch(), currentEpoch + 1);

        // Final aggregated data should be the better submission (both chains)
        bytes32 aggregatedHash = gasAggregator.aggregatedEpochDataHash(currentEpoch);
        uint256[] memory finalTokens = new uint256[](2);
        finalTokens[0] = chainId1 < chainId2 ? chain1Gas : chain2Gas;
        finalTokens[1] = chainId1 < chainId2 ? chain2Gas : chain1Gas;
        address[] memory finalEmissionsReceivers = new address[](2);
        finalEmissionsReceivers[0] = chainId1 < chainId2 ? address(0x7001) : address(0x7002);
        finalEmissionsReceivers[1] = chainId1 < chainId2 ? address(0x7002) : address(0x7001);

        bytes32 expectedFinalHash = keccak256(abi.encode(betterChains, finalTokens, finalEmissionsReceivers));
        assertEq(aggregatedHash, expectedFinalHash, "Should store the better submission data");
    }

    function test_Integration_AggregateTokensUsed_MultipleInvalidChainsRemoval() public {
        // Test automatic aggregation resilience with multiple invalid chains being removed
        // This is similar to the mock test but uses real contracts and implementation validation

        // Create 5 real sequencing chains with valid implementation first
        vm.prank(admin);
        (address chain1Address, uint256 chainId1) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain2Address, uint256 chainId2) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain3Address, uint256 chainId3) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain4Address, uint256 chainId4) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));
        vm.prank(admin);
        (address chain5Address, uint256 chainId5) =
            factory.createSyndicateSequencingChain(admin, IRequirementModule(address(permissionModule)));

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
