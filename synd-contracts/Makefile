# include .env file and export its env vars
# (-include to ignore error if it does not exist)
# Note that any unset variables here will wipe the variables if they are set in
# .zshrc or .bashrc. Make sure that the variables are set in .env, especially if
# you're running into issues with fork tests
-include .env

# deploy SyndicateFactory to Base Sepolia
deploy-syndicate-factory :; forge script script/DeployContractsForSequencingChain.s.sol:DeploySyndicateFactory --rpc-url exo --account deployer --sender ${DEV_PUB_ADDRESS} --skip-simulation --legacy --broadcast -vv

# deploy DeploySyndicateSequencingChainPlusSetupWithAlwaysAllowModule to Syndicate Exo
deploy-syndicate-sequencing-chain-plus-setup-with-always-allow-module :; forge script script/DeployContractsForSequencingChain.s.sol:DeploySyndicateSequencingChainPlusSetupWithAlwaysAllowModule --rpc-url exo --account deployer --sender ${DEV_PUB_ADDRESS} --skip-simulation --legacy --broadcast -vv

# run AddBatchTransactionsToSyndicateSequencingChainContract script on Syndicate Exo
add-batch-transactions-to-syndicate-sequencing-chain :; forge script script/AddBatchTransactionsToSyndicateSequencingChainContract.s.sol:AddBatchTransactionsToSyndicateSequencingChainContract --rpc-url exo --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv

## deploy storage related contracts to Syndicate Exo
deploy-syndicate-storage :; forge create --rpc-url exo --account deployer src/backfill/SyndicateStorage.sol:SyndicateStorage --constructor-args ${ADMIN_ADDR} ${MANAGER_ADDR} ${APP_CHAIN_ID} --legacy
allowlist-syndicate-storage-addresses :; forge script script/AllowlistSyndicateStorageAddresses.s.sol:AllowlistSyndicateStorageAddresses --rpc-url exo --account deployer --sender ${DEV_PUB_ADDRESS} --legacy --skip-simulation --broadcast -vvv

# deploy SyndicateToken to Holesky
deploy-syndicate-token :; forge create --rpc-url ${ETH_HOLESKY_RPC_URL} --account deployer src/token/SyndicateToken.sol:SyndicateToken --broadcast --etherscan-api-key ${ETHERSCAN_API_KEY} --verify -vv --constructor-args ${ADMIN_ADDR} ${MANAGER_ADDR}

deploy-testnet-syndicate-token :; forge create --rpc-url sepolia --account deployer src/token/TestnetSyndToken.sol:TestnetSyndToken --broadcast --etherscan-api-key ${ETHERSCAN_API_KEY} --verify -vv --constructor-args ${ADMIN_ADDR} ${MINTER_ADDR}

## grant Minter Role
grant-minter-role :; forge script script/TestnetSyndScript.s.sol:GrantMinterRole --rpc-url sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv

# mint TestnetSynd to addresses
mint-synd-to-addresses :; forge script script/TestnetSyndScript.s.sol:MintSYNDToAddresses --rpc-url sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv

# deploy AgentApplication to Syndicate Exo
deploy-agent-application :; forge create --rpc-url exo --account deployer src/use-cases/dream/AgentApplication.sol:AgentApplication --legacy --broadcast --verify --verifier blockscout --verifier-url 'https://syndicate-exo.explorer.alchemy.com/api/' -vvv --constructor-args ${ADMIN_ADDR}

# verify AgentApplication contract
verify-agent-application :; forge verify-contract --rpc-url exo --verifier blockscout --verifier-url 'https://syndicate-exo.explorer.alchemy.com/api/' ${CONTRACT_ADDRESS} src/use-cases/dream/AgentApplication.sol:AgentApplication

## Tx Limit Test
# run tx limit test against OP Mainnet or Base Sepolia
# and save output to file
# need to change target network in the test file
test-tx-limit :; forge test --match-contract SyndicateSequencingChainIntegrationTest -vv > test/integration/syndicate-tx-limit-data/optimism_test_output_$(date +%Y%m%d_%H%M%S).log 2>&1

test-events-limit :; forge test --match-contract SyndicateSequencingChainEventLimitTest -vv > test/integration/syndicate-events-limit-data/base_sepolia_test_output_$(date +%Y%m%d_%H%M%S).md 2>&1

docs-pre-audit-gen :; forge doc --build --out docs-pre-audit

docs-pre-audit-serve :; forge doc --serve --out docs-pre-audit

test-coverage :; forge coverage --ir-minimum --no-match-coverage "(script|test|src/testing-purposes)"

# Test bridge flows on sepolia
test-arbitrum-bridge-flow :; forge script script/TestArbitrumBridgeFlow.s.sol:TestArbitrumBridgeFlow --rpc-url sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv

test-optimism-bridge-flow :; forge script script/TestOptimismBridgeFlow.s.sol:TestOptimismBridgeFlow --rpc-url sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv

# Test bridge flows on sepolia (simulation only)
simulate-arbitrum-bridge-flow :; forge script script/TestArbitrumBridgeFlow.s.sol:TestArbitrumBridgeFlow --rpc-url sepolia --sender ${DEV_PUB_ADDRESS} -vv

simulate-optimism-bridge-flow :; forge script script/TestOptimismBridgeFlow.s.sol:TestOptimismBridgeFlow --rpc-url sepolia --sender ${DEV_PUB_ADDRESS} -vv

