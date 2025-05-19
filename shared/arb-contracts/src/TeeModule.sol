// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AssertionPoster} from "./AssertionPoster.sol";

import {Ownable} from "@openzeppelin/contracts/ownership/Ownable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

using ECDSA for bytes32;
using MessageHashUtils for bytes32;

interface IL1Block {
    function number() external view returns (uint64);
    function timestamp() external view returns (uint64);
    function hash() external view returns (bytes32);
}

struct TeeTrustedInput {
    // appchain - requires a full node
    bytes32 appchainConfigHash;
    bytes32 appchainStartBlockHash;
    bytes32 appchainDelayedMessagesHash;
    // sequencing chain - requires a full node
    bytes32 seqConfigHash;
    bytes32 seqStartBlockHash;
    bytes32 seqDelayedMessagesHash;
    // settlement & l1 chains - do not require full nodes
    uint64 setStartBlockNumber;
    uint64 setEndBlockNumber;
    uint64 l1StartBlockNumber;
    uint64 l1EndBlockNumber;
    bytes32 setEndBlockHash;
    bytes32 l1EndBlockHash;
}

struct PendingAssertion {
    // appchain
    bytes32 blockHash;
    bytes32 sendRoot;
    bytes32 delayedMessagesHash;
    // sequencing chain
    bytes32 seqBlockHash;
    bytes32 seqDelayedMessagesHash;
}

function hash_input(TeeTrustedInput storage a) view returns (bytes32) {
    return keccak256(
        abi.encodePacked(
            a.appchainConfigHash,
            a.appchainStartBlockHash,
            a.appchainDelayedMessagesHash,
            a.seqConfigHash,
            a.seqStartBlockHash,
            a.seqDelayedMessagesHash,
            a.setStartBlockNumber,
            a.setEndBlockNumber,
            a.setEndBlockHash,
            a.l1StartBlockNumber,
            a.l1EndBlockNumber,
            a.l1EndBlockHash
        )
    );
}

function equals(PendingAssertion calldata a, PendingAssertion storage b) view returns (bool) {
    return a.blockHash == b.blockHash && a.sendRoot == b.sendRoot && a.delayedMessagesHash == b.delayedMessagesHash
        && a.seqBlockHash == b.seqBlockHash && a.seqDelayedMessagesHash == b.seqDelayedMessagesHash;
}

event TeeKeysRevoked();

event TeeProgramAdded(bytes32 hash);

event TeeProgramRemoved(bytes32 hash);

event TeeAppchainConfigHash(bytes32 hash);

event TeeSeqConfigHash(bytes32 hash);

event TeeHacked(uint256);

event ChallengeResolved(PendingAssertion);

/**
 * @title TeeModule Contract
 */
