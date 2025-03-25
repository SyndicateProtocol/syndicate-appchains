// SPDX-License-Identifier: UNLICENSED

pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {AssertionPoster, Assertion, IRollup} from "src/AssertionPoster.sol";
import {
    ExecutionState,
    MachineStatus,
    AssertionState,
    AssertionInputs,
    ConfigData,
    GlobalState
} from "@arbitrum/nitro-contracts/src/rollup/IRollupCore.sol";
import {IGasRefunder} from "@arbitrum/nitro-contracts/src/libraries/IGasRefunder.sol";

/**
 * @title AssertionPosterTest
 * @dev Test suite for the AssertionPoster contract (simplified)
 */
contract AssertionPosterTest is Test {
    // Events for test verification
    event RolePaused();
    event ValidatorsSet(address[] validators, bool[] values);
    event ValidatorAfkBlocksSet(uint64 blocks);
    event AnyTrustFastConfirmerSet(address confimer);
    event BatchPosterSet(address poster, bool authorized);
    event SequencerBatchAdded(uint256 sequenceNumber);
    event ForceCreateNodeCalled(uint64 prevNode, uint256 prevNodeInboxMaxCount, bytes32 expectedNodeHash);
    event ForceConfirmNodeCalled(uint64 nodeNum, bytes32 blockHash, bytes32 sendRoot);
    event FastConfirmNewAssertionCalled(bytes32 expectedAssertionHash);

    // Test addresses
    address private constant OWNER = address(0x1);
    address private constant USER = address(0x2);
    address private constant VALIDATOR1 = address(0x100);
    address private constant VALIDATOR2 = address(0x101);

    // Test constants
    bytes32 private constant TEST_BLOCK_HASH = bytes32(uint256(1));
    bytes32 private constant TEST_SEND_ROOT = bytes32(uint256(2));
    bytes32 private constant TEST_GENESIS_HASH = bytes32(uint256(3));
    bytes32 private constant TEST_WASM_ROOT = bytes32(uint256(4));
    bytes32 private constant TEST_SEQ_BATCH_ACC = bytes32(uint256(5));
    bytes32 private constant COMPUTED_ASSERTION_HASH = bytes32(uint256(100));
    bytes32 private constant EXECUTOR_ROLE = keccak256("EXECUTOR_ROLE");

    // Test contracts
    AssertionPoster private poster;
    MockRollup private rollup;

    /**
     * @notice Set up the test environment
     */
    function setUp() public {
        // Create mock rollup
        rollup = new MockRollup();

        // Set initial values
        rollup.setSequencerMessageCount(2); // Greater than 1
        rollup.setSequencerInboxAcc(0, TEST_SEQ_BATCH_ACC);
        rollup.setWasmModuleRoot(TEST_WASM_ROOT);
        rollup.setBaseStake(1000);
        rollup.setConfirmPeriodBlocks(100);
        rollup.setComputedAssertionHash(COMPUTED_ASSERTION_HASH);

        // Create the assertion poster as owner
        vm.prank(OWNER);
        poster = new AssertionPoster(IRollup(address(rollup)));
    }

    // LEGACY TESTS
    function testConstructorLegacy() public {
        MockRollup legacyRollup = new MockRollup();
        legacyRollup.setLegacyMode(true);

        vm.prank(OWNER);
        AssertionPoster legacyPoster = new AssertionPoster(IRollup(address(legacyRollup)));

        // Successfully created
        assertTrue(address(legacyPoster) != address(0));
    }

    function testInitializeLegacyDirect() public {
        MockRollup legacyRollup = new MockRollup();
        legacyRollup.setLegacyMode(true);

        vm.prank(OWNER);
        AssertionPoster legacyPoster = new AssertionPoster(IRollup(address(legacyRollup)));

        // Direct call should fail
        vm.expectRevert("must initialize via upgradeExecutor.execute(AssertionPoster.initialize)");
        legacyPoster.initialize();
    }

    function testPostAssertionLegacyAccessControl() public {
        MockRollup legacyRollup = new MockRollup();
        legacyRollup.setLegacyMode(true);

        vm.prank(OWNER);
        AssertionPoster legacyPoster = new AssertionPoster(IRollup(address(legacyRollup)));

        // Should revert if not owner
        vm.prank(USER);
        vm.expectRevert("Ownable: caller is not the owner");
        legacyPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);
    }

    function testPostAssertionLegacy() public {
        MockRollup legacyRollup = new MockRollup();
        legacyRollup.setLegacyMode(true);

        vm.prank(OWNER);
        AssertionPoster legacyPoster = new AssertionPoster(IRollup(address(legacyRollup)));

        // Expect the events to be emitted
        vm.expectEmit(true, true, true, true);
        emit ForceCreateNodeCalled(0, 1, bytes32(0));

        vm.expectEmit(true, true, true, true);
        emit ForceConfirmNodeCalled(0, TEST_BLOCK_HASH, TEST_SEND_ROOT);

        // Owner posts assertion
        vm.prank(OWNER);
        legacyPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);
    }

    // NEW VERSION TESTS
    function testConstructorNew() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);

        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));

        // Successfully created
        assertTrue(address(newPoster) != address(0));
    }

    function testInitializeNewDirect() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);

        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));

        // Direct call should fail
        vm.expectRevert("must initialize via upgradeExecutor.execute(AssertionPoster.initialize)");
        newPoster.initialize();
    }

    function testInitializeNewWithValidators() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);

        // Set validators
        address[] memory validators = new address[](2);
        validators[0] = VALIDATOR1;
        validators[1] = VALIDATOR2;
        newRollup.setValidators(validators);

        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));

        // Simulate being the executor for initialize
        address executorAddr = newRollup.owner();
        bytes memory posterCode = address(newPoster).code;
        vm.etch(executorAddr, posterCode);

        // Expect validator setting event
        vm.expectEmit(true, true, true, true);
        emit ValidatorsSet(validators, new bool[](2));

        // Expect AFK blocks setting event
        vm.expectEmit(true, true, true, true);
        emit ValidatorAfkBlocksSet(type(uint64).max);

        // Expect AnyTrustFastConfirmer setting event
        vm.expectEmit(true, true, true, true);
        emit AnyTrustFastConfirmerSet(executorAddr);

        // Execute initialize from executor address
        vm.prank(executorAddr);
        AssertionPoster(executorAddr).initialize();
    }

    function testInitializeNewWithLowMessageCount() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);
        newRollup.setSequencerMessageCount(1); // Too low

        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));

        // Simulate being the executor for initialize
        address executorAddr = newRollup.owner();
        bytes memory posterCode = address(newPoster).code;
        vm.etch(executorAddr, posterCode);

        // Expect batch poster setting events
        vm.expectEmit(true, true, true, true);
        emit BatchPosterSet(executorAddr, true);

        vm.expectEmit(true, true, true, true);
        emit SequencerBatchAdded(1);

        vm.expectEmit(true, true, true, true);
        emit BatchPosterSet(executorAddr, false);

        // Execute initialize from executor address
        vm.prank(executorAddr);
        AssertionPoster(executorAddr).initialize();
    }

    function testPostAssertionNewAccessControl() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);

        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));

        // Should revert if not owner
        vm.prank(USER);
        vm.expectRevert("Ownable: caller is not the owner");
        newPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);
    }

    function testPostAssertionNew() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);
        newRollup.setComputedAssertionHash(COMPUTED_ASSERTION_HASH);

        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));

        // Expect the fast confirm event
        vm.expectEmit(true, true, true, true);
        emit FastConfirmNewAssertionCalled(COMPUTED_ASSERTION_HASH);

        // Owner posts assertion
        vm.prank(OWNER);
        newPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);
    }
}

