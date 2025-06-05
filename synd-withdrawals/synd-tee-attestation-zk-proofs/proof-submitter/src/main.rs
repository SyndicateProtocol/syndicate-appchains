//! TODO

use alloy::{
    hex,
    network::EthereumWallet,
    primitives::Address,
    providers::{PendingTransactionError, ProviderBuilder},
    signers::local::{LocalSignerError, PrivateKeySigner},
    transports::{RpcError, TransportErrorKind},
};
use clap::{Parser, ValueEnum};
use contract_bindings::synd::teekeymanager::TeeKeyManager;
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder};
use shared::parse::parse_address;
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use std::{path::PathBuf, str::FromStr, time::Duration};
use synd_tee_attestation_zk_proofs_aws_nitro::verify_aws_nitro_attestation;
use synd_tee_attestation_zk_proofs_sp1_script::shared::TEE_ATTESTATION_VALIDATION_ELF;
use x509_cert::der::{DecodePem, Encode};

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

    /// The address of the contract to submit the proof to
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
}

/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    Plonk,
    Groth16,
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum ProofSubmitterError {
    #[error("Failed to get attestation doc")]
    GetAttestationDoc(#[from] jsonrpsee::core::client::Error),

    #[error("Failed to decode attestation doc")]
    DecodeAttestationDoc(#[from] hex::FromHexError),

    #[error("Failed to read root certificate")]
    ReadRootCertificate(#[from] std::io::Error),

    #[error("Failed to parse root certificate")]
    ParseRootCertificate(#[from] x509_cert::der::Error),

    #[error("Invalid attestation document: {0:?}")]
    InvalidAttestationDocument(synd_tee_attestation_zk_proofs_aws_nitro::VerificationError),

    #[error("Failed to generate proof: {0}")]
    GenerateProof(String),

    #[error("Failed to parse private key: {0}")]
    ParsePrivateKey(#[from] LocalSignerError),

    #[error("Failed to connect to chain: {0}")]
    ConnectToChain(#[from] RpcError<TransportErrorKind>),

    #[error("Failed to submit proof to chain: {0}")]
    SubmitProofToChain(String),

    #[error("Failed to wait for pending transaction: {0}")]
    WaitForPendingTransaction(#[from] PendingTransactionError),
}

const AWS_NITRO_ROOT_CERT_PEM: &[u8] =
    include_bytes!("../../aws-nitro/src/testdata/aws_nitro_root.pem");

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match run(args).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}

async fn run(args: Args) -> Result<(), ProofSubmitterError> {
    // get attestation doc CBOR
    let attestation_doc_hex = get_attestation_doc(args.enclave_rpc_url).await?;
    println!("Attestation doc: {}", attestation_doc_hex);
    let cbor_attestation_doc = hex::decode(attestation_doc_hex)?;

    // get root certificate DER
    let pem_root_cert = if let Some(root_certificate_path) = args.root_certificate_path {
        std::fs::read(root_certificate_path)?
    } else {
        AWS_NITRO_ROOT_CERT_PEM.to_vec()
    };

    let der_root_cert = x509_cert::Certificate::from_pem(&pem_root_cert)?.to_der()?;

    // make sure the attestation is vaild for the provided root certificate
    verify_aws_nitro_attestation(&cbor_attestation_doc, &der_root_cert)
        .map_err(ProofSubmitterError::InvalidAttestationDocument)?;

    // TODO pass `generate_proof` as a parameter so it can be mocked in tests
    // generate the proof
    let proof = generate_proof(cbor_attestation_doc, der_root_cert, args.proof_system).await?;
    println!("Proof generated successfully");

    match (args.chain_rpc_url, args.private_key, args.contract_address) {
        (Some(chain_rpc_url), Some(private_key), Some(contract_address)) => {
            submit_proof_to_chain(chain_rpc_url, contract_address, private_key, proof).await?;
        }
        _ => {
            println!("Skipping submission to chain");
            println!("proof: 0x{}", hex::encode(proof.bytes()));
        }
    }

    Ok(())
}

async fn submit_proof_to_chain(
    chain_rpc_url: String,
    contract_address: Address,
    private_key: String,
    proof: SP1ProofWithPublicValues,
) -> Result<(), ProofSubmitterError> {
    let signer = PrivateKeySigner::from_str(&private_key)?;

    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect(chain_rpc_url.as_str())
        .await?;

    let contract = TeeKeyManager::new(contract_address, provider);

    let receipt = contract
        .addKey(proof.public_values.to_vec().into(), proof.bytes().into())
        .send()
        .await
        .map_err(|e| ProofSubmitterError::SubmitProofToChain(e.to_string()))?
        .watch()
        .await
        .map_err(ProofSubmitterError::WaitForPendingTransaction)?;

    println!("Successfully submitted proof to chain. Receipt: {:?}", receipt);

    Ok(())
}

async fn generate_proof(
    cbor_attestation_doc: Vec<u8>,
    der_root_cert: Vec<u8>,
    proof_system: ProofSystem,
) -> Result<SP1ProofWithPublicValues, ProofSubmitterError> {
    // Set up the prover client.
    let client = ProverClient::from_env();

    let (pk, _vk) = client.setup(TEE_ATTESTATION_VALIDATION_ELF);
    let mut stdin = SP1Stdin::new();
    stdin.write(&cbor_attestation_doc);
    stdin.write(&der_root_cert);

    let proof = match proof_system {
        ProofSystem::Plonk => client.prove(&pk, &stdin).plonk().run(),
        ProofSystem::Groth16 => client.prove(&pk, &stdin).groth16().run(),
    }
    .map_err(|e| ProofSubmitterError::GenerateProof(e.to_string()))?;

    Ok(proof)
}

async fn get_attestation_doc(enclave_rpc_url: String) -> Result<String, ProofSubmitterError> {
    let client = HttpClientBuilder::default()
        .request_timeout(Duration::from_secs(10))
        .build(enclave_rpc_url)?;

    Ok(client.request::<String, [(); 0]>("enclave_signerAttestation", []).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_attestation_doc_success() {
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        let expected_attestation_doc = "test_attestation_doc_hex";
        let mock_response =
            format!(r#"{{"jsonrpc":"2.0","id":0,"result":"{}"}}"#, expected_attestation_doc);

        server.mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .match_body(mockito::Matcher::JsonString("{\"jsonrpc\":\"2.0\",\"method\":\"enclave_signerAttestation\",\"params\":[],\"id\":0}".to_string()))
            .create_async().await;

        let result = get_attestation_doc(mock_url.clone()).await;
        drop(server);
        assert!(result.is_ok(), "get_attestation_doc call failed: {:?}", result.err());
        assert_eq!(result.unwrap(), expected_attestation_doc);
    }

    #[tokio::test]
    async fn post_attestaion_proof_onchain() {
        // TODO
        // setup anvil
        // mock anvil timestamp
        // deploy AttestationDocVerifier and TeeKeyManager (use contract bindings)
        // mock the enclave RPC server, output the attestation doc from `testdata`
        // call `run``
        // assert the proof was submitted to the chain and the key was added to the contract
    }
}
