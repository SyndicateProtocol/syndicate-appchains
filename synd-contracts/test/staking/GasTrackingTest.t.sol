// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GasAggregator, AppchainFactory} from "../../src/staking/GasAggregator.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
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
    address public emissionsReceiver;
    address public implementation;

    constructor(address _implementation) {
        implementation = _implementation;
    }

    function getTokensForEpoch(uint256 epoch) external view returns (uint256) {
        return tokensUsedPerEpoch[epoch];
    }

    function setTokensForEpoch(uint256 epoch, uint256 tokens) external {
        tokensUsedPerEpoch[epoch] = tokens;
    }

    function getEmissionsReceiver() external view returns (address) {
        return emissionsReceiver;
    }

    function setEmissionsReceiver(address _emissionsReceiver) external {
        emissionsReceiver = _emissionsReceiver;
    }
}

contract MockSequencingChain {
    address public implementation;

    constructor(address _implementation) {
        implementation = _implementation;
    }

    function setImplementation(address _implementation) external {
        implementation = _implementation;
    }
}

contract SimpleMockProxy {
    address public implementation;
    mapping(uint256 => uint256) public tokensForEpoch;
    address public emissionsReceiver;

    constructor(address _implementation) {
        implementation = _implementation;
        emissionsReceiver = address(0x2001); // Default emissions receiver
    }

    function getTokensForEpoch(uint256 epoch) external view returns (uint256) {
        return tokensForEpoch[epoch];
    }

    function getEmissionsReceiver() external view returns (address) {
        return emissionsReceiver;
    }

    function setTokensForEpoch(uint256 epoch, uint256 tokens) external {
        tokensForEpoch[epoch] = tokens;
    }

    function setEmissionsReceiver(address receiver) external {
        emissionsReceiver = receiver;
    }
}

contract MockAppchainFactory is AppchainFactory {
    mapping(uint256 => address) public appchainContracts;
    mapping(address => bool) public isImplementationAllowed;
    mapping(address => address) public mockImplementations; // Mock implementation mapping
    bytes public proxyBytecode = abi.encodePacked("mock proxy bytecode");

    function addAppchain(uint256 chainId, address contractAddr) external {
        appchainContracts[chainId] = contractAddr;
    }

    function setImplementationAllowed(address impl, bool allowed) external {
        isImplementationAllowed[impl] = allowed;
    }

    function setMockImplementation(address contractAddr, address impl) external {
        mockImplementations[contractAddr] = impl;
    }

    function computeSequencingChainAddress(uint256 chainId) external view returns (address) {
        // For testing, compute deterministic address like the real factory
        return Create2.computeAddress(bytes32(chainId), keccak256(proxyBytecode), address(this));
    }

    function getProxyBytecode() external view returns (bytes memory) {
        return proxyBytecode;
    }
}

