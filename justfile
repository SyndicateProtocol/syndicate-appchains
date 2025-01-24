# Set the shell to zsh
# The -u flag is used to exit the script if an uncaught error occurs. This
# ensures that we can place exit 0 at the end of scripts to jump back into just
# without inadvertently continuing after failed commands
# The -c flag is used to run the script in a subshell. This is recommended by
# just: https://just.systems/man/en/settings.html#settings
set shell := ["zsh", "-cu"]

# Define the PATH variable to include Foundry's bin directory
foundry_path := env_var('PATH') + ":" + env_var('HOME') + "/.foundry/bin"

# Define fully qualified path to forge binary
forge := env_var('HOME') + "/.foundry/bin/forge"

# Define a non-zero number to identify the layer-3 chain. The Chain ID does not
# matter as long as it's not attached to a live chain. This chain ID is for the
# Syndicate testnet, which is not live at this point on this chain ID.
l3_chain_id := "5100"

# Define a private key authorized to deploy contracts on Optimism devnet
# This private key is common knowledge, you should not use it on any network other than this dev network.
# Using this private key on mainnet, or even a testnet, will most likely result in a loss of funds.
# https://docs.optimism.io/chain/testing/dev-node#additional-info
op_devnet_private_key := "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

# Define layer-2 devnet RPC URL launched by the Optimism devnet
op_devnet_l2_rpc_url := "http://127.0.0.1:9545"

# Define layer-2 rollup chain ID in decimal
op_devnet_l2_chain_id := "901"

# Default localnet chainID
arb_orbit_l2_chain_id := "412346"

# Default localnet port
arb_orbit_l2_port := "8547"

arb_orbit_l2_rpc_url := "http://127.0.0.1:" + arb_orbit_l2_port

metabased_sequencer_port := "8456"

metabased_sequencer_url := "http://127.0.0.1:" + metabased_sequencer_port

op_translator_port := "9999"

op_translator_url := "http://127.0.0.1:" + op_translator_port

op_network_config_file := repository_root + "/.op-network-config.yaml"

# Dev account private key - https://docs.arbitrum.io/run-arbitrum-node/run-nitro-dev-node#development-account-used-by-default
arb_orbit_l2_private_key := "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"

# Define root directory of the git repository
repository_root := justfile_directory()

# Define root directory of the metabased contracts project
contracts_root := repository_root + "/metabased-contracts"

# Define root directory of the metabased sequencer project
sequencer_root := repository_root + "/metabased-sequencer"

# Define root directory of the integration tests
e2e_tests_root := repository_root + "/metabased-translator/tests"

# Define root directory of the metabased translator project
metabased_translator_root := repository_root + "/metabased-translator"
metabased_translator_contracts_root := metabased_translator_root + "/contracts" 

# Define file for localnet environment variables
envrc_file := repository_root + "/.envrc"

# Define deploy file created on latest run of the deploy script for metabased sequencer chain contract
op_contract_deploy_file := contracts_root + "/broadcast/DeployContractsForSequencerChain_.s.sol/" + op_devnet_l2_chain_id + "/run-latest.json"

# Define deploy file created on latest run of the deploy script for metabased sequencer chain contract
arb_contract_deploy_file := contracts_root + "/broadcast/DeployContractsForSequencerChain_.s.sol/" + arb_orbit_l2_chain_id + "/run-latest.json"

# Add Foundry's bin directory to the PATH for all recipes
export PATH := foundry_path

# List all recipes
default:
    @just --list


# Clone the Optimism repository
op-clone:
    @just _log-start "op-clone"

    # The op-devnet sometimes breaks with the error `t=2024-10-22T23:38:03+0000 lvl=crit msg="Application failed" message="failed to fetch startBlock from SystemConfig: failed to call startBlock: failed to unpack result: failed to unpack data: abi: attempting to unmarshal an empty string while arguments are expected"`
    # This is true across 1.9.3, 1.9.4, and develop, but it appears to happen
    # less frequently on develop. We'll change this to use the latest release
    # once it's fixed
    # If you encounter this error, you can try running `op-reclone` to clean up
    # and re-clone the repository
    git clone --branch develop --single-branch --depth 1 https://github.com/ethereum-optimism/optimism.git ~/optimism || echo skipping optimism clone
    git clone --branch optimism --single-branch --depth 1 https://github.com/ethereum-optimism/op-geth.git ~/op-geth || echo skipping op-geth clone

    @just _log-end "op-clone"

