use sp1_sdk::{include_elf, HashableKey, Prover, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const X509_ELF: &[u8] = include_elf!("synd-tee-attestion-zk-proofs-sp1-program");

fn main() {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(X509_ELF);
    println!("{}", vk.bytes32());
}
