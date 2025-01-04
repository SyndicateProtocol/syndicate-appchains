#!/bin/bash
set -e

# This script is for CLI-based dev container launches only.
# It is not necessary if you're using an IDE (like VS Code) which handles container setup automatically.
# Use this script when you need to launch the dev container from a terminal.
# This is particularly useful for fresh machines without existing Dev Container
# support, like for development with Devin.
# If you're using GitHub Actions, you can use devcontainers-ci:
# (https://github.com/marketplace/actions/dev-container-build-and-run-action)

# Check if devcontainer CLI is installed
if ! command -v devcontainer >/dev/null 2>&1; then
    echo "Installing devcontainer CLI..."
    npm install -g @devcontainers/cli
fi

# Path to the devcontainer.json
DEVCONTAINER_PATH="$(pwd)/.devcontainer/devcontainer.json"

# Check if we're in the right directory
if [ ! -f "$DEVCONTAINER_PATH" ]; then
    echo "Error: devcontainer.json not found. Please run this script from the repository root."
    exit 1
fi

# Note about Ubuntu version requirement
echo "Note: The dev container requires Ubuntu 24.04 or later for full functionality."
echo "The container will use Ubuntu 24.04 regardless of your host OS version."
echo "This ensures compatibility with required tools like 'just'."
echo ""

# Launch the dev container
echo "Launching dev container..."
devcontainer up --workspace-folder . || {
    echo "Error: Failed to launch dev container"
    exit 1
}

# Wait for container to be ready
echo "Waiting for container to be ready..."
sleep 10

# Verify container setup
echo "Verifying container setup..."
if ! devcontainer exec --workspace-folder . bash -c "cd /workspaces/metabased-rollup && just --list"; then
    echo "Error: Container verification failed"
    exit 1
fi

echo "Dev container setup complete!"
