// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AssertionPoster} from "./AssertionPoster.sol";

import {Ownable} from "@openzeppelin/contracts/ownership/Ownable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

using ECDSA for bytes32;
using MessageHashUtils for bytes32;

interface IL1Block {
    function timestamp() external view returns (uint256);
    function hash() external view returns (bytes32);
}

struct TeeTrustedInput {
    bytes32 appchainConfigHash;
    bytes32 setEndBlockHash;
    bytes32 l1EndBlockHash;
    bytes32 appchainStartBlockHash;
}

struct PendingAssertion {
    bytes32 blockHash;
    bytes32 sendRoot;
}

function hash_input(TeeTrustedInput storage a) view returns (bytes32) {
    return
        keccak256(abi.encodePacked(a.appchainConfigHash, a.setEndBlockHash, a.l1EndBlockHash, a.appchainStartBlockHash));
}

function equals(PendingAssertion calldata a, PendingAssertion storage b) view returns (bool) {
    return a.blockHash == b.blockHash && a.sendRoot == b.sendRoot;
}

event TeeKeysRevoked();

event TeeProgramAdded(bytes32 hash);

event TeeProgramRemoved(bytes32 hash);

event TeeAppchainConfigHash(bytes32 hash);

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
    constructor(AssertionPoster poster_, bytes32 appchainConfigHash_) {
        poster = poster_;
        teeTrustedInput.appchainConfigHash = appchainConfigHash_;
        emit TeeAppchainConfigHash(appchainConfigHash_);
    }

    function closeChallengeWindow() public {
        IL1Block l1block = IL1Block(address(0x4200000000000000000000000000000000000015));
        require(pendingAssertions.length <= 1, "cannot close challenge window - too many assertions");
        require(
            l1block.timestamp() > challengeWindowEnd, "cannot close challenge window - insufficient time has passed"
        );
        if (pendingAssertions.length > 0) {
            teeTrustedInput.appchainStartBlockHash = pendingAssertions[0].blockHash;
            poster.postAssertion(pendingAssertions[0].blockHash, pendingAssertions[0].sendRoot);
        }
        teeTrustedInput.setEndBlockHash = blockhash(block.number - 1);
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
}