# Remove OP devnet directory
op-clean:
    @just _log-start "op-clean"

    rm -rf ~/optimism
    rm -rf ~/op-geth

    @just _log-end "op-clean"

# Re-clone OP devnet directory
op-reclone: op-down op-clean op-clone

# Initialize op-devnet
# TODO: Migrate to Kurtosis
op-up:
    @just _log-start "op-up"

    # OP Devnet setup based on https://docs.optimism.io/chain/testing/dev-node
    # FAILURE: Recipe failed - make: *** No rule to make target 'devnet-up'. The Optimism repository Makefile structure may have changed.
    PATH={{foundry_path}} make --directory ~/optimism devnet-up
    @echo "OP Devnet initialized"

    @just _log-end "op-up"

# Shut down devnet
# TODO: Migrate to Kurtosis
op-down:
    @just _log-start "op-down"

    @if [ -d ~/optimism ]; then \
    # FAILURE: Recipe failed - make: *** No rule to make target 'devnet-down'. The Optimism repository Makefile structure may have changed.
    PATH={{foundry_path}} make --directory ~/optimism devnet-down; \
    echo "OP Devnet shut down"; \
    else \
    echo "OP Devnet not running"; \
    fi

    @just _log-end "op-down"

# Starts Arbitrum Orbit devnet if one is not already running
arb-up: create-envrc contracts-setup
    @just _log-start "arb-up"

    @# Only run the node if health check fails
    @if ! just arb-health-check &>/dev/null; then \
        just _run-arb-nitro-dev-node; \
        echo "Arbitrum node RPC is now running at {{arb_orbit_l2_rpc_url}}"; \
    else \
        echo "Arbitrum node RPC is already running at {{arb_orbit_l2_rpc_url}}"; \
    fi

    @just _log-end "arb-up"

# Stops Arbitrum docker container created by script above
arb-down:
    @just _log-start "arb-down"

    @echo "Stopping Arbitrum node..."
    docker stop nitro-dev
    @echo "Arbitrum node stopped."

    @just _log-end "arb-down"

# Removes all Docker infra assocaited with the Arbitrum, returning to a blank slate
arb-teardown:
    @just _log-start "arb-teardown"

    @# Stop the node if it's running
    @if just arb-health-check &>/dev/null; then \
        just arb-down; \
    else \
        echo "Arbitrum node is already stopped"; \
    fi

    @echo "Removing Arbitrum container..."
    docker rm nitro-dev 2>/dev/null || true
    @echo "Removing associated volumes..."
    docker volume rm $(docker volume ls -q -f name=nitro-dev) 2>/dev/null || true
    @echo "Removing associated networks..."
    docker network rm $(docker network ls -q -f name=nitro-dev) 2>/dev/null || true
    @echo "Arbitrum node infrastructure removed."

    @just _log-end "arb-teardown"

# Deploy MetabasedSequencerChain smart contract to Optimism devnet
# TODO: Requires running RPC. Will handle soon
op-deploy-chain:
    @just _log-start "op-deploy-chain"

    cat {{ contracts_root }}/script/DeployContractsForSequencerChain.s.sol | sed -E 's/(l3ChainId = )0;/\1{{ l3_chain_id }};/' > {{ contracts_root }}/script/DeployContractsForSequencerChain_.s.sol
    [ -f {{ op_contract_deploy_file }} ] || forge script --root {{ contracts_root }} {{ contracts_root }}/script/DeployContractsForSequencerChain_.s.sol:DeployMetabasedSequencerChainPlusSetupWithAlwaysAllowModule --rpc-url {{ op_devnet_l2_rpc_url }} --private-key {{ op_devnet_private_key }} --broadcast -vvvv
    # TODO: Execute clean even if deploy fails. Also merge this in with op-clean-chain
    rm {{ contracts_root }}/script/DeployContractsForSequencerChain_.s.sol

    @just _log-end "op-deploy-chain"

