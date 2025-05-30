// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";
import {IAttestationDocVerifier} from "./IAttestationDocVerifier.sol";

struct PublicValuesStruct {
    bytes cbor_encoded_attestation_document; // TODO check if we can remove this
    bytes der_encoded_root_cert;
    uint64 validity_window_start;
    uint64 validity_window_end;
    bytes public_key;
}

contract AttestationDocVerifier is IAttestationDocVerifier {
    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments
    address public verifier;

    /// @notice The verification key for the cert verifier.
    bytes32 public attestationDocVerifierVKey;

    constructor(address _verifier, bytes32 _attestationDocVerifierVKey) {
        verifier = _verifier;
        attestationDocVerifierVKey = _attestationDocVerifierVKey;
    }

    /// @notice The entrypoint for verifying the proof of a certificate.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    function verifyAttestationDocProof(
        bytes calldata _publicValues,
        bytes calldata _proofBytes
    ) public view returns (address) {
        PublicValuesStruct memory publicValues = abi.decode(
            _publicValues,
            (PublicValuesStruct)
        );

        // TODO check root cert

        require(
            block.timestamp >= publicValues.validity_window_start,
            "Validity window has not started"
        );
        require(
            block.timestamp <= publicValues.validity_window_end,
            "Validity window has ended"
        );

        ISP1Verifier(verifier).verifyProof(
            attestationDocVerifierVKey,
            _publicValues,
            _proofBytes
        );

        bytes32 hash = keccak256(publicValues.public_key);

        return address(uint160(uint256(hash)));
    }
}
