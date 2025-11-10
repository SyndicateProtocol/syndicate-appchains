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
    primitives::{fixed_bytes, Address, Bytes, B256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
};
use clap::Parser;
use contract_bindings::synd::{
    attestation_doc_verifier::AttestationDocVerifier,
    tee_key_manager::TeeKeyManager::{self, TeeKeyManagerInstance},
};
use shared::parse::parse_address;
use std::{path::PathBuf, str::FromStr, time::Duration};
use synd_tee_attestation_zk_proofs_aws_nitro::verify_aws_nitro_attestation;
use synd_tee_attestation_zk_proofs_submitter::{
    generate_proof, get_attestation_doc, get_vkey, ProofSubmitterError, ProofSystem,
};
use tracing::{error, info};
use zeroize::{Zeroize, Zeroizing};

const ROOT_CERT_HASH: B256 =
    fixed_bytes!("0x311d96fcd5c5e0ccf72ef548e2ea7d4c0cd53ad7c4cc49e67471aed41d61f185");

/// from <https://docs.rs/risc0-zkos-v1compat/2.2.0/src/risc0_zkos_v1compat/lib.rs.html>
/// the elf file is located in risc0/zkos/v1compat/elfs/v1compat.elf
const V1COMPAT_ELF: &[u8] = include_bytes!("../elfs/v1compat.elf");

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

    #[arg(long, value_enum, default_value = "sp1")]
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
}

#[tokio::main]
async fn main() {
    _ = shared::tracing::setup_global_logging();
    let args = Args::parse();
    match run(args, generate_proof, get_vkey, get_elf_bytes).await {
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
        &[u8],
        ProofSystem,
        &[u8],
        &[u8; 60],
    ) -> Result<Vec<u8>, ProofSubmitterError>,
    get_vkey_fn: impl Fn(ProofSystem, &[u8]) -> [u8; 32],
    get_elf_bytes_fn: impl Fn(PathBuf, ProofSystem) -> Result<Vec<u8>, ProofSubmitterError>,
) -> Result<(), ProofSubmitterError> {
    let attestation_doc_hex = match (args.attestation_document, args.enclave_rpc_url) {
        (Some(attestation_document), None) => attestation_document,
        (None, Some(enclave_rpc_url)) => get_attestation_doc(enclave_rpc_url).await?,
        (Some(_), Some(_)) => {
            return Err(ProofSubmitterError::AttestationDocumentAndEnclaveRpcUrlAreMutuallyExclusive)
        }
        (None, None) => return Err(ProofSubmitterError::AttestationDocumentOrEnclaveRpcUrlRequired),
    };

    // get attestation doc CBOR
    info!("Attestation doc: {attestation_doc_hex}");
    let cbor_attestation_doc = hex::decode(attestation_doc_hex)?;

    // make sure the attestation is vaild for the provided root certificate
    let attestation_result = verify_aws_nitro_attestation(&cbor_attestation_doc)
        .map_err(ProofSubmitterError::InvalidAttestationDocument)?;
    let public_data = attestation_result.public_data();

    if attestation_result.root_cert_hash != ROOT_CERT_HASH {
        return Err(ProofSubmitterError::RootCertificateHashMismatch(
            attestation_result.root_cert_hash,
            ROOT_CERT_HASH,
        ))
    }

    info!(
        "Attestation valid - signing key: {}, public data: {}, root cert hash: {}",
        attestation_result.tee_signing_key,
        hex::encode(public_data),
        attestation_result.root_cert_hash
    );

    let elf_bytes = get_elf_bytes_fn(args.elf_file_path, args.proof_system)?;
    let vk_bytes = get_vkey_fn(args.proof_system, &elf_bytes);
    info!("Vkey: 0x{}", hex::encode(vk_bytes));

    let Some(chain_rpc_url) = args.chain_rpc_url else {
        info!("Skipping submission to chain");

        let proof =
            generate_proof_fn(&cbor_attestation_doc, args.proof_system, &elf_bytes, &public_data)?;
        info!("Proof: 0x{}", hex::encode(&proof));
        return Ok(());
    };
    info!("Submitting proof to chain");
    let mut private_key = args.private_key.ok_or(ProofSubmitterError::PrivateKeyRequired)?;
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
            .map_err(|e| {
                error!("Error deploying attestation doc verifier contract: {e}");
                ProofSubmitterError::DeployNewContract(e)
            })?;
            info!(
                "Attestation doc verifier contract deployed to: {}",
                attestation_doc_verifier_contract.address()
            );

            let contract = TeeKeyManager::deploy(
                provider.clone(),
                *attestation_doc_verifier_contract.address(),
            )
            .await
            .map_err(|e| {
                error!("Error deploying tee key manager contract: {e}");
                ProofSubmitterError::DeployNewContract(e)
            })?;
            info!("Tee key manager contract deployed to: {}", contract.address());
            *contract.address()
        }
        (None, None) => return Err(ProofSubmitterError::ContractAddressRequired),
        (Some(_), Some(_)) => {
            return Err(ProofSubmitterError::ContractAddressAndDeployAreMutuallyExclusive)
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

    let proof =
        generate_proof_fn(&cbor_attestation_doc, args.proof_system, &elf_bytes, &public_data)?;
    info!("Proof: 0x{}", hex::encode(&proof));
    submit_proof_to_chain(contract, attestation_result.calldata().into(), proof.into()).await?;
    Ok(())
}

