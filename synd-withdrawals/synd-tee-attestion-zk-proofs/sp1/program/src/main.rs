//! A simple program that takes an attestation document and a root certificate as input, and writes
//! the public key and validity window start and end as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use synd_tee_attestion_zk_proofs_aws_nitro::{verify_aws_nitro_attestation, PublicValuesStruct};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let cbor_encoded_attestation_document = sp1_zkvm::io::read::<Vec<u8>>();
    let der_encoded_root_cert = sp1_zkvm::io::read::<Vec<u8>>();

    // Verify the attestation document and get the public key and validity window
    let (public_key, validity_window_start, validity_window_end) =
        verify_aws_nitro_attestation(&cbor_encoded_attestation_document, &der_encoded_root_cert)
            .expect("Invalid attestation document");

    // Encode the public values of the program.
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        cbor_encoded_attestation_document: cbor_encoded_attestation_document.into(),
        der_encoded_root_cert: der_encoded_root_cert.into(),
        validity_window_start,
        validity_window_end,
        public_key: public_key.into(),
    });

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
