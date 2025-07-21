// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IAssertionPoster} from "./IAssertionPoster.sol";
import {ITeeKeyManager} from "./ITeeKeyManager.sol";

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import {IBridge} from "@arbitrum/nitro-contracts/src/bridge/IBridge.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

using ECDSA for bytes32;

interface IL1Block {
    function timestamp() external view returns (uint64);
    function hash() external view returns (bytes32);
}

struct TeeTrustedInput {
    bytes32 configHash;
    // appchain - requires a custom node
    bytes32 appStartBlockHash;
    // sequencing chain - requires a custom node
    bytes32 seqStartBlockHash;
    // settlement chain - does not require a custom node
    bytes32 setDelayedMessageAcc;
    // l1 chain - does not require a custom node
    // the start sequencing chain batch accumulator on the l1 chain
    bytes32 l1StartBatchAcc;
    // l1EndHash is either a l1 block hash or a seq batch accumulator hash
    bytes32 l1EndHash;
}

struct PendingAssertion {
    // appchain
    bytes32 appBlockHash;
    bytes32 appSendRoot;
    // sequencing chain
    bytes32 seqBlockHash;
    // l1 chain
    bytes32 l1BatchAcc;
}

function hash_object(TeeTrustedInput storage a) view returns (bytes32) {
    return keccak256(
        abi.encodePacked(
            a.configHash,
            a.appStartBlockHash,
            a.seqStartBlockHash,
            a.setDelayedMessageAcc,
            a.l1StartBatchAcc,
            a.l1EndHash
        )
    );
}

function hash_object(PendingAssertion storage a) view returns (bytes32) {
    return keccak256(abi.encodePacked(a.appBlockHash, a.appSendRoot, a.seqBlockHash, a.l1BatchAcc));
}

function hash_object(PendingAssertion calldata a) pure returns (bytes32) {
    return keccak256(abi.encodePacked(a.appBlockHash, a.appSendRoot, a.seqBlockHash, a.l1BatchAcc));
}

event TeeConfigHash(bytes32 configHash);

event TeeHacked(uint256);

event ChallengeResolved(PendingAssertion);

event TeeInput(TeeTrustedInput input);

/**
 * @title TeeModule Contract
 */
