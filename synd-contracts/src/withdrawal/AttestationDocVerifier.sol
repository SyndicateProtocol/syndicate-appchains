// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IAttestationDocVerifier} from "./IAttestationDocVerifier.sol";

/// @title SP1 Verifier Interface
/// @author Succinct Labs
/// @notice This contract is the interface for the SP1 Verifier.
interface ISP1Verifier {
    /// @notice Verifies a proof with given public values and vkey.
    /// @dev It is expected that the first 4 bytes of proofBytes must match the first 4 bytes of
    /// target verifier's VERIFIER_HASH.
    /// @param programVKey The verification key for the RISC-V program.
    /// @param publicValues The public values encoded as bytes.
    /// @param proofBytes The proof of the program execution the SP1 zkVM encoded as bytes.
    function verifyProof(bytes32 programVKey, bytes calldata publicValues, bytes calldata proofBytes) external view;
}

/// @notice Verifier interface for RISC Zero receipts of execution.
interface IRiscZeroVerifier {
    /// @notice Verify that the given seal is a valid RISC Zero proof of execution with the
    ///     given image ID and journal digest. Reverts on failure.
    /// @dev This method additionally ensures that the input hash is all-zeros (i.e. no
    /// committed input), the exit code is (Halted, 0), and there are no assumptions (i.e. the
    /// receipt is unconditional).
    /// @param seal The encoded cryptographic proof (i.e. SNARK).
    /// @param imageId The identifier for the guest program.
    /// @param journalDigest The SHA-256 digest of the journal bytes.
    function verify(bytes calldata seal, bytes32 imageId, bytes32 journalDigest) external view;
}

interface IOpenVmHalo2Verifier {
    function verify(bytes calldata publicValues, bytes calldata proofData, bytes32 appExeCommit, bytes32 appVmCommit)
        external
        view;
}

contract AttestationDocVerifier is IAttestationDocVerifier {
    /// @notice The address of the verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported SP1 verifiers on each chain, see:
    ///      https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments
    ///
    ///      Risc0 verifiers are also supported, for more info see:
    ///      https://dev.risczero.com/api/blockchain-integration/contracts/verifier
    address public immutable verifier;

    /// @notice The commit hash of the synd-appchains repo used to generate the proof circuit.
    string public syndCommitHash;

    /// @notice The verification key for the cert verifier.
    bytes32 public immutable attestationDocVerifierVKey;

    /// @notice The commitment to the VM configuration
    bytes32 public constant appVmCommit = bytes32(0x005cb541cc5d9796ffb07fabe7099c6a09c9c5412d1d6e8746b60b1cbaeac858);

    /// @notice The expected hash value for important fields in the attestation document.
    bytes32 public immutable dataHash;

    uint64 public immutable expirationTolerance;

    enum ProofSystem {
        RISC0,
        SP1,
        OpenVM
    }

    ProofSystem public immutable proofSystem;

    constructor(
        address _verifier, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 _attestationDocVerifierVKey, //#olympix-ignore-no-parameter-validation-in-constructor
        bytes32 _dataHash, //#olympix-ignore-no-parameter-validation-in-constructor
        uint64 _expirationTolerance, //#olympix-ignore-no-parameter-validation-in-constructor
        string memory _syndCommitHash, //#olympix-ignore-no-parameter-validation-in-constructor
        ProofSystem _proofSystem //#olympix-ignore-no-parameter-validation-in-constructor
    ) {
        verifier = _verifier;
        attestationDocVerifierVKey = _attestationDocVerifierVKey;
        dataHash = _dataHash;
        expirationTolerance = _expirationTolerance;
        syndCommitHash = _syndCommitHash;
        proofSystem = _proofSystem;
        require(
            proofSystem == ProofSystem.RISC0 || proofSystem == ProofSystem.SP1 || proofSystem == ProofSystem.OpenVM,
            "invalid proof system"
        );
    }

    /// @notice The entrypoint for verifying the proof of a certificate.
    /// @param proofBytes The encoded proof.
    /// @param publicValues The encoded public values.
    function verifyAttestationDocProof(bytes calldata publicValues, bytes calldata proofBytes)
        public
        view
        returns (address)
    {
        (uint64 validityWindowEnd, address teeSigningKey) = abi.decode(publicValues, (uint64, address));
        require(block.timestamp <= validityWindowEnd + expirationTolerance, "Validity window has ended");

        bytes memory publicData = abi.encodePacked(dataHash, validityWindowEnd, teeSigningKey);

        if (proofSystem == ProofSystem.SP1) {
            ISP1Verifier(verifier).verifyProof(attestationDocVerifierVKey, publicData, proofBytes);
        } else if (proofSystem == ProofSystem.RISC0) {
            IRiscZeroVerifier(verifier).verify(proofBytes, attestationDocVerifierVKey, sha256(publicData));
        } else {
            IOpenVmHalo2Verifier(verifier)
                .verify(bytes.concat(keccak256(publicData)), proofBytes, attestationDocVerifierVKey, appVmCommit);
        }

        return teeSigningKey;
    }
}
