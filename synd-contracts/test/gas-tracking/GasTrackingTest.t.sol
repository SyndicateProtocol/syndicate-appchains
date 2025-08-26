// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GasAggregator, AppchainFactory, StakingAppchain} from "../../src/staking/GasAggregator.sol";
import {EpochTracker} from "../../src/staking/EpochTracker.sol";
import {SyndicateFactory} from "../../src/factory/SyndicateFactory.sol";
import {SyndicateSequencingChain} from "../../src/SyndicateSequencingChain.sol";
import {AlwaysAllowedModule} from "../../src/sequencing-modules/AlwaysAllowedModule.sol";
import {RequireAndModule} from "../../src/requirement-modules/RequireAndModule.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

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
    uint256 public epoch;

    function pushData(uint256[] memory _chainIDs, uint256[] memory _tokens, uint256 _epoch) external {
        chainIDs = _chainIDs;
        tokens = _tokens;
        epoch = _epoch;
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

        // Deploy using TransparentUpgradeableProxy pattern like the deployment script

        // 1. Deploy ProxyAdmin contract
        ProxyAdmin proxyAdmin = new ProxyAdmin(admin);

        // 2. Deploy GasAggregator implementation
        GasAggregator implementation = new GasAggregator();

        // 3. Warp to exactly the epoch start timestamp (beginning of epoch 1) BEFORE proxy deployment
        vm.warp(implementation.START_TIMESTAMP());

        // 4. Prepare initialization data
        bytes memory initData = abi.encodeWithSelector(
            GasAggregator.initialize.selector, mockFactory, new MockStakingAppchain(), admin, 24 hours
        );

        // 5. Deploy TransparentUpgradeableProxy
        TransparentUpgradeableProxy proxy =
            new TransparentUpgradeableProxy(address(implementation), address(proxyAdmin), initData);

        // Cast proxy to GasAggregator interface
        gasAggregator = GasAggregator(address(proxy));

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
        assertEq(gasAggregator.pendingEpoch(), currentEpoch);
    }

    function test_Constructor_ZeroAdmin() public {
        MockStakingAppchain stakingAppchain = new MockStakingAppchain();

        // Deploy using proxy pattern to test initialization validation
        ProxyAdmin proxyAdmin = new ProxyAdmin(admin);
        GasAggregator implementation = new GasAggregator();

        // Prepare initialization data with zero admin address
        bytes memory initData = abi.encodeWithSelector(
            GasAggregator.initialize.selector,
            mockFactory,
            stakingAppchain,
            address(0), // This should trigger ZeroAddress error
            24 hours
        );

        // Expect the ZeroAddress error when deploying the proxy
        vm.expectRevert(GasAggregator.ZeroAddress.selector);
        new TransparentUpgradeableProxy(address(implementation), address(proxyAdmin), initData);
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
        uint256 epoch = 1;
        mockGasCounter1.setTokensForEpoch(epoch, 100);
        mockGasCounter2.setTokensForEpoch(epoch, 200);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        gasAggregator.aggregateTokensUsed();

        // Should increment epoch
        assertEq(gasAggregator.pendingEpoch(), epoch + 1);

        // Verify staking appchain data
        MockStakingAppchain stakingAppchain = MockStakingAppchain(address(gasAggregator.stakingAppchain()));
        assertEq(stakingAppchain.getChainIDs().length, 2);
        assertEq(stakingAppchain.getTokens().length, 2);
        assertEq(stakingAppchain.getChainIDs()[0], 1);
        assertEq(stakingAppchain.getChainIDs()[1], 2);
        assertEq(stakingAppchain.epoch(), epoch);
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
        // pendingEpoch should be the current epoch, so it's not completed yet
        vm.expectRevert(
            abi.encodeWithSelector(
                GasAggregator.EpochNotOver.selector, gasAggregator.pendingEpoch(), gasAggregator.getCurrentEpoch()
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
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(currentEpoch, 100);
        mockGasCounter2.setTokensForEpoch(currentEpoch, 200);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 1;
        chainIDs[1] = 2;

        // Record submission time to verify challenge window starts now
        uint256 submissionTime = block.timestamp;
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Verify first submission time is set
        assertEq(gasAggregator.pendingEpochFirstSubmissionTime(), submissionTime);

        // Check pending data
        assertEq(gasAggregator.pendingTotalTokensUsed(), 300);

        // Verify the hash is set
        uint256[] memory expectedChainIDs = new uint256[](2);
        expectedChainIDs[0] = 1;
        expectedChainIDs[1] = 2;
        uint256[] memory expectedTokens = new uint256[](2);
        expectedTokens[0] = 100;
        expectedTokens[1] = 200;
        bytes32 expectedHash = keccak256(abi.encode(expectedChainIDs, expectedTokens));
        assertEq(gasAggregator.pendingDataHash(), expectedHash);
    }

    function test_SubmitOffchainTopChains_ChainIDsNotAscending() public {
        // Setup
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Set gas usage for pending epoch first
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(currentEpoch, 100);
        mockGasCounter2.setTokensForEpoch(currentEpoch, 200);

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

        // Set gas usage for pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(currentEpoch, 100);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        uint256[] memory chainIDs = new uint256[](1);
        chainIDs[0] = 1;

        // First submission
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Second submission with lower total
        mockGasCounter1.setTokensForEpoch(gasAggregator.pendingEpoch(), 50);
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
                GasAggregator.EpochNotOver.selector, gasAggregator.pendingEpoch(), gasAggregator.getCurrentEpoch()
            )
        );
        gasAggregator.submitOffchainTopChains(chainIDs);
    }

    function test_PushEpochDataToStakingAppchain_Success() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Set gas usage for current pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(currentEpoch, 100);
        mockGasCounter2.setTokensForEpoch(currentEpoch, 200);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit data (this starts the challenge window)
        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 1;
        chainIDs[1] = 2;

        uint256 submissionTime = block.timestamp;
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Wait for challenge window (from submission time, not epoch end)
        vm.warp(submissionTime + CHALLENGE_WINDOW + 1);

        uint256[] memory pushChainIDs = new uint256[](2);
        pushChainIDs[0] = 1;
        pushChainIDs[1] = 2;
        uint256[] memory pushTokensUsed = new uint256[](2);
        pushTokensUsed[0] = 100;
        pushTokensUsed[1] = 200;

        gasAggregator.pushEpochDataToStakingAppchain(pushChainIDs, pushTokensUsed, currentEpoch);

        // Should increment epoch and clear pending data
        assertEq(gasAggregator.pendingEpoch(), gasAggregator.getCurrentEpoch());
    }

    function test_PushEpochDataToStakingAppchain_ChallengeWindowNotOver() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));

        // Set gas usage for current pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(currentEpoch, 100);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit data (this starts the challenge window)
        uint256[] memory chainIDs = new uint256[](1);
        chainIDs[0] = 1;
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Try to push before challenge window is over (immediately after submission)
        uint256[] memory dummyChainIDs = new uint256[](1);
        dummyChainIDs[0] = 1;
        uint256[] memory dummyTokens = new uint256[](1);
        dummyTokens[0] = 100;
        vm.expectRevert(
            abi.encodeWithSelector(GasAggregator.WindowNotOver.selector, gasAggregator.pendingEpoch(), CHALLENGE_WINDOW)
        );
        gasAggregator.pushEpochDataToStakingAppchain(dummyChainIDs, dummyTokens, currentEpoch);
    }

    function test_PushEpochDataToStakingAppchain_InvalidHash() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Set gas usage for current pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(currentEpoch, 100);
        mockGasCounter2.setTokensForEpoch(currentEpoch, 200);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit data
        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 1;
        chainIDs[1] = 2;
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Wait for challenge window
        vm.warp(block.timestamp + CHALLENGE_WINDOW + 1);

        // Try to push with wrong data (different chain IDs)
        uint256[] memory wrongChainIDs = new uint256[](2);
        wrongChainIDs[0] = 2;
        wrongChainIDs[1] = 1;
        uint256[] memory tokensUsed = new uint256[](2);
        tokensUsed[0] = 100;
        tokensUsed[1] = 200;

        vm.expectRevert(GasAggregator.InvalidDataHash.selector);
        gasAggregator.pushEpochDataToStakingAppchain(wrongChainIDs, tokensUsed, currentEpoch);

        // Try to push with wrong tokens
        uint256[] memory correctChainIDs = new uint256[](2);
        correctChainIDs[0] = 1;
        correctChainIDs[1] = 2;
        uint256[] memory wrongTokens = new uint256[](2);
        wrongTokens[0] = 999;
        wrongTokens[1] = 888;

        vm.expectRevert(GasAggregator.InvalidDataHash.selector);
        gasAggregator.pushEpochDataToStakingAppchain(correctChainIDs, wrongTokens, currentEpoch);

        // Should work with correct data
        uint256[] memory correctTokens = new uint256[](2);
        correctTokens[0] = 100;
        correctTokens[1] = 200;
        gasAggregator.pushEpochDataToStakingAppchain(correctChainIDs, correctTokens, currentEpoch);
    }

    function test_Integration_CompleteWorkflow() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));
        mockFactory.addAppchain(3, address(mockGasCounter3));

        // Set initial gas usage
        uint256 epoch1 = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(epoch1, 100);
        mockGasCounter2.setTokensForEpoch(epoch1, 200);
        mockGasCounter3.setTokensForEpoch(epoch1, 300);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit offchain data (this starts challenge window)
        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 2; // 200 tokens
        chainIDs[1] = 3; // 300 tokens

        uint256 submissionTime = block.timestamp;
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Wait for challenge window (from submission time)
        vm.warp(submissionTime + CHALLENGE_WINDOW + 1);

        // Push data
        uint256[] memory pushChainIDs = new uint256[](2);
        pushChainIDs[0] = 2;
        pushChainIDs[1] = 3;
        uint256[] memory pushTokens = new uint256[](2);
        pushTokens[0] = 200;
        pushTokens[1] = 300;

        gasAggregator.pushEpochDataToStakingAppchain(pushChainIDs, pushTokens, epoch1);

        // Verify staking appchain data
        MockStakingAppchain stakingAppchain = MockStakingAppchain(address(gasAggregator.stakingAppchain()));
        assertEq(stakingAppchain.getChainIDs().length, 2);
        assertEq(stakingAppchain.getTokens().length, 2);
        assertEq(stakingAppchain.getChainIDs()[0], 2);
        assertEq(stakingAppchain.getChainIDs()[1], 3);
        assertEq(stakingAppchain.getTokens()[0], 200);
        assertEq(stakingAppchain.getTokens()[1], 300);
        assertEq(stakingAppchain.epoch(), epoch1);
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
        assertEq(gasAggregator.pendingEpoch(), gasAggregator.getCurrentEpoch());
    }

    function test_ChallengeWindowMechanism() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Set gas usage for current pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(currentEpoch, 100);
        mockGasCounter2.setTokensForEpoch(currentEpoch, 200);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Expected first submission time after the warp
        uint256 expectedFirstSubmissionTime = gasAggregator.START_TIMESTAMP() + EPOCH_DURATION + 1;

        uint256[] memory chainIDs1 = new uint256[](1);
        chainIDs1[0] = 1;

        uint256[] memory chainIDs2 = new uint256[](2);
        chainIDs2[0] = 1;
        chainIDs2[1] = 2;

        // First submission should work (starts challenge window)
        gasAggregator.submitOffchainTopChains(chainIDs1);
        assertEq(gasAggregator.pendingEpochFirstSubmissionTime(), expectedFirstSubmissionTime);
        assertEq(gasAggregator.pendingTotalTokensUsed(), 100);

        // Second submission during challenge window should work if higher total
        vm.warp(expectedFirstSubmissionTime + CHALLENGE_WINDOW / 2);
        gasAggregator.submitOffchainTopChains(chainIDs2);
        assertEq(gasAggregator.pendingTotalTokensUsed(), 300);
        // First submission time should not change (it records the FIRST submission)
        assertEq(gasAggregator.pendingEpochFirstSubmissionTime(), expectedFirstSubmissionTime);

        // Third submission after challenge window should fail
        vm.warp(expectedFirstSubmissionTime + CHALLENGE_WINDOW + 1);
        vm.expectRevert(
            abi.encodeWithSelector(GasAggregator.WindowOver.selector, gasAggregator.pendingEpoch(), CHALLENGE_WINDOW)
        );
        gasAggregator.submitOffchainTopChains(chainIDs1);

        // But push should now work
        uint256[] memory finalChainIDs = new uint256[](2);
        finalChainIDs[0] = 1;
        finalChainIDs[1] = 2;
        uint256[] memory finalTokens = new uint256[](2);
        finalTokens[0] = 100;
        finalTokens[1] = 200;

        gasAggregator.pushEpochDataToStakingAppchain(finalChainIDs, finalTokens, currentEpoch);

        // Verify data was pushed
        MockStakingAppchain stakingAppchain = MockStakingAppchain(address(gasAggregator.stakingAppchain()));
        assertEq(stakingAppchain.getChainIDs().length, 2);
        assertEq(stakingAppchain.getTokens()[0], 100);
        assertEq(stakingAppchain.getTokens()[1], 200);
        assertEq(stakingAppchain.epoch(), currentEpoch);

        // Epoch should be incremented and submission time reset
        assertEq(gasAggregator.pendingEpoch(), currentEpoch + 1);
        assertEq(gasAggregator.pendingEpochFirstSubmissionTime(), 0);
    }

    function test_ResubmissionOfHistoricalData() public {
        // Setup: above threshold for offchain aggregation
        mockFactory.setTotalAppchains(3);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Set gas usage for epoch 1
        uint256 epoch1 = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(epoch1, 100);
        mockGasCounter2.setTokensForEpoch(epoch1, 200);

        // Move to next epoch so epoch1 is completed
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Submit offchain data for epoch1
        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 1;
        chainIDs[1] = 2;
        uint256 submissionTime = block.timestamp;
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Wait for challenge window to pass
        vm.warp(submissionTime + CHALLENGE_WINDOW + 1);

        // Push data for epoch1 (this will store it in aggregatedEpochDataHash)
        uint256[] memory tokens = new uint256[](2);
        tokens[0] = 100;
        tokens[1] = 200;
        gasAggregator.pushEpochDataToStakingAppchain(chainIDs, tokens, epoch1);

        // Verify the data was stored in aggregatedEpochDataHash
        bytes32 expectedHash = keccak256(abi.encode(chainIDs, tokens));
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch1), expectedHash);

        // Move forward some time to simulate a later point where we want to re-submit historical data
        vm.warp(block.timestamp + EPOCH_DURATION * 3 + 1);

        // Now test re-submission of historical data (epoch1)
        // This should succeed since the data matches the stored hash
        gasAggregator.pushEpochDataToStakingAppchain(chainIDs, tokens, epoch1);

        // Verify the staking appchain received the re-submitted data
        MockStakingAppchain stakingAppchain = MockStakingAppchain(address(gasAggregator.stakingAppchain()));
        assertEq(stakingAppchain.epoch(), epoch1); // Should show epoch1 as the last pushed epoch
        assertEq(stakingAppchain.getChainIDs()[0], 1);
        assertEq(stakingAppchain.getChainIDs()[1], 2);
        assertEq(stakingAppchain.getTokens()[0], 100);
        assertEq(stakingAppchain.getTokens()[1], 200);

        // Test re-submission with wrong data should fail
        uint256[] memory wrongTokens = new uint256[](2);
        wrongTokens[0] = 999;
        wrongTokens[1] = 888;
        vm.expectRevert(GasAggregator.InvalidDataHash.selector);
        gasAggregator.pushEpochDataToStakingAppchain(chainIDs, wrongTokens, epoch1);

        // Test re-submission with wrong chainIDs should fail
        uint256[] memory wrongChainIDs = new uint256[](2);
        wrongChainIDs[0] = 2;
        wrongChainIDs[1] = 1;
        vm.expectRevert(GasAggregator.InvalidDataHash.selector);
        gasAggregator.pushEpochDataToStakingAppchain(wrongChainIDs, tokens, epoch1);

        // Test re-submission of non-existent historical epoch should fail
        // Use epoch1 itself but with data that doesn't match the stored hash
        uint256[] memory someTokens = new uint256[](1);
        someTokens[0] = 50;
        uint256[] memory someChainIDs = new uint256[](1);
        someChainIDs[0] = 1;
        vm.expectRevert(GasAggregator.InvalidDataHash.selector);
        gasAggregator.pushEpochDataToStakingAppchain(someChainIDs, someTokens, epoch1); // Different data structure
    }

    function test_ResubmissionOfAutomaticAggregationData() public {
        // Setup: below threshold for automatic aggregation
        mockFactory.setTotalAppchains(1);
        mockFactory.addAppchain(1, address(mockGasCounter1));
        mockFactory.addAppchain(2, address(mockGasCounter2));

        // Set gas usage for epoch 1
        uint256 epoch1 = gasAggregator.pendingEpoch();
        mockGasCounter1.setTokensForEpoch(epoch1, 100);
        mockGasCounter2.setTokensForEpoch(epoch1, 200);

        // Move to next epoch so epoch1 is completed
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Use automatic aggregation for epoch1
        gasAggregator.aggregateTokensUsed();

        // Verify the data was stored in aggregatedEpochDataHash
        uint256[] memory expectedChainIDs = new uint256[](2);
        expectedChainIDs[0] = 1;
        expectedChainIDs[1] = 2;
        uint256[] memory expectedTokens = new uint256[](2);
        expectedTokens[0] = 100;
        expectedTokens[1] = 200;
        bytes32 expectedHash = keccak256(abi.encode(expectedChainIDs, expectedTokens));
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch1), expectedHash);

        // Move forward in time to simulate a later point where we want to re-submit historical data
        vm.warp(block.timestamp + EPOCH_DURATION * 3 + 1);

        // Now test re-submission of historical data that was originally aggregated automatically
        // This should succeed since the data matches the stored hash
        gasAggregator.pushEpochDataToStakingAppchain(expectedChainIDs, expectedTokens, epoch1);

        // Verify the staking appchain received the re-submitted data
        MockStakingAppchain stakingAppchain = MockStakingAppchain(address(gasAggregator.stakingAppchain()));
        assertEq(stakingAppchain.epoch(), epoch1); // Should show epoch1 as the last pushed epoch
        assertEq(stakingAppchain.getChainIDs().length, 2);
        assertEq(stakingAppchain.getChainIDs()[0], 1);
        assertEq(stakingAppchain.getChainIDs()[1], 2);
        assertEq(stakingAppchain.getTokens()[0], 100);
        assertEq(stakingAppchain.getTokens()[1], 200);

        // Test re-submission with wrong data should fail
        uint256[] memory wrongTokens = new uint256[](2);
        wrongTokens[0] = 999;
        wrongTokens[1] = 888;
        vm.expectRevert(GasAggregator.InvalidDataHash.selector);
        gasAggregator.pushEpochDataToStakingAppchain(expectedChainIDs, wrongTokens, epoch1);

        // Test re-submission with different chain order should fail
        uint256[] memory wrongOrderChainIDs = new uint256[](2);
        wrongOrderChainIDs[0] = 2;
        wrongOrderChainIDs[1] = 1;
        vm.expectRevert(GasAggregator.InvalidDataHash.selector);
        gasAggregator.pushEpochDataToStakingAppchain(wrongOrderChainIDs, expectedTokens, epoch1);

        // Test that we can still re-submit with correct data multiple times
        gasAggregator.pushEpochDataToStakingAppchain(expectedChainIDs, expectedTokens, epoch1);
        assertEq(stakingAppchain.epoch(), epoch1); // Still should show epoch1
    }
}