# Deploy MetabasedSequencerChain smart contract to Arbitrum Orbit devnet
arb-deploy-chain: arb-up
    @just _log-start "arb-deploy-chain"

    cat {{ contracts_root }}/script/DeployContractsForSequencerChain.s.sol | sed -E 's/(l3ChainId = )0;/\1{{ l3_chain_id }};/' > {{ contracts_root }}/script/DeployContractsForSequencerChain_.s.sol
    [ -f {{ arb_contract_deploy_file }} ] || forge script --root {{ contracts_root }} {{ contracts_root }}/script/DeployContractsForSequencerChain_.s.sol:DeployMetabasedSequencerChainPlusSetupWithAlwaysAllowModule --rpc-url {{ arb_orbit_l2_rpc_url }} --private-key {{ arb_orbit_l2_private_key }} --broadcast --skip-simulation -vvvv
    # TODO: Execute clean even if deploy fails. Also merge this in with arb-clean-chain
    rm {{ contracts_root }}/script/DeployContractsForSequencerChain_.s.sol

    @just _log-end "arb-deploy-chain"

# Runs sequencer if not already running
metabased-sequencer-up: create-envrc
    @just _log-start "metabased-sequencer-up"

    @# Check if sequencer is already running by attempting a health check
    @if just metabased-sequencer-health-check >/dev/null 2>&1; then \
        echo "Metabased sequencer is already running at {{ metabased_sequencer_url }}"; \
    else \
        echo "Starting metabased sequencer..."; \
        (. {{ envrc_file }} && cd {{ sequencer_root }} && cargo run -p interceptor &); \
        echo "Waiting for sequencer to be ready..."; \
        while ! just metabased-sequencer-health-check >/dev/null 2>&1; do \
            sleep 1; \
        done; \
        echo "Metabased sequencer is now running at {{ metabased_sequencer_url }}"; \
    fi

    @just _log-end "metabased-sequencer-up"

metabased-sequencer-down:
    @just _log-start "metabased-sequencer-down"

    @# Stop sequencer if it's running
    @if just metabased-sequencer-health-check >/dev/null 2>&1; then \
        echo "Stopping metabased sequencer..."; \
        pkill -q -f interceptor; \
        echo "Metabased sequencer stopped."; \
    else \
        echo "Metabased sequencer already stopped."; \
    fi

    @just _log-end "metabased-sequencer-down"


# Removes files generated by deploying metabased sequencer chain contract to Optimism devnet
op-clean-chain:
    @just _log-start "op-clean-chain"

    rm -rf {{ op_contract_deploy_file }}

    @just _log-end "op-clean-chain"

# Removes files generated by deploying metabased sequencer chain contract to Arbitrum devnet
arb-clean-chain:
    @just _log-start "arb-clean-chain"

    rm -rf {{ arb_contract_deploy_file }}

    @just _log-end "arb-clean-chain"

# Puts contract address into localnet .envrc file
# Works by finding the value corresponding to the key in the .envrc, and replacing it with the address found in the `deploy_file`
# TODO: Requires running RPC. Will handle soon
op-update-chain-address: op-deploy-chain create-envrc
    @just _log-start "op-update-chain-address"

    cat {{ envrc_file }} | grep -v METABASED_SEQUENCER_CHAIN_CONTRACT_ADDRESS= > {{ envrc_file }}.tmp
    echo METABASED_SEQUENCER_CHAIN_CONTRACT_ADDRESS=0x$(cat {{ op_contract_deploy_file }} | grep MetabasedSequencerChain -A1 | grep contractAddress | sed 's/[^x]*0x//' | cut -c 1-40 | uniq) >> {{ envrc_file }}.tmp
    mv {{ envrc_file }}.tmp {{ envrc_file }}

    @just _log-end "op-update-chain-address"

