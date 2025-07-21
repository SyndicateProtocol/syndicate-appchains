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

use alloy::{
    hex,
    network::EthereumWallet,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
};
use clap::Parser;
#[cfg(not(debug_assertions))]
use contract_bindings::synd::attestation_doc_verifier::AttestationDocVerifier;
use contract_bindings::synd::tee_key_manager::TeeKeyManager::{self, TeeKeyManagerInstance};
use shared::parse::parse_address;
#[cfg(not(debug_assertions))]
use sp1_sdk::{HashableKey, ProverClient};
use std::{path::PathBuf, str::FromStr};
use synd_tee_attestation_zk_proofs_aws_nitro::verify_aws_nitro_attestation;
use synd_tee_attestation_zk_proofs_sp1_script::shared::TEE_ATTESTATION_VALIDATION_ELF;
use synd_tee_attestation_zk_proofs_submitter::{
    generate_proof, get_attestation_doc, pem_to_der, GenerateProofResult, ProofSubmitterError,
    ProofSystem, AWS_NITRO_ROOT_CERT_PEM,
};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

/// The arguments for the command.
#[derive(Parser, Debug)]
pub struct Args {
    /// The URL of the enclave RPC server
    #[arg(long, required = true)]
    enclave_rpc_url: String,

    /// path for the root certificate in PEM format. Will use the built-in aws nitro root
    /// certificate if not provided.
    #[arg(long)]
    root_certificate_path: Option<PathBuf>,

    #[arg(long, value_enum, default_value = "groth16")]
    proof_system: ProofSystem,

    /// The address of the `TeeKeyManager` contract to submit the proof to
    /// (if missing, on-chain submission will be skipped)
    #[arg(long, value_parser = parse_address)]
    contract_address: Option<Address>,

    /// The URL of the chain RPC server
    /// (if missing, on-chain submission will be skipped)
    #[arg(long)]
    chain_rpc_url: Option<String>,

    /// The private key to submit the proof
    /// (if missing, on-chain submission will be skipped)
    #[arg(long)]
    private_key: Option<String>,

    #[arg(long)]
    elf_file_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    // setup tracing subscriber, default to INFO level
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy(),
        )
        .init();
    let args = Args::parse();
    match run(args, generate_proof).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
}

#[allow(clippy::cognitive_complexity)]
async fn run(
    args: Args,
    generate_proof_fn: impl Fn(
        Vec<u8>,
        Vec<u8>,
        ProofSystem,
        Vec<u8>,
    ) -> Result<GenerateProofResult, ProofSubmitterError>,
) -> Result<(), ProofSubmitterError> {
    // get attestation doc CBOR
    let attestation_doc_hex = get_attestation_doc(args.enclave_rpc_url).await?;
    info!("Attestation doc: {attestation_doc_hex}");
    let cbor_attestation_doc = hex::decode(attestation_doc_hex)?;

    // get root certificate DER
    let pem_root_cert = if let Some(root_certificate_path) = args.root_certificate_path {
        std::fs::read(root_certificate_path).map_err(|e| {
            info!("Error reading root certificate: {e}");
            ProofSubmitterError::ReadRootCertificate(e)
        })?
    } else {
        AWS_NITRO_ROOT_CERT_PEM.to_vec()
    };

    let der_root_cert = pem_to_der(&pem_root_cert)?;

    // make sure the attestation is vaild for the provided root certificate
    verify_aws_nitro_attestation(&cbor_attestation_doc, &der_root_cert)
        .map_err(ProofSubmitterError::InvalidAttestationDocument)?;

    let custom_elf_bytes = args
        .elf_file_path
        .map(|path| {
            std::fs::read(path).map_err(|e| {
                info!("Error reading ELF file: {e}");
                ProofSubmitterError::ReadElfFile(e)
            })
        })
        .transpose()?;

    let elf_bytes = custom_elf_bytes.map_or_else(
        || TEE_ATTESTATION_VALIDATION_ELF.into(),
        |bytes| {
            info!("Using custom ELF bytes");
            bytes
        },
    );

    match (args.chain_rpc_url, args.private_key, args.contract_address) {
        (Some(chain_rpc_url), Some(private_key), Some(contract_address)) => {
            let signer = PrivateKeySigner::from_str(&private_key)?;
            let provider = ProviderBuilder::new()
                .wallet(EthereumWallet::from(signer))
                .connect(chain_rpc_url.as_str())
                .await?;
            let contract = TeeKeyManager::new(contract_address, provider);

            // assert our ELF file matches the contract's vkey before generating the proof
            #[cfg(not(debug_assertions))]
            assert_vkey_matches(&elf_bytes, contract.clone()).await?;

            let proof = generate_proof_fn(
                cbor_attestation_doc,
                der_root_cert,
                args.proof_system,
                elf_bytes,
            )?;

            info!("Public values: 0x{}", hex::encode(&proof.public_values));
            info!("Proof: 0x{}", hex::encode(&proof.proof));

            submit_proof_to_chain(contract, proof).await?;
        }
        _ => {
            info!("Skipping submission to chain");

            let proof = generate_proof_fn(
                cbor_attestation_doc,
                der_root_cert,
                args.proof_system,
                elf_bytes,
            )?;
            info!("Public values: 0x{}", hex::encode(&proof.public_values));
            info!("Proof: 0x{}", hex::encode(&proof.proof));
        }
    }

    Ok(())
}

