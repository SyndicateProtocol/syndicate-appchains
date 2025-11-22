//! A simple program that takes an attestation document and a root certificate as input, validates
//! the certificate validity and commits to its contents

#![no_main]
#![no_std]
#![allow(unexpected_cfgs)] // succinct is not an official rust target

#[cfg(all(target_os = "zkvm", feature = "openvm"))]
use openvm::io::read_vec;
#[cfg(all(target_os = "zkvm", feature = "risc0"))]
use risc0_zkvm::guest::env::{commit_slice, read_frame as read_vec};
#[cfg(all(target_os = "zkvm", target_vendor = "succinct"))]
use sp1_zkvm::io::{commit_slice, read_vec};
use synd_tee_attestation_zk_proofs_aws_nitro::verify_aws_nitro_attestation;

#[cfg(all(target_os = "zkvm", feature = "openvm"))]
openvm::init!();

#[cfg(all(target_os = "zkvm", feature = "openvm"))]
fn commit_slice(data: &[u8]) {
    openvm::io::reveal_bytes32(openvm_keccak256::keccak256(data));
}

#[cfg(not(target_os = "zkvm"))]
extern crate alloc;

#[cfg(not(target_os = "zkvm"))]
fn read_vec() -> alloc::vec::Vec<u8> {
    unimplemented!()
}

#[cfg(not(target_os = "zkvm"))]
fn commit_slice(_: &[u8]) {
    unimplemented!()
}

#[no_mangle]
pub fn main() {
    // Read an input to the program.
    let cbor_encoded_attestation_document = read_vec();

    // Verify the attestation document & encode the public values of the program
    let bytes = verify_aws_nitro_attestation(&cbor_encoded_attestation_document)
        .expect("Invalid attestation document")
        .public_data();

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    commit_slice(bytes.as_slice());
}