// Simple mocks that implement only what's needed for the tests

/**
 * @title MockBridge
 * @dev Simple mock bridge implementation
 */
contract MockBridge {
    uint256 private _sequencerMessageCount;
    mapping(uint256 => bytes32) private _sequencerInboxAccs;

    function setSequencerMessageCount(uint256 count) public {
        _sequencerMessageCount = count;
    }

    function setSequencerInboxAcc(uint256 index, bytes32 acc) public {
        _sequencerInboxAccs[index] = acc;
    }

    function sequencerMessageCount() external view returns (uint256) {
        return _sequencerMessageCount;
    }

    function sequencerInboxAccs(uint256 index) external view returns (bytes32) {
        return _sequencerInboxAccs[index];
    }
}

/**
 * @title MockSequencerInbox
 * @dev Simple mock sequencer inbox implementation
 */
contract MockSequencerInbox {
    event BatchPosterSet(address poster, bool authorized);
    event SequencerBatchAdded(uint256 sequenceNumber);

    mapping(address => bool) private _isBatchPoster;

    function isBatchPoster(address poster) external view returns (bool) {
        return _isBatchPoster[poster];
    }

    function setIsBatchPoster(address poster, bool authorized) external {
        _isBatchPoster[poster] = authorized;
        emit BatchPosterSet(poster, authorized);
    }

    function addSequencerL2Batch(uint256 sequenceNumber, bytes calldata, uint256, IGasRefunder, uint256, uint256)
        external
    {
        emit SequencerBatchAdded(sequenceNumber);
    }
}

/**
 * @title MockRollup
 * @dev Simplified mock rollup implementation that only includes what's needed for tests
 */