# Make sure to add to .gitignore
# TODO: Complete this step
create-op-network-config:
    @just _log-start "create-op-network-config"

    @#! /usr/bin/env zsh
    @# Safer scripting for Just: https://just.systems/man/en/safer-bash-shebang-recipes.html
    @set -euxo pipefail

    @# Based on https://docs.optimism.io/chain/testing/dev-node
    @echo -e \
    "optimism_package:\n"\
    "    chains: # you can define multiple L2s, which will be deployed against the same L1 as a single Superchain\n"\
    "        - participants: # each participant is a node in the network. here we've defined two, one running op-geth and one running op-reth\n"\
    "            - el_type: op-geth # this node will be the sequencer since it's first in the list\n"\
    "            - el_type: op-reth\n"\
    "        network_params:\n"\
    "            name: rollup-1 # can be anything as long as it is unique\n"\
    "            network_id: 12345 # can be anything as long as it is unique \n"\
    > {{ op_network_config_file }}

    @echo "Created OP network config file at {{ op_network_config_file }}"

    @just _log-end "create-op-network-config"

# TODO(SEQ-312): Merge METABASED_SEQUENCER_CHAIN_RPC_ADDRESS -> SEQUENCING_CHAIN_RPC_URL
# Copy of `.envrc.example` using vars set earlier in the file
create-envrc:
    @just _log-start "create-envrc"

    @#! /usr/bin/env zsh
    @# Safer scripting for Just: https://just.systems/man/en/safer-bash-shebang-recipes.html
    @set -euxo pipefail

    @echo -e \
    "# Common\n"\
    "export SETTLEMENT_CHAIN_RPC_URL={{ op_devnet_l2_rpc_url }}\n"\
    "export SETTLEMENT_CHAIN_RPC_URL_WS=wss://base-rpc.publicnode.com # Optional if using WS\n"\
    "export SEQUENCING_CHAIN_RPC_URL={{ arb_orbit_l2_rpc_url }}\n"\
    "export METABASED_CHAIN_RPC_URL=http://localhost:8555\n"\
    "export METAFILLER_URL=https://staging-metafiller.metabased.org/batch/5101\n"\
    "export SEQUENCING_CONTRACT_ADDRESS=0xD77Aa8b1743326Baeb548357f8334df911A4E58f\n"\
    "export BATCH_INBOX_ADDRESS=0x97395dd253e2d096a0caa62a574895c3c2f2b2e0\n"\
    "export BATCHER_PRIVATE_KEY=fcd8aa9464a41a850d5bbc36cd6c4b6377e308a37869add1c2cf466b8d65826d\n"\
    "export SETTLEMENT_CHAIN_ID={{ op_devnet_l2_chain_id }}\n"\
    "export CUTOVER_EPOCH_BLOCK=0\n"\
    "export SETTLEMENT_CHAIN_BLOCK_TIME=2s\n"\
    "export SETTLEMENT_START_BLOCK=10289263\n"\
    "export ALT_DA_URL=http://localhost:3100\n"\
    "# Op Translator\n"\
    "export OP_TRANSLATOR_PORT={{ op_translator_port }}\n"\
    "export OP_TRANSLATOR_RPC_URL={{ op_translator_url }}\n"\
    "export OP_TRANSLATOR_LOG_LEVEL=debug\n"\
    "export OP_TRANSLATOR_PRETTY=true\n"\
    "# metabased-publisher\n"\
    "export MB_PUBLISHER_POLL_INTERVAL=1m\n"\
    "export MB_PUBLISHER_NETWORK_TIMEOUT=30s\n"\
    "export MB_PUBLISHER_BLOB_UPLOAD_TIMEOUT=10m\n"\
    "# metabased-sequencer\n"\
    "export METABASED_SEQUENCER_CHAIN_RPC_ADDRESS={{ arb_orbit_l2_rpc_url }}\n"\
    "export METABASED_SEQUENCER_PRIVATE_KEY={{ arb_orbit_l2_private_key }}\n"\
    "export METABASED_SEQUENCER_PORT={{ metabased_sequencer_port }}\n"\
    "export METABASED_SEQUENCER_CHAIN_CONTRACT_ADDRESS=0x0000000000000000000000000000000000000000\n"\
    "export ROLLUP_TYPE=ARB"\
    > {{ envrc_file }}

    @echo "Created .envrc file at {{ envrc_file }}"

    @exit 0

    @just _log-end "create-envrc"

