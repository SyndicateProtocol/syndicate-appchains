//! This script is used to generate a proof of a program that can be verified on-chain.

use alloy::sol_types::SolValue;
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use sp1_sdk::{HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};
use std::path::PathBuf;
use synd_tee_attestation_zk_proofs_aws_nitro::PublicValuesStruct;
use synd_tee_attestation_zk_proofs_sp1_script::shared::{
    read_and_decode_attestation_docs, TEE_ATTESTATION_VALIDATION_ELF,
};

/// The arguments for the EVM command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct EVMArgs {
    /// path for the attestation document, it expects a hex encoded file
    #[arg(long)]
    att_doc: String,

    // path for the root certificate, it expects a PEM file
    #[arg(long)]
    root_cert: String,

    #[arg(long, value_enum, default_value = "groth16")]
    system: ProofSystem,
}

/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    Plonk,
    Groth16,
}

/// A fixture that can be used to test the verification of SP1 zkVM proofs inside Solidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1x509ProofFixture {
    vkey: String,
    public_values: String,
    proof: String,
    root_cert_hash: String,
    pcr_0: String,
    pcr_1: String,
    pcr_2: String,
    pcr_8: String,
}

fn main() {
    // Set up the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = EVMArgs::parse();

    // Set up the prover client.
    let client = ProverClient::from_env();

    // Set up the program.
    let (pk, vk) = client.setup(TEE_ATTESTATION_VALIDATION_ELF);

    // Set up the inputs.
    let mut stdin = SP1Stdin::new();

    let (cbor_encoded_attestation_document, der_encoded_root_cert) =
        read_and_decode_attestation_docs(&args.att_doc, &args.root_cert);

    stdin.write(&cbor_encoded_attestation_document);
    stdin.write(&der_encoded_root_cert);

    println!("Proof System: {:?}", args.system);

    // Generate the proof based on the selected proof system.
    let proof = match args.system {
        ProofSystem::Plonk => client.prove(&pk, &stdin).plonk().run(),
        ProofSystem::Groth16 => client.prove(&pk, &stdin).groth16().run(),
    }
    .expect("failed to generate proof");

    create_proof_fixture(&proof, &vk, args.system);
}

/// Create a fixture for the given proof.
fn create_proof_fixture(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
    system: ProofSystem,
) {
    // Deserialize the public values.
    let public_values_bytes = proof.public_values.as_slice();

    let pub_values = PublicValuesStruct::abi_decode(public_values_bytes, true).unwrap();

    // Create the testing fixture so we can test things end-to-end.
    let fixture = SP1x509ProofFixture {
        vkey: vk.bytes32().to_string(),
        public_values: format!("0x{}", hex::encode(public_values_bytes)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
        root_cert_hash: format!("0x{}", hex::encode(pub_values.root_cert_hash)),
        pcr_0: format!("0x{}", hex::encode(pub_values.pcr_0)),
        pcr_1: format!("0x{}", hex::encode(pub_values.pcr_1)),
        pcr_2: format!("0x{}", hex::encode(pub_values.pcr_2)),
        pcr_8: format!("0x{}", hex::encode(pub_values.pcr_8)),
    };

    // The verification key is used to verify that the proof corresponds to the execution of the
    // program on the given input.
    //
    // Note that the verification key stays the same regardless of the input.
    println!("Verification Key: {}", fixture.vkey);

    // The public values are the values which are publicly committed to by the zkVM.
    //
    // If you need to expose the inputs or outputs of your program, you should commit them in
    // the public values.
    println!("Public Values: {}", fixture.public_values);

    // The proof proves to the verifier that the program was executed with some inputs that led to
    // the give public values.
    println!("Proof Bytes: {}", fixture.proof);

    // Save the fixture to a file.
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../synd-contracts/test/withdrawal/fixtures");
    std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");
    std::fs::write(
        fixture_path.join(format!("{system:?}-fixture.json").to_lowercase()),
        serde_json::to_string_pretty(&fixture).unwrap(),
    )
    .expect("failed to write fixture");
}
