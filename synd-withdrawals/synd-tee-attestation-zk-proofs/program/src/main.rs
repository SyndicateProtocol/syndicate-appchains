//! A simple program that takes an attestation document and a root certificate as input, validates
//! the certificate validity and commits to its contents

#![no_main]
#![no_std]
#![allow(unexpected_cfgs)] // succinct is not an official rust target

extern crate alloc;
use alloc::vec::Vec;
#[cfg(target_vendor = "risc0")]
use risc0_zkvm::guest::env::{commit_slice, read};
#[cfg(target_vendor = "succinct")]
use sp1_zkvm::io::{commit_slice, read};
use synd_tee_attestation_zk_proofs_aws_nitro::verify_aws_nitro_attestation;

#[cfg(not(any(target_vendor = "succinct", target_vendor = "risc0")))]
fn read<T>() -> T {
    unimplemented!()
}

#[cfg(not(any(target_vendor = "succinct", target_vendor = "risc0")))]
fn commit_slice(_: &[u8]) {
    unimplemented!()
}

#[no_mangle]
pub fn main() {
    // Read an input to the program.
    //
    // this compiles to a custom system call which handles reading inputs from the prover.
    let cbor_encoded_attestation_document = read::<Vec<u8>>();

    // Verify the attestation document & encode the public values of the program
    let bytes = verify_aws_nitro_attestation(&cbor_encoded_attestation_document)
        .expect("Invalid attestation document")
        .public_data();

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    commit_slice(bytes.as_slice());
}
