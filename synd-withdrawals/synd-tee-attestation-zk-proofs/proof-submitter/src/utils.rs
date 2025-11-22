#![allow(unreachable_pub)]

use crate::proof::ProofSystem;
use alloy::{
    primitives::{Bytes, B256},
    providers::Provider,
    rpc::types::TransactionReceipt,
};
use contract_bindings::synd::{
    attestation_doc_verifier::AttestationDocVerifier,
    tee_key_manager::TeeKeyManager::TeeKeyManagerInstance,
};
use eyre::eyre;
use jsonrpsee::{
    core::client::ClientT,
    http_client::{HeaderMap, HeaderValue, HttpClientBuilder},
};
use std::{path::PathBuf, time::Duration};

/// from <https://docs.rs/risc0-zkos-v1compat/2.2.0/src/risc0_zkos_v1compat/lib.rs.html>
/// the elf file is located in risc0/zkos/v1compat/elfs/v1compat.elf
#[cfg(feature = "risc0")]
const V1COMPAT_ELF: &[u8] = include_bytes!("../elfs/v1compat.elf");

/// Get elf program
pub async fn get_elf_bytes(
    elf_file_path: PathBuf,
    proof_system: ProofSystem,
) -> eyre::Result<Vec<u8>> {
    let elf =
        tokio::fs::read(elf_file_path).await.map_err(|e| eyre!("Error reading ELF file: {e}"))?;
    #[cfg(feature = "risc0")]
    if proof_system == ProofSystem::RISC0 {
        return Ok(risc0_binfmt::ProgramBinary::new(&elf, V1COMPAT_ELF).encode());
    }
    Ok(elf)
}

/// Get the TEE attestation document from the enclave RPC server
pub async fn get_attestation_doc(enclave_rpc_url: String) -> eyre::Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("synd-withdrawals/proof-submitter"));

    let client = HttpClientBuilder::default()
        .request_timeout(Duration::from_secs(10))
        .set_headers(headers)
        .build(enclave_rpc_url)?;

    Ok(client.request::<String, [(); 0]>("enclave_signerAttestation", []).await?)
}

pub async fn assert_vkey_and_data_hash_match<P: Provider>(
    vkey: &[u8; 32],
    data_hash: &B256,
    proof_system: ProofSystem,
    contract: TeeKeyManagerInstance<P>,
) -> eyre::Result<()> {
    let att_doc_verifier_address = contract
        .attestationDocVerifier()
        .call()
        .await
        .map_err(|e| eyre!("Error getting attestation doc verifier address: {e}"))?;
    let att_doc_verifier_contract =
        AttestationDocVerifier::new(att_doc_verifier_address, contract.provider());

    let att_doc_proof_system = att_doc_verifier_contract
        .proofSystem()
        .call()
        .await
        .map_err(|e| eyre!("Error getting attestation doc verifier proof system: {e}"))?;

    // match proof system
    if att_doc_proof_system != proof_system as u8 {
        return Err(eyre!(
            "Proof system mismatch: {att_doc_proof_system} != {}",
            proof_system as u8
        ));
    }

    let att_doc_verifier_vkey = att_doc_verifier_contract
        .attestationDocVerifierVKey()
        .call()
        .await
        .map_err(|e| eyre!("Error getting attestation doc verifier vkey hash: {e}"))?;

    // match vkey
    if vkey != att_doc_verifier_vkey {
        return Err(eyre!("Vkey mismatch"));
    }

    // match data hash
    let expected_data_hash = att_doc_verifier_contract
        .dataHash()
        .call()
        .await
        .map_err(|e| eyre!("Error getting data hash: {e}"))?;
    if data_hash != &expected_data_hash {
        return Err(eyre!("Data hash mismatch: got {data_hash}, expected {expected_data_hash}"))
    }

    Ok(())
}

pub async fn submit_proof_to_chain<P: Provider>(
    contract: TeeKeyManagerInstance<P>,
    public_values: Bytes,
    proof: Bytes,
) -> eyre::Result<TransactionReceipt> {
    let tx = contract.addKey(public_values, proof);

    let receipt = tx
        .send()
        .await
        .map_err(|e| eyre!("Error sending transaction: {e}"))?
        .get_receipt()
        .await
        .map_err(|e| eyre!("Error getting receipt: {e}"))?;

    if !receipt.status() {
        return Err(eyre!("Error submitting proof to chain. Receipt: {receipt:?}"))
    }

    Ok(receipt)
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
