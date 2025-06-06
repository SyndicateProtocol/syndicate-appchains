use sp1_sdk::{HashableKey, Prover, ProverClient};
use synd_tee_attestation_zk_proofs_sp1_script::shared::TEE_ATTESTATION_VALIDATION_ELF;

fn main() {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(TEE_ATTESTATION_VALIDATION_ELF);
    println!("{}", vk.bytes32());
}
