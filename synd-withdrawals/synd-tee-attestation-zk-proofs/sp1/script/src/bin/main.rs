//! This script is used to execute the program or generate a proof of the program.

use alloy::{primitives::keccak256, sol_types::SolType};
use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};
use synd_tee_attestation_zk_proofs_aws_nitro::{verify_aws_nitro_attestation, PublicValuesStruct};
use synd_tee_attestation_zk_proofs_sp1_script::shared::{
    read_and_decode_attestation_docs, TEE_ATTESTATION_VALIDATION_ELF,
};

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    /// path for the attestation document, it expects a hex encoded file
    #[arg(long)]
    att_doc: String,

    // path for the root certificate, it expects a PEM file
    #[arg(long)]
    root_cert: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    let (cbor_encoded_attestation_document, der_encoded_root_cert) =
        read_and_decode_attestation_docs(&args.att_doc, &args.root_cert);

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&cbor_encoded_attestation_document);
    stdin.write(&der_encoded_root_cert);

    if args.execute {
        // Execute the program
        let (output, report) =
            client.execute(TEE_ATTESTATION_VALIDATION_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let PublicValuesStruct {
            root_cert_hash,
            validity_window_start,
            validity_window_end,
            tee_signing_key,
            pcr_0,
            pcr_1,
            pcr_2,
        } = PublicValuesStruct::abi_decode_validate(output.as_slice()).unwrap();
        println!("root_cert_hash: {root_cert_hash}");
        println!("validity_window_start: {validity_window_start}");
        println!("validity_window_end: {validity_window_end}");
        println!("tee_signing_key: {tee_signing_key}");
        println!("root_cert_hash: {}", hex::encode(keccak256(&der_encoded_root_cert)));
        println!("pcr_0: {}", hex::encode(pcr_0));
        println!("pcr_1: {}", hex::encode(pcr_1));
        println!("pcr_2: {}", hex::encode(pcr_2));

        let expected = verify_aws_nitro_attestation(
            &cbor_encoded_attestation_document,
            &der_encoded_root_cert,
        )
        .expect("Invalid attestation document");
        assert_eq!(tee_signing_key, expected.tee_signing_key);
        assert_eq!(validity_window_start, expected.validity_window_start);
        assert_eq!(validity_window_end, expected.validity_window_end);
        assert_eq!(root_cert_hash, keccak256(&der_encoded_root_cert));
        assert_eq!(pcr_0, keccak256(expected.pcr_0));
        assert_eq!(pcr_1, keccak256(expected.pcr_1));
        assert_eq!(pcr_2, keccak256(expected.pcr_2));

        println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(TEE_ATTESTATION_VALIDATION_ELF);

        // Generate the proof
        let proof = client.prove(&pk, &stdin).run().expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
