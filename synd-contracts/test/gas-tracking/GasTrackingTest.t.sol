// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GasAggregator, GasAggregatorUtils} from "../../src/staking/GasAggregator.sol";
import {EpochTracker} from "../../src/staking/EpochTracker.sol";
import {SyndicateFactory} from "../../src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "../../src/SyndicateSequencingChain.sol";
import {AlwaysAllowedModule} from "../../src/sequencing-modules/AlwaysAllowedModule.sol";
import {RequireAndModule} from "../../src/requirement-modules/RequireAndModule.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Arrays} from "@openzeppelin/contracts/utils/Arrays.sol";
import {Comparators} from "@openzeppelin/contracts/utils/Comparators.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";

contract MockGasCounter {
    mapping(uint256 => uint256) public tokensUsedPerEpoch;

    function setTokensForEpoch(uint256 epoch, uint256 tokens) external {
        tokensUsedPerEpoch[epoch] = tokens;
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
        assertEq(gasAggregator.currentEpoch(), 1);

        vm.warp(gasAggregator.getEpochStart(1));
        assertEq(gasAggregator.getCurrentEpoch(), 1);
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
        vm.warp(block.timestamp + EPOCH_DURATION);

        // Simulate aggregation
        uint256 nextAggregateIndex;
        uint256[] memory chainIds;
        uint256[] memory tokens;
        (nextAggregateIndex, chainIds, tokens) =
            gasAggregator.simulateAggregateTokens(0, new uint256[](0), new uint256[](0));
        require(nextAggregateIndex == 0);
        require(chainIds.length == chains.length);
        require(gasUsage.length == tokens.length);
        for (uint256 i = 0; i < chains.length; i++) {
            require(chains[i] == chainIds[i]);
            require(gasUsage[i] == tokens[i]);
        }

        // Aggregate
        vm.expectEmit(true, false, false, true);
        emit GasAggregator.AggregatedTokens(epoch, chains, gasUsage);
        gasAggregator.aggregateTokens(new uint256[](0), new uint256[](0));
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch), keccak256(abi.encode(chains, gasUsage)));

        // Should increment epoch
        assertEq(gasAggregator.currentEpoch(), epoch + 1);
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
        vm.warp(block.timestamp + EPOCH_DURATION);

        // Aggregate
        uint256[] memory prevChainIds;
        uint256[] memory prevGas;
        vm.expectEmit(true, true, true, true);
        emit GasAggregator.AggregationPending(epoch, 2);
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
        vm.expectEmit(true, true, true, true);
        emit GasAggregator.AggregationPending(epoch, 1);
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
        vm.expectEmit(true, true, true, true);
        emit GasAggregator.AggregatedTokens(epoch, topChains, topGas);
        gasAggregator.aggregateTokens(prevChainIds, prevGas);
        assertEq(gasAggregator.currentEpoch(), epoch + 1);
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch), keccak256(abi.encode(topChains, topGas)));

        assertFalse(gasAggregator.paused());
    }

    function test_UnpauseDuringAggregation() public {
        // Set maxAppchainsToQuery to 1
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(1);

        // Set up chains 1 and 2
        uint256[] memory chains = new uint256[](2);
        chains[0] = 1;
        chains[1] = 2;
        setupChainsWithOverrides(chains);

        uint256[] memory gasUsage = new uint256[](2);
        gasUsage[0] = 100;
        gasUsage[1] = 101;

        // Set gas usage for current epoch
        uint256 epoch = 1;
        mockGasCounter1.setTokensForEpoch(epoch, gasUsage[0]);
        mockGasCounter2.setTokensForEpoch(epoch, gasUsage[1]);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION);

        // Aggregate
        uint256[] memory prevChainIds;
        uint256[] memory prevGas;
        vm.expectEmit(true, true, true, true);
        emit GasAggregator.AggregationPending(epoch, 1);
        gasAggregator.aggregateTokens(prevChainIds, prevGas);

        // Confirm contract is paused
        assertEq(gasAggregator.paused(), true);

        // Unpause
        vm.prank(admin);
        gasAggregator.unpause();
        assertEq(gasAggregator.paused(), false);
        assertEq(gasAggregator.pendingDataHash(), 0);
        assertEq(gasAggregator.currentAggregateIndex(), 0);

        // Retry aggregation
        vm.expectEmit(true, true, true, true);
        emit GasAggregator.AggregationPending(epoch, 1);
        gasAggregator.aggregateTokens(prevChainIds, prevGas);

        // Simulate aggregation
        uint256 chunk;
        (chunk, prevChainIds, prevGas) = gasAggregator.simulateAggregateTokens(0, new uint256[](0), new uint256[](0));
        assertEq(chunk, 1);
        assertEq(prevChainIds.length, 1);
        assertEq(prevChainIds[0], 1);
        assertEq(prevGas.length, 1);
        assertEq(prevGas[0], 100);

        // Finish aggregation
        uint256[] memory topChains = new uint256[](1);
        uint256[] memory topGas = new uint256[](1);
        topChains[0] = 2;
        topGas[0] = 101;
        vm.expectEmit(true, true, true, true);
        emit GasAggregator.AggregatedTokens(epoch, topChains, topGas);
        gasAggregator.aggregateTokens(prevChainIds, prevGas);
        assertEq(gasAggregator.currentEpoch(), epoch + 1);
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch), keccak256(abi.encode(topChains, topGas)));

        assertFalse(gasAggregator.paused());
    }

    function test_EdgeCase_EmptyAppchainList() public {
        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION);

        // Should fail
        vm.expectRevert(GasAggregator.NoChainsAdded.selector);
        gasAggregator.aggregateTokens(new uint256[](0), new uint256[](0));
    }

    function test_EdgeCase_EpochNotOver() public {
        // Aggregate at start of epoch, should fail
        vm.expectRevert(abi.encodeWithSelector(GasAggregator.EpochNotOver.selector, 1, 1));
        gasAggregator.simulateAggregateTokens(0, new uint256[](0), new uint256[](0));

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.EpochNotOver.selector, 1, 1));
        gasAggregator.aggregateTokens(new uint256[](0), new uint256[](0));

        // Move to the end of the epoch
        vm.warp(block.timestamp + EPOCH_DURATION - 1);

        // Should still fail
        vm.expectRevert(abi.encodeWithSelector(GasAggregator.EpochNotOver.selector, 1, 1));
        gasAggregator.simulateAggregateTokens(0, new uint256[](0), new uint256[](0));

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.EpochNotOver.selector, 1, 1));
        gasAggregator.aggregateTokens(new uint256[](0), new uint256[](0));
    }

    function test_pause() public {
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
        vm.warp(block.timestamp + EPOCH_DURATION);

        vm.prank(admin);
        gasAggregator.pause();
        assertEq(gasAggregator.paused(), true);

        vm.expectRevert(Pausable.EnforcedPause.selector);
        gasAggregator.aggregateTokens(new uint256[](0), new uint256[](0));

        vm.prank(admin);
        gasAggregator.unpause();
        assertEq(gasAggregator.paused(), false);

        gasAggregator.aggregateTokens(new uint256[](0), new uint256[](0));
    }

    function test_quickSort() public pure {
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
        GasAggregatorUtils.sort(keys, values);

        assertEq(keys.length, 5);
        assertEq(keys[0], 3);
        assert(keys[1] == 4 || keys[1] == 0);
        assert(keys[2] == 4 || keys[2] == 0);
        assert(keys[1] != keys[2]);
        assertEq(keys[3], 2);
        assertEq(keys[4], 1);

        assertEq(values.length, 5);
        assertEq(values[0], type(uint256).max);
        assertEq(values[1], 3);
        assertEq(values[2], 3);
        assertEq(values[3], 1);
        assertEq(values[4], 0);
    }

    function test_quickSelect() public pure {
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
        GasAggregatorUtils.select(keys, values, 1);

        assertEq(keys.length, 1);
        assertEq(keys[0], 3);
        assertEq(values.length, 1);
        assertEq(values[0], type(uint256).max);
    }

    function test_utilsGasComparisonSorted() public view {
        uint256[] memory v1 = new uint256[](100);
        uint256[] memory v2 = new uint256[](100);
        uint256[] memory k = new uint256[](100);
        for (uint256 i = 0; i < v1.length; i++) {
            v1[i] = v1.length - i;
            v2[i] = v1[i];
        }
        uint256 gasUsed = gasleft();
        GasAggregatorUtils.sort(k, v1);
        console.log("utils.sort", gasUsed - gasleft());
        gasUsed = gasleft();
        Arrays.sort(v2, Comparators.gt);
        console.log("arrays.sort", gasUsed - gasleft());
        for (uint256 i = 0; i < v1.length; i++) {
            assertEq(v1[i], v2[i]);
        }
        uint256[] memory v3 = new uint256[](200);
        uint256[] memory k3 = new uint256[](200);
        for (uint256 i = 0; i < v3.length; i++) {
            v3[i] = v3.length - i;
        }
        gasUsed = gasleft();
        GasAggregatorUtils.select(k3, v3, 100);
        console.log("utils.select", gasUsed - gasleft());
    }

    function test_utilsGasComparisonRandom() public {
        uint256 utilsGasUsed;
        uint256 utilsMaxGasUsed;
        uint256 arraysGasUsed;
        uint256 arraysMaxGasUsed;
        uint256[] memory v1 = new uint256[](100);
        uint256[] memory v2 = new uint256[](100);
        uint256[] memory k = new uint256[](100);
        for (uint256 j = 0; j < 500; j++) {
            for (uint256 i = 0; i < v1.length; i++) {
                v1[i] = vm.randomUint();
                v2[i] = v1[i];
            }
            uint256 gasUsed = gasleft();
            GasAggregatorUtils.sort(k, v1);
            gasUsed -= gasleft();
            if (gasUsed > utilsMaxGasUsed) {
                utilsMaxGasUsed = gasUsed;
            }
            utilsGasUsed += gasUsed;
            gasUsed = gasleft();
            Arrays.sort(v2, Comparators.gt);
            gasUsed -= gasleft();
            if (gasUsed > arraysMaxGasUsed) {
                arraysMaxGasUsed = gasUsed;
            }
            arraysGasUsed += gasUsed;
            for (uint256 i = 0; i < v1.length; i++) {
                assertEq(v1[i], v2[i]);
            }
        }
        utilsGasUsed /= 500;
        arraysGasUsed /= 500;
        console.log("utils.sort", utilsGasUsed, utilsMaxGasUsed);
        console.log("arrays.sort", arraysGasUsed, arraysMaxGasUsed);

        uint256 selectGasUsed;
        uint256 selectMaxGasUsed;
        uint256[] memory v3 = new uint256[](200);
        uint256[] memory v4 = new uint256[](200);
        uint256[] memory k3 = new uint256[](200);
        for (uint256 j = 0; j < 500; j++) {
            for (uint256 i = 0; i < v3.length; i++) {
                v3[i] = vm.randomUint();
                v4[i] = v3[i];
            }
            uint256 gasUsed = gasleft();
            GasAggregatorUtils.select(k3, v3, 100);
            gasUsed -= gasleft();
            if (gasUsed > selectMaxGasUsed) {
                selectMaxGasUsed = gasUsed;
            }
            selectGasUsed += gasUsed;
            assertEq(v3.length, 100);
            assertEq(k3.length, 100);
            Arrays.sort(v3, Comparators.gt);
            Arrays.sort(v4, Comparators.gt);
            for (uint256 i = 0; i < v3.length; i++) {
                assertEq(v3[i], v4[i]);
            }
            assembly {
                mstore(v3, 200)
                mstore(k3, 200)
            }
        }
        selectGasUsed /= 500;
        console.log("utils.select", selectGasUsed, selectMaxGasUsed);
    }
}
