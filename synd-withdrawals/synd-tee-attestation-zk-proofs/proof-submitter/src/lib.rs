//! The `proof-submitter` library contains the functions for obtaining TEE
//! attestation and generating ZK proofs.

use alloy::{
    hex,
    providers::PendingTransactionError,
    signers::local::LocalSignerError,
    transports::{RpcError, TransportErrorKind},
};
use clap::ValueEnum;
use jsonrpsee::{
    core::client::ClientT,
    http_client::{HeaderMap, HeaderValue, HttpClientBuilder},
};
use sp1_sdk::{ProverClient, SP1Stdin};
use std::time::Duration;
use x509_cert::der::{DecodePem, Encode};

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum ProofSubmitterError {
    #[error("Failed to get attestation doc")]
    GetAttestationDoc(#[from] jsonrpsee::core::client::Error),

    #[error("Failed to decode attestation doc")]
    DecodeAttestationDoc(#[from] hex::FromHexError),

    #[error("Failed to read root certificate")]
    ReadRootCertificate(std::io::Error),

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

    #[error("Failed to read ELF file: {0}")]
    ReadElfFile(std::io::Error),

    #[error("Failed to get attestation doc verifier address")]
    GetAttestationDocVerifierAddress(alloy::contract::Error),

    #[error("Failed to get attestation doc verifier vkey hash")]
    GetAttestationDocVerifierVKeyHash(alloy::contract::Error),

    #[error("Vkey mismatch")]
    VkeyMismatch,
}

#[allow(missing_docs)]
/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ProofSystem {
    Plonk,
    Groth16,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
    pub public_values: Vec<u8>,
}

/// Generate a ZK proof for the TEE attestation document
pub fn generate_proof(
    cbor_attestation_doc: Vec<u8>,
    der_root_cert: Vec<u8>,
    proof_system: ProofSystem,
    elf_bytes: Vec<u8>,
) -> Result<GenerateProofResult, ProofSubmitterError> {
    // Set up the prover client.
    let client = ProverClient::from_env();

    let (pk, _) = client.setup(&elf_bytes);

    let mut stdin = SP1Stdin::new();
    stdin.write(&cbor_attestation_doc);
    stdin.write(&der_root_cert);

    let proof = match proof_system {
        ProofSystem::Plonk => client.prove(&pk, &stdin).plonk().run(),
        ProofSystem::Groth16 => client.prove(&pk, &stdin).groth16().run(),
    }
    .map_err(|e| ProofSubmitterError::GenerateProof(e.to_string()))?;

    Ok(GenerateProofResult { proof: proof.bytes(), public_values: proof.public_values.to_vec() })
}

/// Get the TEE attestation document from the enclave RPC server
pub async fn get_attestation_doc(enclave_rpc_url: String) -> Result<String, ProofSubmitterError> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("synd-withdrawals/proof-submitter"));

    let client = HttpClientBuilder::default()
        .request_timeout(Duration::from_secs(10))
        .set_headers(headers)
        .build(enclave_rpc_url)?;

    Ok(client.request::<String, [(); 0]>("enclave_signerAttestation", []).await?)
}

/// Gets the public key from the TEE, no attestation. Used for testing only
pub async fn get_signer_public_key(enclave_rpc_url: String) -> Result<String, ProofSubmitterError> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("synd-withdrawals/proof-submitter"));

    let client = HttpClientBuilder::default()
        .request_timeout(Duration::from_secs(10))
        .build(enclave_rpc_url)?;

    Ok(client.request::<String, [(); 0]>("enclave_signerPublicKey", []).await?)
}

/// The AWS Nitro root certificate in PEM format
pub const AWS_NITRO_ROOT_CERT_PEM: &[u8] =
    include_bytes!("../../aws-nitro/src/testdata/aws_nitro_root.pem");

/// Convert a PEM-encoded certificate to DER-encoded certificate
pub fn pem_to_der(pem: &[u8]) -> Result<Vec<u8>, ProofSubmitterError> {
    let cert = x509_cert::Certificate::from_pem(pem)?;
    Ok(cert.to_der()?)
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
            format!(r#"{{"jsonrpc":"2.0","id":0,"result":"{expected_attestation_doc}"}}"#);

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
}
