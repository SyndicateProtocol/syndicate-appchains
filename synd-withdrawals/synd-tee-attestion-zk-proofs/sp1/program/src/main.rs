//! A simple program that takes an attestation document and a root certificate as input, and writes
//! the public key and validity window start and end as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy::{primitives::keccak256, sol_types::SolType};
use synd_tee_attestion_zk_proofs_aws_nitro::{
    verify_aws_nitro_attestation, PublicValuesStruct, ValidationResult,
};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let cbor_encoded_attestation_document = sp1_zkvm::io::read::<Vec<u8>>();
    let der_encoded_root_cert = sp1_zkvm::io::read::<Vec<u8>>();

    // Verify the attestation document and get the public key and validity window
    let ValidationResult {
        tee_signing_key,
        validity_window_start,
        validity_window_end,
        pcr_0,
        pcr_1,
        pcr_2,
        pcr_8,
    } = verify_aws_nitro_attestation(&cbor_encoded_attestation_document, &der_encoded_root_cert)
        .expect("Invalid attestation document");

    // Encode the public values of the program.
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        root_cert_hash: keccak256(&der_encoded_root_cert),
        validity_window_start,
        validity_window_end,
        tee_signing_key,
        pcr_0: keccak256(&pcr_0),
        pcr_1: keccak256(&pcr_1),
        pcr_2: keccak256(&pcr_2),
        pcr_8: keccak256(&pcr_8),
    });

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
