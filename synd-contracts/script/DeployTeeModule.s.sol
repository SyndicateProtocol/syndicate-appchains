// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {TeeModule} from "src/withdrawal/TeeModule.sol";
import {AssertionPoster, IRollup} from "src/withdrawal/AssertionPoster.sol";
import {TeeKeyManager} from "src/withdrawal/TeeKeyManager.sol";
import {AttestationDocVerifier} from "src/withdrawal/AttestationDocVerifier.sol";
import {IBridge} from "@arbitrum/nitro-contracts/src/bridge/IBridge.sol";

address constant L1_BLOCK = address(0x4200000000000000000000000000000000000015);

contract DeployTeeModule is Script {
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
                vm.envBytes32("ATTESTATION_DOC_VERIFIER_V_KEY"), // cargo run --release --bin vkey
                vm.envBytes32("ROOT_CERT_HASH"), // https://github.com/SyndicateProtocol/syndicate-appchains/blob/7e148d3b7118786f30ae0b1d22339b888edaf8e9/synd-contracts/test/withdrawal/fixtures/groth16-fixture.json
                keccak256(vm.envBytes("PCR_0")), // Get from TEE
                keccak256(vm.envBytes("PCR_1")), // Get from TEE
                keccak256(vm.envBytes("PCR_2")), // Get from TEE
                keccak256(vm.envBytes("PCR_8")), // Get from TEE
                uint64(vm.envUint("EXPIRATION_TOLERANCE")) // Arbitrary value, usually 24h
            );
            console2.log("Attestation doc verifier deployed to:", address(attestationDocVerifier));
        }
        // 2. Deploy the key manager
        TeeKeyManager keyManager = new TeeKeyManager(attestationDocVerifier);

        // 3. Deploy the assertion poster
        address assertionPosterAddress = vm.envOr("ASSERTION_POSTER_ADDRESS", address(0));
        AssertionPoster poster;
        if (assertionPosterAddress != address(0)) {
            console2.log("Assertion poster already deployed to:", assertionPosterAddress);
            poster = AssertionPoster(assertionPosterAddress);
        } else {
            console2.log("Deploying assertion poster...");
            IRollup rollup = IRollup(vm.envAddress("ROLLUP_CONTRACT_ADDRESS"));
            poster = new AssertionPoster(rollup);
        }

        // 4. Deploy the tee module
        IBridge bridge = IBridge(vm.envAddress("APPCHAIN_BRIDGE_ADDRESS"));
        address seqContract = vm.envAddress("SEQUENCING_CONTRACT_ADDRESS");
        address seqBridge = vm.envAddress("SEQUENCING_BRIDGE_ADDRESS");
        uint64 setDelay = uint64(vm.envUint("SET_DELAY")); // Usually set to 60s
        bytes32 appBlockHash = vm.envBytes32("APP_BLOCK_HASH"); // Most recent appchain block hash
        bytes32 seqBlockHash = vm.envBytes32("SEQ_BLOCK_HASH"); // Sequencing chain block hash that corresponds to the appchain block hash
        bytes32 l1BatchAcc = vm.envBytes32("L1_BATCH_ACC"); // The sequencing chain start batch accumulator which corresponds to the SEQ_BLOCK_HASH
        uint64 challengeWindowDuration = uint64(vm.envUint("CHALLENGE_WINDOW_DURATION"));
        TeeModule teeModule = new TeeModule(
            poster,
            bridge,
            keccak256(abi.encodePacked(seqContract, seqBridge, setDelay)),
            appBlockHash,
            seqBlockHash,
            l1BatchAcc,
            L1_BLOCK,
            false,
            challengeWindowDuration,
            keyManager
        );
        console2.log("TeeModule deployed to:", address(teeModule));
    }
}
