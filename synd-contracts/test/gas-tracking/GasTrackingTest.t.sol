// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {GasAggregator, AppchainFactory, StakingAppchain} from "../../src/GasAggregator.sol";
import {SyndicateFactory} from "../../src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "../../src/SyndicateSequencingChain.sol";
import {AlwaysAllowedModule} from "../../src/sequencing-modules/AlwaysAllowedModule.sol";
import {RequireAndModule} from "../../src/requirement-modules/RequireAndModule.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract MockGasCounter {
    mapping(uint256 => uint256) public tokensUsedPerEpoch;

    function getTokensForEpoch(uint256 epoch) external view returns (uint256) {
        return tokensUsedPerEpoch[epoch];
    }

    function setTokensForEpoch(uint256 epoch, uint256 tokens) external {
        tokensUsedPerEpoch[epoch] = tokens;
    }
}

contract MockAppchainFactory is AppchainFactory {
    uint256 public totalAppchains;
    mapping(uint256 => address) public appchainContracts;
    uint256[] public appchainChainIDs;

    function setTotalAppchains(uint256 count) external {
        totalAppchains = count;
    }

    function addAppchain(uint256 chainId, address contractAddr) external {
        appchainContracts[chainId] = contractAddr;
        appchainChainIDs.push(chainId);
    }

    function getTotalAppchains() external view returns (uint256) {
        return totalAppchains;
    }

    function getContractsForAppchains(uint256[] memory chainIDs) external view returns (address[] memory) {
        address[] memory contracts = new address[](chainIDs.length);
        for (uint256 i = 0; i < chainIDs.length; i++) {
            contracts[i] = appchainContracts[chainIDs[i]];
        }
        return contracts;
    }

    function getAppchainsAndContracts() external view returns (uint256[] memory, address[] memory) {
        address[] memory contracts = new address[](appchainChainIDs.length);
        for (uint256 i = 0; i < appchainChainIDs.length; i++) {
            contracts[i] = appchainContracts[appchainChainIDs[i]];
        }
        return (appchainChainIDs, contracts);
    }
}

