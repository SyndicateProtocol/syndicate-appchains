#!/bin/bash
set -e

# This script is for CLI-based dev container launches only.
# It is not necessary if you're using an IDE (like VS Code) which handles container setup automatically.
# Use this script when you need to launch the dev container from a terminal.
# IMPORTANT: This script must be run from the repository root directory.
# This is particularly useful for fresh machines without existing Dev Container
# support, like for development with Devin.
# If you're using GitHub Actions, you can use devcontainers-ci:
# (https://github.com/marketplace/actions/dev-container-build-and-run-action)

# Function to check for existing dev containers
check_existing_container() {
    local workspace_name=$1
    # Look for running dev containers for this workspace
    container_id=$(docker ps --filter "name=_${workspace_name}_" --filter "status=running" --format "{{.ID}}" | head -n1)
    echo "$container_id"
}

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

# Ready to proceed
echo ""

# Check for existing container
# The workspace name is extracted from the current directory name because
# dev containers are named with the pattern "_workspace-name_" where workspace-name
# is the name of the repository directory they were launched from
WORKSPACE_NAME=$(basename $(pwd))
CONTAINER_ID=$(check_existing_container "$WORKSPACE_NAME")

if [ -n "$CONTAINER_ID" ]; then
    echo "Found existing dev container. Connecting..."
    docker exec -it "$CONTAINER_ID" zsh
    exit 0
fi

# Launch new dev container if none exists
echo "No existing container found. Launching new dev container..."
devcontainer up --workspace-folder . || {
    echo "Error: Failed to launch dev container"
    exit 1
}

# Wait for container to be ready
echo "Waiting for container to be ready..."
sleep 10

# Verify container setup
echo "Verifying container setup..."
if ! devcontainer exec --workspace-folder . bash -c "cd /workspaces/metabased-rollup/ && just --list"; then
    echo "Error: Container verification failed"
    exit 1
fi

echo "Dev container setup complete!"
