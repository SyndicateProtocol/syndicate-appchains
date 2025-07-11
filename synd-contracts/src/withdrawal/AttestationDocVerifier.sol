// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";
import {IAttestationDocVerifier} from "./IAttestationDocVerifier.sol";

struct PublicValuesStruct {
    bytes32 root_cert_hash;
    uint64 validity_window_start;
    uint64 validity_window_end;
    // https://docs.aws.amazon.com/enclaves/latest/user/set-up-attestation.html#where
    // each pcr is 48 bytes, that is subsequently keccak256'd
    bytes32 pcr_0;
    bytes32 pcr_1;
    bytes32 pcr_2;
    bytes32 pcr_8;
    address tee_signing_key;
}

contract AttestationDocVerifier is IAttestationDocVerifier {
    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments
    address public immutable verifier;

    /// @notice The verification key for the cert verifier.
    bytes32 public immutable attestationDocVerifierVKey;

    /// @notice The expected values for the attestation document.
    bytes32 public immutable rootCertHash;
    bytes32 public immutable pcr0;
    bytes32 public immutable pcr1;
    bytes32 public immutable pcr2;
    bytes32 public immutable pcr8;

    uint64 public immutable expirationTolerance;

    constructor(
        address _verifier, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 _attestationDocVerifierVKey, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 _rootCertHash, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 _pcr0, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 _pcr1, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 _pcr2, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 _pcr8, //#olympix-ignore-no-parameter-validation-in-constructor
        uint64 _expirationTolerance //#olympix-ignore-no-parameter-validation-in-constructor
    ) {
        verifier = _verifier;
        attestationDocVerifierVKey = _attestationDocVerifierVKey;
        rootCertHash = _rootCertHash;
        pcr0 = _pcr0;
        pcr1 = _pcr1;
        pcr2 = _pcr2;
        pcr8 = _pcr8;
        expirationTolerance = _expirationTolerance;
    }

    /// @notice The entrypoint for verifying the proof of a certificate.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    function verifyAttestationDocProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (address)
    {
        PublicValuesStruct memory publicValues = abi.decode(_publicValues, (PublicValuesStruct));

        require(publicValues.root_cert_hash == rootCertHash, "Root cert hash mismatch");

        require(block.timestamp >= publicValues.validity_window_start, "Validity window has not started");
        require(block.timestamp <= publicValues.validity_window_end + expirationTolerance, "Validity window has ended");

        require(publicValues.pcr_0 == pcr0, "PCR0 mismatch");
        require(publicValues.pcr_1 == pcr1, "PCR1 mismatch");
        require(publicValues.pcr_2 == pcr2, "PCR2 mismatch");
        require(publicValues.pcr_8 == pcr8, "PCR8 mismatch");

        ISP1Verifier(verifier).verifyProof(attestationDocVerifierVKey, _publicValues, _proofBytes);

        return publicValues.tee_signing_key;
    }
}
