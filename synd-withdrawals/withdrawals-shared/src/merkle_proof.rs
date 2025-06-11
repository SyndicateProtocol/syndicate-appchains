//! Merkle proof helper functions for the verifiers

use crate::error::VerifierError;
use alloy::{
    primitives::{keccak256, Address, B256},
    rpc::types::{EIP1186AccountProofResponse, Header},
};
use alloy_trie::{proof::verify_proof, Nibbles, TrieAccount};

fn verify_account_proof_response(
    proof: &EIP1186AccountProofResponse,
    state_root: B256,
) -> Result<(), VerifierError> {
    let key: Nibbles = Nibbles::unpack(keccak256(proof.address));
    let expected_value = alloy::rlp::encode(TrieAccount {
        nonce: proof.nonce,
        balance: proof.balance,
        storage_root: proof.storage_hash,
        code_hash: proof.code_hash,
    });
    verify_proof(state_root, key, Some(expected_value), &proof.account_proof)
        .map_err(|e| VerifierError::ErrorVerifyingProof(e.to_string()))?;
    for p in &proof.storage_proof {
        let k = Nibbles::unpack(keccak256(p.key.as_b256()));
        let expected_value = Some(alloy::rlp::encode_fixed_size(&p.value).to_vec());
        verify_proof(proof.storage_hash, k, expected_value, &p.proof)
            .map_err(|e| VerifierError::ErrorVerifyingProof(e.to_string()))?;
    }
    Ok(())
}

/// Verify a merkle proof
pub fn verify_merkle_proof(
    proof: &EIP1186AccountProofResponse,
    header: &Header,
    block_hash: B256,
    contract_address: Address,
    slot: Vec<B256>,
) -> Result<(), VerifierError> {
    // Verify header
    let actual_block_hash = header.hash_slow();
    if actual_block_hash != block_hash {
        return Err(VerifierError::InvalidInput {
            reason: "Invalid block hash".to_string(),
            expected: block_hash.to_string(),
            actual: actual_block_hash.to_string(),
        });
    }
    // Verify end merkle proof
    verify_account_proof_response(proof, header.state_root)?;

    // Verify there are the same number of storage proofs as slots
    if proof.storage_proof.len() != slot.len() {
        return Err(VerifierError::InvalidInput {
            reason: "Invalid number of storage proofs".to_string(),
            expected: slot.len().to_string(),
            actual: proof.storage_proof.len().to_string(),
        });
    }

    // Verify storage slot
    for (i, slot) in slot.iter().enumerate() {
        let storage_proof = &proof.storage_proof[i];

        if storage_proof.key.as_b256() != *slot {
            return Err(VerifierError::InvalidInput {
                reason: "Invalid storage slot".to_string(),
                expected: slot.to_string(),
                actual: storage_proof.key.as_b256().to_string(),
            });
        }
    }

    // Verify address
    if proof.address != contract_address {
        return Err(VerifierError::InvalidInput {
            reason: "Invalid address".to_string(),
            expected: contract_address.to_string(),
            actual: proof.address.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::U256,
        providers::{Provider as _, ProviderBuilder, RootProvider},
    };
    use std::str::FromStr;

    #[tokio::test]
    async fn test_verify_account_proof_response() {
        let client: RootProvider = ProviderBuilder::default()
            .connect("https://syndicate-exo.g.alchemy.com/v2/K6cAUXQhrUT3KJPd9a-glciOF5ZA_F8Y")
            .await
            .unwrap();

        let address = Address::from_str("0x180972BF154c9Aea86c43149D83B7Ea078c33f48").unwrap();
        let test_slot = B256::ZERO;
        let proof: EIP1186AccountProofResponse = client
            .raw_request(
                "eth_getProof".into(),
                (address, vec![U256::from_be_bytes(test_slot.0)], "latest"),
            )
            .await
            .unwrap();

        let block = client
            .get_block_by_number(alloy::eips::BlockNumberOrTag::Latest)
            .await
            .unwrap()
            .unwrap();
        let state_root = block.header.state_root;

        let result = verify_account_proof_response(&proof, state_root);

        assert!(result.is_ok());
    }
}
