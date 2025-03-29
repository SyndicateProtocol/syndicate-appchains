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

    function setUp() public {
        rollup = new MockRollup();

        // Set initial values
        rollup.setSequencerMessageCount(2); // > 1 so that _configureNew doesn't try to post an initial batch
        rollup.setSequencerInboxAcc(0, TEST_SEQ_BATCH_ACC);
        rollup.setWasmModuleRoot(TEST_WASM_ROOT);
        rollup.setBaseStake(1000);
        rollup.setConfirmPeriodBlocks(100);
        rollup.setComputedAssertionHash(COMPUTED_ASSERTION_HASH);

        vm.startPrank(OWNER);
        poster = new AssertionPoster(IRollup(address(rollup)));
        vm.stopPrank();
    }

    // LEGACY TESTS

    function testConstructorLegacy() public {
        MockRollup legacyRollup = new MockRollup();
        legacyRollup.setLegacyMode(true);
        vm.prank(OWNER);
        AssertionPoster legacyPoster = new AssertionPoster(IRollup(address(legacyRollup)));
        assertTrue(address(legacyPoster) != address(0));
    }

    function testConfigureLegacyDirect() public {
        MockRollup legacyRollup = new MockRollup();
        legacyRollup.setLegacyMode(true);
        vm.prank(OWNER);
        AssertionPoster legacyPoster = new AssertionPoster(IRollup(address(legacyRollup)));
        vm.expectRevert("must configure via upgradeExecutor.execute(AssertionPoster.configure)");
        legacyPoster.configure();
    }

    function testPostAssertionLegacyAccessControl() public {
        MockRollup legacyRollup = new MockRollup();
        legacyRollup.setLegacyMode(true);
        vm.prank(OWNER);
        AssertionPoster legacyPoster = new AssertionPoster(IRollup(address(legacyRollup)));
        vm.prank(USER);
        vm.expectRevert("Ownable: caller is not the owner");
        legacyPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);
    }

    function testPostAssertionLegacySuccess() public {
        MockRollup legacyRollup = new MockRollup();
        legacyRollup.setLegacyMode(true);

        vm.startPrank(OWNER);
        // Set the executor to a mock that will forward calls
        legacyRollup.setOwner(address(new MockExecutor()));
        AssertionPoster legacyPoster = new AssertionPoster(IRollup(address(legacyRollup)));

        // Expect events from both executor.executeCall calls:
        vm.expectEmit(true, true, true, true);
        emit ForceCreateNodeCalled(0, 1, bytes32(0));
        vm.expectEmit(true, true, true, true);
        emit ForceConfirmNodeCalled(1, TEST_BLOCK_HASH, TEST_SEND_ROOT);

        legacyPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);
        vm.stopPrank();
    }

    // NEW VERSION TESTS

    function testConstructorNew() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);
        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));
        assertTrue(address(newPoster) != address(0));
    }

    function testConfigureNewDirect() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);
        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));
        vm.expectRevert("must configure via upgradeExecutor.execute(AssertionPoster.configure)");
        newPoster.configure();
    }

    function testPostAssertionNewAccessControl() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);

        vm.startPrank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));
        vm.stopPrank();

        vm.startPrank(USER);
        vm.expectRevert("Ownable: caller is not the owner");
        newPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);
        vm.stopPrank();
    }

    function testPostAssertionNew() public {
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);
        newRollup.setComputedAssertionHash(COMPUTED_ASSERTION_HASH);

        vm.startPrank(OWNER);

        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));
        vm.expectEmit(true, true, true, true);
        emit FastConfirmNewAssertionCalled(COMPUTED_ASSERTION_HASH);
        newPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);

        vm.stopPrank();
    }

    function testPostAssertionNewTwice() public {
        // This covers both branches in _postNewAssertion
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);
        newRollup.setComputedAssertionHash(COMPUTED_ASSERTION_HASH);

        vm.startPrank(OWNER);

        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));
        // First call: state.u64Vals[0] is default (0), branch not taken
        vm.expectEmit(true, true, true, true);
        emit FastConfirmNewAssertionCalled(COMPUTED_ASSERTION_HASH);
        newPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);
        // Second call: now state.u64Vals[0] == 1 so branch is taken
        vm.expectEmit(true, true, true, true);
        emit FastConfirmNewAssertionCalled(COMPUTED_ASSERTION_HASH);
        newPoster.postAssertion(TEST_BLOCK_HASH, TEST_SEND_ROOT);

        vm.stopPrank();
    }

    // DELEGATECALL TESTS FOR INITIALIZE

    function testConfigureLegacyDelegatecall() public {
        MockRollup legacyRollup = new MockRollup();
        legacyRollup.setLegacyMode(true);
        TestExecutorCaller caller = new TestExecutorCaller();
        legacyRollup.setOwner(address(caller));

        vm.startPrank(OWNER);
        AssertionPoster legacyPoster = new AssertionPoster(IRollup(address(legacyRollup)));
        vm.expectEmit(true, true, true, true);
        emit RolePaused();
        caller.delegateConfigure(address(legacyPoster));
        vm.stopPrank();
    }

    function testConfigureNewDelegatecall() public {
        // Deploy new rollup and set owner to our executor caller.
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);
        TestExecutorCaller caller = new TestExecutorCaller();
        newRollup.setOwner(address(caller));
        // Ensure sequencer message count is already >1 so that initial batch branch is not taken.
        newRollup.setSequencerMessageCount(2);
        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));
        // Delegatecall configure via caller should succeed (events from new branch not easily asserted)
        caller.delegateConfigure(address(newPoster));
    }

    function testConfigureNewDelegatecallWithInitialBatch() public {
        // Deploy new rollup in a state that forces posting an initial batch.
        MockRollup newRollup = new MockRollup();
        newRollup.setGenesisAssertionHash(TEST_GENESIS_HASH);
        // Set sequencer message count to 1 so that _postInitialBatch is called.
        newRollup.setSequencerMessageCount(1);
        TestExecutorCaller caller = new TestExecutorCaller();
        newRollup.setOwner(address(caller));
        vm.prank(OWNER);
        AssertionPoster newPoster = new AssertionPoster(IRollup(address(newRollup)));
        // Expect events from _postInitialBatch: BatchPosterSet and SequencerBatchAdded.
        vm.expectEmit(true, true, true, true);
        emit BatchPosterSet(address(newRollup.owner()), true);
        vm.expectEmit(true, true, true, true);
        emit SequencerBatchAdded(1);
        caller.delegateConfigure(address(newPoster));
        // After delegatecall, sequencer message count should be updated to 2.
        assertGt(newRollup.bridge().sequencerMessageCount(), 1);
    }
}

