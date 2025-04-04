// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {
    IRollupCore,
    ExecutionState,
    MachineStatus,
    AssertionState,
    AssertionInputs,
    ConfigData,
    GlobalState
} from "@arbitrum/nitro-contracts/src/rollup/IRollupCore.sol";
import {IRollupAdmin} from "@arbitrum/nitro-contracts/src/rollup/IRollupAdmin.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/IAccessControl.sol";
import {Ownable} from "@openzeppelin/contracts/ownership/Ownable.sol";
import {IOwnable} from "@arbitrum/nitro-contracts/src/bridge/IOwnable.sol";
import {IUpgradeExecutor} from "@offchainlabs/upgrade-executor/src/IUpgradeExecutor.sol";
import {IGasRefunder} from "@arbitrum/nitro-contracts/src/libraries/IGasRefunder.sol";

/**
 * @title IRollup Interface
 * @dev Extends IRollupCore and IOwnable to define additional functions needed for assertion management
 */
interface IRollup is IRollupCore, IOwnable {
    /**
     * @notice Forces the creation of a new node in the rollup chain
     * @param prevNode Index of the previous node
     * @param prevNodeInboxMaxCount Max inbox count for the previous node
     * @param assertion The assertion data
     * @param expectedNodeHash Expected hash of the new node
     */
    function forceCreateNode(
        uint64 prevNode,
        uint256 prevNodeInboxMaxCount,
        Assertion memory assertion,
        bytes32 expectedNodeHash
    ) external;

    /**
     * @notice Forces confirmation of a node in the rollup chain
     * @param nodeNum Index of the node to confirm
     * @param blockHash Hash of the block
     * @param sendRoot Root of the sends
     */
    function forceConfirmNode(uint64 nodeNum, bytes32 blockHash, bytes32 sendRoot) external;

    /**
     * @notice Computes the hash of an assertion
     * @param prevAssertionHash The hash of the assertion's parent
     * @param state The execution state for the assertion
     * @param inboxAcc The inbox batch accumulator
     * @return bytes32 The computed assertion hash
     */
    function computeAssertionHash(bytes32 prevAssertionHash, AssertionState calldata state, bytes32 inboxAcc)
        external
        pure
        returns (bytes32);

    /**
     * @notice Allows immediate creation and confirmation of an assertion by the anyTrustFastConfirmer
     * @dev Only intended for AnyTrust chains with sufficient signatures
     * @param assertion The assertion inputs
     * @param expectedAssertionHash Expected hash of the assertion
     */
    function fastConfirmNewAssertion(AssertionInputs calldata assertion, bytes32 expectedAssertionHash) external;
}

/**
 * @title Assertion Data Structure
 * @dev Defines the structure of an assertion in the legacy rollup system
 */
struct Assertion {
    ExecutionState beforeState;
    ExecutionState afterState;
    uint64 numBlocks;
}

/**
 * @title AssertionPoster Contract
 * @dev Facilitates posting assertions to an Arbitrum rollup chain
 * @notice Supports both legacy (v2) and new (v3) Nitro contracts
 */
