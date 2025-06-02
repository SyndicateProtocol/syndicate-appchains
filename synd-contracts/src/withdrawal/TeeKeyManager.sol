// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ITeeKeyManager} from "./ITeeKeyManager.sol";
import {IAttestationDocVerifier} from "./IAttestationDocVerifier.sol";

/**
 * @title TeeKeyManager Contract
 * @notice Manages TEE program hashes and their associated public keys.
 * Allows anyone to add a key, by providing a proof for a valid attestation document.
 * to check if a given key is valid (i.e., associated with an active program).
 */
contract TeeKeyManager is ITeeKeyManager, Ownable {
    event KeyAdded(address indexed key);
    event KeysRevoked();

    IAttestationDocVerifier public attestationDocVerifier;

    mapping(address => bool) internal validKeys;

    constructor(IAttestationDocVerifier _attestationDocVerifier) Ownable(msg.sender) {
        attestationDocVerifier = _attestationDocVerifier;
    }

    /**
     * @notice Checks if a public key is considered a valid TEE key.
     * @param publicKey The public key to check.
     * @return True if the key is valid (i.e., marked as valid in `keyValidity`), false otherwise.
     */
    function isKeyValid(address publicKey) external view override returns (bool) {
        return validKeys[publicKey];
    }

    /**
     * @notice Adds a public key to the valid keys mapping.
     * @param _proofBytes The encoded proof.
     * @param _publicValues The encoded public values.
     */
    function addKey(bytes calldata _publicValues, bytes calldata _proofBytes) external {
        address publicKey = attestationDocVerifier.verifyAttestationDocProof(_publicValues, _proofBytes);
        validKeys[publicKey] = true;
        emit KeyAdded(publicKey);
    }

    /**
     * @notice Revokes all keys.
     */
    function revokeAllKeys() public onlyOwner {
        validKeys[msg.sender] = false;
        emit KeysRevoked();
    }

    /**
     * @notice Updates the attestation doc verifier.
     * @param _attestationDocVerifier The new attestation doc verifier.
     */
    function updateAttestationDocVerifier(IAttestationDocVerifier _attestationDocVerifier) external onlyOwner {
        revokeAllKeys();
        attestationDocVerifier = _attestationDocVerifier;
    }
}