contract TeeModule is Ownable(msg.sender), ReentrancyGuard {
    // Maximum number of pending assertions allowed to prevent DoS attacks via unbounded array growth.
    // The value is set to 2 based on the following considerations:
    // 1. Normal operation expects 0-1 pending assertions at any given time, ensuring efficient processing.
    // 2. A limit of 2 enables the detection of potential TEE hacks, as the presence of exactly 2 assertions
    //    triggers the TEE hack detection logic, which is a critical security feature of the system.
    // 3. The closeChallengeWindow() function requires the number of pending assertions to be either 0 or 1
    //    for proper processing. A higher limit would complicate this logic and potentially introduce errors.
    uint256 public constant MAX_PENDING_ASSERTIONS = 2;

    // Immutable state variables
    IAssertionPoster public immutable poster;
    IBridge public immutable bridge;
    // the l1 block contract or bridge from the l1 chain to the sequencing chain (when the settlement chain is the same as the l1 chain)
    address public immutable l1BlockOrBridge;
    bool public immutable isL1Chain;
    ITeeKeyManager public immutable teeKeyManager;

    // TEE variables
    TeeTrustedInput public teeTrustedInput;
    PendingAssertion[] public pendingAssertions;
    //#olympix-ignore-uninitialized-state-variable
    uint256 public teeHackCount;
    //#olympix-ignore-uninitialized-state-variable
    uint64 public challengeWindowEnd;
    uint64 public challengeWindowDuration;

    receive() external payable {}

    /**
     * @notice Constructs the TeeModule contract
     * @param poster_ Address of the assertion poster contract
     * @param bridge_ Settlement chain address of the appchain `Bridge` contract
     * @param configHash_ Hash of the configuration data passed to the TEE
     * @param appStartBlockHash_ The starting block hash of the appchain
     * @param seqStartBlockHash_ The starting block hash of the sequencing chain
     * @param l1StartBatchAcc_ The sequencing chain start batch accumulator
     * @param l1BlockOrBridge_ Address of the l1 block contract - 0x4200000000000000000000000000000000000015 for bedrock rollups - or the l1 <-> sequencing chain bridge if the settlement chain is the same as the l1 chain.
     * @param isL1Chain_ True if l1BlockOrBridge is the bridge address and false if it is the l1 block contract address instead
     * @param challengeWindowDuration_ The duration of the challenge window in seconds
     * @param teeKeyManager_ The address of the TEE key manager contract
     * Note that the AssertionPoster must be owned by the TeeModule for closing the challenge window to work properly
     */
    constructor(
        IAssertionPoster poster_,
        IBridge bridge_,
        bytes32 configHash_, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 appStartBlockHash_, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 seqStartBlockHash_, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 l1StartBatchAcc_, //#olympix-ignore-no-parameter-validation-in-constructor
        address l1BlockOrBridge_,
        bool isL1Chain_,
        uint64 challengeWindowDuration_, //#olympix-ignore-no-parameter-validation-in-constructor
        ITeeKeyManager teeKeyManager_
    ) {
        challengeWindowDuration = challengeWindowDuration_;
        l1BlockOrBridge = l1BlockOrBridge_;
        isL1Chain = isL1Chain_;
        teeTrustedInput.configHash = configHash_;
        emit TeeConfigHash(configHash_);

        if (isL1Chain) {
            require(
                l1BlockOrBridge != address(0x4200000000000000000000000000000000000015), "unexpected seq bridge address"
            );
            require(
                IBridge(l1BlockOrBridge).sequencerMessageCount() > 0, "sequencing chain must have at least one batch"
            );
        } else {
            // require(
            //     l1BlockOrBridge == address(0x4200000000000000000000000000000000000015), "unexpected l1 block address"
            // );
            require(
                IL1Block(l1BlockOrBridge).timestamp() > 0 && IL1Block(l1BlockOrBridge).hash() > 0,
                "l1 block contract invalid"
            );
        }

        require(address(poster_).code.length > 0, "poster address does not have any code");
        poster = poster_;
        require(bridge_.delayedMessageCount() > 0, "insufficient delayed messages in bridge");
        bridge = bridge_;

        require(address(teeKeyManager_).code.length > 0, "teeKeyManager address does not have any code");
        teeKeyManager = teeKeyManager_;

        // appchain
        teeTrustedInput.appStartBlockHash = appStartBlockHash_;

        // sequencing chain
        teeTrustedInput.seqStartBlockHash = seqStartBlockHash_;

        // l1 chain
        teeTrustedInput.l1StartBatchAcc = l1StartBatchAcc_;

        closeChallengeWindow();
    }

    function closeChallengeWindow() public nonReentrant {
        require(
            (isL1Chain ? uint64(block.timestamp) : IL1Block(l1BlockOrBridge).timestamp()) > challengeWindowEnd,
            "cannot close challenge window - insufficient time has passed"
        );

        challengeWindowEnd = uint64(block.timestamp) + challengeWindowDuration;

        if (pendingAssertions.length == 1) {
            // l1 chain
            teeTrustedInput.l1StartBatchAcc = pendingAssertions[0].l1BatchAcc;

            // sequencing chain
            teeTrustedInput.seqStartBlockHash = pendingAssertions[0].seqBlockHash;

            // appchain
            if (teeTrustedInput.appStartBlockHash != pendingAssertions[0].appBlockHash) {
                teeTrustedInput.appStartBlockHash = pendingAssertions[0].appBlockHash;
                poster.postAssertion(pendingAssertions[0].appBlockHash, pendingAssertions[0].appSendRoot);
            }

            delete pendingAssertions;
        } else {
            require(pendingAssertions.length == 0, "cannot close challenge window - too many assertions");
        }

        // settlement chain
        teeTrustedInput.setDelayedMessageAcc = bridge.delayedInboxAccs(bridge.delayedMessageCount() - 1);

        // l1 chain
        if (isL1Chain) {
            teeTrustedInput.l1EndHash =
                IBridge(l1BlockOrBridge).sequencerInboxAccs(IBridge(l1BlockOrBridge).sequencerMessageCount() - 1);
        } else {
            teeTrustedInput.l1EndHash = IL1Block(l1BlockOrBridge).hash();
        }

        emit TeeInput(teeTrustedInput);
    }

    function submitAssertion(PendingAssertion calldata assertion, bytes calldata signature, address rewardAddr)
        external
        nonReentrant
    {
        require(pendingAssertions.length < MAX_PENDING_ASSERTIONS, "TeeModule: Too many pending assertions");
        require(rewardAddr != address(0), "reward address cannot be zero");
        require(signature.length == 65, "invalid signature length");
        bytes32 assertionHash = hash_object(assertion);
        bytes32 payload_hash = keccak256(abi.encodePacked(hash_object(teeTrustedInput), assertionHash));
        require(teeKeyManager.isKeyValid(payload_hash.recover(signature)), "invalid tee signature");
        require(!isL1Chain || assertion.l1BatchAcc == teeTrustedInput.l1EndHash, "unexpected l1 end batch acc");
        for (uint256 i = 0; i < pendingAssertions.length; i++) {
            require(assertionHash != hash_object(pendingAssertions[i]), "assertion already exists");
        }
        if (pendingAssertions.length == 0) {
            challengeWindowEnd = uint64(block.timestamp) + challengeWindowDuration;
        }
        pendingAssertions.push(assertion);
        if (pendingAssertions.length == 2) {
            teeHackCount += 1;
            emit TeeHacked(teeHackCount);

            // pay out rewards
            //#olympix-ignore-low-level-call-params-verified
            (bool success,) = payable(rewardAddr).call{value: address(this).balance}("");
            require(success, "payment failed");
        }
    }

    function resolveChallenge(PendingAssertion calldata assertion) external onlyOwner nonReentrant {
        require(pendingAssertions.length > 1, "challenge does not exist");
        bytes32 assertionHash = hash_object(assertion);
        for (uint256 i = 0; i < pendingAssertions.length; i++) {
            if (assertionHash == hash_object(pendingAssertions[i])) {
                delete pendingAssertions;
                pendingAssertions.push(assertion);
                challengeWindowEnd = 0;
                closeChallengeWindow();
                emit ChallengeResolved(assertion);
                return;
            }
        }
        revert("assertion not found");
    }

    function transferAssertionPosterOwner(address newOwner) public onlyOwner {
        Ownable(address(poster)).transferOwnership(newOwner);
    }
}
