//! The `proof-submitter` library contains the functions for obtaining TEE
//! attestation and generating ZK proofs.
#![allow(clippy::unwrap_used)]

use alloy::{
    hex,
    primitives::B256,
    providers::PendingTransactionError,
    signers::local::LocalSignerError,
    transports::{RpcError, TransportErrorKind},
};
use clap::ValueEnum;
use jsonrpsee::{
    core::client::ClientT,
    http_client::{HeaderMap, HeaderValue, HttpClientBuilder},
};
use sp1_sdk::{HashableKey as _, ProverClient, SP1Stdin};
use std::time::Duration;
use tracing::{error, info};

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum ProofSubmitterError {
    #[error("RPC URL was provided, but Private key is missing")]
    PrivateKeyRequired,

    #[error("RPC URL was provided, but contract address is missing. You can use `--deploy-new-contract-with-sp1-verifier` to deploy a new instance")]
    ContractAddressRequired,

    #[error(
        "`--contract-address` and `--deploy-new-contract-with-sp1-verifier` are mutually exclusive"
    )]
    ContractAddressAndDeployAreMutuallyExclusive,

    #[error("Failed to get attestation doc")]
    GetAttestationDoc(#[from] jsonrpsee::core::client::Error),

    #[error("Failed to decode attestation doc")]
    DecodeAttestationDoc(#[from] hex::FromHexError),

    #[error("Root certificate hash mismatch: got {0}, expected {1}")]
    RootCertificateHashMismatch(B256, B256),

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

    #[error("Failed to get attestation doc verifier proof system")]
    GetAttestationDocVerifierProofSystem(alloy::contract::Error),

    #[error("Vkey mismatch")]
    ProofSystemMismatch,

    #[error("Vkey mismatch")]
    VkeyMismatch,

    #[error("Data hash mismatch")]
    DataHashMismatch,

    #[error("Public data mismatch")]
    PublicDataMismatch,

    #[error("Receipt not Groth16")]
    ReceiptNotGroth16,

    #[error("Failed to deploy new contracts: {0}")]
    DeployNewContract(alloy::contract::Error),

    #[error("Attestation document and enclave RPC URL are mutually exclusive")]
    AttestationDocumentAndEnclaveRpcUrlAreMutuallyExclusive,

    #[error("Attestation document or enclave RPC URL is required")]
    AttestationDocumentOrEnclaveRpcUrlRequired,
}

#[allow(missing_docs)]
/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ProofSystem {
    RISC0,
    SP1,
}

/// Get vkey
pub fn get_vkey(proof_system: ProofSystem, elf_bytes: &[u8]) -> [u8; 32] {
    match proof_system {
        ProofSystem::RISC0 => {
            risc0_binfmt::compute_image_id(elf_bytes).unwrap().as_bytes().try_into().unwrap()
        }
        ProofSystem::SP1 => {
            let client = ProverClient::from_env();
            let (_, vk) = client.setup(elf_bytes);
            vk.bytes32_raw()
        }
    }
}

/// Generate a ZK proof for the TEE attestation document
#[allow(clippy::cognitive_complexity)]
pub fn generate_proof(
    cbor_attestation_doc: &[u8],
    proof_system: ProofSystem,
    elf_bytes: &[u8],
    public_data: &[u8; 60],
) -> Result<Vec<u8>, ProofSubmitterError> {
    match proof_system {
        ProofSystem::SP1 => {
            // Set up the prover client.
            let client = ProverClient::from_env();
            let (pk, _) = client.setup(elf_bytes);

            let mut stdin = SP1Stdin::new();
            stdin.write(&cbor_attestation_doc);
            let proof = client
                .prove(&pk, &stdin)
                .groth16()
                .run()
                .map_err(|e| ProofSubmitterError::GenerateProof(e.to_string()))?;
            info!("sp1 zkvm version: {}", proof.sp1_version);
            if public_data != proof.public_values.as_slice() {
                error!(
                    "public data mismatch: got {}, expected {}",
                    hex::encode(proof.public_values),
                    hex::encode(public_data),
                );
                return Err(ProofSubmitterError::PublicDataMismatch)
            }

            Ok(proof.bytes())
        }
        ProofSystem::RISC0 => {
            let env = risc0_zkvm::ExecutorEnv::builder()
                .write(&cbor_attestation_doc)
                .unwrap()
                .build()
                .map_err(|e| ProofSubmitterError::GenerateProof(e.to_string()))?;
            let prover = risc0_zkvm::default_prover();

            let receipt = prover
                .prove_with_opts(env, elf_bytes, &risc0_zkvm::ProverOpts::groth16())
                .map_err(|e| ProofSubmitterError::GenerateProof(e.to_string()))?
                .receipt;
            info!("risc0 verifier parameters digest: {}", receipt.metadata.verifier_parameters);
            receipt.verify(get_vkey(ProofSystem::RISC0, elf_bytes)).unwrap();
            if public_data != receipt.journal.bytes.as_slice() {
                error!(
                    "public data mismatch: got {}, expected {}",
                    hex::encode(receipt.journal.bytes),
                    hex::encode(public_data),
                );
                return Err(ProofSubmitterError::PublicDataMismatch)
            }
            match receipt.inner {
                risc0_zkvm::InnerReceipt::Groth16(x) => Ok(x.seal),
                _ => Err(ProofSubmitterError::ReceiptNotGroth16),
            }
        }
    }
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
