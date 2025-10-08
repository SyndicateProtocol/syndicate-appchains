// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GasAggregator} from "../../src/staking/GasAggregator.sol";
import {EpochTracker} from "../../src/staking/EpochTracker.sol";
import {SyndicateFactory} from "../../src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "../../src/SyndicateSequencingChain.sol";
import {AlwaysAllowedModule} from "../../src/sequencing-modules/AlwaysAllowedModule.sol";
import {RequireAndModule} from "../../src/requirement-modules/RequireAndModule.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract MockGasCounter {
    mapping(uint256 => uint256) public tokensUsedPerEpoch;

    function setTokensForEpoch(uint256 epoch, uint256 tokens) external {
        tokensUsedPerEpoch[epoch] = tokens;
    }
}

contract GasAggregatorUtils is GasAggregator {
    constructor() GasAggregator(1, 0, 0) {}

    function quickSort(uint256[] memory keys, uint256[] memory values)
        public
        pure
        returns (uint256[] memory, uint256[] memory)
    {
        _quickSort(keys, values);
        return (keys, values);
    }
}

contract MockAppchainFactory {}

contract GasAggregatorTest is Test {
    GasAggregator public gasAggregator;
    MockAppchainFactory public mockFactory;
    MockGasCounter public mockGasCounter1;
    MockGasCounter public mockGasCounter2;
    MockGasCounter public mockGasCounter3;

    address public admin = address(0x1);
    address public user = address(0x2);

    uint256 public constant EPOCH_DURATION = 30 days;
    uint256 public constant CHALLENGE_WINDOW = 24 hours;

    event TopChainsDataSubmitted(uint256[] appchainIDs, uint256[] tokens, uint256 total);

    function setUp() public {
        mockFactory = new MockAppchainFactory();
        mockGasCounter1 = new MockGasCounter();
        mockGasCounter2 = new MockGasCounter();
        mockGasCounter3 = new MockGasCounter();

        // Deploy GasAggregator contract
        vm.prank(admin);
        gasAggregator = new GasAggregator(1, 0, 2);
        assertEq(gasAggregator.epoch(), 1);
    }

    /// @notice Helper function to set up chain overrides and add chains to the aggregator
    /// @param chainIds Array of chain IDs to set up (1=mockGasCounter1, 2=mockGasCounter2, 3=mockGasCounter3)
    function setupChainsWithOverrides(uint256[] memory chainIds) internal {
        for (uint256 i = 0; i < chainIds.length; i++) {
            uint256 chainId = chainIds[i];
            address mockContract;

            if (chainId == 1) {
                mockContract = address(mockGasCounter1);
            } else if (chainId == 2) {
                mockContract = address(mockGasCounter2);
            } else if (chainId == 3) {
                mockContract = address(mockGasCounter3);
            } else {
                revert("Invalid chain ID");
            }

            vm.prank(admin);
            gasAggregator.addLegacyChain(chainId, mockContract);
        }
    }

    function test_SetMaxAppchainsToQuery() public {
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(5);
        assertEq(gasAggregator.maxAppchainsToQuery(), 5);
    }

    function test_SetMaxAppchainsToQuery_NonAdmin() public {
        vm.prank(user);
        vm.expectRevert();
        gasAggregator.setMaxAppchainsToQuery(5);
    }

    function test_SetFactory() public {
        MockAppchainFactory newFactory = new MockAppchainFactory();
        vm.prank(admin);
        gasAggregator.setFactory(address(newFactory), keccak256(type(MockAppchainFactory).creationCode));
        assertEq(address(gasAggregator.factory()), address(newFactory));
    }

    function test_SetFactory_NonAdmin() public {
        vm.prank(user);
        vm.expectRevert();
        gasAggregator.setFactory(address(mockFactory), keccak256(type(MockAppchainFactory).creationCode));
    }

    function test_AggregateTokensUsed_Success() public {
        // Setup: below threshold for automatic aggregation
        // First increase maxAppchainsToQuery to stay below threshold
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(3);

        // Set up chains 1 and 2
        uint256[] memory chains = new uint256[](2);
        chains[0] = 1;
        chains[1] = 2;
        setupChainsWithOverrides(chains);

        uint256[] memory gasUsage = new uint256[](2);
        gasUsage[0] = 100;
        gasUsage[1] = 200;

        // Set gas usage for current epoch
        uint256 epoch = 1;
        mockGasCounter1.setTokensForEpoch(epoch, gasUsage[0]);
        mockGasCounter2.setTokensForEpoch(epoch, gasUsage[1]);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        vm.expectEmit(true, false, false, true);
        emit GasAggregator.AggregatedTokens(epoch, chains, gasUsage);
        gasAggregator.aggregateTokens(new uint256[](0), new uint256[](0));
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch), keccak256(abi.encode(chains, gasUsage)));

        // Should increment epoch
        assertEq(gasAggregator.epoch(), epoch + 1);
    }

    function test_AggregateTokensUsed_Top1() public {
        // Set maxAppchainsToQuery to 1
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(1);

        // Set up chains 1, 2, and 3
        uint256[] memory chains = new uint256[](3);
        chains[0] = 1;
        chains[1] = 2;
        chains[2] = 3;
        setupChainsWithOverrides(chains);

        uint256[] memory gasUsage = new uint256[](3);
        gasUsage[0] = 100;
        gasUsage[1] = 101;
        gasUsage[2] = 100;

        // Set gas usage for current epoch
        uint256 epoch = 1;
        mockGasCounter1.setTokensForEpoch(epoch, gasUsage[0]);
        mockGasCounter2.setTokensForEpoch(epoch, gasUsage[1]);
        mockGasCounter3.setTokensForEpoch(epoch, gasUsage[2]);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Aggregate
        uint256[] memory prevChainIds;
        uint256[] memory prevGas;
        gasAggregator.aggregateTokens(prevChainIds, prevGas);

        // Simulate aggregation
        uint256 chunk;
        (chunk, prevChainIds, prevGas) = gasAggregator.simulateAggregateTokens(0, new uint256[](0), new uint256[](0));
        assertEq(chunk, 1);
        assertEq(prevChainIds.length, 1);
        assertEq(prevChainIds[0], 1);
        assertEq(prevGas.length, 1);
        assertEq(prevGas[0], 100);

        // Aggregate
        gasAggregator.aggregateTokens(prevChainIds, prevGas);

        // Simulate aggregation
        (chunk, prevChainIds, prevGas) = gasAggregator.simulateAggregateTokens(chunk, prevChainIds, prevGas);
        assertEq(chunk, 2);
        assertEq(prevChainIds.length, 1);
        assertEq(prevChainIds[0], 2);
        assertEq(prevGas.length, 1);
        assertEq(prevGas[0], 101);

        // Aggregate. The third time should finish aggregation
        uint256[] memory topChains = new uint256[](1);
        uint256[] memory topGas = new uint256[](1);
        topChains[0] = 2;
        topGas[0] = 101;
        vm.expectEmit(true, false, false, true);
        emit GasAggregator.AggregatedTokens(epoch, topChains, topGas);
        gasAggregator.aggregateTokens(prevChainIds, prevGas);
        assertEq(gasAggregator.epoch(), epoch + 1);
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch), keccak256(abi.encode(topChains, topGas)));

        // Simulate aggregation
        (chunk, prevChainIds, prevGas) = gasAggregator.simulateAggregateTokens(chunk, prevChainIds, prevGas);
        assertEq(chunk, 0);
        assertEq(prevChainIds.length, 1);
        assertEq(prevChainIds[0], 2);
        assertEq(prevGas.length, 1);
        assertEq(prevGas[0], 101);
    }

    function test_EdgeCase_EmptyAppchainList() public {
        // Should fail
        vm.expectRevert(GasAggregator.NoChainsAdded.selector);
        gasAggregator.aggregateTokens(new uint256[](0), new uint256[](0));
    }

    function test_quickSort() public {
        GasAggregatorUtils utils = new GasAggregatorUtils();
        uint256[] memory keys = new uint256[](5);
        keys[0] = 0;
        keys[1] = 1;
        keys[2] = 2;
        keys[3] = 3;
        keys[4] = 4;
        uint256[] memory values = new uint256[](5);
        values[0] = 3;
        values[1] = 0;
        values[2] = 1;
        values[3] = type(uint256).max;
        values[4] = 3;
        (keys, values) = utils.quickSort(keys, values);

        assertEq(keys.length, 5);
        assertEq(keys[0], 3);
        assertEq(keys[1], 0);
        assertEq(keys[2], 4);
        assertEq(keys[3], 2);
        assertEq(keys[4], 1);

        assertEq(values.length, 5);
        assertEq(values[0], type(uint256).max);
        assertEq(values[1], 3);
        assertEq(values[2], 3);
        assertEq(values[3], 1);
        assertEq(values[4], 0);
    }
}