contract TeeModule is Ownable {
    // Immutable state variables
    AssertionPoster public immutable poster;

    // TEE variables
    TeeTrustedInput public teeTrustedInput;
    PendingAssertion[] public pendingAssertions;
    uint256 public teeHackCount;
    bytes32[] public teePrograms;
    mapping(bytes32 => address[]) public teeProgramKeys;
    mapping(address => bool) public isTeeKey;
    uint256 public challengeWindowEnd;
    uint256 public challengeWindowDuration;

    /**
     * @notice Constructs the AssertionPoster contract
     * @param poster_ Address of the assertion poster contract
     * @param appchainConfigHash_ Hash of the appchain configuration data passed to the TEE
     * Note that the AssertionPoster must be owned by the TeeModule for closing the challenge window to work properly
     */
    constructor(
        AssertionPoster poster_,
        bytes32 appchainConfigHash_,
        bytes32 appchainStartBlockHash_,
        bytes32 appchainDelayedMessagesHash_,
        bytes32 seqConfigHash_,
        bytes32 seqStartBlockHash_,
        bytes32 seqDelayedMessagesHash_,
        uint64 setStartBlockNumber_,
        uint64 l1StartBlockNumber_
    ) {
        poster = poster_;

        // appchain
        teeTrustedInput.appchainConfigHash = appchainConfigHash_;
        teeTrustedInput.appchainStartBlockHash = appchainStartBlockHash_;
        teeTrustedInput.appchainDelayedMessagesHash = appchainDelayedMessagesHash_;
        emit TeeAppchainConfigHash(appchainConfigHash_);

        // sequencing chain
        teeTrustedInput.seqConfigHash = seqConfigHash_;
        teeTrustedInput.seqStartBlockHash = seqStartBlockHash_;
        teeTrustedInput.seqDelayedMessagesHash = seqDelayedMessagesHash_;
        emit TeeSeqConfigHash(seqConfigHash_);

        // settlement & l1 chains
        teeTrustedInput.setStartBlockNumber = setStartBlockNumber_;
        teeTrustedInput.l1StartBlockNumber = l1StartBlockNumber_;

        closeChallengeWindow();
    }

    function closeChallengeWindow() public {
        IL1Block l1block = IL1Block(address(0x4200000000000000000000000000000000000015));
        require(pendingAssertions.length <= 1, "cannot close challenge window - too many assertions");
        require(
            l1block.timestamp() > challengeWindowEnd, "cannot close challenge window - insufficient time has passed"
        );
        if (pendingAssertions.length > 0) {
            // appchain
            teeTrustedInput.appchainDelayedMessagesHash = pendingAssertions[0].delayedMessagesHash;
            teeTrustedInput.appchainStartBlockHash = pendingAssertions[0].blockHash;

            // sequencing chain
            teeTrustedInput.seqDelayedMessagesHash = pendingAssertions[0].seqDelayedMessagesHash;
            teeTrustedInput.seqStartBlockHash = pendingAssertions[0].seqBlockHash;
            teeTrustedInput.setStartBlockNumber = teeTrustedInput.setEndBlockNumber + 1;

            // l1 chain
            teeTrustedInput.l1StartBlockNumber = teeTrustedInput.l1EndBlockNumber + 1;

            poster.postAssertion(pendingAssertions[0].blockHash, pendingAssertions[0].sendRoot);
        }

        // only process settlement blocks up to the l1 timestamp
        uint64 delta = (uint64(block.timestamp) - l1block.timestamp()) / 2;

        // blockhash only works for the most recent 256 blocks, use 255 instead of 256 just in case
        if (delta > 255) {
            delta = 255;
        }

        // settlement chain
        teeTrustedInput.setEndBlockNumber = uint64(block.number) - delta;
        teeTrustedInput.setEndBlockHash = blockhash(teeTrustedInput.setEndBlockNumber);
        require(teeTrustedInput.setEndBlockHash != 0, "unexpected blockhash of 0");

        // l1 chain
        teeTrustedInput.l1EndBlockNumber = l1block.number();
        teeTrustedInput.l1EndBlockHash = l1block.hash();

        challengeWindowEnd = block.timestamp + challengeWindowDuration;
    }

    function submitAssertion(PendingAssertion calldata assertion, bytes calldata signature, address rewardAddr)
        external
    {
        require(signature.length == 65, "invalid signature length");
        bytes32 payload_hash =
            keccak256(abi.encodePacked(hash_input(teeTrustedInput), assertion.blockHash, assertion.sendRoot));
        require(isTeeKey[payload_hash.toEthSignedMessageHash().recover(signature)], "invalid tee signature");
        require(assertion.blockHash != teeTrustedInput.appchainStartBlockHash, "appchain block hash unchanged");
        for (uint256 i = 0; i < pendingAssertions.length; i++) {
            require(!equals(assertion, pendingAssertions[i]), "assertion already exists");
        }
        if (pendingAssertions.length == 0) {
            challengeWindowEnd = block.timestamp + challengeWindowDuration;
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

    function setChallengeWindowDuration(uint256 duration) external onlyOwner {
        challengeWindowDuration = duration;
    }

    function resolveChallenge(PendingAssertion calldata assertion) external onlyOwner {
        require(pendingAssertions.length > 1, "challenge does not exist");
        for (uint256 i = 0; i < pendingAssertions.length; i++) {
            if (equals(assertion, pendingAssertions[i])) {
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

    function addTeeProgram(bytes32 hash) external onlyOwner {
        for (uint256 i = 0; i < teePrograms.length; i++) {
            require(teePrograms[i] != hash, "tee program already exists");
        }
        teePrograms.push(hash);
        emit TeeProgramAdded(hash);
    }

    function removeTeeProgram(bytes32 hash) public onlyOwner {
        for (uint256 i = 0; i < teeProgramKeys[hash].length; i++) {
            isTeeKey[teeProgramKeys[hash][i]] = false;
        }
        delete teeProgramKeys[hash];
        emit TeeProgramRemoved(hash);
    }

    function setAppchainConfigHash(bytes32 hash) external onlyOwner {
        teeTrustedInput.appchainConfigHash = hash;
        emit TeeAppchainConfigHash(hash);
    }

    function setSeqConfigHash(bytes32 hash) external onlyOwner {
        teeTrustedInput.seqConfigHash = hash;
        emit TeeSeqConfigHash(hash);
    }
}
