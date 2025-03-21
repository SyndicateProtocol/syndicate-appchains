// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "@arbitrum/nitro-contracts/src/rollup/IRollupAdmin.sol";
import "@arbitrum/nitro-contracts/src/rollup/IRollupCore.sol";
import "@openzeppelin/contracts/access/IAccessControl.sol";
import "@openzeppelin/contracts/ownership/Ownable.sol";
import "@offchainlabs/upgrade-executor/src/IUpgradeExecutor.sol";

struct Assertion {
    ExecutionState beforeState;
    ExecutionState afterState;
    uint64 numBlocks;
}

interface IRollup is IRollupCore, IOwnable {
    function forceCreateNode(
        uint64 prevNode,
        uint256 prevNodeInboxMaxCount,
        Assertion memory assertion,
        bytes32 expectedNodeHash
    ) external;

    function forceConfirmNode(uint64 nodeNum, bytes32 blockHash, bytes32 sendRoot) external;

    /**
     * @notice Computes the hash of an assertion
     * @param state The execution state for the assertion
     * @param prevAssertionHash The hash of the assertion's parent
     * @param inboxAcc The inbox batch accumulator
     */
    function computeAssertionHash(bytes32 prevAssertionHash, AssertionState calldata state, bytes32 inboxAcc)
        external
        pure
        returns (bytes32);

    /**
     * @notice This allow the anyTrustFastConfirmer to immediately create and confirm an assertion
     *         the anyTrustFastConfirmer is supposed to be set only on an AnyTrust chain to
     *         a contract that can call this function when received sufficient signatures
     *         The logic in this function is similar to stakeOnNewAssertion, but without staker checks
     *
     *         We trust the anyTrustFastConfirmer to not call this function multiple times on the same prev,
     *         as doing so would result in incorrect accounting of withdrawable funds in the loserStakeEscrow.
     *         This is because the protocol assume there is only 1 unique confirmable child assertion.
     */
    function fastConfirmNewAssertion(AssertionInputs calldata assertion, bytes32 expectedAssertionHash) external;
}

contract AssertionPoster is Ownable {
    // common
    address immutable self;
    IRollup immutable rollup;
    IUpgradeExecutor immutable executor;
    // legacy refers to v2 of the nitro contracts, non-legacy is v3 - v2 and v3 have slightly different ABIs
    bool immutable legacy;
    GlobalState state;
    uint64 nodeNum;

    // legacy
    uint64 currentInboxSize;

    // new
    bytes32 immutable seqBatchAcc;
    bytes32 assertionHash;
    bytes32 prevAssertionHash;
    ConfigData data;

    constructor(IRollup rollup_) {
        self = address(this);
        rollup = rollup_;
        executor = IUpgradeExecutor(rollup_.owner());
        try rollup_.genesisAssertionHash() returns (bytes32 assertionHash_) {
            assertionHash = assertionHash_;
            seqBatchAcc = rollup_.bridge().sequencerInboxAccs(0);
            data.wasmModuleRoot = rollup_.wasmModuleRoot();
            data.requiredStake = rollup_.baseStake();
            data.challengeManager = address(rollup_.challengeManager());
            data.confirmPeriodBlocks = rollup_.confirmPeriodBlocks();
            data.nextInboxPosition = 1;
        } catch {
            legacy = true;
            currentInboxSize = 1;
        }
    }

    // Note: this function does not need any access modifiers as it is delegate-called via the
    // upgrade executor contract and does not modify or have access to the contract state.
    // It does have access to immutable variables as they are read from the runtime bytecode.
    function initialize() external {
        require(
            address(this) == address(executor),
            "must initialize via upgradeExecutor.execute(AssertionPoster.initialize)"
        );
        if (legacy) {
            IAccessControl(address(executor)).grantRole(keccak256("EXECUTOR_ROLE"), self);
            // prevent anyone except the admin account from creating assertions
            IRollupAdmin(address(rollup)).pause();
        } else {
            address[] memory validators = rollup.getValidators();
            // prevent validators from creating assertions
            IRollupAdmin(address(rollup)).setValidator(validators, new bool[](validators.length));
            require(rollup.getValidators().length == 0, "validators not empty");
            // prevent the validator whitelist from being disabled due to inactivity
            IRollupAdmin(address(rollup)).setValidatorAfkBlocks(type(uint64).max);
            IRollupAdmin(address(rollup)).setAnyTrustFastConfirmer(self);
            // post a batch if seq inbox count is too low
            if (rollup.bridge().sequencerMessageCount() == 1) {
                bool isBatchPoster = rollup.sequencerInbox().isBatchPoster(address(executor));
                if (!isBatchPoster) {
                    rollup.sequencerInbox().setIsBatchPoster(address(executor), true);
                }
                rollup.sequencerInbox().addSequencerL2Batch(1, "", 1, IGasRefunder(address(0)), 0, 0);
                if (!isBatchPoster) {
                    rollup.sequencerInbox().setIsBatchPoster(address(executor), false);
                }
            }
            require(rollup.bridge().sequencerMessageCount() > 1, "sequencer message count too low");
        }
    }

    function postAssertion(bytes32 blockHash, bytes32 sendRoot) external onlyOwner {
        if (legacy) {
            Assertion memory assertion;
            assertion.beforeState.globalState = state;
            assertion.beforeState.machineStatus = MachineStatus.FINISHED;
            state.bytes32Vals = [blockHash, sendRoot];
            assertion.afterState.globalState = state;
            assertion.afterState.machineStatus = MachineStatus.FINISHED;
            executor.executeCall(
                address(rollup), abi.encodeCall(IRollup.forceCreateNode, (nodeNum++, currentInboxSize, assertion, 0))
            );
            // just in case a batch is posted, update the current inbox size
            currentInboxSize = uint64(rollup.bridge().sequencerMessageCount());
            executor.executeCall(
                address(rollup), abi.encodeCall(IRollup.forceConfirmNode, (nodeNum, blockHash, sendRoot))
            );
        } else {
            AssertionInputs memory assertion;
            assertion.beforeStateData.configData = data;
            // just in case a batch is posted, update the next inbox position
            data.nextInboxPosition = uint64(rollup.bridge().sequencerMessageCount());
            // just in case any cfg variables change, update them
            data.wasmModuleRoot = rollup.wasmModuleRoot();
            data.requiredStake = rollup.baseStake();
            data.challengeManager = address(rollup.challengeManager());
            data.confirmPeriodBlocks = rollup.confirmPeriodBlocks();
            if (state.u64Vals[0] == 1) {
                assertion.beforeStateData.sequencerBatchAcc = seqBatchAcc;
            }
            assertion.beforeStateData.prevPrevAssertionHash = prevAssertionHash;
            assertion.beforeState.globalState = state;
            assertion.beforeState.machineStatus = MachineStatus.FINISHED;
            state.u64Vals = [1, nodeNum++];
            state.bytes32Vals = [blockHash, sendRoot];
            assertion.afterState.globalState = state;
            assertion.afterState.machineStatus = MachineStatus.FINISHED;
            prevAssertionHash = assertionHash;
            assertionHash = rollup.computeAssertionHash(prevAssertionHash, assertion.afterState, seqBatchAcc);
            rollup.fastConfirmNewAssertion(assertion, assertionHash);
        }
    }
}