# Puts arb contract address into localnet ENV file
# Works by finding the value corresponding to the key in the .envrc, and replacing it with the address found in the `deploy_file`
arb-update-chain-address: arb-deploy-chain create-envrc
    @just _log-start "arb-update-chain-address"

    cat {{ envrc_file }} | grep -v METABASED_SEQUENCER_CHAIN_CONTRACT_ADDRESS= > {{ envrc_file }}.tmp
    echo export METABASED_SEQUENCER_CHAIN_CONTRACT_ADDRESS=0x$(cat {{ arb_contract_deploy_file }} | grep MetabasedSequencerChain -A1 | grep contractAddress | sed 's/[^x]*0x//' | cut -c 1-40 | uniq) >> {{ envrc_file }}.tmp
    mv {{ envrc_file }}.tmp {{ envrc_file }}

    @just _log-end "arb-update-chain-address"

# TODO(SEQ-312): refactor duplicate
# Puts Arb contract address into localnet ENV file
update-chain-address: arb-deploy-chain create-envrc
    @just _log-start "update-chain-address"

    #! /usr/bin/env zsh
    # Safer scripting for Just: https://just.systems/man/en/safer-bash-shebang-recipes.html
    set -euxo pipefail
    # Get the contract address from arb deployment file
    contract_address=$(cat {{ arb_contract_deploy_file }} | grep MetabasedSequencerChain -A1 | grep contractAddress | sed 's/[^x]*0x//' | cut -c 1-40 | uniq)
    # Remove both old addresses and create temp file
    cat {{ envrc_file }} | grep -v METABASED_SEQUENCER_CHAIN_CONTRACT_ADDRESS= | grep -v SEQUENCING_CONTRACT_ADDRESS= > {{ envrc_file }}.tmp
    # Add both new addresses
    echo "METABASED_SEQUENCER_CHAIN_CONTRACT_ADDRESS=0x${contract_address}" >> {{ envrc_file }}.tmp
    echo "SEQUENCING_CONTRACT_ADDRESS=0x${contract_address}" >> {{ envrc_file }}.tmp
    mv {{ envrc_file }}.tmp {{ envrc_file }}
    exit 0

    @just _log-end "update-chain-address"

# Runs Go install for Go projects within the monorepo
go-install:
    @just _log-start "go-install"

    # go install {{ repository_root }}/metabased-publisher/cmd

    @just _log-end "go-install"

# Install foundryup (Foundry installer)
foundry-setup:
    @just _log-start "foundry-setup"

    # Based on https://book.getfoundry.sh/getting-started/installation
    @if ! command -v foundryup >/dev/null 2>&1; then \
        curl -L https://foundry.paradigm.xyz | bash; \
    else \
        echo "foundryup is already installed"; \
    fi

    # Add Foundry bin to PATH in .zshenv if it exists
    @if test -f "{{env_var('HOME')}}/.zshenv"; then \
        if ! grep -q "/.foundry/bin" "{{env_var('HOME')}}/.zshenv"; then \
            echo 'export PATH="$PATH:{{env_var('HOME')}}/.foundry/bin"' >> "{{env_var('HOME')}}/.zshenv" && \
            echo "Added Foundry bin to PATH in .zshenv"; \
            @echo "Open a new zsh terminal window to use Foundry" \
        else \
            echo "Foundry bin already in PATH in .zshenv"; \
        fi; \
    fi

    @just _log-end "foundry-setup"

# Install or upgrade Foundry with foundryup
foundry-upgrade: foundry-setup
    @just _log-start "foundry-upgrade"

    # Only run foundryup if forge isn't found or if the installed version is not the latest
    # Based on https://book.getfoundry.sh/getting-started/installation
    @if ! command -v forge >/dev/null 2>&1; then \
        echo "Installing Foundry..."; \
        foundryup; \
    elif ! foundryup --list | grep -q "$(forge --version | cut -d' ' -f2)"; then \
        echo "Updating Foundry..."; \
        foundryup; \
    else \
        echo "Foundry is already at the latest version"; \
    fi

    @just _log-end "foundry-upgrade"