// Helper contract to simulate delegatecall via the upgrade executor
contract TestExecutorCaller {
    function delegateConfigure(address poster) external {
        (bool success,) = poster.delegatecall(abi.encodeWithSignature("configure()"));
        require(success, "delegatecall failed");
    }

    function grantRole(bytes32, address) external {}

    function hasRole(bytes32, address) external pure returns (bool) {
        return true;
    }

    function getRoleAdmin(bytes32) external pure returns (bytes32) {
        return bytes32(0);
    }

    function revokeRole(bytes32, address) external {}
    function renounceRole(bytes32, address) external {}
}

// Mocks

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

contract MockExecutor {
    function executeCall(address target, bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory result) = target.call(data);
        require(success, "executeCall failed");
        return result;
    }
}

contract MockSequencerInbox {
    event BatchPosterSet(address poster, bool authorized);
    event SequencerBatchAdded(uint256 sequenceNumber);

    mapping(address => bool) private _isBatchPoster;
    MockBridge private _bridge;

    constructor(MockBridge bridge_) {
        _bridge = bridge_;
    }

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
        _bridge.setSequencerMessageCount(2);
        emit SequencerBatchAdded(sequenceNumber);
    }
}

contract MockRollup {
    event RolePaused();
    event ValidatorsSet(address[] validators, bool[] values);
    event ValidatorAfkBlocksSet(uint64 blocks);
    event AnyTrustFastConfirmerSet(address confimer);
    event ForceCreateNodeCalled(uint64 prevNode, uint256 prevNodeInboxMaxCount, bytes32 expectedNodeHash);
    event ForceConfirmNodeCalled(uint64 nodeNum, bytes32 blockHash, bytes32 sendRoot);
    event FastConfirmNewAssertionCalled(bytes32 expectedAssertionHash);

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

    MockBridge private _bridge;
    MockSequencerInbox private _sequencerInbox;

    constructor() {
        _bridge = new MockBridge();
        _sequencerInbox = new MockSequencerInbox(_bridge);
    }

    // Additional setter to allow delegatecall tests to work.
    function setOwner(address newOwner) external {
        _owner = newOwner;
    }

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
        emit ValidatorsSet(validators, new bool[](validators.length));
    }

    function owner() external view returns (address) {
        return _owner;
    }

    function genesisAssertionHash() external view returns (bytes32) {
        require(!_legacyMode, "Legacy mode: no genesis assertion hash");
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
        return _computedAssertionHash;
    }

    function fastConfirmNewAssertion(AssertionInputs calldata, bytes32 expectedAssertionHash) external {
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