## Crosschain Token Direct Deployment Commands

# Deploy SyndicateTokenCrosschain directly (mainnet)
deploy-syndicate-crosschain-token :; forge script script/DeployCrosschainTokenDirect.s.sol:DeploySyndicateTokenCrosschainDirect --rpc-url mainnet --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast --etherscan-api-key ${ETHERSCAN_API_KEY} --verify -vv

# Deploy TestnetSyndTokenCrosschain directly (testnet)
deploy-testnet-crosschain-token :; forge script script/DeployCrosschainTokenDirect.s.sol:DeployTestnetSyndTokenCrosschainDirect --rpc-url sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast --etherscan-api-key ${ETHERSCAN_API_KEY} --verify -vv

# Verify crosschain address consistency (simulation only)
verify-crosschain-consistency :; forge script script/DeployCrosschainTokenDirect.s.sol:VerifyCrosschainAddressConsistency --sender ${DEV_PUB_ADDRESS} -vv

# verify contract on etherscan example
verify-contract-direct :; forge verify-contract --rpc-url sepolia --watch --etherscan-api-key ${ETHERSCAN_API_KEY} ${contractAddress} ${contractPath}:${contractName} --constructor-args ${constructorArgsABIEncoded}

# Example of how to ABI encode constructor args
# cast abi-encode "constructor(address,address)" 0x45d6450fC59A6F8D9c753126cE1EFF6fa4D7e0fb 0x45d6450fC59A6F8D9c753126cE1EFF6fa4D7e0fb

## Airdrop Commands

# Deploy airdrop contract
deploy-airdrop :; forge script script/airdrop/AirdropScript.s.sol:DeployAirdrop --rpc-url ${RPC_URL} --account ${ACCOUNT} --sender ${DEV_PUB_ADDRESS} --broadcast --etherscan-api-key ${ETHERSCAN_API_KEY} --verify -vv

# Execute airdrop with bash script (dry run)
airdrop-dry-run :; DRY_RUN=true AIRDROP_CONTRACT=${AIRDROP_CONTRACT} TOKEN_ADDRESS=${TOKEN_ADDRESS} RPC_URL=${RPC_URL} ACCOUNT=${ACCOUNT} DEV_PUB_ADDRESS=${DEV_PUB_ADDRESS} ./script/airdrop/airdrop.sh

# Execute airdrop with bash script
airdrop-execute :; AIRDROP_CONTRACT=${AIRDROP_CONTRACT} TOKEN_ADDRESS=${TOKEN_ADDRESS} RPC_URL=${RPC_URL} ACCOUNT=${ACCOUNT} DEV_PUB_ADDRESS=${DEV_PUB_ADDRESS} ./script/airdrop/airdrop.sh

## Storage Layout Validation Commands

# Create storage layout baseline (run this on main branch)
storage-layout-baseline :; @echo "Creating storage layout baseline..." && \
	forge build && \
	mkdir -p .storage-layouts && \
	node scripts/storage-checks/forge-extract.js > .storage-layouts/main-layout.json && \
	echo "Baseline created at .storage-layouts/main-layout.json"

# Extract current storage layout
storage-layout-extract :; @echo "Extracting current storage layout..." && \
	forge build && \
	mkdir -p .storage-layouts && \
	node scripts/storage-checks/forge-extract.js > .storage-layouts/current-layout.json && \
	echo "Current layout extracted to .storage-layouts/current-layout.json"

# Compare storage layouts (requires baseline)
storage-layout-compare :; @echo "Comparing storage layouts..." && \
	if [ ! -f .storage-layouts/main-layout.json ]; then \
		echo "ERROR: No baseline layout found. Run 'make storage-layout-baseline' first."; \
		exit 1; \
	fi && \
	npm run storage:compare -- --ref .storage-layouts/main-layout.json --head .storage-layouts/current-layout.json && \
	echo "Storage layout comparison completed successfully"

# Check inheritance ordering
storage-layout-inheritance :; @echo "Checking inheritance ordering..." && \
	forge build --build-info && \
	mkdir -p .storage-layouts && \
	cp out/build-info/*.json .storage-layouts/temp-build.json && \
	npm run storage:inheritance .storage-layouts/temp-build.json && \
	rm -f .storage-layouts/temp-build.json && \
	echo "Inheritance ordering check completed successfully"

# Full storage layout validation (extract + compare + inheritance)
storage-layout-check :; @echo "Running full storage layout validation..." && \
	$(MAKE) storage-layout-extract && \
	$(MAKE) storage-layout-inheritance && \
	$(MAKE) storage-layout-compare && \
	echo "All storage layout checks passed!"

# Clean storage layout artifacts
storage-layout-clean :; rm -rf .storage-layouts out/build-info

# Auto-update baseline when contracts change (for development)
storage-layout-auto-update :; @echo "🔄 Auto-updating storage layout baseline..." && \
	forge build && \
	mkdir -p .storage-layouts && \
	node scripts/storage-checks/forge-extract.js > .storage-layouts/main-layout.json && \
	echo "✅ Storage layout baseline auto-updated"

# Smart build that updates baseline if needed
build-with-storage :; @echo "🏗️ Building with storage layout tracking..." && \
	$(MAKE) storage-layout-auto-update && \
	forge build && \
	echo "Build complete with updated storage layouts"
