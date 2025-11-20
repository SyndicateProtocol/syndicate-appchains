// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {
    IRollupCore,
    ExecutionState,
    MachineStatus,
    GlobalState
} from "@arbitrum/nitro-contracts/src/rollup/IRollupCore.sol";
import {IRollupAdmin} from "@arbitrum/nitro-contracts/src/rollup/IRollupAdmin.sol";
import {IAccessControl} from "@openzeppelin/contracts/access/IAccessControl.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IOwnable} from "@arbitrum/nitro-contracts/src/bridge/IOwnable.sol";
import {IUpgradeExecutor} from "@offchainlabs/upgrade-executor/src/IUpgradeExecutor.sol";
import {IGasRefunder} from "@arbitrum/nitro-contracts/src/libraries/IGasRefunder.sol";
import {IAssertionPoster} from "./IAssertionPoster.sol";
import {GlobalState} from "@arbitrum/nitro-contracts/src/state/GlobalState.sol";
import {Assertion} from "@arbitrum/nitro-contracts/src/rollup/Node.sol";
import {GlobalStateLib} from "@arbitrum/nitro-contracts/src/state/GlobalState.sol";

// Grabbing these from the official v3 Arb contracts because they're missing in these v2 Eigen versions
// https://github.com/OffchainLabs/nitro-contracts/blob/main/src/rollup/Assertion.sol
struct AssertionState {
    GlobalState globalState;
    MachineStatus machineStatus;
    bytes32 endHistoryRoot;
}

struct BeforeStateData {
    // The assertion hash of the prev of the beforeState(prev)
    bytes32 prevPrevAssertionHash;
    // The sequencer inbox accumulator asserted by the beforeState(prev)
    bytes32 sequencerBatchAcc;
    // below are the components of config hash
    ConfigData configData;
}

struct AssertionInputs {
    // Additional data used to validate the before state
    BeforeStateData beforeStateData;
    AssertionState beforeState;
    AssertionState afterState;
}

struct ConfigData {
    bytes32 wasmModuleRoot;
    uint256 requiredStake;
    address challengeManager;
    uint64 confirmPeriodBlocks;
    uint64 nextInboxPosition;
}

interface INewRollup {
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

    function genesisAssertionHash() external pure returns (bytes32);
    function getValidators() external view returns (address[] memory);
    function latestConfirmed() external view returns (bytes32);
    function validatorAfkBlocks() external view returns (uint64);
    function setValidatorAfkBlocks(uint64 newAfkBlocks) external;
    function anyTrustFastConfirmer() external view returns (address);
    function setAnyTrustFastConfirmer(address _anyTrustFastConfirmer) external;
}

interface IPausable {
    function paused() external view returns (bool);
}

/**
 * @title AssertionPoster Contract
 * @dev Facilitates posting assertions to an Arbitrum rollup chain
 * @notice Supports both legacy (v2) and new (v3) Nitro contracts
 */