contract AssertionPoster is Ownable {
    // Immutable state variables
    address private immutable self;
    IRollup private immutable rollup;
    IUpgradeExecutor private immutable executor;
    bool private immutable legacy;
    bytes32 private immutable seqBatchAcc;

    // State variables
    GlobalState private state;
    uint64 private nodeNum;

    // Legacy-specific variables
    uint64 private currentInboxSize;

    // New version-specific variables
    bytes32 private assertionHash;
    bytes32 private prevAssertionHash;
    ConfigData private data;

    /**
     * @notice Constructs the AssertionPoster contract
     * @param rollup_ Address of the rollup contract
     */
    constructor(IRollup rollup_) {
        self = address(this);
        rollup = rollup_;
        executor = IUpgradeExecutor(rollup_.owner());

        // Detect if we're using legacy or new version and configure accordingly
        try rollup_.genesisAssertionHash() returns (bytes32 assertionHash_) {
            // New version initialization (v3)
            assertionHash = assertionHash_;
            seqBatchAcc = rollup_.bridge().sequencerInboxAccs(0);
            data.wasmModuleRoot = rollup_.wasmModuleRoot();
            data.requiredStake = rollup_.baseStake();
            data.challengeManager = address(rollup_.challengeManager());
            data.confirmPeriodBlocks = rollup_.confirmPeriodBlocks();
            data.nextInboxPosition = 1;
        } catch {
            // Legacy version initialization (v2)
            legacy = true;
            currentInboxSize = 1;
        }
    }

    /**
     * @notice Configures the contract by configuring the rollup's permissions
     * @dev Must be called via upgrade executor delegatecall
     */
    function configure() external {
        require(
            address(this) == address(executor), "must configure via upgradeExecutor.execute(AssertionPoster.configure)"
        );

        if (legacy) {
            _configureLegacy();
        } else {
            _configureNew();
        }
    }

    /**
     * @notice Posts a new assertion to the rollup
     * @param blockHash Hash of the block
     * @param sendRoot Root of the sends
     */
    function postAssertion(bytes32 blockHash, bytes32 sendRoot) external onlyOwner {
        if (legacy) {
            _postLegacyAssertion(blockHash, sendRoot);
        } else {
            _postNewAssertion(blockHash, sendRoot);
        }
    }

    /**
     * @notice Configures the contract for legacy rollup (v2)
     * @dev Sets up executor role and pauses regular assertion creation
     */
    function _configureLegacy() private {
        IAccessControl(address(executor)).grantRole(keccak256("EXECUTOR_ROLE"), self);
        // Prevent anyone except the admin account from creating assertions
        if (!rollup.paused()) {
            IRollupAdmin(address(rollup)).pause();
        }
    }

    /**
     * @notice Configures the contract for new rollup (v3)
     * @dev Sets up validators, posts initial batch if needed
     */
    function _configureNew() private {
        address[] memory validators = rollup.getValidators();
        // Prevent validators from creating assertions
        IRollupAdmin(address(rollup)).setValidator(validators, new bool[](validators.length));
        require(rollup.getValidators().length == 0, "validators not empty");

        // Prevent the validator whitelist from being disabled due to inactivity
        IRollupAdmin(address(rollup)).setValidatorAfkBlocks(type(uint64).max);
        IRollupAdmin(address(rollup)).setAnyTrustFastConfirmer(self);

        // Post a batch if sequencer inbox count is too low
        if (rollup.bridge().sequencerMessageCount() == 1) {
            _postInitialBatch();
        }

        require(rollup.bridge().sequencerMessageCount() > 1, "sequencer message count too low");
    }

    /**
     * @notice Posts an initial batch to the sequencer inbox
     * @dev Temporarily sets executor as batch poster if needed
     */
    function _postInitialBatch() private {
        bool isBatchPoster = rollup.sequencerInbox().isBatchPoster(address(executor));

        if (!isBatchPoster) {
            rollup.sequencerInbox().setIsBatchPoster(address(executor), true);
        }

        rollup.sequencerInbox().addSequencerL2Batch(1, "", 1, IGasRefunder(address(0)), 0, 0);

        if (!isBatchPoster) {
            rollup.sequencerInbox().setIsBatchPoster(address(executor), false);
        }
    }

    /**
     * @notice Posts an assertion using the legacy rollup protocol (v2)
     * @param blockHash Hash of the block
     * @param sendRoot Root of the sends
     */
    function _postLegacyAssertion(bytes32 blockHash, bytes32 sendRoot) private {
        Assertion memory assertion;

        // Set up assertion states
        assertion.beforeState.globalState = state;
        assertion.beforeState.machineStatus = MachineStatus.FINISHED;

        state.bytes32Vals = [blockHash, sendRoot];

        assertion.afterState.globalState = state;
        assertion.afterState.machineStatus = MachineStatus.FINISHED;

        // Execute force create node
        executor.executeCall(
            address(rollup), abi.encodeCall(IRollup.forceCreateNode, (nodeNum++, currentInboxSize, assertion, 0))
        );

        // Update inbox size in case a batch was posted
        currentInboxSize = uint64(rollup.bridge().sequencerMessageCount());

        // Execute force confirm node
        executor.executeCall(address(rollup), abi.encodeCall(IRollup.forceConfirmNode, (nodeNum, blockHash, sendRoot)));
    }

    /**
     * @notice Posts an assertion using the new rollup protocol (v3)
     * @param blockHash Hash of the block
     * @param sendRoot Root of the sends
     */
    function _postNewAssertion(bytes32 blockHash, bytes32 sendRoot) private {
        AssertionInputs memory assertion;

        // Set up config data
        assertion.beforeStateData.configData = data;

        // Update data with latest values
        _updateConfigData();

        // Set sequencer batch accumulator if needed
        if (state.u64Vals[0] == 1) {
            assertion.beforeStateData.sequencerBatchAcc = seqBatchAcc;
        }

        // Set up assertion states
        assertion.beforeStateData.prevPrevAssertionHash = prevAssertionHash;
        assertion.beforeState.globalState = state;
        assertion.beforeState.machineStatus = MachineStatus.FINISHED;

        state.u64Vals = [1, nodeNum++];
        state.bytes32Vals = [blockHash, sendRoot];

        assertion.afterState.globalState = state;
        assertion.afterState.machineStatus = MachineStatus.FINISHED;

        // Update assertion hashes
        prevAssertionHash = assertionHash;
        assertionHash = rollup.computeAssertionHash(prevAssertionHash, assertion.afterState, seqBatchAcc);

        // Fast confirm the assertion
        rollup.fastConfirmNewAssertion(assertion, assertionHash);
    }

    /**
     * @notice Updates the config data with latest values from the rollup
     */
    function _updateConfigData() private {
        // Update in case a batch is posted
        data.nextInboxPosition = uint64(rollup.bridge().sequencerMessageCount());

        // Update in case any config variables change
        data.wasmModuleRoot = rollup.wasmModuleRoot();
        data.requiredStake = rollup.baseStake();
        data.challengeManager = address(rollup.challengeManager());
        data.confirmPeriodBlocks = rollup.confirmPeriodBlocks();
    }
}