contract MockStakingAppchain is StakingAppchain {
    uint256[] public chainIDs;
    uint256[] public tokens;

    function pushData(uint256[] memory _chainIDs, uint256[] memory _tokens) external {
        chainIDs = _chainIDs;
        tokens = _tokens;
    }

    function getChainIDs() external view returns (uint256[] memory) {
        return chainIDs;
    }

    function getTokens() external view returns (uint256[] memory) {
        return tokens;
    }
}

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

        // Deploy GasAggregator with mock factory
        gasAggregator = new GasAggregator(mockFactory, new MockStakingAppchain(), admin);

        // Set initial values using admin role
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);
        vm.prank(admin);
        gasAggregator.setChallengeWindow(CHALLENGE_WINDOW);
    }

    function test_Constructor() public view {
        assertEq(address(gasAggregator.factory()), address(mockFactory));
        assertTrue(gasAggregator.hasRole(gasAggregator.DEFAULT_ADMIN_ROLE(), admin));

        // Should start with current epoch
        uint256 currentEpoch = gasAggregator.getCurrentEpoch();
        assertEq(gasAggregator.currentEpochToAggregate(), currentEpoch);
    }

    function test_Constructor_ZeroAdmin() public {
        MockStakingAppchain stakingAppchain = new MockStakingAppchain();
        vm.expectRevert(GasAggregator.ZeroAddress.selector);
        new GasAggregator(mockFactory, stakingAppchain, address(0));
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

    function test_SetChallengeWindow() public {
        vm.prank(admin);
        gasAggregator.setChallengeWindow(48 hours);
        assertEq(gasAggregator.challengeWindow(), 48 hours);
    }

    function test_SetChallengeWindow_NonAdmin() public {
        vm.prank(user);
        vm.expectRevert();
        gasAggregator.setChallengeWindow(48 hours);
    }

    function test_SetFactory() public {
        MockAppchainFactory newFactory = new MockAppchainFactory();
        vm.prank(admin);
        gasAggregator.setFactory(newFactory);
        assertEq(address(gasAggregator.factory()), address(newFactory));
    }

    function test_SetFactory_NonAdmin() public {
        vm.prank(user);
        vm.expectRevert();
        gasAggregator.setFactory(mockFactory);
    }

    function test_FallbackToOffchainAggregation() public {
        // Below threshold
        mockFactory.setTotalAppchains(1);
        assertFalse(gasAggregator.fallbackToOffchainAggregation());

        // At threshold (should return true since contract uses >=)
        mockFactory.setTotalAppchains(2);
        assertTrue(gasAggregator.fallbackToOffchainAggregation());

        // Above threshold
        mockFactory.setTotalAppchains(3);
        assertTrue(gasAggregator.fallbackToOffchainAggregation());
    }

    function test_AggregateTokensUsed_Success() public {
        // Setup: below threshold for automatic aggregation
        mockFactory.setTotalAppchains(1);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Set gas usage for current epoch
        uint256 currentEpoch = gasAggregator.currentEpochToAggregate();
        mockGasCounter1.setTokensForEpoch(currentEpoch, 100);
        mockGasCounter2.setTokensForEpoch(currentEpoch, 200);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        gasAggregator.aggregateTokensUsed();

        // Should increment epoch
        assertEq(gasAggregator.currentEpochToAggregate(), currentEpoch + 1);

        // Verify staking appchain data
        MockStakingAppchain stakingAppchain = MockStakingAppchain(address(gasAggregator.stakingAppchain()));
        assertEq(stakingAppchain.getChainIDs().length, 2);
        assertEq(stakingAppchain.getTokens().length, 2);
        assertEq(stakingAppchain.getChainIDs()[0], 1);
        assertEq(stakingAppchain.getChainIDs()[1], 2);
    }

    function test_AggregateTokensUsed_AboveThreshold() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        vm.expectRevert(GasAggregator.MustUseOffchainAggregation.selector);
        gasAggregator.aggregateTokensUsed();
    }

    function test_AggregateTokensUsed_EpochNotCompleted() public {
        // Try to aggregate before epoch is complete
        vm.expectRevert(
            abi.encodeWithSelector(
                GasAggregator.CurrentEpochToAggregateNotOver.selector,
                gasAggregator.currentEpochToAggregate(),
                gasAggregator.getCurrentEpoch()
            )
        );
        gasAggregator.aggregateTokensUsed();
    }

    function test_SubmitOffchainTopChains_Success() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Set gas usage
        uint256 currentEpoch = gasAggregator.currentEpochToAggregate();
        mockGasCounter1.setTokensForEpoch(currentEpoch, 100);
        mockGasCounter2.setTokensForEpoch(currentEpoch, 200);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 1;
        chainIDs[1] = 2;

        gasAggregator.submitOffchainTopChains(chainIDs);

        // Check pending data
        assertEq(gasAggregator.pendingTotalTokensUsed(), 300);
        assertEq(gasAggregator.pendingChainIDs(0), 1);
        assertEq(gasAggregator.pendingChainIDs(1), 2);
        assertEq(gasAggregator.pendingTokensUsed(0), 100);
        assertEq(gasAggregator.pendingTokensUsed(1), 200);
    }

    function test_SubmitOffchainTopChains_ChainIDsNotAscending() public {
        // Setup
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 2; // Higher first
        chainIDs[1] = 1; // Lower second

        vm.expectRevert(GasAggregator.ChainIDsMustBeInAscendingOrder.selector);
        gasAggregator.submitOffchainTopChains(chainIDs);
    }

    function test_SubmitOffchainTopChains_NotHigherThanPending() public {
        // Setup
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        uint256[] memory chainIDs = new uint256[](1);
        chainIDs[0] = 1;

        // First submission
        mockGasCounter1.setTokensForEpoch(gasAggregator.currentEpochToAggregate(), 100);
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Second submission with lower total
        mockGasCounter1.setTokensForEpoch(gasAggregator.currentEpochToAggregate(), 50);
        vm.expectRevert(abi.encodeWithSelector(GasAggregator.NotHigherThanPendingTotal.selector, 50, 100));
        gasAggregator.submitOffchainTopChains(chainIDs);
    }

    function test_SubmitOffchainTopChains_EpochNotCompleted() public {
        // Setup
        mockFactory.setTotalAppchains(3);

        uint256[] memory chainIDs = new uint256[](1);
        chainIDs[0] = 1;

        vm.expectRevert(
            abi.encodeWithSelector(
                GasAggregator.CurrentEpochToAggregateNotOver.selector,
                gasAggregator.currentEpochToAggregate(),
                gasAggregator.getCurrentEpoch()
            )
        );
        gasAggregator.submitOffchainTopChains(chainIDs);
    }

    function test_PushTopChainsDataToStakingAppchain_Success() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit data
        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 1;
        chainIDs[1] = 2;

        mockGasCounter1.setTokensForEpoch(gasAggregator.currentEpochToAggregate(), 100);
        mockGasCounter2.setTokensForEpoch(gasAggregator.currentEpochToAggregate(), 200);
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Wait for challenge window
        vm.warp(block.timestamp + CHALLENGE_WINDOW + 1);

        // Uncomment when the function is implemented
        gasAggregator.pushTopChainsDataToStakingAppchain();

        // Should increment epoch and clear pending data
        assertEq(gasAggregator.currentEpochToAggregate(), gasAggregator.getCurrentEpoch());
        assertEq(gasAggregator.pendingTotalTokensUsed(), 0);
    }

    function test_PushTopChainsDataToStakingAppchain_BelowThreshold() public {
        // Setup: below threshold
        mockFactory.setTotalAppchains(1);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        vm.expectRevert(GasAggregator.MustUseAutomaticAggregation.selector);
        gasAggregator.pushTopChainsDataToStakingAppchain();
    }

    function test_PushTopChainsDataToStakingAppchain_ChallengeWindowNotOver() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit data
        uint256[] memory chainIDs = new uint256[](1);
        chainIDs[0] = 1;

        mockGasCounter1.setTokensForEpoch(gasAggregator.currentEpochToAggregate(), 100);
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Try to push before challenge window is over
        vm.expectRevert(
            abi.encodeWithSelector(
                GasAggregator.WindowNotOver.selector, gasAggregator.currentEpochToAggregate(), CHALLENGE_WINDOW
            )
        );
        gasAggregator.pushTopChainsDataToStakingAppchain();
    }

    function test_PushTopChainsDataToStakingAppchain_NoPendingData() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Wait for challenge window
        vm.warp(block.timestamp + CHALLENGE_WINDOW + 1);

        vm.expectRevert(GasAggregator.NoPendingDataToPush.selector);
        gasAggregator.pushTopChainsDataToStakingAppchain();
    }

    function test_PushTopChainsDataToStakingAppchain_EpochNotCompleted() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);

        vm.expectRevert(
            abi.encodeWithSelector(
                GasAggregator.CurrentEpochToAggregateNotOver.selector,
                gasAggregator.currentEpochToAggregate(),
                gasAggregator.getCurrentEpoch()
            )
        );
        gasAggregator.pushTopChainsDataToStakingAppchain();
    }

    function test_Integration_CompleteWorkflow() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));
        mockFactory.addAppchain(3, address(mockGasCounter3));

        // Set initial gas usage
        uint256 epoch1 = gasAggregator.currentEpochToAggregate();
        mockGasCounter1.setTokensForEpoch(epoch1, 100);
        mockGasCounter2.setTokensForEpoch(epoch1, 200);
        mockGasCounter3.setTokensForEpoch(epoch1, 300);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit offchain data
        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 2; // 200 tokens
        chainIDs[1] = 3; // 300 tokens

        gasAggregator.submitOffchainTopChains(chainIDs);

        // Wait for challenge window
        vm.warp(block.timestamp + CHALLENGE_WINDOW + 1);

        // Push data
        gasAggregator.pushTopChainsDataToStakingAppchain();

        // Verify state
        assertEq(gasAggregator.currentEpochToAggregate(), epoch1 + 1);
        assertEq(gasAggregator.pendingTotalTokensUsed(), 0);

        // Verify staking appchain data
        MockStakingAppchain stakingAppchain = MockStakingAppchain(address(gasAggregator.stakingAppchain()));
        assertEq(stakingAppchain.getChainIDs().length, 2);
        assertEq(stakingAppchain.getTokens().length, 2);
        assertEq(stakingAppchain.getChainIDs()[0], 2);
        assertEq(stakingAppchain.getChainIDs()[1], 3);
        assertEq(stakingAppchain.getTokens()[0], 200);
        assertEq(stakingAppchain.getTokens()[1], 300);
    }

    function test_EdgeCase_ZeroGasPrice() public {
        // This test would require mocking tx.gasprice to 0
        // The contract has a workaround setting it to 1
        // This is tested in GasCounter tests
    }

    function test_EdgeCase_LargeNumberOfAppchains() public {
        // Test with maximum uint8 value
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(255);
        mockFactory.setTotalAppchains(255);

        // At threshold (should return true since contract uses >=)
        assertTrue(gasAggregator.fallbackToOffchainAggregation());

        mockFactory.setTotalAppchains(256);
        assertTrue(gasAggregator.fallbackToOffchainAggregation());
    }

    function test_EdgeCase_EmptyAppchainList() public {
        // Setup: no appchains
        mockFactory.setTotalAppchains(0);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Should work with empty arrays
        gasAggregator.aggregateTokensUsed();

        // Should increment epoch
        assertEq(gasAggregator.currentEpochToAggregate(), gasAggregator.getCurrentEpoch());
    }
}
