//! This script is used to execute the program or generate a proof of the program.

#[path = "../utils.rs"]
mod utils;

use alloy_sol_types::SolType;
use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use synd_tee_attestion_zk_proofs_aws_nitro::{verify_aws_nitro_attestation, PublicValuesStruct};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const X509_ELF: &[u8] = include_elf!("synd-tee-attestion-zk-proofs-sp1-program");

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
        utils::read_and_decode_attestation_docs(&args.att_doc, &args.root_cert);

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&cbor_encoded_attestation_document);
    stdin.write(&der_encoded_root_cert);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(X509_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = PublicValuesStruct::abi_decode(output.as_slice()).unwrap();
        let PublicValuesStruct {
            cbor_encoded_attestation_document,
            der_encoded_root_cert,
            validity_window_start,
            validity_window_end,
            public_key,
        } = decoded;
        println!("cbor_encoded_attestation_document: {}", cbor_encoded_attestation_document);
        println!("der_encoded_root_cert: {}", der_encoded_root_cert);
        println!("validity_window_start: {}", validity_window_start);
        println!("validity_window_end: {}", validity_window_end);
        println!("public_key: {}", public_key);

        let (expected_public_key, expected_validity_window_start, expected_validity_window_end) =
            verify_aws_nitro_attestation(
                &cbor_encoded_attestation_document,
                &der_encoded_root_cert,
            )
            .expect("Invalid attestation document");
        assert_eq!(expected_public_key, public_key);
        assert_eq!(expected_validity_window_start, validity_window_start);
        assert_eq!(expected_validity_window_end, validity_window_end);

        println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(X509_ELF);

        // Generate the proof
        let proof = client.prove(&pk, &stdin).run().expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
