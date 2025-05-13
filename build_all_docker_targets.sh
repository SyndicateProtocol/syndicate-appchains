#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status.

# Define targets and their desired image tags
# Format: "stage_name:image_tag"
targets=(
    "metabased-translator:metabased-translator:latest"
    "metabased-poster:metabased-poster:latest"
    "maestro:maestro:latest"
    "synd-batch-sequencer:synd-batch-sequencer:latest"
    "mchain:mchain:latest"
    "metabased-translator-debug:metabased-translator:debug"
    "chain-ingestor:chain-ingestor:latest"
)

# Get build profile from argument or default to release
BUILD_PROFILE=${1:-release}
echo "Using build profile: ${BUILD_PROFILE}"

# Loop through targets and build each one
for target_info in "${targets[@]}"; do
    IFS=":" read -r stage_name image_tag <<< "$target_info" # Split string by colon
    echo "Building target: ${stage_name} -> ${image_tag}"
    docker build \
        --build-arg BUILD_PROFILE=${BUILD_PROFILE} \
        --target "${stage_name}" \
        -t "${image_tag}" \
        .
done

echo "All target builds complete."
