# include .env file and export its env vars
# (-include to ignore error if it does not exist)
# Note that any unset variables here will wipe the variables if they are set in
# .zshrc or .bashrc. Make sure that the variables are set in .env, especially if
# you're running into issues with fork tests
-include .env

# deploy SyndicateFactory to Base Sepolia
deploy-syndicate-factory :; forge script script/DeployContractsForSequencingChain.s.sol:DeploySyndicateFactory --rpc-url exo --private-key ${ETH_PRIVATE_KEY} --skip-simulation --legacy --broadcast -vv

# deploy DeploySyndicateSequencingChainPlusSetupWithAlwaysAllowModule to Syndicate Exo
deploy-syndicate-sequencing-chain-plus-setup-with-always-allow-module :; forge script script/DeployContractsForSequencingChain.s.sol:DeploySyndicateSequencingChainPlusSetupWithAlwaysAllowModule --rpc-url exo --private-key ${ETH_PRIVATE_KEY} --skip-simulation --legacy --broadcast -vv

# run AddBatchTransactionsToSyndicateSequencingChainContract script on Syndicate Exo
add-batch-transactions-to-syndicate-sequencing-chain :; forge script script/AddBatchTransactionsToSyndicateSequencingChainContract.s.sol:AddBatchTransactionsToSyndicateSequencingChainContract --rpc-url exo --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

## deploy storage related contracts to Syndicate Exo
deploy-syndicate-storage :; forge create --rpc-url exo --private-key ${ETH_PRIVATE_KEY} src/backfill/SyndicateStorage.sol:SyndicateStorage --constructor-args ${ADMIN_ADDR} ${MANAGER_ADDR} ${APP_CHAIN_ID} --legacy
allowlist-syndicate-storage-addresses :; forge script script/AllowlistSyndicateStorageAddresses.s.sol:AllowlistSyndicateStorageAddresses --rpc-url exo --private-key ${ETH_PRIVATE_KEY} --legacy --skip-simulation --broadcast -vvv

# deploy SyndicateToken to Holesky
deploy-syndicate-token :; forge create --rpc-url ${ETH_HOLESKY_RPC_URL} --private-key ${ETH_PRIVATE_KEY} src/token/SyndicateToken.sol:SyndicateToken --broadcast --etherscan-api-key ${ETHERSCAN_API_KEY} --verify -vv --constructor-args ${ADMIN_ADDR} ${MANAGER_ADDR}

## grant Minter Role
grant-minter-role :; forge script script/SYNDScript.s.sol:GrantMinterRole --rpc-url holesky --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# mint SYND to addresses
mint-synd-to-addresses :; forge script script/SYNDScript.s.sol:MintSYNDToAddresses --rpc-url holesky --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy AgentApplication to Syndicate Exo
deploy-agent-application :; forge create --rpc-url exo --private-key ${ETH_PRIVATE_KEY} src/use-cases/dream/AgentApplication.sol:AgentApplication --legacy --broadcast --verify --verifier blockscout --verifier-url 'https://syndicate-exo.explorer.alchemy.com/api/' -vvv --constructor-args ${ADMIN_ADDR}

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

test-coverage :; forge coverage --ir-minimum --no-match-coverage "(script|test)"