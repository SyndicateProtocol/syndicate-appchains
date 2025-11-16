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
git clone https://github.com/SyndicateProtocol/syndicate-appchains.git

echo "🦀 Installing Rust (non-interactive)..."
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"

echo "⚙️ Installing SP1..."
curl -L https://sp1up.succinct.xyz | bash
~/.sp1/bin/sp1up


echo "🐳 Installing Docker via get.docker.com script..."
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

echo "🔁 Adding user to docker group..."
sudo usermod -aG docker $USER
sudo systemctl enable docker
sudo systemctl start docker