contract AssertionPoster is Ownable, IAssertionPoster {
    // Immutable state variables
    address private immutable self;
    address private immutable rollup;
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
     * @param sendRoot the send root of the latest assertion
     * @param batchCount the batch count of the latest assertion
     * @param inboxSize the batch count when the assertion was submitted
     */
    //#olympix-ignore-no-parameter-validation-in-constructor
    constructor(address rollup_, bytes32 sendRoot, uint64 batchCount, uint64 inboxSize) Ownable(msg.sender) {
        self = address(this);
        rollup = rollup_;
        executor = IUpgradeExecutor(IOwnable(rollup_).owner());

        // Detect if we're using legacy or new version and configure accordingly
        try INewRollup(rollup_).genesisAssertionHash() returns (bytes32 genesisAssertionHash) {
            // New version initialization (v3)
            require(
                INewRollup(rollup).latestConfirmed() == genesisAssertionHash, "rollup already has confirmed assertions"
            );
            assertionHash = genesisAssertionHash;
            seqBatchAcc = IRollupCore(rollup_).bridge().sequencerInboxAccs(0);
            _updateConfigData();
            require(data.nextInboxPosition > 0, "missing genesis batch");
            data.nextInboxPosition = 1;
        } catch {
            // Legacy version initialization (v2)
            legacy = true;
            currentInboxSize = inboxSize;
            require(currentInboxSize >= batchCount && currentInboxSize > 0, "invalid inbox size / batch count");
            nodeNum = IRollupCore(rollup).latestConfirmed();
            require(nodeNum == IRollupCore(rollup).latestNodeCreated(), "unconfirmed nodes exist");
            state.bytes32Vals = [IRollupCore(rollup_).outbox().roots(sendRoot)];
            state.u64Vals[0] = batchCount;
            require(
                IRollupCore(rollup_).getNode(nodeNum).stateHash
                    == keccak256(
                        abi.encodePacked(GlobalStateLib.hash(state), uint256(currentInboxSize), MachineStatus.FINISHED)
                    ),
                "stateHash mismatch"
            );
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
        if (!IPausable(rollup).paused()) {
            IRollupAdmin(rollup).pause();
        }
    }

    /**
     * @notice Configures the contract for new rollup (v3)
     * @dev Sets up validators, posts initial batch if needed
     */
    function _configureNew() private {
        address[] memory validators = INewRollup(rollup).getValidators();
        // Prevent validators from creating assertions
        if (validators.length > 0) {
            IRollupAdmin(rollup).setValidator(validators, new bool[](validators.length));
        }
        require(INewRollup(rollup).getValidators().length == 0, "validators not empty");

        // Prevent the validator whitelist from being disabled due to inactivity
        if (INewRollup(rollup).validatorAfkBlocks() != 0) {
            INewRollup(rollup).setValidatorAfkBlocks(0);
        }
        if (IRollupCore(rollup).validatorWhitelistDisabled()) {
            IRollupAdmin(rollup).setValidatorWhitelistDisabled(false);
        }
        if (INewRollup(rollup).anyTrustFastConfirmer() != self) {
            INewRollup(rollup).setAnyTrustFastConfirmer(self);
        }

        // Post a batch if sequencer inbox count is too low
        if (IRollupCore(rollup).bridge().sequencerMessageCount() == 1) {
            _postInitialBatch();
        }

        require(IRollupCore(rollup).bridge().sequencerMessageCount() > 1, "sequencer message count too low");
    }

    /**
     * @notice Posts an initial batch to the sequencer inbox
     * @dev Temporarily sets executor as batch poster if needed
     */
    function _postInitialBatch() private {
        bool isBatchPoster = IRollupCore(rollup).sequencerInbox().isBatchPoster(address(executor));

        if (!isBatchPoster) {
            IRollupCore(rollup).sequencerInbox().setIsBatchPoster(address(executor), true);
        }

        IRollupCore(rollup).sequencerInbox().addSequencerL2Batch(1, "", 1, IGasRefunder(address(0)), 0, 0);

        if (!isBatchPoster) {
            IRollupCore(rollup).sequencerInbox().setIsBatchPoster(address(executor), false);
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
            rollup, abi.encodeCall(IRollupAdmin.forceCreateNode, (nodeNum++, currentInboxSize, assertion, 0))
        );

        // Update inbox size in case a batch was posted
        currentInboxSize = uint64(IRollupCore(rollup).bridge().sequencerMessageCount());

        // Execute force confirm node
        executor.executeCall(rollup, abi.encodeCall(IRollupAdmin.forceConfirmNode, (nodeNum, blockHash, sendRoot)));
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
        assertionHash = INewRollup(rollup).computeAssertionHash(prevAssertionHash, assertion.afterState, seqBatchAcc);

        // Fast confirm the assertion
        INewRollup(rollup).fastConfirmNewAssertion(assertion, assertionHash);
    }

    /**
     * @notice Updates the config data with latest values from the rollup
     */
    function _updateConfigData() private {
        // Update in case a batch is posted
        data.nextInboxPosition = uint64(IRollupCore(rollup).bridge().sequencerMessageCount());

        // Update in case any config variables change
        data.wasmModuleRoot = IRollupCore(rollup).wasmModuleRoot();
        data.requiredStake = IRollupCore(rollup).baseStake();
        data.challengeManager = address(IRollupCore(rollup).challengeManager());
        data.confirmPeriodBlocks = IRollupCore(rollup).confirmPeriodBlocks();
    }
}