# Install Foundry and dependencies for all monorepo smart contracts via forge
contracts-setup: foundry-upgrade
    @just _log-start "contracts-setup"
    
    cd {{ contracts_root }} && forge install
    cd {{ metabased_translator_contracts_root }} && forge install

    @echo "Foundry and contract dependencies installed"
    @echo "Open a new zsh terminal window to use Foundry"

    @just _log-end "contracts-setup"

# Run all OP steps in sequence
# We initialize and then spin down the devnet to get the initialization time out
# of the way upfront
op-all: op-clone contracts-setup
    @echo "Post-setup OP script completed successfully. Ready to bring up the OP Stack devnet with op-up."

setup-new-arb-chain-and-metabased-sequencer: arb-down metabased-sequencer-down arb-deploy-chain arb-update-chain-address metabased-sequencer-up
    @echo "Arbitrum Sequencer setup completed successfully. Running sequencer."

# Health check for Arbitrum node. Exits with error if RPC endpoint is not responding
arb-health-check:
    @just _log-start "arb-health-check"

    @if curl -s -X POST -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"net_version","id":1}' \
    {{ arb_orbit_l2_rpc_url }} | grep -q "result"; then \
        echo "RPC endpoint healthy at {{ arb_orbit_l2_rpc_url }}"; \
    else \
        echo "RPC endpoint not responding"; \
        exit 1; \
    fi

    @just _log-end "arb-health-check"

metabased-sequencer-health-check:
    @just _log-start "metabased-sequencer-health-check"
    # Exits with error (exit code 7) if sequencer is not responding
    curl --location {{ metabased_sequencer_url }} \
    --header 'Content-Type: application/json' \
    --data '{"jsonrpc":"2.0","method":"health","id":1}'
    @just _log-end "metabased-sequencer-health-check"

# Health check for Arbitrum node
arb-verify: arb-up
    @just _log-start "arb-verify"

    # Check if the Arbitrum node is ready via health check
    @echo "[STATUS] Waiting for Arbitrum node to be ready..."
    @until just arb-health-check >/dev/null 2>&1; do \
        sleep 10; \
    done
    @echo "[STATUS] Arbitrum node is ready"

    @just _log-end "arb-verify"

# Setup and verify sequencer
metabased-sequencer-verify: metabased-sequencer-up
    @just _log-start "metabased-sequencer-verify"

    # Wait for sequencer to be ready via health check
    @echo "[STATUS] Waiting for metabased sequencer to be ready..."
    @until just metabased-sequencer-health-check >/dev/null 2>&1; do \
        sleep 10; \
    done
    @echo "[STATUS] Metabased sequencer is ready"

    @just _log-end "metabased-sequencer-verify"

# Sends a new transaction on a new test chain and metabased sequencer
arb-verify-sendRawTransaction: arb-up metabased-sequencer-up
    @just _log-start "arb-test-sendRawTransaction"

    curl --silent --location {{ metabased_sequencer_url }} \
    --header 'Content-Type: application/json' \
    --data '{"jsonrpc":"2.0","method":"eth_sendRawTransaction","params":["0xb85902f85682038501808088ffffffffffffffff808080c001a0d555dc3a308d5bde3d5bc665796f9e7d7125c1554667325533fe237c1aa120b5a07d97dae06082d3eb7fa8966b33f6ce51d7127dcddd5da3d8be9c448a72150a90"],"id":1}' | \
    tee >(jq '.' 2>/dev/null || cat) | \
    if grep -qi "error"; then \
        echo "Error detected in response" >&2 && \
        exit 1; \
    fi

    @just _log-end "arb-test-sendRawTransaction"

# Aggregated command for CI pipeline to run all verifications
verify-all: arb-verify metabased-sequencer-verify arb-verify-sendRawTransaction

# Helper functions for command logging
# Underscores are used to indicate a private function that can be called only
# within the justfile
# Functions with underscores cannot be called externally
_log-start command:
    @echo "┌──────────────────────────────────────────┐"
    @echo "│ Starting command: {{command}}            │"
    @echo "└──────────────────────────────────────────┘"
