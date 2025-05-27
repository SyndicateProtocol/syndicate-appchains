// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AssertionPoster} from "./AssertionPoster.sol";

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import "@arbitrum/nitro-contracts/src/bridge/IBridge.sol";

using ECDSA for bytes32;

interface IL1Block {
    function timestamp() external view returns (uint64);
    function hash() external view returns (bytes32);
}

struct TeeTrustedInput {
    // appchain - requires a full node
    bytes32 appchainConfigHash;
    bytes32 appchainStartBlockHash;
    // sequencing chain - requires a full node
    bytes32 seqConfigHash;
    bytes32 seqStartBlockHash;
    // settlement chain - does not require a full node
    bytes32 setDelayedMessageAcc;
    // l1 chain - does not require a full node
    bytes32 l1StartBlockHash;
    bytes32 l1EndBlockHash;
}

struct PendingAssertion {
    // appchain
    bytes32 blockHash;
    bytes32 sendRoot;
    // sequencing chain
    bytes32 seqBlockHash;
}

function hash_object(TeeTrustedInput storage a) view returns (bytes32) {
    return keccak256(
        abi.encodePacked(
            a.appchainConfigHash,
            a.appchainStartBlockHash,
            a.seqConfigHash,
            a.seqStartBlockHash,
            a.setDelayedMessageAcc,
            a.l1StartBlockHash,
            a.l1EndBlockHash
        )
    );
}

function hash_object(PendingAssertion storage a) view returns (bytes32) {
    return keccak256(abi.encodePacked(a.blockHash, a.sendRoot, a.seqBlockHash));
}

function hash_object(PendingAssertion calldata a) pure returns (bytes32) {
    return keccak256(abi.encodePacked(a.blockHash, a.sendRoot, a.seqBlockHash));
}

event TeeKeysRevoked();

event TeeProgramAdded(bytes32 hash);

event TeeProgramRemoved(bytes32 hash);

event TeeAppchainConfigHash(bytes32 configHash, bytes32 blockHash);

event TeeSeqConfigHash(bytes32 configHash, bytes32 blockHash);

event TeeHacked(uint256);

event ChallengeResolved(PendingAssertion);

event TeeInput(TeeTrustedInput input);

/**
 * @title TeeModule Contract
 */
