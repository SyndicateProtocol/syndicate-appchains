#!/bin/bash
set -e

# This script generates alloy bindings for the V5 contracts in the sp1-contracts submodule.
# It temporarily renames contracts with conflicting names within their files to allow `forge` to build them all.
# All local changes to the sp1-contracts submodule are reverted upon script completion or exit.

# Get the absolute path to the script's directory
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

SP1_CONTRACTS_SUBMODULE_DIR=$(realpath "$SCRIPT_DIR/../../synd-contracts/lib/sp1-contracts")
CONTRACTS_DIR="$SP1_CONTRACTS_SUBMODULE_DIR/contracts"
V5_DIR="$CONTRACTS_DIR/src/v5.0.0"
V5_DEPLOY_SCRIPT_DIR="$CONTRACTS_DIR/script/deploy/v5.0.0"
BINDINGS_OUT_DIR="$SCRIPT_DIR/src"

SP1_GROTH16_VERIFIER_FILE="$V5_DIR/SP1VerifierGroth16.sol"
SP1_PLONK_VERIFIER_FILE="$V5_DIR/SP1VerifierPlonk.sol"
SP1_GROTH16_VERIFIER_DEPLOY_SCRIPT_FILE="$V5_DEPLOY_SCRIPT_DIR/SP1VerifierGroth16.s.sol"
SP1_PLONK_VERIFIER_DEPLOY_SCRIPT_FILE="$V5_DEPLOY_SCRIPT_DIR/SP1VerifierPlonk.s.sol"

function cleanup {
  echo "Reverting changes to sp1-contracts submodule..."
  pushd "$SP1_CONTRACTS_SUBMODULE_DIR" > /dev/null
  git reset --hard HEAD > /dev/null
  popd > /dev/null
  echo "Reverted changes."
}

# Ensure cleanup is called on script exit (even on error)
trap cleanup EXIT

echo "Temporarily modifying contract names in V5 files..."

# Use sed to rename the contracts. The -i '' flag modifies in-place without a backup file on macOS.
sed -i '' 's/contract SP1Verifier is Groth16Verifier/contract SP1VerifierGroth16 is Groth16Verifier/' "$SP1_GROTH16_VERIFIER_FILE"
sed -i '' 's/contract SP1Verifier is PlonkVerifier/contract SP1VerifierPlonk is PlonkVerifier/' "$SP1_PLONK_VERIFIER_FILE"

echo "Temporarily modifying contract names in V5 deployment scripts..."
sed -i '' 's/import {SP1Verifier} from/import {SP1VerifierGroth16} from/' "$SP1_GROTH16_VERIFIER_DEPLOY_SCRIPT_FILE"
sed -i '' 's/new SP1Verifier{/new SP1VerifierGroth16{/' "$SP1_GROTH16_VERIFIER_DEPLOY_SCRIPT_FILE"
sed -i '' 's/import {SP1Verifier} from/import {SP1VerifierPlonk} from/' "$SP1_PLONK_VERIFIER_DEPLOY_SCRIPT_FILE"
sed -i '' 's/new SP1Verifier{/new SP1VerifierPlonk{/' "$SP1_PLONK_VERIFIER_DEPLOY_SCRIPT_FILE"

echo "Running forge build to compile contracts..."
pushd "$CONTRACTS_DIR" > /dev/null
forge build

echo "Generating alloy bindings..."
# forge bind --module creates a `bindings` directory in the `out` dir.
rm -rf out/bindings
forge bind --alloy --select SP1VerifierGroth16 --select SP1VerifierPlonk --select Groth16Verifier --select PlonkVerifier --select SP1VerifierGateway --module --overwrite

# Move the generated bindings to the desired location.
SP1_BINDINGS_DIR="$BINDINGS_OUT_DIR/sp1"
rm -rf "$SP1_BINDINGS_DIR"
mv out/bindings "$SP1_BINDINGS_DIR"

popd > /dev/null

echo "Bindings generated successfully at $SP1_BINDINGS_DIR"
echo "Note: Contract changes are automatically reverted by the script's exit trap." 