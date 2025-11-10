// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {TeeKeyManager} from "src/withdrawal/TeeKeyManager.sol";
import {AttestationDocVerifier} from "src/withdrawal/AttestationDocVerifier.sol";

contract UpdateAttestationDocVerifier is Script {
    function run() public {
        // Start broadcasting transactions
        vm.startBroadcast();

        // 1. Deploy the attestation doc verifier if no address provided
        address attestationDocVerifierAddress = vm.envOr("ATTESTATION_DOC_VERIFIER_ADDRESS", address(0));
        AttestationDocVerifier attestationDocVerifier;
        if (attestationDocVerifierAddress != address(0)) {
            console2.log("Attestation doc verifier already deployed to:", attestationDocVerifierAddress);
            attestationDocVerifier = AttestationDocVerifier(attestationDocVerifierAddress);
        } else {
            console2.log("Deploying attestation doc verifier...");
            attestationDocVerifier = new AttestationDocVerifier(
                vm.envAddress("SP1_VERIFIER_ADDRESS"), // https://github.com/succinctlabs/sp1-contracts/blob/main/contracts/deployments/84532.json
                vm.envBytes32("ATTESTATION_DOC_VERIFIER_V_KEY"), // use proof submitter to get the vkey
                keccak256(
                    abi.encodePacked(
                        vm.envOr(
                            "ROOT_CERT_HASH",
                            bytes32(0x311d96fcd5c5e0ccf72ef548e2ea7d4c0cd53ad7c4cc49e67471aed41d61f185)
                        ),
                        vm.envBytes("PCR_0"),
                        vm.envBytes("PCR_1"),
                        vm.envBytes("PCR_2")
                    )
                ),
                uint64(vm.envUint("EXPIRATION_TOLERANCE")), // Arbitrary value, usually 24h
                vm.envString("SYND_COMMIT_HASH"),
                AttestationDocVerifier.ProofSystem(
                    vm.envOr("PROOF_SYSTEM", uint256(AttestationDocVerifier.ProofSystem.SP1))
                )
            );
            console2.log("Attestation doc verifier deployed to:", address(attestationDocVerifier));
        }
        // 2. Already deployed key manager
        address teeKeyManagerAddress = vm.envAddress("TEE_KEY_MANAGER_ADDRESS");
        TeeKeyManager teeKeyManager = TeeKeyManager(teeKeyManagerAddress);
        console2.log("TeeKeyManager in address:", address(teeKeyManager));

        // 3. Update the attestation doc verifier
        teeKeyManager.updateAttestationDocVerifier(attestationDocVerifier);
        console2.log("TeeKeyManager attestation doc verifier updated to:", address(attestationDocVerifier));
    }
}