fn get_elf_bytes(
    elf_file_path: PathBuf,
    proof_system: ProofSystem,
) -> Result<Vec<u8>, ProofSubmitterError> {
    let elf = std::fs::read(elf_file_path).map_err(|e| {
        error!("Error reading ELF file: {e}");
        ProofSubmitterError::ReadElfFile(e)
    })?;
    Ok(match proof_system {
        ProofSystem::RISC0 => risc0_binfmt::ProgramBinary::new(&elf, V1COMPAT_ELF).encode(),
        ProofSystem::SP1 => elf,
    })
}

async fn assert_vkey_and_data_hash_match<P: Provider>(
    vkey: &[u8; 32],
    data_hash: &B256,
    proof_system: ProofSystem,
    contract: TeeKeyManagerInstance<P>,
) -> Result<(), ProofSubmitterError> {
    let att_doc_verifier_address = contract.attestationDocVerifier().call().await.map_err(|e| {
        error!("Error getting attestation doc verifier address: {e}");
        ProofSubmitterError::GetAttestationDocVerifierAddress(e)
    })?;
    let att_doc_verifier_contract =
        AttestationDocVerifier::new(att_doc_verifier_address, contract.provider());

    let att_doc_proof_system =
        att_doc_verifier_contract.proofSystem().call().await.map_err(|e| {
            error!("Error getting attestation doc verifier proof system: {e}");
            ProofSubmitterError::GetAttestationDocVerifierProofSystem(e)
        })?;

    // match proof system
    if att_doc_proof_system != proof_system as u8 {
        return Err(ProofSubmitterError::ProofSystemMismatch);
    }

    let att_doc_verifier_vkey =
        att_doc_verifier_contract.attestationDocVerifierVKey().call().await.map_err(|e| {
            error!("Error getting attestation doc verifier vkey hash: {e}");
            ProofSubmitterError::GetAttestationDocVerifierVKeyHash(e)
        })?;

    // match vkey
    if vkey != att_doc_verifier_vkey {
        return Err(ProofSubmitterError::VkeyMismatch);
    }

    // match data hash
    let expected_data_hash = att_doc_verifier_contract.dataHash().call().await.map_err(|e| {
        error!("Error getting data hash: {e}");
        ProofSubmitterError::DataHashMismatch
    })?;
    if data_hash != &expected_data_hash {
        error!("Data hash mismatch: got {}, expected {}", data_hash, expected_data_hash);
        return Err(ProofSubmitterError::DataHashMismatch);
    }

    Ok(())
}

async fn submit_proof_to_chain<P: Provider>(
    contract: TeeKeyManagerInstance<P>,
    public_values: Bytes,
    proof: Bytes,
) -> Result<(), ProofSubmitterError> {
    let tx = contract.addKey(public_values, proof);

    let receipt = tx
        .send()
        .await
        .map_err(|e| {
            error!("Error sending transaction: {e}");
            ProofSubmitterError::SubmitProofToChain(e.to_string())
        })?
        .get_receipt()
        .await
        .map_err(|e| {
            error!("Error getting receipt: {e}");
            ProofSubmitterError::SubmitProofToChain(e.to_string())
        })?;

    info!("Successfully submitted proof to chain. Receipt: {receipt:?}");

    if !receipt.status() {
        return Err(ProofSubmitterError::SubmitProofToChain("receipt status is: failed".to_string()))
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::ext::AnvilApi;
    use contract_bindings::synd::{
        attestation_doc_verifier::AttestationDocVerifier, dummy_sp1_verifier::DummySP1Verifier,
        tee_key_manager::TeeKeyManager,
    };
    use test_utils::{anvil::start_anvil, chain_info::PRIVATE_KEY};

    #[tokio::test]
    async fn post_attestation_proof_onchain() {
        shared::tracing::setup_global_logging().unwrap();

        let chain_info = start_anvil(1).await.unwrap();
        let provider = chain_info.provider;
        provider.anvil_set_auto_mine(true).await.unwrap();
        provider.anvil_set_time(1748509951).await.unwrap();

        let verifier = DummySP1Verifier::deploy(&provider).await.unwrap();
        let attestation_doc_verifier_contract = AttestationDocVerifier::deploy(
            &provider,
            verifier.address().to_owned(),
            Default::default(),
            fixed_bytes!("0xb81743c43da8243554a4c316218f9ae15786a3e5c2e19ed404244df90fc5edc5"),
            0,
            String::new(),
            ProofSystem::SP1 as u8,
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
            proof_system: ProofSystem::SP1,
            contract_address: Some(*key_mgr_contract.address()),
            deploy_new_contract_with_sp1_verifier: None,
            deploy_expiration_tolerance: Duration::from_secs(3600),
            chain_rpc_url: Some(chain_info.ws_url.clone()),
            private_key: Some(Zeroizing::new(PRIVATE_KEY.to_string())),
            elf_file_path: "".into(),
        };

        let mock_generate_proof =
            |_: &[u8],
             _: ProofSystem,
             _: &[u8],
             _: &[u8; 60]|
             -> Result<Vec<u8>, ProofSubmitterError> { Ok(Default::default()) };

        let mock_get_vkey = |_: ProofSystem, _: &[u8]| -> [u8; 32] { Default::default() };

        let mock_get_elf_bytes =
            |_: PathBuf, _: ProofSystem| -> Result<Vec<u8>, ProofSubmitterError> {
                Ok(Default::default())
            };

        let result = run(args, mock_generate_proof, mock_get_vkey, mock_get_elf_bytes).await;
        drop(mock_enclave_server);

        assert!(result.is_ok(), "run function failed: {:?}", result.err());
    }
}
