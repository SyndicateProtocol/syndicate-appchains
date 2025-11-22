#![allow(unreachable_pub)]

use alloy::hex;
use clap::ValueEnum;
use eyre::eyre;
use tracing::info;

#[cfg(feature = "openvm")]
const OPENVM_CONFIG: &str = include_str!("../../program/openvm.toml");

#[allow(missing_docs)]
/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ProofSystem {
    #[cfg(feature = "risc0")]
    RISC0 = 0,
    #[cfg(feature = "sp1")]
    SP1 = 1,
    #[cfg(feature = "openvm")]
    OpenVM = 2,
}

/// Get vkey
pub fn get_vkey(proof_system: ProofSystem, elf_bytes: &[u8]) -> eyre::Result<[u8; 32]> {
    match proof_system {
        #[cfg(feature = "risc0")]
        ProofSystem::RISC0 => Ok(risc0_binfmt::compute_image_id(elf_bytes)
            .map_err(|e| eyre!("risc0: error computing image id: {e}"))?
            .as_bytes()
            .try_into()
            .map_err(|e| eyre!("risc0: invalid image id: {e}"))?),
        #[cfg(feature = "sp1")]
        ProofSystem::SP1 => {
            use sp1_sdk::HashableKey as _;

            let client = sp1_sdk::ProverClient::from_env();
            let (_, vk) = client.setup(elf_bytes);
            Ok(vk.bytes32_raw())
        }
        #[cfg(feature = "openvm")]
        ProofSystem::OpenVM => {
            let sdk =
                openvm_sdk::Sdk::new(openvm_sdk::config::SdkVmConfig::from_toml(OPENVM_CONFIG)?)?;
            let commit = sdk.app_prover(elf_bytes)?.app_commit();
            info!("openvm app vm commit: 0x{}", hex::encode(commit.app_vm_commit.as_slice()));
            Ok(commit.app_exe_commit.as_slice().to_owned())
        }
    }
}

/// Generate a ZK proof for the TEE attestation document
#[allow(clippy::cognitive_complexity)]
pub fn generate_proof(
    cbor_attestation_doc: &[u8],
    proof_system: ProofSystem,
    mock_proof: bool,
    elf_bytes: &[u8],
    public_data: &[u8; 60],
) -> eyre::Result<Vec<u8>> {
    match proof_system {
        #[cfg(feature = "risc0")]
        ProofSystem::RISC0 => {
            let env = risc0_zkvm::ExecutorEnv::builder()
                .write_frame(cbor_attestation_doc)
                .build()
                .map_err(|e| eyre!("risc0: error building executor env: {e}"))?;

            let prover = risc0_zkvm::default_prover();

            let receipt = prover
                .prove_with_opts(
                    env,
                    elf_bytes,
                    &risc0_zkvm::ProverOpts::groth16().with_dev_mode(mock_proof),
                )
                .map_err(|e| eyre!("risc0: error generating proof: {e}"))?
                .receipt;
            info!("risc0 verifier parameters digest: {}", receipt.metadata.verifier_parameters);
            if !mock_proof {
                info!("verifying risc0 proof");
                receipt.verify(get_vkey(ProofSystem::RISC0, elf_bytes)?)?;
            }
            if public_data != receipt.journal.bytes.as_slice() {
                return Err(eyre!(
                    "public data mismatch: got {}, expected {}",
                    hex::encode(receipt.journal.bytes),
                    hex::encode(public_data),
                ));
            }
            match receipt.inner {
                risc0_zkvm::InnerReceipt::Groth16(x) => Ok(x.seal),
                risc0_zkvm::InnerReceipt::Fake(_) => Ok(Default::default()),
                risc0_zkvm::InnerReceipt::Composite(_) => panic!("got composite risc0 receipt"),
                risc0_zkvm::InnerReceipt::Succinct(_) => panic!("got succinct risc0 receipt"),
                _ => panic!("got unknown risc0 receipt"),
            }
        }
        #[cfg(feature = "sp1")]
        ProofSystem::SP1 => {
            // Set up the prover client.
            let client: Box<dyn sp1_sdk::Prover<_>> = if mock_proof {
                Box::new(sp1_sdk::CpuProver::mock())
            } else {
                Box::new(sp1_sdk::ProverClient::from_env())
            };

            let (pk, _) = client.setup(elf_bytes);

            let mut stdin = sp1_sdk::SP1Stdin::new();
            stdin.write_slice(cbor_attestation_doc);
            // plonk has less trust assumptions than groth16
            // the verification gas cost for both is similar
            // see https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types for more info
            let proof = client
                .prove(&pk, &stdin, sp1_sdk::SP1ProofMode::Plonk)
                .map_err(|e| eyre!("sp1: error generating proof: {e}"))?;
            info!("sp1 zkvm version: {}", proof.sp1_version);
            if public_data != proof.public_values.as_slice() {
                return Err(eyre!(
                    "public data mismatch: got {}, expected {}",
                    hex::encode(proof.public_values),
                    hex::encode(public_data),
                ));
            }

            Ok(proof.bytes())
        }
        #[cfg(feature = "openvm")]
        ProofSystem::OpenVM => {
            let sdk =
                openvm_sdk::Sdk::new(openvm_sdk::config::SdkVmConfig::from_toml(OPENVM_CONFIG)?)?;

            let input = openvm_sdk::StdIn::from_bytes(cbor_attestation_doc);
            let public_data_hash = alloy::primitives::keccak256(public_data).0;
            if mock_proof {
                let public_values = sdk.execute_metered(elf_bytes, input)?.0;
                if public_values != public_data_hash {
                    return Err(eyre!(
                        "public data mismatch: got {}, expected {}",
                        hex::encode(public_values),
                        hex::encode(public_data_hash),
                    ));
                }
                return Ok(Default::default());
            }

            let agg_pk = openvm_sdk::fs::read_object_from_file(
                std::env::var("HOME")? + "/.openvm/agg_stark.pk",
            )
            .map_err(|e|eyre!("Failed to read aggregation proving key: {e}\nPlease run 'cargo openvm setup --evm' first"))?;

            let halo2_pk = openvm_sdk::fs::read_object_from_file(
                std::env::var("HOME")? + "/.openvm/agg_halo2.pk",
            )
            .map_err(|e|
                eyre!(
                    "Failed to read halo2 proving key: {e}\nPlease run 'cargo openvm setup --evm' first"
                )
            )?;

            let sdk = sdk.with_agg_pk(agg_pk).with_halo2_pk(halo2_pk);

            let proof = sdk.prove_evm(elf_bytes, input)?;
            info!("openvm zkvm version: {}", proof.version);
            if proof.user_public_values != public_data_hash {
                return Err(eyre!(
                    "public data mismatch: got {}, expected {}",
                    hex::encode(proof.user_public_values),
                    hex::encode(public_data_hash),
                ));
            }
            let mut proof_data = proof.proof_data.accumulator.clone();
            proof_data.extend_from_slice(&proof.proof_data.proof);

            info!("verifying openvm proof");
            openvm_sdk::Sdk::verify_evm_halo2_proof(
                &sdk.generate_halo2_verifier_solidity()?,
                proof,
            )?;

            Ok(proof_data)
        }
    }
}
