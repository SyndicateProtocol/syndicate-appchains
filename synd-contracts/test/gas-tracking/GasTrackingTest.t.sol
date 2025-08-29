// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GasAggregator, AppchainFactory} from "../../src/staking/GasAggregator.sol";
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

    function getTotalAppchainsForGasTracking() external view returns (uint256) {
        return totalAppchains;
    }

    function getContractsForGasTracking(uint256[] memory chainIDs) external view returns (address[] memory) {
        address[] memory contracts = new address[](chainIDs.length);
        for (uint256 i = 0; i < chainIDs.length; i++) {
            contracts[i] = appchainContracts[chainIDs[i]];
        }
        return contracts;
    }

    function getAppchainsAndContractsForGasTracking() external view returns (uint256[] memory, address[] memory) {
        address[] memory contracts = new address[](appchainChainIDs.length);
        for (uint256 i = 0; i < appchainChainIDs.length; i++) {
            contracts[i] = appchainContracts[appchainChainIDs[i]];
        }
        return (appchainChainIDs, contracts);
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
        bytes memory initData = abi.encodeWithSelector(GasAggregator.initialize.selector, mockFactory, admin, 24 hours);

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
        // Deploy using proxy pattern to test initialization validation
        ProxyAdmin proxyAdmin = new ProxyAdmin(admin);
        GasAggregator implementation = new GasAggregator();

        // Prepare initialization data with zero admin address
        bytes memory initData = abi.encodeWithSelector(
            GasAggregator.initialize.selector,
            mockFactory,
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

    function test_SealPendingEpoch_Success() public {
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

        gasAggregator.sealPendingEpoch();

        // Should increment epoch and clear pending data
        assertEq(gasAggregator.pendingEpoch(), currentEpoch + 1);
        assertEq(gasAggregator.pendingEpochFirstSubmissionTime(), 0);
        assertEq(gasAggregator.pendingDataHash(), bytes32(0));
        assertEq(gasAggregator.pendingTotalTokensUsed(), 0);
    }

    function test_SealPendingEpoch_ChallengeWindowNotOver() public {
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

        // Try to seal before challenge window is over (immediately after submission)
        vm.expectRevert(
            abi.encodeWithSelector(GasAggregator.WindowNotOver.selector, gasAggregator.pendingEpoch(), CHALLENGE_WINDOW)
        );
        gasAggregator.sealPendingEpoch();
    }

    function test_SealPendingEpoch_ValidData() public {
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

        // Should work to seal the epoch
        gasAggregator.sealPendingEpoch();

        // Verify epoch was sealed and data was stored
        uint256[] memory expectedTokens = new uint256[](2);
        expectedTokens[0] = 100;
        expectedTokens[1] = 200;
        bytes32 expectedHash = keccak256(abi.encode(chainIDs, expectedTokens));
        assertEq(gasAggregator.aggregatedEpochDataHash(currentEpoch), expectedHash);
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

        gasAggregator.sealPendingEpoch();
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

        // But seal should now work
        gasAggregator.sealPendingEpoch();

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

        // Seal epoch1 (this will store it in aggregatedEpochDataHash)
        gasAggregator.sealPendingEpoch();

        // Verify the data was stored in aggregatedEpochDataHash
        uint256[] memory tokens = new uint256[](2);
        tokens[0] = 100;
        tokens[1] = 200;
        bytes32 expectedHash = keccak256(abi.encode(chainIDs, tokens));
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch1), expectedHash);

        // Move forward some time to simulate a later point where we want to re-submit historical data
        vm.warp(block.timestamp + EPOCH_DURATION * 3 + 1);

        // Historical data can no longer be re-submitted with sealPendingEpoch
        // as it only works on the current pending epoch
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

        // Historical data can no longer be re-submitted with sealPendingEpoch
        // as it only works on the current pending epoch
    }

    function test_SubmitOffchainTopChains_CannotSubmitNextEpochUntilSealed() public {
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

        // Submit data for epoch1
        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 1;
        chainIDs[1] = 2;
        gasAggregator.submitOffchainTopChains(chainIDs);

        // The contract should still be on pendingEpoch = epoch1 until we call seal
        assertEq(gasAggregator.pendingEpoch(), epoch1);

        // Wait for challenge window to pass
        vm.warp(gasAggregator.pendingEpochFirstSubmissionTime() + CHALLENGE_WINDOW + 1);

        // Try to submit again after window has passed - should fail with WindowOver
        vm.expectRevert(abi.encodeWithSelector(GasAggregator.WindowOver.selector, epoch1, CHALLENGE_WINDOW));
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Now seal the pending epoch to allow progress
        gasAggregator.sealPendingEpoch();

        // Verify epoch progressed
        uint256 epoch2 = epoch1 + 1;
        assertEq(gasAggregator.pendingEpoch(), epoch2);

        // Set gas usage for epoch2 and move past its completion
        mockGasCounter1.setTokensForEpoch(epoch2, 150);
        mockGasCounter2.setTokensForEpoch(epoch2, 250);
        vm.warp(gasAggregator.START_TIMESTAMP() + EPOCH_DURATION * 3 + 1);

        // Now we can submit for epoch2
        gasAggregator.submitOffchainTopChains(chainIDs);
    }

    function test_SealPendingEpoch_NoSubmissionYet() public {
        // Setup: above threshold
        mockFactory.setTotalAppchains(3);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Try to seal without any submission - should fail because pendingEpochFirstSubmissionTime is 0
        vm.expectRevert(
            abi.encodeWithSelector(GasAggregator.WindowNotOver.selector, gasAggregator.pendingEpoch(), CHALLENGE_WINDOW)
        );
        gasAggregator.sealPendingEpoch();
    }
}