contract GasAggregatorTest is Test {
    GasAggregator public gasAggregator;
    MockAppchainFactory public mockFactory;
    MockGasCounter public mockGasCounter1;
    MockGasCounter public mockGasCounter2;
    MockGasCounter public mockGasCounter3;
    MockSequencingChain public mockSequencingChain1;
    MockSequencingChain public mockSequencingChain2;
    MockSequencingChain public mockSequencingChain3;

    address public admin = address(0x1);
    address public user = address(0x2);

    address public allowedImpl = address(0x100);
    address public disallowedImpl = address(0x200);

    uint256 public constant CHAIN_ID_1 = 1001;
    uint256 public constant CHAIN_ID_2 = 1002;
    uint256 public constant CHAIN_ID_3 = 1003;

    uint256 public constant EPOCH_DURATION = 30 days;
    uint256 public constant CHALLENGE_WINDOW = 24 hours;
    uint256 public constant ADD_CHAIN_FEE = 0.1 ether;

    event TopChainsDataSubmitted(uint256[] appchainIDs, uint256[] tokens, uint256 total);

    function setUp() public {
        mockFactory = new MockAppchainFactory();
        mockGasCounter1 = new MockGasCounter(allowedImpl);
        mockGasCounter2 = new MockGasCounter(allowedImpl);
        mockGasCounter3 = new MockGasCounter(allowedImpl);

        // Create mock sequencing chains
        mockSequencingChain1 = new MockSequencingChain(allowedImpl);
        mockSequencingChain2 = new MockSequencingChain(allowedImpl);
        mockSequencingChain3 = new MockSequencingChain(disallowedImpl);

        // Set up factory state
        mockFactory.setImplementationAllowed(allowedImpl, true);
        mockFactory.setImplementationAllowed(disallowedImpl, false);

        // Deploy mock contracts at the computed deterministic addresses
        _deployMockAtComputedAddress(CHAIN_ID_1, allowedImpl);
        _deployMockAtComputedAddress(CHAIN_ID_2, allowedImpl);
        _deployMockAtComputedAddress(CHAIN_ID_3, disallowedImpl);

        // Add common test chain IDs that many tests use
        // Deploy mock contracts at computed addresses for chain IDs 1, 2, 3
        _deployMockAtComputedAddress(1, allowedImpl);
        _deployMockAtComputedAddress(2, allowedImpl);
        _deployMockAtComputedAddress(3, allowedImpl);

        // Deploy using TransparentUpgradeableProxy pattern like the deployment script

        // 1. Deploy ProxyAdmin contract
        ProxyAdmin proxyAdmin = new ProxyAdmin(admin);

        // 2. Deploy GasAggregator implementation
        GasAggregator implementation = new GasAggregator();

        // 3. Warp to exactly the epoch start timestamp (beginning of epoch 1) BEFORE proxy deployment
        vm.warp(implementation.START_TIMESTAMP());

        // 4. Prepare initialization data
        bytes memory initData =
            abi.encodeWithSelector(GasAggregator.initialize.selector, mockFactory, admin, 24 hours, ADD_CHAIN_FEE);

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

        // Add chains to the aggregator registry for tests that need them
        vm.deal(user, 10 ether); // Give user enough ETH for multiple chain additions
    }

    function test_Constructor() public view {
        assertEq(address(gasAggregator.factory()), address(mockFactory));
        assertTrue(gasAggregator.hasRole(gasAggregator.DEFAULT_ADMIN_ROLE(), admin));
        assertEq(gasAggregator.addChainFee(), ADD_CHAIN_FEE);

        // Should start with current epoch
        uint256 currentEpoch = gasAggregator.getCurrentEpoch();
        assertEq(gasAggregator.pendingEpoch(), currentEpoch);

        // Should start with no tracked chains
        assertEq(gasAggregator.getTotalTrackedChains(), 0);
    }

    // =============================================================================
    // CHAIN REGISTRY TESTS
    // =============================================================================

    function test_AddChain_Success() public {
        // Add a chain with allowed implementation
        vm.deal(user, ADD_CHAIN_FEE);

        // Compute the expected deterministic address
        address expectedChainAddress = mockFactory.computeSequencingChainAddress(CHAIN_ID_1);

        vm.expectEmit(true, true, true, false);
        emit GasAggregator.ChainAdded(CHAIN_ID_1, expectedChainAddress, user);

        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_1);

        // Verify chain was added
        assertTrue(gasAggregator.isChainTracked(CHAIN_ID_1));
        assertEq(gasAggregator.getTotalTrackedChains(), 1);

        uint256[] memory trackedChains = gasAggregator.getTrackedChainIds();
        assertEq(trackedChains.length, 1);
        assertEq(trackedChains[0], CHAIN_ID_1);
    }

    function test_AddChain_InsufficientFee() public {
        vm.deal(user, ADD_CHAIN_FEE - 1);

        vm.expectRevert(
            abi.encodeWithSelector(GasAggregator.InsufficientFee.selector, ADD_CHAIN_FEE, ADD_CHAIN_FEE - 1)
        );

        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE - 1}(CHAIN_ID_1);
    }

    function test_AddChain_AlreadyTracked() public {
        // Add chain first
        vm.deal(user, ADD_CHAIN_FEE);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_1);

        // Try to add again
        vm.deal(user, ADD_CHAIN_FEE);
        vm.expectRevert(abi.encodeWithSelector(GasAggregator.ChainAlreadyTracked.selector, CHAIN_ID_1));

        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_1);
    }

    function test_AddChain_ChainNotFound() public {
        uint256 nonExistentChainId = 9999;
        vm.deal(user, ADD_CHAIN_FEE);

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.ChainNotFound.selector, nonExistentChainId));

        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(nonExistentChainId);
    }

    function test_AddChain_InvalidImplementation() public {
        // Try to add chain with disallowed implementation
        vm.deal(user, ADD_CHAIN_FEE);

        vm.expectRevert(abi.encodeWithSelector(GasAggregator.InvalidImplementation.selector, disallowedImpl));

        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_3);
    }

    function test_RemoveChain_Success() public {
        // Add a chain first
        vm.deal(user, ADD_CHAIN_FEE);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_1);

        // Remove the chain
        vm.expectEmit(true, true, false, false);
        emit GasAggregator.ChainRemoved(CHAIN_ID_1);

        vm.prank(admin);
        gasAggregator.removeChain(CHAIN_ID_1);

        // Verify chain was removed
        assertFalse(gasAggregator.isChainTracked(CHAIN_ID_1));
        assertEq(gasAggregator.getTotalTrackedChains(), 0);
    }

    function test_RemoveChain_NotTracked() public {
        vm.expectRevert(abi.encodeWithSelector(GasAggregator.ChainNotTracked.selector, CHAIN_ID_1));

        vm.prank(admin);
        gasAggregator.removeChain(CHAIN_ID_1);
    }

    function test_RemoveChain_OnlyAdmin() public {
        // Add a chain first
        vm.deal(user, ADD_CHAIN_FEE);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_1);

        // Try to remove as non-admin
        vm.prank(user);
        vm.expectRevert();
        gasAggregator.removeChain(CHAIN_ID_1);
    }

    function test_SetAddChainFee() public {
        uint256 newFee = 0.2 ether;

        vm.expectEmit(true, true, false, false);
        emit GasAggregator.AddChainFeeUpdated(ADD_CHAIN_FEE, newFee);

        vm.prank(admin);
        gasAggregator.setAddChainFee(newFee);

        assertEq(gasAggregator.addChainFee(), newFee);
    }

    function test_WithdrawFees() public {
        // Add some chains to collect fees
        vm.deal(user, ADD_CHAIN_FEE * 2);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_2);

        assertEq(gasAggregator.getBalance(), ADD_CHAIN_FEE * 2);

        address payable recipient = payable(address(0x999));
        uint256 initialBalance = recipient.balance;

        vm.prank(admin);
        gasAggregator.withdrawFees(recipient, ADD_CHAIN_FEE);

        assertEq(recipient.balance, initialBalance + ADD_CHAIN_FEE);
        assertEq(gasAggregator.getBalance(), ADD_CHAIN_FEE);
    }

    function test_WithdrawFees_All() public {
        // Add some chains to collect fees
        vm.deal(user, ADD_CHAIN_FEE * 2);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_1);
        vm.prank(user);
        gasAggregator.addChain{value: ADD_CHAIN_FEE}(CHAIN_ID_2);

        address payable recipient = payable(address(0x999));
        uint256 initialBalance = recipient.balance;

        vm.prank(admin);
        gasAggregator.withdrawFees(recipient, 0); // 0 means withdraw all

        assertEq(recipient.balance, initialBalance + ADD_CHAIN_FEE * 2);
        assertEq(gasAggregator.getBalance(), 0);
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
            24 hours,
            ADD_CHAIN_FEE
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
        // Set threshold to 2
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Below threshold (1 chain)
        vm.deal(user, 1 ether);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        assertFalse(gasAggregator.fallbackToOffchainAggregation());

        // At threshold (2 chains - should return true since contract uses >=)
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        assertTrue(gasAggregator.fallbackToOffchainAggregation());

        // Above threshold (3 chains)
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);
        assertTrue(gasAggregator.fallbackToOffchainAggregation());
    }

    function test_AggregateTokensUsed_MultipleInvalidChains() public {
        // Test with multiple invalid chains to ensure no skipping occurs
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(10);

        vm.deal(user, 10 ether);

        // Add chains: valid(1), invalid(2), valid(3), invalid(4), valid(5)
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);

        // Add invalid chain 2
        mockFactory.setImplementationAllowed(disallowedImpl, true);
        _deployMockAtComputedAddress(2, disallowedImpl);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        mockFactory.setImplementationAllowed(disallowedImpl, false);

        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Add another invalid chain 4
        mockFactory.setImplementationAllowed(disallowedImpl, true);
        _deployMockAtComputedAddress(4, disallowedImpl);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(4);
        mockFactory.setImplementationAllowed(disallowedImpl, false);

        _deployMockAtComputedAddress(5, allowedImpl);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(5);

        // Set up data for all chains
        for (uint256 i = 1; i <= 5; i++) {
            address chainContract = mockFactory.computeSequencingChainAddress(i);
            SimpleMockProxy(chainContract).setEmissionsReceiver(address(uint160(0x2000 + i)));
            SimpleMockProxy(chainContract).setTokensForEpoch(1, i * 100);
        }

        // Verify we start with 5 chains
        assertEq(gasAggregator.getTotalTrackedChains(), 5);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Should succeed and remove chains 2 and 4
        gasAggregator.aggregateTokensUsed();

        // Should only have valid chains 1, 3, 5 remaining
        assertEq(gasAggregator.getTotalTrackedChains(), 3);
        assertTrue(gasAggregator.isChainTracked(1));
        assertFalse(gasAggregator.isChainTracked(2)); // removed
        assertTrue(gasAggregator.isChainTracked(3));
        assertFalse(gasAggregator.isChainTracked(4)); // removed
        assertTrue(gasAggregator.isChainTracked(5));
    }

    function test_AggregateTokensUsed_ResilientToInvalidChains() public {
        // Setup: Add chains with mixed valid/invalid implementations
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(10); // Set high threshold

        vm.deal(user, 2 ether);

        // Add valid chains
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);

        // Add an invalid chain (chain 3 has disallowed implementation)
        // First make the implementation allowed temporarily to add it
        mockFactory.setImplementationAllowed(disallowedImpl, true);
        _deployMockAtComputedAddress(999, disallowedImpl);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(999);

        // Now make it disallowed again to simulate an upgrade that invalidated it
        mockFactory.setImplementationAllowed(disallowedImpl, false);

        // Set up data for valid chains
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);
        address chain999Contract = mockFactory.computeSequencingChainAddress(999);

        SimpleMockProxy(chain1Contract).setEmissionsReceiver(address(0x2001));
        SimpleMockProxy(chain2Contract).setEmissionsReceiver(address(0x2002));
        SimpleMockProxy(chain999Contract).setEmissionsReceiver(address(0x2999));

        uint256 epoch = 1;
        SimpleMockProxy(chain1Contract).setTokensForEpoch(epoch, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(epoch, 200);
        SimpleMockProxy(chain999Contract).setTokensForEpoch(epoch, 500);

        // Verify we have 3 chains tracked initially
        assertEq(gasAggregator.getTotalTrackedChains(), 3);
        assertTrue(gasAggregator.isChainTracked(1));
        assertTrue(gasAggregator.isChainTracked(2));
        assertTrue(gasAggregator.isChainTracked(999));

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Expect ChainRemoved event for the invalid chain
        vm.expectEmit(true, true, false, false);
        emit GasAggregator.ChainRemoved(999);

        // This should succeed, removing the invalid chain and continuing with valid ones
        gasAggregator.aggregateTokensUsed();

        // Verify the invalid chain was removed
        assertEq(gasAggregator.getTotalTrackedChains(), 2);
        assertTrue(gasAggregator.isChainTracked(1));
        assertTrue(gasAggregator.isChainTracked(2));
        assertFalse(gasAggregator.isChainTracked(999)); // Should be removed

        // Should increment epoch
        assertEq(gasAggregator.pendingEpoch(), epoch + 1);
    }

    function test_AggregateTokensUsed_Success() public {
        // Setup: Add chains to aggregator registry (below threshold for automatic aggregation)
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(5); // Set high threshold

        vm.deal(user, 1 ether);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);

        // Set emissions receivers and gas usage on the deployed contracts
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);

        SimpleMockProxy(chain1Contract).setEmissionsReceiver(address(0x2001));
        SimpleMockProxy(chain2Contract).setEmissionsReceiver(address(0x2002));

        // Set gas usage for current epoch
        uint256 epoch = 1;
        SimpleMockProxy(chain1Contract).setTokensForEpoch(epoch, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(epoch, 200);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        gasAggregator.aggregateTokensUsed();

        // Should increment epoch
        assertEq(gasAggregator.pendingEpoch(), epoch + 1);
    }

    function test_AggregateTokensUsed_AboveThreshold() public {
        // Setup: Add chains above threshold
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        vm.deal(user, 1 ether);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

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
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set emissions receivers and gas usage on the deployed contracts
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);

        SimpleMockProxy(chain1Contract).setEmissionsReceiver(address(0x1001));
        SimpleMockProxy(chain2Contract).setEmissionsReceiver(address(0x1002));

        // Set gas usage
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        SimpleMockProxy(chain1Contract).setTokensForEpoch(currentEpoch, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(currentEpoch, 200);

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
        address[] memory expectedEmissionsReceivers = new address[](2);
        expectedEmissionsReceivers[0] = address(0x1001);
        expectedEmissionsReceivers[1] = address(0x1002);
        bytes32 expectedHash = keccak256(abi.encode(expectedChainIDs, expectedTokens, expectedEmissionsReceivers));
        assertEq(gasAggregator.pendingDataHash(), expectedHash);
    }

    function test_SubmitOffchainTopChains_ChainIDsNotAscending() public {
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set gas usage for pending epoch first
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);
        SimpleMockProxy(chain1Contract).setTokensForEpoch(currentEpoch, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(currentEpoch, 200);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        uint256[] memory chainIDs = new uint256[](2);
        chainIDs[0] = 2; // Higher first
        chainIDs[1] = 1; // Lower second

        vm.expectRevert(GasAggregator.ChainIDsMustBeInAscendingOrder.selector);
        gasAggregator.submitOffchainTopChains(chainIDs);
    }

    function test_SubmitOffchainTopChains_NotHigherThanPending() public {
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set gas usage for pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        SimpleMockProxy(chain1Contract).setTokensForEpoch(currentEpoch, 100);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        uint256[] memory chainIDs = new uint256[](1);
        chainIDs[0] = 1;

        // First submission
        gasAggregator.submitOffchainTopChains(chainIDs);

        // Second submission with lower total
        SimpleMockProxy(chain1Contract).setTokensForEpoch(gasAggregator.pendingEpoch(), 50);
        vm.expectRevert(abi.encodeWithSelector(GasAggregator.NotHigherThanPendingTotal.selector, 50, 100));
        gasAggregator.submitOffchainTopChains(chainIDs);
    }

    function test_SubmitOffchainTopChains_EpochNotCompleted() public {
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

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
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set gas usage for current pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);
        SimpleMockProxy(chain1Contract).setTokensForEpoch(currentEpoch, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(currentEpoch, 200);

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
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set gas usage for current pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        SimpleMockProxy(chain1Contract).setTokensForEpoch(currentEpoch, 100);

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
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set gas usage for current pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);
        SimpleMockProxy(chain1Contract).setTokensForEpoch(currentEpoch, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(currentEpoch, 200);

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
        address[] memory expectedEmissionsReceivers = new address[](2);
        expectedEmissionsReceivers[0] = address(0);
        expectedEmissionsReceivers[1] = address(0);
        bytes32 expectedHash = keccak256(abi.encode(chainIDs, expectedTokens, expectedEmissionsReceivers));
        assertEq(gasAggregator.aggregatedEpochDataHash(currentEpoch), expectedHash);
    }

    function test_Integration_CompleteWorkflow() public {
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set initial gas usage
        uint256 epoch1 = gasAggregator.pendingEpoch();
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);
        address chain3Contract = mockFactory.computeSequencingChainAddress(3);
        SimpleMockProxy(chain1Contract).setTokensForEpoch(epoch1, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(epoch1, 200);
        SimpleMockProxy(chain3Contract).setTokensForEpoch(epoch1, 300);

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
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to reach and exceed the threshold
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);

        // At threshold (should return true since contract uses >=)
        assertTrue(gasAggregator.fallbackToOffchainAggregation());

        // Add one more chain to go above threshold
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);
        assertTrue(gasAggregator.fallbackToOffchainAggregation());
    }

    function test_EdgeCase_EmptyAppchainList() public {
        // Setup: no appchains - don't add any chains to the registry
        // The aggregator starts with 0 tracked chains by default

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Should work with empty arrays
        gasAggregator.aggregateTokensUsed();

        // Should increment epoch
        assertEq(gasAggregator.pendingEpoch(), gasAggregator.getCurrentEpoch());
    }

    function test_ChallengeWindowMechanism() public {
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set gas usage for current pending epoch
        uint256 currentEpoch = gasAggregator.pendingEpoch();
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);
        SimpleMockProxy(chain1Contract).setTokensForEpoch(currentEpoch, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(currentEpoch, 200);

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
        // Setup: above threshold for offchain aggregation - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set gas usage for epoch 1
        uint256 epoch1 = gasAggregator.pendingEpoch();
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);
        SimpleMockProxy(chain1Contract).setTokensForEpoch(epoch1, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(epoch1, 200);

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
        address[] memory expectedEmissionsReceivers = new address[](2);
        expectedEmissionsReceivers[0] = address(0);
        expectedEmissionsReceivers[1] = address(0);
        bytes32 expectedHash = keccak256(abi.encode(chainIDs, tokens, expectedEmissionsReceivers));
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch1), expectedHash);

        // Move forward some time to simulate a later point where we want to re-submit historical data
        vm.warp(block.timestamp + EPOCH_DURATION * 3 + 1);

        // Historical data can no longer be re-submitted with sealPendingEpoch
        // as it only works on the current pending epoch
    }

    function test_ResubmissionOfAutomaticAggregationData() public {
        // Setup: below threshold for automatic aggregation - set high threshold with few chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(5);

        // Add only 2 chains to stay below threshold
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);

        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);
        SimpleMockProxy(chain1Contract).setEmissionsReceiver(address(0x3001));
        SimpleMockProxy(chain2Contract).setEmissionsReceiver(address(0x3002));

        // Set gas usage for epoch 1
        uint256 epoch1 = gasAggregator.pendingEpoch();
        SimpleMockProxy(chain1Contract).setTokensForEpoch(epoch1, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(epoch1, 200);

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
        address[] memory expectedEmissionsReceivers = new address[](2);
        expectedEmissionsReceivers[0] = address(0x3001);
        expectedEmissionsReceivers[1] = address(0x3002);
        bytes32 expectedHash = keccak256(abi.encode(expectedChainIDs, expectedTokens, expectedEmissionsReceivers));
        assertEq(gasAggregator.aggregatedEpochDataHash(epoch1), expectedHash);

        // Move forward in time to simulate a later point where we want to re-submit historical data
        vm.warp(block.timestamp + EPOCH_DURATION * 3 + 1);

        // Historical data can no longer be re-submitted with sealPendingEpoch
        // as it only works on the current pending epoch
    }

    function test_SubmitOffchainTopChains_CannotSubmitNextEpochUntilSealed() public {
        // Setup: above threshold for offchain aggregation - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Set gas usage for epoch 1
        uint256 epoch1 = gasAggregator.pendingEpoch();
        address chain1Contract = mockFactory.computeSequencingChainAddress(1);
        address chain2Contract = mockFactory.computeSequencingChainAddress(2);
        SimpleMockProxy(chain1Contract).setTokensForEpoch(epoch1, 100);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(epoch1, 200);

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
        SimpleMockProxy(chain1Contract).setTokensForEpoch(epoch2, 150);
        SimpleMockProxy(chain2Contract).setTokensForEpoch(epoch2, 250);
        vm.warp(gasAggregator.START_TIMESTAMP() + EPOCH_DURATION * 3 + 1);

        // Now we can submit for epoch2
        gasAggregator.submitOffchainTopChains(chainIDs);
    }

    function test_SealPendingEpoch_NoSubmissionYet() public {
        // Setup: above threshold - set low threshold and add more chains
        vm.prank(admin);
        gasAggregator.setMaxAppchainsToQuery(2);

        // Add chains to aggregator registry
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(1);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(2);
        vm.prank(user);
        gasAggregator.addChain{value: 0.1 ether}(3);

        // Move to next epoch
        vm.warp(block.timestamp + EPOCH_DURATION + 1);

        // Try to seal without any submission - should fail because pendingEpochFirstSubmissionTime is 0
        vm.expectRevert(
            abi.encodeWithSelector(GasAggregator.WindowNotOver.selector, gasAggregator.pendingEpoch(), CHALLENGE_WINDOW)
        );
        gasAggregator.sealPendingEpoch();
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
    // Helper function to deploy mock contracts at computed addresses

    function _deployMockAtComputedAddress(uint256 chainId, address implementation) internal {
        address computedAddress = mockFactory.computeSequencingChainAddress(chainId);

        // Deploy a simple contract that returns the expected implementation
        SimpleMockProxy proxy = new SimpleMockProxy(implementation);

        // Use vm.etch to put this contract's code at the computed address
        vm.etch(computedAddress, address(proxy).code);

        // Set the implementation storage slot manually
        vm.store(computedAddress, bytes32(0), bytes32(uint256(uint160(implementation))));

        // Verify the deployment worked
        require(computedAddress.code.length > 0, "No code at computed address");
    }
}
