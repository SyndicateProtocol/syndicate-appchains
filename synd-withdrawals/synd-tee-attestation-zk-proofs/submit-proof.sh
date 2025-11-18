#!/bin/bash

set -euo pipefail

echo "📁 Navigating to proof-submitter..."
cd ~/syndicate-appchains/synd-withdrawals/synd-tee-attestation-zk-proofs/proof-submitter

echo "🔐 Loading environment variables..."
# Optionally load from .env file
if [ -f "$HOME/.env" ]; then
  export $(grep -v '^#' "$HOME/.env" | xargs)
fi

echo "📦 Sourcing Rust environment..."
source "$HOME/.cargo/env"

echo "🚀 Running proof-submitter..."
cd ~/syndicate-appchains/synd-withdrawals/synd-tee-attestation-zk-proofs/proof-submitter


# V5_0_0_SP1_VERIFIER_GROTH16 from https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments
V5_0_0_SP1_VERIFIER_GROTH16_ADDRESS="0xYOUR_SP1_VERIFIER_ADDRESS"

# Destination where AttestationDocVerifier and TeeKeyManager will be deployed
TARGET_RPC_URL="URL_OF_TARGET_RPC"

# This wallet must be funded on TARGET_RPC_URL
PRIVATE_KEY="0xYOUR_PRIVATE_KEY"

# To get the attestation document, run the following command in the enclave: cast rpc --rpc-url localhost:7333 enclave_signerAttestation
# You will need to have foundry installed in the enclave. (curl -L https://foundry.paradigm.xyz | bash)
ATTESTATION_DOCUMENT="0xYOUR_ATTESTATION_DOCUMENT"

# Download the elf file from the github release & set the file path
ELF_FILE_PATH="~/sp1-zk-v1.0.7-rc.1.elf"

SP1_PROVER=cuda cargo run --release --bin proof-submitter \
  -- --deploy-new-contract-with-sp1-verifier "$V5_0_0_SP1_VERIFIER_GROTH16_ADDRESS" \
  --chain-rpc-url "$TARGET_RPC_URL" \
  --elf-file-path "$ELF_FILE_PATH" \
  --private-key "$PRIVATE_KEY" \
  --attestation-document "$ATTESTATION_DOCUMENT"
