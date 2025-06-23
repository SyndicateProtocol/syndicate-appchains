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

## Crosschain Token Factory Commands

# Deploy SyndicateTokenCrosschain factory (mainnet)
deploy-syndicate-crosschain-factory :; forge script script/DeployCrosschainTokenFactories.s.sol:DeploySyndicateTokenCrosschainFactory --rpc-url mainnet --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast --etherscan-api-key ${ETHERSCAN_API_KEY} --verify -vv

# Deploy SyndicateTokenCrosschain via factory (deterministic addressing)
deploy-syndicate-crosschain-token :; forge script script/DeployCrosschainTokenFactories.s.sol:DeploySyndicateTokenCrosschainViaFactory --rpc-url mainnet --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv


# Deploy TestnetSyndTokenCrosschain factory (testnet)
deploy-testnet-crosschain-factory :; forge script script/DeployCrosschainTokenFactories.s.sol:DeployTestnetSyndTokenCrosschainFactory --rpc-url sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast --etherscan-api-key ${ETHERSCAN_API_KEY} --verify -vv

# Deploy TestnetSyndTokenCrosschain via factory (deterministic addressing)
deploy-testnet-crosschain-token :; forge script script/DeployCrosschainTokenFactories.s.sol:DeployTestnetSyndTokenCrosschainViaFactory --rpc-url sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv

# Configure bridges for crosschain token
configure-crosschain-bridges :; forge script script/DeployCrosschainTokenFactories.s.sol:ConfigureCrosschainTokenBridges --rpc-url sepolia --account deployer --sender ${DEV_PUB_ADDRESS} --broadcast -vv

# Verify crosschain address consistency (simulation only)
verify-crosschain-consistency :; forge script script/DeployCrosschainTokenFactories.s.sol:VerifyCrosschainAddressConsistency --sender ${DEV_PUB_ADDRESS} -vv