contract TeeModule is Ownable(msg.sender) {
    // Immutable state variables
    AssertionPoster public immutable poster;
    IBridge public immutable bridge;
    IL1Block public immutable l1block;

    // TEE variables
    TeeTrustedInput public teeTrustedInput;
    PendingAssertion[] public pendingAssertions;
    uint256 public teeHackCount;
    bytes32[] public teePrograms;
    mapping(bytes32 => address[]) public teeProgramKeys;
    mapping(address => bool) public isTeeKey;
    uint64 public challengeWindowEnd;
    uint64 public challengeWindowDuration;

    receive() external payable {}

    /**
     * @notice Constructs the AssertionPoster contract
     * @param poster_ Address of the assertion poster contract
     * @param bridge_ Settlement chain address of the appchain `Bridge` contract
     * @param appchainConfigHash_ Hash of the appchain configuration data passed to the TEE
     * @param seqConfigHash_ Hash of the sequencing chain configuration data passed to the TEE - currently unused
     * @param l1block_ Address of the l1 block contract - 0x4200000000000000000000000000000000000015 for bedrock rollups
     * and zero for ethereum itself
     * Note that the AssertionPoster must be owned by the TeeModule for closing the challenge window to work properly
     */
    constructor(
        AssertionPoster poster_,
        IBridge bridge_,
        bytes32 appchainConfigHash_,
        bytes32 appchainStartBlockHash_,
        bytes32 seqConfigHash_,
        bytes32 seqStartBlockHash_,
        bytes32 l1StartBlockHash_,
        IL1Block l1block_,
        uint64 challengeWindowDuration_,
        bytes32 teeProgram
    ) {
        challengeWindowDuration = challengeWindowDuration_;
        require(
            address(l1block_) == address(0) || (l1block_.timestamp() > 0 && l1block_.hash() > 0), "l1 contract invalid"
        );
        l1block = l1block_;

        require(address(poster_).code.length > 0, "poster address does not have any code");
        poster = poster_;
        require(bridge_.delayedMessageCount() > 0, "insufficient delayed messages in bridge");
        bridge = bridge_;

        // appchain
        teeTrustedInput.appchainConfigHash = appchainConfigHash_;
        teeTrustedInput.appchainStartBlockHash = appchainStartBlockHash_;
        emit TeeAppchainConfigHash(appchainConfigHash_, appchainStartBlockHash_);

        // sequencing chain
        teeTrustedInput.seqConfigHash = seqConfigHash_;
        teeTrustedInput.seqStartBlockHash = seqStartBlockHash_;
        emit TeeSeqConfigHash(seqConfigHash_, seqStartBlockHash_);

        // l1 chain
        teeTrustedInput.l1StartBlockHash = l1StartBlockHash_;

        require(teeProgram != 0, "tee program hash must be non-zero");
        teePrograms.push(teeProgram);
        emit TeeProgramAdded(teeProgram);

        closeChallengeWindow();
    }

    function closeChallengeWindow() public {
        require(pendingAssertions.length <= 1, "cannot close challenge window - too many assertions");
        require(
            (address(l1block) == address(0) ? uint64(block.timestamp) : l1block.timestamp()) > challengeWindowEnd,
            "cannot close challenge window - insufficient time has passed"
        );
        if (pendingAssertions.length > 0) {
            // appchain
            teeTrustedInput.appchainStartBlockHash = pendingAssertions[0].blockHash;

            // sequencing chain
            teeTrustedInput.seqStartBlockHash = pendingAssertions[0].seqBlockHash;

            // l1 chain
            teeTrustedInput.l1StartBlockHash = teeTrustedInput.l1EndBlockHash;

            poster.postAssertion(pendingAssertions[0].blockHash, pendingAssertions[0].sendRoot);

            delete pendingAssertions;
        }

        // settlement chain
        teeTrustedInput.setDelayedMessageAcc = bridge.delayedInboxAccs(bridge.delayedMessageCount() - 1);

        // l1 chain
        teeTrustedInput.l1EndBlockHash = (address(l1block) == address(0) ? blockhash(block.number - 1) : l1block.hash());

        emit TeeInput(teeTrustedInput);

        challengeWindowEnd = uint64(block.timestamp) + challengeWindowDuration;
    }

    function submitAssertion(PendingAssertion calldata assertion, bytes calldata signature, address rewardAddr)
        external
    {
        require(signature.length == 65, "invalid signature length");
        bytes32 assertionHash = hash_object(assertion);
        bytes32 payload_hash = keccak256(abi.encodePacked(hash_object(teeTrustedInput), assertionHash));
        require(isTeeKey[payload_hash.recover(signature)], "invalid tee signature");
        require(assertion.blockHash != teeTrustedInput.appchainStartBlockHash, "appchain block hash unchanged");
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
            (bool success,) = payable(rewardAddr).call{value: address(this).balance}("");
            require(success, "payment failed");
        }
    }

    function resolveChallenge(PendingAssertion calldata assertion) external onlyOwner {
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

    function addTeeKey(address publicKey, bytes32 programHash, bytes calldata zkProof) external {
        // todo: validate the zk proof
        require(zkProof.length == 1, "todo: validate zk zkProof");
        require(!isTeeKey[publicKey], "key already added");
        if (teeProgramKeys[programHash].length == 0) {
            bool found = false;
            for (uint256 i = teePrograms.length; i > 0; i--) {
                if (teePrograms[i - 1] == programHash) {
                    found = true;
                    break;
                }
            }
            if (!found) {
                return;
            }
        }
        isTeeKey[publicKey] = true;
        teeProgramKeys[programHash].push(publicKey);
    }

    function revokeAllTeeKeys() external onlyOwner {
        for (uint256 i = 0; i < teePrograms.length; i++) {
            removeTeeProgram(teePrograms[i]);
        }
        delete teePrograms;
        emit TeeKeysRevoked();
    }

    // TODO: should this function be removed?
    function addTeeProgram(bytes32 hash) external onlyOwner {
        for (uint256 i = 0; i < teePrograms.length; i++) {
            require(teePrograms[i] != hash, "tee program already exists");
        }
        teePrograms.push(hash);
        emit TeeProgramAdded(hash);
    }

    // TODO: should this function be removed?
    function removeTeeProgram(bytes32 hash) public onlyOwner {
        require(pendingAssertions.length == 0, "cannot remove tee program while assertion is pending");
        for (uint256 i = 0; i < teeProgramKeys[hash].length; i++) {
            isTeeKey[teeProgramKeys[hash][i]] = false;
        }
        delete teeProgramKeys[hash];
        emit TeeProgramRemoved(hash);
    }

    // TODO: should this function be removed?
    function setChallengeWindowDuration(uint64 duration) external onlyOwner {
        require(pendingAssertions.length == 0, "cannot update challenge window while assertion is pending");
        challengeWindowDuration = duration;
    }

    // TODO: should this function be removed?
    function setAppchainConfigHash(bytes32 hash) external onlyOwner {
        require(pendingAssertions.length == 0, "cannot update config hash while assertions are pending");
        teeTrustedInput.appchainConfigHash = hash;
        emit TeeAppchainConfigHash(hash, teeTrustedInput.appchainStartBlockHash);
    }

    // TODO: should this function be removed?
    function setSeqConfigHash(bytes32 hash) external onlyOwner {
        require(pendingAssertions.length == 0, "cannot update config hash while assertions are pending");
        teeTrustedInput.seqConfigHash = hash;
        emit TeeSeqConfigHash(hash, teeTrustedInput.seqStartBlockHash);
    }
}
