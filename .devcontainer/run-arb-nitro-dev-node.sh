#!/bin/bash

# SOURCE: https://github.com/OffchainLabs/nitro-devnode/blob/main/run-dev-node.sh , 10/31/24

# Variable defaults
PORT=${ARB_ORBIT_PORT:-8547}
DEV_PRIVATE_KEY=${ARB_ORBIT_PRIVATE_KEY:-"0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"}
RPC_URL=${ARB_ORBIT_L2_RPC_URL:-"http://127.0.0.1:$PORT"}


# Start Nitro dev node in the background
echo "Starting Nitro dev node on $PORT..."
docker run --rm --name nitro-dev -p $PORT:$PORT offchainlabs/nitro-node:v3.2.1-d81324d --dev --http.addr 0.0.0.0 \
  --http.port $PORT &

# Wait for the node to initialize
echo "Waiting for the Nitro node to initialize on port $PORT..."

until [[ "$(curl -s -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"net_version","params":[],"id":1}' \
  "$RPC_URL")" == *"result"* ]]; do
    sleep 0.1
done


# Check if node is running
curl_output=$(curl -s -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"net_version","params":[],"id":1}' \
  "$RPC_URL")

if [[ "$curl_output" == *"result"* ]]; then
  echo "Nitro node is running!"
else
  echo "Failed to start Nitro node."
  exit 1
fi

# Make the caller a chain owner
echo "Setting chain owner to pre-funded dev account..."
cast send 0x00000000000000000000000000000000000000FF "becomeChainOwner()" \
  --private-key $DEV_PRIVATE_KEY \
  --rpc-url $RPC_URL

# Deploy Cache Manager Contract
echo "Deploying Cache Manager contract..."
deploy_output=$(cast send --private-key $DEV_PRIVATE_KEY \
  --rpc-url $RPC_URL \
  --create 0x60a06040523060805234801561001457600080fd5b50608051611d1c61003060003960006105260152611d1c6000f3fe)

# Extract contract address using awk from plain text output
contract_address=$(echo "$deploy_output" | awk '/contractAddress/ {print $2}')

# Check if contract deployment was successful
if [[ -z "$contract_address" ]]; then
  echo "Error: Failed to extract contract address. Full output:"
  echo "$deploy_output"
  exit 1
fi

echo "Cache Manager contract deployed at address: $contract_address"

# Register the deployed Cache Manager contract
echo "Registering Cache Manager contract as a WASM cache manager..."
registration_output=$(cast send --private-key $DEV_PRIVATE_KEY \
  --rpc-url $RPC_URL \
  0x0000000000000000000000000000000000000070 \
  "addWasmCacheManager(address)" "$contract_address")

# Check if registration was successful
if [[ "$registration_output" == *"error"* ]]; then
  echo "Failed to register Cache Manager contract. Registration output:"
  echo "$registration_output"
  exit 1
fi

# If no errors, print success message
echo "Cache Manager deployed and registered successfully. Nitro node is ready..."
wait  # Keep the script alive and the node running