contract MockRollup {
    // Events for tracking mock interactions
    event RolePaused();
    event ValidatorsSet(address[] validators, bool[] values);
    event ValidatorAfkBlocksSet(uint64 blocks);
    event AnyTrustFastConfirmerSet(address confimer);
    event ForceCreateNodeCalled(uint64 prevNode, uint256 prevNodeInboxMaxCount, bytes32 expectedNodeHash);
    event ForceConfirmNodeCalled(uint64 nodeNum, bytes32 blockHash, bytes32 sendRoot);
    event FastConfirmNewAssertionCalled(bytes32 expectedAssertionHash);

    // Mock state
    bool private _legacyMode = false;
    bytes32 private _genesisAssertionHash;
    bytes32 private _wasmModuleRoot;
    uint256 private _baseStake;
    uint64 private _confirmPeriodBlocks;
    bytes32 private _computedAssertionHash;
    address[] private _validators;
    address private _owner = address(0x999);
    address private _challengeManager = address(0x998);
    uint64 private _validatorAfkBlocks = 10000;

    // Mock objects
    MockBridge private _bridge;
    MockSequencerInbox private _sequencerInbox;

    constructor() {
        _bridge = new MockBridge();
        _sequencerInbox = new MockSequencerInbox();
    }

    // Configuration setters
    function setLegacyMode(bool mode) external {
        _legacyMode = mode;
    }

    function setGenesisAssertionHash(bytes32 hash) external {
        _genesisAssertionHash = hash;
    }

    function setWasmModuleRoot(bytes32 root) external {
        _wasmModuleRoot = root;
    }

    function setBaseStake(uint256 stake) external {
        _baseStake = stake;
    }

    function setConfirmPeriodBlocks(uint64 blocks) external {
        _confirmPeriodBlocks = blocks;
    }

    function setComputedAssertionHash(bytes32 hash) external {
        _computedAssertionHash = hash;
    }

    function setSequencerMessageCount(uint256 count) external {
        _bridge.setSequencerMessageCount(count);
    }

    function setSequencerInboxAcc(uint256 index, bytes32 acc) external {
        _bridge.setSequencerInboxAcc(index, acc);
    }

    function setValidators(address[] memory validators) external {
        _validators = validators;
    }

    // Mock implementation of functions used by AssertionPoster
    function owner() external view returns (address) {
        return _owner;
    }

    function genesisAssertionHash() external view returns (bytes32) {
        if (_legacyMode) {
            revert("Legacy mode: no genesis assertion hash");
        }
        return _genesisAssertionHash;
    }

    function wasmModuleRoot() external view returns (bytes32) {
        return _wasmModuleRoot;
    }

    function baseStake() external view returns (uint256) {
        return _baseStake;
    }

    function challengeManager() external view returns (address) {
        return _challengeManager;
    }

    function confirmPeriodBlocks() external view returns (uint64) {
        return _confirmPeriodBlocks;
    }

    function bridge() external view returns (MockBridge) {
        return _bridge;
    }

    function sequencerInbox() external view returns (MockSequencerInbox) {
        return _sequencerInbox;
    }

    function getValidators() external view returns (address[] memory) {
        return _validators;
    }

    // Legacy methods
    function forceCreateNode(
        uint64 prevNode,
        uint256 prevNodeInboxMaxCount,
        Assertion memory, /* assertion */
        bytes32 expectedNodeHash
    ) external {
        emit ForceCreateNodeCalled(prevNode, prevNodeInboxMaxCount, expectedNodeHash);
    }

    function forceConfirmNode(uint64 nodeNum, bytes32 blockHash, bytes32 sendRoot) external {
        emit ForceConfirmNodeCalled(nodeNum, blockHash, sendRoot);
    }

    // New version methods
    function computeAssertionHash(
        bytes32, /* prevAssertionHash */
        AssertionState calldata, /* state */
        bytes32 /* inboxAcc */
    ) external view returns (bytes32) {
        // Return the pre-set hash for testing
        return _computedAssertionHash;
    }

    function fastConfirmNewAssertion(AssertionInputs calldata, /* assertion */ bytes32 expectedAssertionHash)
        external
    {
        emit FastConfirmNewAssertionCalled(expectedAssertionHash);
    }

    // Mock admin methods
    function pause() external {
        emit RolePaused();
    }

    function setValidator(address[] memory validators, bool[] memory values) external {
        emit ValidatorsSet(validators, values);
    }

    function setValidatorAfkBlocks(uint64 blocks) external {
        _validatorAfkBlocks = blocks;
        emit ValidatorAfkBlocksSet(blocks);
    }

    function setAnyTrustFastConfirmer(address confimer) external {
        emit AnyTrustFastConfirmerSet(confimer);
    }
}
