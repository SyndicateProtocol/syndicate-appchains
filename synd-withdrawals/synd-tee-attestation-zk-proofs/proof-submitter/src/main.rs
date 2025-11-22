//! A command-line tool for generating and submitting TEE attestation ZK proofs.
//!
//! This tool orchestrates the process of obtaining a TEE attestation document
//! from an AWS Nitro Enclave, generating a zero-knowledge proof of its validity
//! using SP1, and submitting this proof to an on-chain contract.
//!
//! ## Functionality
//!
//! 1. **Fetch Attestation**: Retrieves the attestation document from a specified enclave RPC
//!    endpoint.
//! 2. **Verify Attestation**: Validates the attestation document against a provided or default root
//!    certificate.
//! 3. **Generate ZK Proof**: Creates a ZK proof (either Groth16 or Plonk) for the attestation
//!    verification logic. The proof generation is handled by SP1.
//! 4. **Submit On-chain**: If configured, it submits the generated proof and public values to a
//!    `TeeKeyManager` smart contract on an EVM-compatible chain.
//!
//! ## Usage
//!
//! The tool is configured via command-line arguments. Key parameters include the
//! enclave URL, chain RPC URL, contract address, and the private key for on-chain
//! transactions.
#![allow(clippy::unwrap_used)]

mod proof;
mod utils;

