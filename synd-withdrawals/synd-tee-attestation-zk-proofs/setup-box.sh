#!/bin/bash

set -euo pipefail

echo "🔧 Fixing hostname resolution..."
HOSTNAME=$(hostname)
grep -q "$HOSTNAME" /etc/hosts || echo "127.0.1.1 $HOSTNAME" | sudo tee -a /etc/hosts

echo "🚀 Updating system and removing Docker conflicts..."
sudo apt update
sudo apt remove -y docker docker-engine docker.io containerd containerd.io || true

echo "🧹 Cleaning old Docker sources..."
sudo rm -f /etc/apt/sources.list.d/docker.list
sudo apt update

echo "📦 Installing base dependencies..."
sudo apt install -y git pkg-config libssl-dev curl nano

echo "🐙 Cloning appchains repo..."
git clone https://squibwarb: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@github.com/SyndicateProtocol/syndicate-appchains.git

echo "🦀 Installing Rust (non-interactive)..."
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"

echo "⚙️ Installing SP1..."
curl -L https://sp1up.succinct.xyz | bash
source ~/.bashrc || true
sp1up || true


echo "🐳 Installing Docker via get.docker.com script..."
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

echo "🔁 Adding user to docker group..."
sudo usermod -aG docker $USER
sudo systemctl enable docker
sudo systemctl start docker

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

export SP1_PROVER=cuda


KEY_MANAGER_ADDRESS="0xYOUR_KEY_MANAGER_ADDRESS"
PRIVATE_KEY="0xYOUR_PRIVATE_KEY"
# TODO: once builds are deterministic, we can use the generated synd-tee-attestation-zk-proofs-sp1-program instead of scp'ing it from the local machine
ELF_PATH="$HOME/synd-tee-attestation-zk-proofs-sp1-program"


cargo run --release --bin proof-submitter \
  -- --enclave-rpc-url https://verifier.direct.us-east-2.aws.testnet.syndicate.io \
  --contract-address "$KEY_MANAGER_ADDRESS" \
  --chain-rpc-url wss://base-sepolia.g.alchemy.com/v2/qZP155RkIA6rfHFaYT6Gm \
  --private-key "$PRIVATE_KEY" \
  --elf-file-path "$ELF_PATH"
