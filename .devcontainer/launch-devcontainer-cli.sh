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
    local workspace_path="$(pwd)"
    # Look for running dev containers for this workspace using devcontainer labels
    >&2 echo "Checking for existing Dev Container with label devcontainer.local_folder=$workspace_path..."
    container_id=$(docker ps --filter "label=devcontainer.local_folder=$workspace_path" --filter "status=running" --format "{{.ID}}" | head -n1)
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

# Get workspace information
WORKSPACE_NAME=$(basename $(pwd))
WORKSPACE_PATH="/workspaces/$WORKSPACE_NAME"

# Ready to proceed
echo ""

# Check for existing container
# We use the full workspace path to match the devcontainer.local_folder label
# This ensures we find the exact container for this workspace
workspace_path="$(pwd)"
echo "Workspace path: $workspace_path"
CONTAINER_ID=$(check_existing_container "$workspace_path")
echo "Found container ID: $CONTAINER_ID"

if [ -n "$CONTAINER_ID" ]; then
    echo "Found existing dev container. Connecting..."
    docker exec -it -w "$WORKSPACE_PATH" "$CONTAINER_ID" zsh
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
# First verify we can connect to the container
if ! devcontainer exec --workspace-folder . zsh -c "cd $WORKSPACE_PATH && echo 'Container connection successful'"; then
    echo "Error: Cannot connect to container"
    exit 1
fi

# Then verify the workspace is accessible and set as working directory
if ! devcontainer exec --workspace-folder . zsh -c "cd $WORKSPACE_PATH && pwd"; then
    echo "Error: Cannot access workspace directory"
    exit 1
fi

# Finally check if just is available (but don't fail if it isn't)
if ! devcontainer exec --workspace-folder . zsh -c "just --list"; then
    echo "Warning: 'just' command not available yet - container is usable but some tools may need installation"
fi

echo "Dev container setup complete!"
