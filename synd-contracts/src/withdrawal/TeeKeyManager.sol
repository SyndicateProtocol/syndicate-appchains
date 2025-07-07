// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {ITeeKeyManager} from "./ITeeKeyManager.sol";
import {IAttestationDocVerifier} from "./IAttestationDocVerifier.sol";

/**
 * @title TeeKeyManager Contract
 * @notice Manages TEE program hashes and their associated public keys.
 * Allows anyone to add a key, by providing a proof for a valid attestation document.
 * to check if a given key is valid (i.e., associated with an active program).
 */
contract TeeKeyManager is ITeeKeyManager, Ownable {
    using EnumerableSet for EnumerableSet.AddressSet;

    event KeyAdded(address indexed key);
    event KeysRevoked();

    IAttestationDocVerifier public attestationDocVerifier;

    EnumerableSet.AddressSet internal validKeys;

    //#olympix-ignore-no-parameter-validation-in-constructor
    constructor(IAttestationDocVerifier _attestationDocVerifier) Ownable(msg.sender) {
        attestationDocVerifier = _attestationDocVerifier;
    }

    /**
     * @notice Checks if a public key is considered a valid TEE key.
     * @param publicKey The public key to check.
     * @return True if the key is valid (i.e., marked as valid in `keyValidity`), false otherwise.
     */
    function isKeyValid(address publicKey) external view override returns (bool) {
        return validKeys.contains(publicKey);
    }

    /**
     * @notice Adds a public key to the valid keys mapping.
     * @param _proofBytes The encoded proof.
     * @param _publicValues The encoded public values.
     */
    //#olympix-ignore-reentrancy-events
    function addKey(bytes calldata _publicValues, bytes calldata _proofBytes) external {
        address publicKey = attestationDocVerifier.verifyAttestationDocProof(_publicValues, _proofBytes);
        bool added = validKeys.add(publicKey);
        require(added, "TeeKeyManager: Key already exists or failed to add");
        emit KeyAdded(publicKey);
    }

    /**
     * @notice Revokes all keys.
     */
    function revokeAllKeys() public onlyOwner {
        address[] memory keys = validKeys.values();
        for (uint256 i = 0; i < keys.length; i++) {
            require(validKeys.remove(keys[i]), "TeeKeyManager: Failed to remove key");
        }
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