use crate::{
    proof::{generate_proof, get_vkey, ProofSystem},
    utils::{
        assert_vkey_and_data_hash_match, get_attestation_doc, get_elf_bytes, submit_proof_to_chain,
    },
};
use alloy::{
    hex,
    network::EthereumWallet,
    primitives::{fixed_bytes, Address, B256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use clap::Parser;
use contract_bindings::synd::{
    attestation_doc_verifier::AttestationDocVerifier,
    tee_key_manager::TeeKeyManager::{self},
};
use eyre::{eyre, OptionExt};
use shared::parse::parse_address;
use std::{path::PathBuf, str::FromStr, sync::Arc, time::Duration};
use synd_tee_attestation_zk_proofs_aws_nitro::verify_aws_nitro_attestation;
use tokio::task::spawn_blocking;
use tracing::info;
use zeroize::{Zeroize, Zeroizing};

// openvm keygen needs a larger stack size
const THREAD_STACK_SIZE: usize = 4 * 1024 * 1024;

const ROOT_CERT_HASH: B256 =
    fixed_bytes!("0x311d96fcd5c5e0ccf72ef548e2ea7d4c0cd53ad7c4cc49e67471aed41d61f185");

/// The arguments for the command.
#[derive(Parser, Debug)]
pub struct Args {
    /// The URL of the enclave RPC server
    #[arg(long)]
    enclave_rpc_url: Option<String>,

    #[arg(long)]
    attestation_document: Option<String>,

    /// path for the root certificate in PEM format. Will use the built-in aws nitro root
    /// certificate if not provided.
    #[arg(long)]
    root_certificate_path: Option<PathBuf>,

    #[arg(long, value_enum)]
    proof_system: ProofSystem,

    /// The address of the `TeeKeyManager` contract to submit the proof to
    /// (if missing, on-chain submission will be skipped)
    #[arg(long, value_parser = parse_address)]
    contract_address: Option<Address>,

    /// If passed, a new `TeeKeyManager` contract with a respective new `AttestationDocVerifier`
    /// contract will be deployed - see: <https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments>
    #[arg(long, value_parser = parse_address)]
    deploy_new_contract_with_sp1_verifier: Option<Address>,

    /// The expiration tolerance to be used if a new contract is deployed
    /// (default is 24 hours)
    #[arg(long, default_value = "24h",  value_parser = humantime::parse_duration )]
    deploy_expiration_tolerance: Duration,

    /// The URL of the chain RPC server
    /// (if missing, on-chain submission will be skipped)
    #[arg(long)]
    chain_rpc_url: Option<String>,

    /// The private key to submit the proof
    /// (if missing, on-chain submission will be skipped)
    #[arg(long)]
    private_key: Option<Zeroizing<String>>,

    #[arg(long)]
    elf_file_path: PathBuf,

    #[arg(long, default_value_t = false)]
    mock_proof: bool,
}

fn runtime(mut builder: tokio::runtime::Builder) -> tokio::runtime::Runtime {
    builder.enable_all().thread_stack_size(THREAD_STACK_SIZE).build().unwrap()
}

fn main() {
    _ = shared::tracing::setup_global_logging();
    runtime(tokio::runtime::Builder::new_multi_thread()).block_on(async {
        let args = Args::parse();
        match run(args).await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    })
}

#[allow(clippy::cognitive_complexity)]
async fn run(args: Args) -> eyre::Result<()> {
    let attestation_doc_hex = match (args.attestation_document, args.enclave_rpc_url) {
        (Some(attestation_document), None) => attestation_document,
        (None, Some(enclave_rpc_url)) => get_attestation_doc(enclave_rpc_url).await?,
        (Some(_), Some(_)) => {
            return Err(eyre!("attestation document and enclave rpc url are mutually exclusive"))
        }
        (None, None) => return Err(eyre!("attestation document or enclave rpc url required")),
    };

    // get attestation doc CBOR
    info!("Attestation doc: {attestation_doc_hex}");
    let cbor_attestation_doc = hex::decode(attestation_doc_hex)?;

    // make sure the attestation is vaild for the provided root certificate
    let attestation_result = verify_aws_nitro_attestation(&cbor_attestation_doc)
        .map_err(|e| eyre!("invalid attestation document: {e:?}"))?;
    let public_data = attestation_result.public_data();

    if attestation_result.root_cert_hash != ROOT_CERT_HASH {
        return Err(eyre!(
            "Root certificate hash mismatch: got {}, expected {}",
            attestation_result.root_cert_hash,
            ROOT_CERT_HASH,
        ));
    }

    info!(
        "Attestation valid - signing key: {}, public data: {}, root cert hash: {}",
        attestation_result.tee_signing_key,
        hex::encode(public_data),
        attestation_result.root_cert_hash
    );

    let elf_bytes = Arc::new(get_elf_bytes(args.elf_file_path, args.proof_system).await?);
    let vk_bytes = spawn_blocking({
        let elf_bytes = elf_bytes.clone();
        move || get_vkey(args.proof_system, &elf_bytes)
    })
    .await
    .unwrap()?;
    info!("Vkey: 0x{}", hex::encode(vk_bytes));

    let Some(chain_rpc_url) = args.chain_rpc_url else {
        info!("Skipping submission to chain");

        let proof = spawn_blocking(move || {
            generate_proof(
                &cbor_attestation_doc,
                args.proof_system,
                args.mock_proof,
                &elf_bytes,
                &public_data,
            )
        })
        .await
        .unwrap()?;
        info!("Proof: 0x{}", hex::encode(&proof));
        return Ok(());
    };
    info!("Submitting proof to chain");
    let mut private_key = args.private_key.ok_or_eyre("private key required")?;
    let signer = PrivateKeySigner::from_str(&private_key)?;
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect(chain_rpc_url.as_str())
        .await?;
    private_key.zeroize(); // zeroize the private key after use

    let contract_address = match (args.contract_address, args.deploy_new_contract_with_sp1_verifier)
    {
        (Some(contract_address), None) => contract_address,
        (None, Some(verifier_address)) => {
            info!("Deploying new attestation doc verifier contract");

            let git_hash = env!("GIT_HASH");
            info!("synd-appchains commit hash: {}", git_hash);

            let attestation_doc_verifier_contract = AttestationDocVerifier::deploy(
                provider.clone(),
                verifier_address,
                vk_bytes.into(),
                attestation_result.data_hash(),
                args.deploy_expiration_tolerance.as_secs(),
                git_hash.into(),
                args.proof_system as u8,
            )
            .await
            .map_err(|e| eyre!("Error deploying attestation doc verifier contract: {e}"))?;
            info!(
                "Attestation doc verifier contract deployed to: {}",
                attestation_doc_verifier_contract.address()
            );

            let contract = TeeKeyManager::deploy(
                provider.clone(),
                *attestation_doc_verifier_contract.address(),
            )
            .await
            .map_err(|e| eyre!("Error deploying tee key manager contract: {e}"))?;
            info!("Tee key manager contract deployed to: {}", contract.address());
            *contract.address()
        }
        (None, None) => return Err(eyre!("contract address required")),
        (Some(_), Some(_)) => {
            return Err(eyre!("contract address and deploy are mutally exclusive"))
        }
    };

    info!("contract address: {}", contract_address);
    let contract = TeeKeyManager::new(contract_address, provider);

    // assert our ELF file matches the contract's vkey before generating the proof
    assert_vkey_and_data_hash_match(
        &vk_bytes,
        &attestation_result.data_hash(),
        args.proof_system,
        contract.clone(),
    )
    .await?;

    let proof = spawn_blocking({
        let elf_bytes = elf_bytes.clone();
        move || {
            generate_proof(
                &cbor_attestation_doc,
                args.proof_system,
                args.mock_proof,
                &elf_bytes,
                &public_data,
            )
        }
    })
    .await
    .unwrap()?;
    info!("Proof: 0x{}", hex::encode(&proof));
    let receipt =
        submit_proof_to_chain(contract, attestation_result.calldata().into(), proof.into()).await?;
    info!("Successfully submitted proof to chain. Receipt: {receipt:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::ext::AnvilApi;
    use contract_bindings::synd::{
        attestation_doc_verifier::AttestationDocVerifier, tee_key_manager::TeeKeyManager,
    };
    use test_utils::{anvil::start_anvil, chain_info::PRIVATE_KEY};

    alloy::sol! {
        #[sol(rpc, bytecode = "0x60038060093d393df35f5ff3")]
        contract DummyVerifier {
            fallback() external payable {}
        }
    }

    const ELF_DIR: &str = concat!(
        env!("CARGO_WORKSPACE_DIR"),
        "/synd-withdrawals/synd-tee-attestation-zk-proofs/proof-submitter/elfs"
    );

    #[ctor::ctor]
    fn init() {
        _ = shared::tracing::setup_global_logging();
    }

    #[cfg(feature = "risc0")]
    #[test]
    fn post_attestation_proof_onchain_risc0() {
        runtime(tokio::runtime::Builder::new_current_thread()).block_on(
            post_attestation_proof_onchain_base(
                ProofSystem::RISC0,
                PathBuf::from(ELF_DIR).join("risc0.elf"),
            ),
        )
    }

    #[cfg(all(feature = "sp1", not(debug_assertions)))]
    #[test]
    fn post_attestation_proof_onchain_sp1() {
        runtime(tokio::runtime::Builder::new_current_thread()).block_on(
            post_attestation_proof_onchain_base(
                ProofSystem::SP1,
                PathBuf::from(ELF_DIR).join("sp1.elf"),
            ),
        )
    }

    #[cfg(all(feature = "openvm", not(debug_assertions)))]
    #[test]
    fn post_attestation_proof_onchain_openvm() {
        runtime(tokio::runtime::Builder::new_current_thread()).block_on(
            post_attestation_proof_onchain_base(
                ProofSystem::OpenVM,
                PathBuf::from(ELF_DIR).join("openvm.elf"),
            ),
        )
    }

    async fn post_attestation_proof_onchain_base(
        proof_system: ProofSystem,
        elf_file_path: PathBuf,
    ) {
        let chain_info = start_anvil(1).await.unwrap();
        let provider = chain_info.provider;
        provider.anvil_set_auto_mine(true).await.unwrap();
        provider.anvil_set_time(1748509951).await.unwrap();

        let verifier = DummyVerifier::deploy(&provider).await.unwrap();
        let elf_bytes = get_elf_bytes(elf_file_path.clone(), proof_system).await.unwrap();
        let attestation_doc_verifier_contract = AttestationDocVerifier::deploy(
            &provider,
            verifier.address().to_owned(),
            spawn_blocking(move || get_vkey(proof_system, &elf_bytes))
                .await
                .unwrap()
                .unwrap()
                .into(),
            fixed_bytes!("0xb81743c43da8243554a4c316218f9ae15786a3e5c2e19ed404244df90fc5edc5"),
            0,
            Default::default(),
            proof_system as u8,
        )
        .await
        .unwrap();

        let key_mgr_contract =
            TeeKeyManager::deploy(&provider, *attestation_doc_verifier_contract.address())
                .await
                .unwrap();

        let mut mock_enclave_server = mockito::Server::new_async().await;

        let attestation_doc_hex = include_str!("../../aws-nitro/src/testdata/att_doc_sample_2.hex");
        let mock_response =
            format!(r#"{{"jsonrpc":"2.0","id":0,"result":"{}"}}"#, attestation_doc_hex.trim());

        mock_enclave_server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.clone())
            .match_body(mockito::Matcher::JsonString(
                r#"{"jsonrpc":"2.0","method":"enclave_signerAttestation","params":[],"id":0}"#
                    .to_string(),
            ))
            .create_async()
            .await;

        let args = Args {
            enclave_rpc_url: Some(mock_enclave_server.url()),
            attestation_document: None,
            root_certificate_path: None,
            proof_system,
            contract_address: Some(*key_mgr_contract.address()),
            deploy_new_contract_with_sp1_verifier: None,
            deploy_expiration_tolerance: Duration::from_secs(3600),
            chain_rpc_url: Some(chain_info.ws_url.clone()),
            private_key: Some(Zeroizing::new(PRIVATE_KEY.to_string())),
            elf_file_path,
            mock_proof: true,
        };

        let result = run(args).await;
        drop(mock_enclave_server);

        assert!(result.is_ok(), "run function failed: {:?}", result.err());
    }
}