/// (this can only run on release builds, otherwise ProviderClient setup will fail)
#[cfg(not(debug_assertions))]
async fn assert_vkey_matches<P: Provider>(
    elf_bytes: &[u8],
    contract: TeeKeyManagerInstance<P>,
) -> Result<(), ProofSubmitterError> {
    let (_, vk) = ProverClient::from_env().setup(elf_bytes);
    info!("using vkey: {}", vk.bytes32());

    let att_doc_verifier_address = contract.attestationDocVerifier().call().await.map_err(|e| {
        info!("Error getting attestation doc verifier address: {e}");
        ProofSubmitterError::GetAttestationDocVerifierAddress(e)
    })?;
    let att_doc_verifier_contract =
        AttestationDocVerifier::new(att_doc_verifier_address, contract.provider());

    let att_doc_verifier_vkey =
        att_doc_verifier_contract.attestationDocVerifierVKey().call().await.map_err(|e| {
            info!("Error getting attestation doc verifier vkey hash: {e}");
            ProofSubmitterError::GetAttestationDocVerifierVKeyHash(e)
        })?;

    if vk.bytes32_raw() != att_doc_verifier_vkey {
        return Err(ProofSubmitterError::VkeyMismatch);
    }
    Ok(())
}

async fn submit_proof_to_chain<P: Provider>(
    contract: TeeKeyManagerInstance<P>,
    proof: GenerateProofResult,
) -> Result<(), ProofSubmitterError> {
    let tx = contract.addKey(proof.public_values.into(), proof.proof.into());

    let receipt = tx
        .send()
        .await
        .map_err(|e| {
            info!("Error sending transaction: {e}");
            ProofSubmitterError::SubmitProofToChain(e.to_string())
        })?
        .get_receipt()
        .await
        .map_err(|e| {
            info!("Error getting receipt: {e}");
            ProofSubmitterError::SubmitProofToChain(e.to_string())
        })?;

    info!("Successfully submitted proof to chain. Receipt: {receipt:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::FixedBytes,
        providers::{ext::AnvilApi, WalletProvider},
    };
    use contract_bindings::{
        sp1::{sp1_verifier_gateway::SP1VerifierGateway, sp1_verifier_groth16::SP1VerifierGroth16},
        synd::{attestation_doc_verifier::AttestationDocVerifier, tee_key_manager::TeeKeyManager},
    };
    use serde::Deserialize;
    use test_utils::{anvil::start_anvil, chain_info::PRIVATE_KEY};

    // NOTE this test relies on the groth16 fixture in synd-contracts (that is generated by
    // executing the binary in sp1/script/evm )
    #[tokio::test]
    async fn post_attestation_proof_onchain() {
        let chain_info = start_anvil(1).await.unwrap();
        let provider = chain_info.provider;
        provider.anvil_set_auto_mine(true).await.unwrap();
        provider.anvil_set_time(1748509951).await.unwrap();

        let sp1_verifier_gateway_contract =
            SP1VerifierGateway::deploy(provider.clone(), provider.default_signer_address())
                .await
                .unwrap();
        let sp1_verifier_contract = SP1VerifierGroth16::deploy(provider.clone()).await.unwrap();

        let version = sp1_verifier_contract.VERSION().call().await.unwrap();
        assert_eq!(version, "v5.0.0");

        sp1_verifier_gateway_contract
            .addRoute(*sp1_verifier_contract.address())
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct Groth16Fixture {
            vkey: String,
            public_values: String,
            proof: String,
            root_cert_hash: String,
            pcr0: String,
            pcr1: String,
            pcr2: String,
            pcr8: String,
        }

        let fixture_str = include_str!(
            "../../../../synd-contracts/test/withdrawal/fixtures/groth16-fixture.json"
        );
        let fixture: Groth16Fixture = serde_json::from_str(fixture_str).unwrap();

        let attestation_doc_verifier_v_key = FixedBytes::from_str(&fixture.vkey).unwrap();
        let root_cert_hash = FixedBytes::from_str(&fixture.root_cert_hash).unwrap();
        let pcr0 = FixedBytes::from_str(&fixture.pcr0).unwrap();
        let pcr1 = FixedBytes::from_str(&fixture.pcr1).unwrap();
        let pcr2 = FixedBytes::from_str(&fixture.pcr2).unwrap();
        let pcr8 = FixedBytes::from_str(&fixture.pcr8).unwrap();

        let proof_bytes =
            hex::decode(fixture.proof.strip_prefix("0x").unwrap_or(&fixture.proof)).unwrap();
        let public_values_bytes =
            hex::decode(fixture.public_values.strip_prefix("0x").unwrap_or(&fixture.public_values))
                .unwrap();

        let expiration_tolerance = 3600; // 1 hour

        let attestation_doc_verifier_contract = AttestationDocVerifier::deploy(
            provider.clone(),
            *sp1_verifier_gateway_contract.address(),
            attestation_doc_verifier_v_key,
            root_cert_hash,
            pcr0,
            pcr1,
            pcr2,
            pcr8,
            expiration_tolerance,
        )
        .await
        .unwrap();

        let key_mgr_contract =
            TeeKeyManager::deploy(provider.clone(), *attestation_doc_verifier_contract.address())
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
            enclave_rpc_url: mock_enclave_server.url(),
            root_certificate_path: None,
            proof_system: ProofSystem::Groth16,
            contract_address: Some(*key_mgr_contract.address()),
            chain_rpc_url: Some(chain_info.ws_url.to_string()),
            private_key: Some(PRIVATE_KEY.to_string()),
            elf_file_path: None,
        };

        let mock_generate_proof = |_: Vec<u8>,
                                   _: Vec<u8>,
                                   _: ProofSystem,
                                   _: Vec<u8>|
         -> Result<GenerateProofResult, ProofSubmitterError> {
            Ok(GenerateProofResult {
                proof: proof_bytes.clone(),
                public_values: public_values_bytes.clone(),
            })
        };

        let result = run(args, mock_generate_proof).await;
        drop(mock_enclave_server);

        assert!(result.is_ok(), "run function failed: {:?}", result.err());
    }
}
