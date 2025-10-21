// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {GasAggregator} from "src/staking/GasAggregator.sol";
import {EpochTracker} from "src/staking/EpochTracker.sol";

contract MockSyndicateProxy {
    mapping(uint256 => uint256) public tokensUsedPerEpoch;

    function setTokensUsedPerEpoch(uint256 epoch, uint256 tokens) external {
        tokensUsedPerEpoch[epoch] = tokens;
    }
}

contract GasAggregatorTest is Test, EpochTracker {
    GasAggregator public gasAggregator;
    MockSyndicateProxy public mockProxy1;
    MockSyndicateProxy public mockProxy2;
    MockSyndicateProxy public mockProxy3;

    address public owner;
    address public user;

    uint256 public constant START_EPOCH = 1;
    uint256 public constant ADD_CHAIN_FEE = 5 ether;
    uint256 public constant MAX_APPCHAINS_TO_QUERY = 100;

    event ChainAdded(uint256 indexed epoch, uint256 indexed chainId, address chainContract, address indexed addedBy);
    event ChainRemoved(uint256 indexed epoch, uint256 indexed chainId);
    event AddChainFeeUpdated(uint256 oldFee, uint256 newFee);
    event AggregationPending(uint256 indexed epoch, uint256 remainingChains);
    event AggregatedTokens(uint256 indexed epoch, uint256[] chainIds, uint256[] tokens);
    event UpdateMaxAppchainsToQuery(uint256 indexed epoch, uint256 maxAppchainsToQuery);

    function setUp() public {
        owner = address(this);
        user = address(0x1234);

        // Set timestamp to after epoch start
        vm.warp(getEpochStart(START_EPOCH) + 1 days);

        // Deploy GasAggregator
        gasAggregator = new GasAggregator(START_EPOCH, ADD_CHAIN_FEE, MAX_APPCHAINS_TO_QUERY);

        // Deploy mock proxies
        mockProxy1 = new MockSyndicateProxy();
        mockProxy2 = new MockSyndicateProxy();
        mockProxy3 = new MockSyndicateProxy();

        vm.deal(user, 100 ether);
    }

    /*//////////////////////////////////////////////////////////////
                        CONSTRUCTOR TESTS
    //////////////////////////////////////////////////////////////*/

    function testConstructor() public view {
        assertEq(gasAggregator.currentEpoch(), START_EPOCH);
        assertEq(gasAggregator.addChainFee(), ADD_CHAIN_FEE);
        assertEq(gasAggregator.maxAppchainsToQuery(), MAX_APPCHAINS_TO_QUERY);
        assertEq(gasAggregator.VERSION(), 1_000_000);
        assertEq(gasAggregator.owner(), owner);
    }

    function testConstructorWithDefaults() public {
        GasAggregator agg = new GasAggregator(1, 0, 0);
        assertEq(agg.addChainFee(), 5 ether);
        assertEq(agg.maxAppchainsToQuery(), 100);
    }

    function testConstructorRevertsOnZeroEpoch() public {
        vm.expectRevert();
        new GasAggregator(0, ADD_CHAIN_FEE, MAX_APPCHAINS_TO_QUERY);
    }

    /*//////////////////////////////////////////////////////////////
                    ADD LEGACY CHAIN TESTS
    //////////////////////////////////////////////////////////////*/

    function testAddLegacyChain() public {
        uint256 chainId = 1;

        vm.expectEmit(true, true, true, true);
        emit ChainAdded(START_EPOCH, chainId, address(mockProxy1), owner);

        gasAggregator.addLegacyChain(chainId, address(mockProxy1));

        assertEq(gasAggregator.getTrackedChainCount(), 1);
        assertEq(gasAggregator.getTrackedChainId(0), chainId);
        assertEq(gasAggregator.appchainContract(chainId), address(mockProxy1));
    }

    function testAddLegacyChainMultiple() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        gasAggregator.addLegacyChain(2, address(mockProxy2));
        gasAggregator.addLegacyChain(3, address(mockProxy3));

        assertEq(gasAggregator.getTrackedChainCount(), 3);

        uint256[] memory chainIds = gasAggregator.getTrackedChainIds();
        assertEq(chainIds.length, 3);
    }

    function testAddLegacyChainRevertsOnZeroChainId() public {
        vm.expectRevert(abi.encodeWithSelector(GasAggregator.ZeroChainId.selector));
        gasAggregator.addLegacyChain(0, address(mockProxy1));
    }

    function testAddLegacyChainRevertsOnDuplicate() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.ChainAlreadyTracked.selector, 1));
        gasAggregator.addLegacyChain(1, address(mockProxy2));
    }

    function testAddLegacyChainRevertsOnNoCode() public {
        address emptyAddress = address(0x9999);

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.ChainNotFound.selector, 1));
        gasAggregator.addLegacyChain(1, emptyAddress);
    }

    function testAddLegacyChainRevertsOnNonOwner() public {
        vm.prank(user);
        vm.expectRevert();
        gasAggregator.addLegacyChain(1, address(mockProxy1));
    }

    function testAddLegacyChainRevertsWhenPaused() public {
        gasAggregator.pause();

        vm.expectRevert();
        gasAggregator.addLegacyChain(1, address(mockProxy1));
    }

    function testAddLegacyChainRevertsAfterFactorySet() public {
        // Set factory to a non-zero address
        bytes32 bytecodeHash = keccak256("test");
        gasAggregator.setFactory(address(0x1234), bytecodeHash);

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.FactoryAlreadySet.selector));
        gasAggregator.addLegacyChain(1, address(mockProxy1));
    }

    /*//////////////////////////////////////////////////////////////
                    AGGREGATION TESTS
    //////////////////////////////////////////////////////////////*/

    function testAggregateTokensSimple() public {
        // Add chains
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        gasAggregator.addLegacyChain(2, address(mockProxy2));

        // Set token usage
        mockProxy1.setTokensUsedPerEpoch(START_EPOCH, 100 ether);
        mockProxy2.setTokensUsedPerEpoch(START_EPOCH, 200 ether);

        // Move to next epoch
        vm.warp(getEpochStart(START_EPOCH + 1) + 1 days);

        // Aggregate
        uint256[] memory emptyChainIds = new uint256[](0);
        uint256[] memory emptyTokens = new uint256[](0);

        uint256[] memory expectedChainIds = new uint256[](2);
        expectedChainIds[0] = 1;
        expectedChainIds[1] = 2;
        uint256[] memory expectedTokens = new uint256[](2);
        expectedTokens[0] = 100 ether;
        expectedTokens[1] = 200 ether;

        vm.expectEmit(true, true, true, true);
        emit AggregatedTokens(START_EPOCH, expectedChainIds, expectedTokens);

        gasAggregator.aggregateTokens(emptyChainIds, emptyTokens);

        assertEq(gasAggregator.currentEpoch(), START_EPOCH + 1);
        assertFalse(gasAggregator.paused());

        bytes32 expectedHash = keccak256(abi.encode(expectedChainIds, expectedTokens));
        assertEq(gasAggregator.aggregatedEpochDataHash(START_EPOCH), expectedHash);
    }

    function testAggregateTokensSkipsZeroGasUsage() public {
        // Add chains
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        gasAggregator.addLegacyChain(2, address(mockProxy2));
        gasAggregator.addLegacyChain(3, address(mockProxy3));

        // Set token usage (chain 2 has zero usage)
        mockProxy1.setTokensUsedPerEpoch(START_EPOCH, 100 ether);
        mockProxy2.setTokensUsedPerEpoch(START_EPOCH, 0);
        mockProxy3.setTokensUsedPerEpoch(START_EPOCH, 300 ether);

        // Move to next epoch
        vm.warp(getEpochStart(START_EPOCH + 1) + 1 days);

        // Aggregate
        uint256[] memory emptyChainIds = new uint256[](0);
        uint256[] memory emptyTokens = new uint256[](0);

        gasAggregator.aggregateTokens(emptyChainIds, emptyTokens);

        // Only 2 chains should be in the result (chain 2 is skipped)
        uint256[] memory expectedChainIds = new uint256[](2);
        expectedChainIds[0] = 1;
        expectedChainIds[1] = 3;
        uint256[] memory expectedTokens = new uint256[](2);
        expectedTokens[0] = 100 ether;
        expectedTokens[1] = 300 ether;

        bytes32 expectedHash = keccak256(abi.encode(expectedChainIds, expectedTokens));
        assertEq(gasAggregator.aggregatedEpochDataHash(START_EPOCH), expectedHash);
    }

    function testAggregateTokensRevertsWhenEpochNotOver() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));

        uint256[] memory emptyChainIds = new uint256[](0);
        uint256[] memory emptyTokens = new uint256[](0);

        vm.expectRevert();
        gasAggregator.aggregateTokens(emptyChainIds, emptyTokens);
    }

    function testAggregateTokensRevertsWithNoChains() public {
        // Move to next epoch
        vm.warp(getEpochStart(START_EPOCH + 1) + 1 days);

        uint256[] memory emptyChainIds = new uint256[](0);
        uint256[] memory emptyTokens = new uint256[](0);

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.NoChainsAdded.selector));
        gasAggregator.aggregateTokens(emptyChainIds, emptyTokens);
    }

    function testAggregateTokensPausesAndUnpauses() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        mockProxy1.setTokensUsedPerEpoch(START_EPOCH, 100 ether);

        // Move to next epoch
        vm.warp(getEpochStart(START_EPOCH + 1) + 1 days);

        assertFalse(gasAggregator.paused());

        uint256[] memory emptyChainIds = new uint256[](0);
        uint256[] memory emptyTokens = new uint256[](0);

        // First call pauses
        gasAggregator.aggregateTokens(emptyChainIds, emptyTokens);

        // After aggregation completes, it should unpause
        assertFalse(gasAggregator.paused());
    }

    /*//////////////////////////////////////////////////////////////
                    ADMIN FUNCTIONS TESTS
    //////////////////////////////////////////////////////////////*/

    function testSetAddChainFee() public {
        uint256 newFee = 10 ether;

        vm.expectEmit(true, true, true, true);
        emit AddChainFeeUpdated(ADD_CHAIN_FEE, newFee);

        gasAggregator.setAddChainFee(newFee);

        assertEq(gasAggregator.addChainFee(), newFee);
    }

    function testSetAddChainFeeRevertsOnNonOwner() public {
        vm.prank(user);
        vm.expectRevert();
        gasAggregator.setAddChainFee(10 ether);
    }

    function testSetMaxAppchainsToQuery() public {
        uint256 newMax = 50;

        vm.expectEmit(true, true, true, true);
        emit UpdateMaxAppchainsToQuery(START_EPOCH, newMax);

        gasAggregator.setMaxAppchainsToQuery(newMax);

        assertEq(gasAggregator.maxAppchainsToQuery(), newMax);
    }

    function testSetMaxAppchainsToQueryRevertsOnNonOwner() public {
        vm.prank(user);
        vm.expectRevert();
        gasAggregator.setMaxAppchainsToQuery(50);
    }

    function testSetMaxAppchainsToQueryRevertsWhenPaused() public {
        gasAggregator.pause();

        vm.expectRevert();
        gasAggregator.setMaxAppchainsToQuery(50);
    }

    function testRemoveAppchains() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        gasAggregator.addLegacyChain(2, address(mockProxy2));

        assertEq(gasAggregator.getTrackedChainCount(), 2);

        uint256[] memory chainsToRemove = new uint256[](1);
        chainsToRemove[0] = 1;

        vm.expectEmit(true, true, true, true);
        emit ChainRemoved(START_EPOCH, 1);

        gasAggregator.removeAppchains(chainsToRemove);

        assertEq(gasAggregator.getTrackedChainCount(), 1);
        assertEq(gasAggregator.getTrackedChainId(0), 2);
    }

    function testRemoveAppchainsMultiple() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        gasAggregator.addLegacyChain(2, address(mockProxy2));
        gasAggregator.addLegacyChain(3, address(mockProxy3));

        uint256[] memory chainsToRemove = new uint256[](2);
        chainsToRemove[0] = 1;
        chainsToRemove[1] = 3;

        gasAggregator.removeAppchains(chainsToRemove);

        assertEq(gasAggregator.getTrackedChainCount(), 1);
        assertEq(gasAggregator.getTrackedChainId(0), 2);
    }

    function testRemoveAppchainsRevertsOnNonOwner() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));

        uint256[] memory chainsToRemove = new uint256[](1);
        chainsToRemove[0] = 1;

        vm.prank(user);
        vm.expectRevert();
        gasAggregator.removeAppchains(chainsToRemove);
    }

    function testRemoveAppchainsRevertsWhenPaused() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        gasAggregator.pause();

        uint256[] memory chainsToRemove = new uint256[](1);
        chainsToRemove[0] = 1;

        vm.expectRevert();
        gasAggregator.removeAppchains(chainsToRemove);
    }

    function testRemoveAppchainsRevertsOnNonExistent() public {
        uint256[] memory chainsToRemove = new uint256[](1);
        chainsToRemove[0] = 999;

        vm.expectRevert();
        gasAggregator.removeAppchains(chainsToRemove);
    }

    function testWithdrawFees() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));

        // Simulate some fees collected
        vm.deal(address(gasAggregator), 10 ether);

        address payable recipient = payable(address(0xdead));
        uint256 balanceBefore = recipient.balance;

        gasAggregator.withdrawFees(recipient, 5 ether);

        assertEq(recipient.balance, balanceBefore + 5 ether);
        assertEq(address(gasAggregator).balance, 5 ether);
    }

    function testWithdrawFeesAll() public {
        vm.deal(address(gasAggregator), 10 ether);

        address payable recipient = payable(address(0xdead));
        uint256 balanceBefore = recipient.balance;

        // Passing 0 withdraws all
        gasAggregator.withdrawFees(recipient, 0);

        assertEq(recipient.balance, balanceBefore + 10 ether);
        assertEq(address(gasAggregator).balance, 0);
    }

    function testWithdrawFeesRevertsOnZeroAddress() public {
        vm.deal(address(gasAggregator), 10 ether);

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.ZeroAddress.selector));
        gasAggregator.withdrawFees(payable(address(0)), 5 ether);
    }

    function testWithdrawFeesRevertsOnInsufficientBalance() public {
        vm.deal(address(gasAggregator), 5 ether);

        vm.expectRevert();
        gasAggregator.withdrawFees(payable(address(0xdead)), 10 ether);
    }

    function testWithdrawFeesRevertsOnNonOwner() public {
        vm.deal(address(gasAggregator), 10 ether);

        vm.prank(user);
        vm.expectRevert();
        gasAggregator.withdrawFees(payable(address(0xdead)), 5 ether);
    }

    function testSetFactory() public {
        address factoryAddress = address(0x1234);
        bytes32 bytecodeHash = keccak256("test");

        gasAggregator.setFactory(factoryAddress, bytecodeHash);

        assertEq(gasAggregator.factory(), factoryAddress);
        assertEq(gasAggregator.syndicateProxyBytecodeHash(), bytecodeHash);
    }

    function testSetFactoryRevertsOnZeroAddress() public {
        bytes32 bytecodeHash = keccak256("test");

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.ZeroAddress.selector));
        gasAggregator.setFactory(address(0), bytecodeHash);
    }

    function testSetFactoryRevertsOnZeroHash() public {
        address factoryAddress = address(0x1234);

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.InvalidDataHash.selector));
        gasAggregator.setFactory(factoryAddress, bytes32(0));
    }

    function testSetFactoryRevertsOnAlreadySet() public {
        address factoryAddress = address(0x1234);
        bytes32 bytecodeHash = keccak256("test");

        gasAggregator.setFactory(factoryAddress, bytecodeHash);

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.FactoryAlreadySet.selector));
        gasAggregator.setFactory(address(0x5678), bytecodeHash);
    }

    function testSetFactoryRevertsOnNonOwner() public {
        address factoryAddress = address(0x1234);
        bytes32 bytecodeHash = keccak256("test");

        vm.prank(user);
        vm.expectRevert();
        gasAggregator.setFactory(factoryAddress, bytecodeHash);
    }

    /*//////////////////////////////////////////////////////////////
                    PAUSE/UNPAUSE TESTS
    //////////////////////////////////////////////////////////////*/

    function testPause() public {
        assertFalse(gasAggregator.paused());

        gasAggregator.pause();

        assertTrue(gasAggregator.paused());
    }

    function testUnpause() public {
        gasAggregator.pause();
        assertTrue(gasAggregator.paused());

        gasAggregator.unpause();

        assertFalse(gasAggregator.paused());
        assertEq(gasAggregator.currentAggregateIndex(), 0);
        assertEq(gasAggregator.pendingDataHash(), bytes32(0));
    }

    function testPauseRevertsOnNonOwner() public {
        vm.prank(user);
        vm.expectRevert();
        gasAggregator.pause();
    }

    function testUnpauseRevertsOnNonOwner() public {
        gasAggregator.pause();

        vm.prank(user);
        vm.expectRevert();
        gasAggregator.unpause();
    }

    /*//////////////////////////////////////////////////////////////
                    VIEW FUNCTIONS TESTS
    //////////////////////////////////////////////////////////////*/

    function testGetTrackedChainCount() public {
        assertEq(gasAggregator.getTrackedChainCount(), 0);

        gasAggregator.addLegacyChain(1, address(mockProxy1));
        assertEq(gasAggregator.getTrackedChainCount(), 1);

        gasAggregator.addLegacyChain(2, address(mockProxy2));
        assertEq(gasAggregator.getTrackedChainCount(), 2);
    }

    function testGetTrackedChainIds() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        gasAggregator.addLegacyChain(2, address(mockProxy2));
        gasAggregator.addLegacyChain(3, address(mockProxy3));

        uint256[] memory chainIds = gasAggregator.getTrackedChainIds();

        assertEq(chainIds.length, 3);
        assertEq(chainIds[0], 1);
        assertEq(chainIds[1], 2);
        assertEq(chainIds[2], 3);
    }

    function testGetTrackedChainId() public {
        gasAggregator.addLegacyChain(10, address(mockProxy1));
        gasAggregator.addLegacyChain(20, address(mockProxy2));

        assertEq(gasAggregator.getTrackedChainId(0), 10);
        assertEq(gasAggregator.getTrackedChainId(1), 20);
    }

    function testAggregatedEpochDataHash() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        mockProxy1.setTokensUsedPerEpoch(START_EPOCH, 100 ether);

        // Move to next epoch and aggregate
        vm.warp(getEpochStart(START_EPOCH + 1) + 1 days);

        uint256[] memory emptyChainIds = new uint256[](0);
        uint256[] memory emptyTokens = new uint256[](0);

        gasAggregator.aggregateTokens(emptyChainIds, emptyTokens);

        bytes32 hash = gasAggregator.aggregatedEpochDataHash(START_EPOCH);
        assertTrue(hash != bytes32(0));
    }

    /*//////////////////////////////////////////////////////////////
                    SIMULATE AGGREGATE TESTS
    //////////////////////////////////////////////////////////////*/

    function testSimulateAggregateTokens() public {
        gasAggregator.addLegacyChain(1, address(mockProxy1));
        gasAggregator.addLegacyChain(2, address(mockProxy2));

        mockProxy1.setTokensUsedPerEpoch(START_EPOCH, 100 ether);
        mockProxy2.setTokensUsedPerEpoch(START_EPOCH, 200 ether);

        // Move to next epoch
        vm.warp(getEpochStart(START_EPOCH + 1) + 1 days);

        uint256[] memory emptyChainIds = new uint256[](0);
        uint256[] memory emptyTokens = new uint256[](0);

        (uint256 nextIndex, uint256[] memory chainIds, uint256[] memory tokens) =
            gasAggregator.simulateAggregateTokens(0, emptyChainIds, emptyTokens);

        assertEq(nextIndex, 0); // All chains processed in one go
        assertEq(chainIds.length, 2);
        assertEq(tokens.length, 2);
        assertEq(chainIds[0], 1);
        assertEq(chainIds[1], 2);
        assertEq(tokens[0], 100 ether);
        assertEq(tokens[1], 200 ether);
    }
}
