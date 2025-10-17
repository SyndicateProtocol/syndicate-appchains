# include .env file and export its env vars
# (-include to ignore error if it does not exist)
# Note that any unset variables here will wipe the variables if they are set in
# .zshrc or .bashrc. Make sure that the variables are set in .env, especially if
# you're running into issues with fork tests
-include .env

# deploy SyndicateFactory to Base Sepolia
deploy-syndicate-factory :; forge script script/DeploySyndicateFactoryDeterministic.s.sol:DeploySyndicateFactoryDeterministic --rpc-url base_sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv

preview-syndicate-factory :; forge script script/DeploySyndicateFactoryDeterministic.s.sol:PreviewSyndicateFactoryAddresses --rpc-url base_sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv

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

# Simple storage layout validation for upgradeable contracts
storage-layout-check :; @echo "Validating storage layouts for upgradeable contracts..." && \
	forge build && \
	$(MAKE) storage-layout-baseline && \
	$(MAKE) storage-layout-compare && \
	echo "Storage layout validation passed!"

# Create baseline storage layout
storage-layout-baseline :; @echo "Creating storage layout baseline..." && \
	mkdir -p .storage-layouts && \
	forge inspect SyndicateFactory storageLayout > .storage-layouts/SyndicateFactory.baseline.json && \
	forge inspect SyndicateSequencingChain storageLayout > .storage-layouts/SyndicateSequencingChain.baseline.json && \
	forge inspect ArbChainConfig storageLayout > .storage-layouts/ArbChainConfig.baseline.json && \
	echo "✅ Storage layout baselines created"

# Compare current storage layout against baseline
storage-layout-compare :; @echo "Comparing storage layouts..." && \
	mkdir -p .storage-layouts && \
	forge inspect SyndicateFactory storageLayout > .storage-layouts/SyndicateFactory.current.json && \
	forge inspect SyndicateSequencingChain storageLayout > .storage-layouts/SyndicateSequencingChain.current.json && \
	forge inspect ArbChainConfig storageLayout > .storage-layouts/ArbChainConfig.current.json && \
	for contract in SyndicateFactory SyndicateSequencingChain ArbChainConfig; do \
		if [ -f .storage-layouts/$$contract.baseline.json ]; then \
			echo "Comparing $$contract..."; \
			if ! cmp -s .storage-layouts/$$contract.baseline.json .storage-layouts/$$contract.current.json; then \
				echo "❌ STORAGE LAYOUT CHANGED: $$contract"; \
				echo "Baseline:"; \
				cat .storage-layouts/$$contract.baseline.json; \
				echo "Current:"; \
				cat .storage-layouts/$$contract.current.json; \
				echo "This change could corrupt storage during upgrades!"; \
				exit 1; \
			else \
				echo "✅ $$contract: Storage layout unchanged"; \
			fi; \
		else \
			echo "ℹ️  $$contract: No baseline found, creating baseline..."; \
			cp .storage-layouts/$$contract.current.json .storage-layouts/$$contract.baseline.json; \
		fi; \
	done && \
	echo "✅ All storage layouts are safe for upgrades"

# Clean storage layout artifacts
storage-layout-clean :; rm -rf .storage-layouts out/build-info

# Update storage layout
storage-layout-update :; @echo "Updating storage layout..." && \
	$(MAKE) storage-layout-clean && \
	mkdir -p .storage-layouts && \
	forge inspect --json SyndicateFactory storageLayout > .storage-layouts/SyndicateFactory.json && \
	forge inspect --json SyndicateSequencingChain storageLayout > .storage-layouts/SyndicateSequencingChain.json && \
	forge inspect --json ArbChainConfig storageLayout > .storage-layouts/ArbChainConfig.json && \
	echo "✅ Storage layouts updated"

## 🚀 Deployment & Upgrade Commands
## See docs/upgrade/README.md for detailed guide

# Deploy SyndicateFactory (deterministic deployment)
# Also auto-deploys: GasAggregator, SyndicateSequencingChain implementation
deploy-factory :; forge script script/DeploySyndicateFactoryDeterministic.s.sol:DeploySyndicateFactoryDeterministic \
	--rpc-url ${RPC_URL} \
	--account ${ACCOUNT} \
	--sender ${DEV_PUB_ADDRESS} \
	--broadcast \
	-vv

# Preview SyndicateFactory deployment addresses (same across all chains)
preview-factory :; forge script script/DeploySyndicateFactoryDeterministic.s.sol:PreviewSyndicateFactoryAddresses \
	--rpc-url ${RPC_URL} \
	-vv

# Deploy ArbConfigManager (optional - only for Arbitrum settlement)
# Required env: OWNER_ADDRESS
deploy-arb-config :; forge script script/DeployArbConfigManager.s.sol:DeployArbConfigManager \
	--rpc-url ${RPC_URL} \
	--account ${ACCOUNT} \
	--sender ${DEV_PUB_ADDRESS} \
	--broadcast \
	-vv

## 🔗 Create Sequencing Chains

# Create a new sequencing chain via factory
# Required env: FACTORY_ADDRESS, NONCE, ADMIN_ADDRESS
# Optional env: PERMISSION_MODULE (deploys AlwaysAllowedModule if not set)
# Example: NONCE=1 make create-sequencing-chain
# Note: Chain ID is computed from nonce by the factory
create-sequencing-chain :; forge script script/upgrade/CreateSequencingChain.s.sol \
	--rpc-url ${RPC_URL} \
	--account ${ACCOUNT} \
	--sender ${DEV_PUB_ADDRESS} \
	--broadcast \
	-vv

## ⚠️  Upgrade Commands - ALWAYS run 'make storage-layout-check' first!

# Upgrade SyndicateFactory to new implementation
# Required env: FACTORY_ADDRESS, ADMIN_ADDRESS
upgrade-factory :; @echo "⚠️  Run 'make storage-layout-check' first!" && \
	forge script script/upgrade/UpgradeContracts.s.sol:UpgradeSyndicateFactory \
	--rpc-url ${RPC_URL} \
	--account ${ACCOUNT} \
	--sender ${DEV_PUB_ADDRESS} \
	--broadcast \
	-vv

# Upgrade GasAggregator to new implementation
# Required env: GAS_AGGREGATOR_ADDRESS, ADMIN_ADDRESS
upgrade-gas-aggregator :; @echo "⚠️  Run 'make storage-layout-check' first!" && \
	forge script script/upgrade/UpgradeContracts.s.sol:UpgradeGasAggregator \
	--rpc-url ${RPC_URL} \
	--account ${ACCOUNT} \
	--sender ${DEV_PUB_ADDRESS} \
	--broadcast \
	-vv

# Upgrade SyndicateSequencingChain to new implementation
# Sets new default in factory and notifies GasAggregator
# Required env: CHAIN_ADDRESS, FACTORY_ADDRESS
upgrade-sequencing-chain :; @echo "⚠️  Run 'make storage-layout-check' first!" && \
	forge script script/upgrade/UpgradeSequencingChain.s.sol \
	--rpc-url ${RPC_URL} \
	--account ${ACCOUNT} \
	--sender ${DEV_PUB_ADDRESS} \
	--broadcast \
	-vv

# Deploy emissions contracts
deploy-emissions :; forge script script/DeployStakingContracts.s.sol:DeployStakingContracts --private-key ${PRIVATE_KEY} --rpc-url eth --broadcast -vvv
