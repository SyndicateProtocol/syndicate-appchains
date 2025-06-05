// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IAttestationDocVerifier {
    /**
     * @notice Verifies the attestation document proof.
     * @param _publicValues The encoded public values.
     * @param _proofBytes The encoded proof.
     * @return The public key hold by the TEE.
     */
    function verifyAttestationDocProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        external
        view
        returns (address);
}
