// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AssertionPoster} from "./AssertionPoster.sol";

import {Ownable} from "@openzeppelin/contracts/ownership/Ownable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import "@arbitrum/nitro-contracts/src/bridge/IBridge.sol";

using ECDSA for bytes32;
using MessageHashUtils for bytes32;

interface IL1Block {
    function timestamp() external view returns (uint64);
    function hash() external view returns (bytes32);
}

IL1Block constant l1block = IL1Block(0x4200000000000000000000000000000000000015);

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

function hash_input(TeeTrustedInput storage a) view returns (bytes32) {
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

function equals(PendingAssertion calldata a, PendingAssertion storage b) view returns (bool) {
    return a.blockHash == b.blockHash && a.sendRoot == b.sendRoot && a.seqBlockHash == b.seqBlockHash;
}

event TeeKeysRevoked();

event TeeProgramAdded(bytes32 hash);

event TeeProgramRemoved(bytes32 hash);

event TeeAppchainConfigHash(bytes32 configHash, bytes32 blockHash);

event TeeSeqConfigHash(bytes32 configHash, bytes32 blockHash);

event TeeHacked(uint256);

event ChallengeResolved(PendingAssertion);

/**
 * @title TeeModule Contract
 */
contract TeeModule is Ownable {
    // Immutable state variables
    AssertionPoster public immutable poster;
    IBridge public immutable bridge;

    // TEE variables
    TeeTrustedInput public teeTrustedInput;
    PendingAssertion[] public pendingAssertions;
    uint256 public teeHackCount;
    bytes32[] public teePrograms;
    mapping(bytes32 => address[]) public teeProgramKeys;
    mapping(address => bool) public isTeeKey;
    uint256 public challengeWindowEnd;
    uint256 public challengeWindowDuration;

    receive() external payable {}

    /**
     * @notice Constructs the AssertionPoster contract
     * @param poster_ Address of the assertion poster contract
     * @param bridge_ Settlement chain address of the appchain `Bridge` contract
     * @param appchainConfigHash_ Hash of the appchain configuration data passed to the TEE
     * @param seqConfigHash_ Hash of the sequencing chain configuration data passed to the TEE - currently unused
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
        bytes32 teeProgram
    ) {
        require(l1block.timestamp() > 0 && l1block.hash() > 0, "chain is not an l2 bedrock rollup");

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
            l1block.timestamp() > challengeWindowEnd, "cannot close challenge window - insufficient time has passed"
        );
        if (pendingAssertions.length > 0) {
            // appchain
            teeTrustedInput.appchainStartBlockHash = pendingAssertions[0].blockHash;

            // sequencing chain
            teeTrustedInput.seqStartBlockHash = pendingAssertions[0].seqBlockHash;

            // l1 chain
            teeTrustedInput.l1StartBlockHash = teeTrustedInput.l1EndBlockHash;

            poster.postAssertion(pendingAssertions[0].blockHash, pendingAssertions[0].sendRoot);
        }

        // settlement chain
        teeTrustedInput.setDelayedMessageAcc = bridge.delayedInboxAccs(bridge.delayedMessageCount() - 1);

        // l1 chain
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
    function setChallengeWindowDuration(uint256 duration) external onlyOwner {
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