_log-end command:
    @echo "┌──────────────────────────────────────────┐"
    @echo "│ Completed command: {{command}}           │"
    @echo "└──────────────────────────────────────────┘"

# SOURCE: https://github.com/OffchainLabs/nitro-devnode/blob/main/run-dev-node.sh , commit c501546 from Nov 2, 2024
# Modifications are:
# 1. Backslashes were added to all if statements and loops to match the whitespace requirements of Just (no newlines without backslashes)
# 2. Environment variables were converted to Justfile variables
# 3. Pattern matching was changed from bash syntax to zsh syntax
# 4. Port configuration uses justfile variables instead of hardcoded values
# 5. http.port parameter was added to the docker run command to make port usage explicit
_run-arb-nitro-dev-node:
    #!/usr/bin/env zsh

    # Start Nitro dev node in the background
    echo "Starting Nitro dev node on {{arb_orbit_l2_port}}..."
    docker run --detach --rm --name nitro-dev -p {{arb_orbit_l2_port}}:{{arb_orbit_l2_port}} offchainlabs/nitro-node:v3.2.1-d81324d --dev --http.addr 0.0.0.0 --http.api=net,web3,eth,debug \
    --http.port {{arb_orbit_l2_port}} \

    # Wait for the node to initialize
    echo "Waiting for the Nitro node to initialize on port {{arb_orbit_l2_port}} and RPC URL {{arb_orbit_l2_rpc_url}}..."

    until [[ "$(curl -s -X POST -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"net_version","params":[],"id":1}' \
    "{{arb_orbit_l2_rpc_url}}")" == *"result"* ]]; do \
        sleep 0.1; \
    done

    # Check if node is running
    curl_output=$(curl -s -X POST -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"net_version","params":[],"id":1}' \
    "{{arb_orbit_l2_rpc_url}}")

    if [[ "$curl_output" == *"result"* ]]; then \
        echo "Nitro node is running!"; \
    else \
        echo "Failed to start Nitro node."; \
        exit 1; \
    fi

    # Make the caller a chain owner
    echo "Setting chain owner to pre-funded dev account..."
    cast send 0x00000000000000000000000000000000000000FF "becomeChainOwner()" \
        --private-key {{arb_orbit_l2_private_key}} \
        --rpc-url {{arb_orbit_l2_rpc_url}}

    # Deploy Cache Manager Contract
    echo "Deploying Cache Manager contract..."
    deploy_output=$(cast send --private-key {{arb_orbit_l2_private_key}} \
        --rpc-url {{arb_orbit_l2_rpc_url}} \
        --create 0x60a06040523060805234801561001457600080fd5b50608051611d1c61003060003960006105260152611d1c6000f3fe)

    # Extract contract address using awk from plain text output
    contract_address=$(echo "$deploy_output" | awk '/contractAddress/ {print $2}')

    # Check if contract deployment was successful
    if [[ -z "$contract_address" ]]; then \
        echo "Error: Failed to extract contract address. Full output:"; \
        echo "$deploy_output"; \
        exit 1; \
    fi

    echo "Cache Manager contract deployed at address: $contract_address"

    # Register the deployed Cache Manager contract
    echo "Registering Cache Manager contract as a WASM cache manager..."
    registration_output=$(cast send --private-key {{arb_orbit_l2_private_key}} \
        --rpc-url {{arb_orbit_l2_rpc_url}} \
        0x0000000000000000000000000000000000000070 \
        "addWasmCacheManager(address)" "$contract_address")

    # Check if registration was successful
    if [[ "$registration_output" == *"error"* ]]; then \
        echo "Failed to register Cache Manager contract. Registration output:"; \
        echo "$registration_output"; \
        exit 1; \
    fi

    # If no errors, print success message
    echo "Cache Manager deployed and registered successfully. Nitro node is ready..."

e2e-tests: create-envrc
    @just _log-start "e2e-tests"
    . {{ envrc_file }} && cd {{ e2e_tests_root }} && cargo test --features e2e-tests
    @just _log-end "e2e-tests"
