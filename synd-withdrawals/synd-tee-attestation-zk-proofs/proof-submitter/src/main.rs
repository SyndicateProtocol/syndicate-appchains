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
    hex, network::EthereumWallet, primitives::Address, providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use clap::Parser;
use contract_bindings::synd::teekeymanager::TeeKeyManager;
use shared::parse::parse_address;
use std::{path::PathBuf, str::FromStr};
use synd_tee_attestation_zk_proofs_aws_nitro::verify_aws_nitro_attestation;
use synd_tee_attestation_zk_proofs_submitter::{
    generate_proof, get_attestation_doc, pem_to_der, GenerateProofResult, ProofSubmitterError,
    ProofSystem, AWS_NITRO_ROOT_CERT_PEM,
};

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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match run(args, generate_proof).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}

async fn run(
    args: Args,
    generate_proof_fn: impl Fn(
        Vec<u8>,
        Vec<u8>,
        ProofSystem,
    ) -> Result<GenerateProofResult, ProofSubmitterError>,
) -> Result<(), ProofSubmitterError> {
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

    let der_root_cert = pem_to_der(&pem_root_cert)?;

    // make sure the attestation is vaild for the provided root certificate
    verify_aws_nitro_attestation(&cbor_attestation_doc, &der_root_cert)
        .map_err(ProofSubmitterError::InvalidAttestationDocument)?;

    let proof = generate_proof_fn(cbor_attestation_doc, der_root_cert, args.proof_system)?;
    println!("Proof generated successfully");

    match (args.chain_rpc_url, args.private_key, args.contract_address) {
        (Some(chain_rpc_url), Some(private_key), Some(contract_address)) => {
            submit_proof_to_chain(chain_rpc_url, contract_address, private_key, proof).await?;
        }
        _ => {
            println!("Skipping submission to chain");
            println!("proof: 0x{}", hex::encode(&proof.proof));
        }
    }

    Ok(())
}

async fn submit_proof_to_chain(
    chain_rpc_url: String,
    contract_address: Address,
    private_key: String,
    proof: GenerateProofResult,
) -> Result<(), ProofSubmitterError> {
    let signer = PrivateKeySigner::from_str(&private_key)?;

    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect(chain_rpc_url.as_str())
        .await?;

    let contract = TeeKeyManager::new(contract_address, provider);

    let receipt = contract
        .addKey(proof.public_values.into(), proof.proof.into())
        .send()
        .await
        .map_err(|e| ProofSubmitterError::SubmitProofToChain(e.to_string()))?
        .watch()
        .await
        .map_err(ProofSubmitterError::WaitForPendingTransaction)?;

    println!("Successfully submitted proof to chain. Receipt: {:?}", receipt);

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
        sp1::{sp1verifiergateway::SP1VerifierGateway, sp1verifiergroth16::SP1VerifierGroth16},
        synd::{attestationdocverifier::AttestationDocVerifier, teekeymanager::TeeKeyManager},
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
        assert_eq!(version._0, "v5.0.0");

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
        };

        let mock_generate_proof = |_: Vec<u8>,
                                   _: Vec<u8>,
                                   _: ProofSystem|
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
