# include .env file and export its env vars
# (-include to ignore error if it does not exist)
# Note that any unset variables here will wipe the variables if they are set in
# .zshrc or .bashrc. Make sure that the variables are set in .env, especially if
# you're running into issues with fork tests
-include .env

# deploy DeployMetabasedSequencerChain to Base Sepolia
deploy-metabased-sequencerchain :; forge script script/DeployContractsForSequencerChain.s.sol:DeployMetabasedSequencerChain --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployMetabasedSequencerChainForTestnet to Metabased Testnet
deploy-metabased-sequencerchain-testnet :; forge script script/DeployContractsForSequencerChain.s.sol:DeployMetabasedSequencerChainForTestnet --rpc-url metabased_testnet --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# run AddModuleToMetabasedSequencerChain script on Base Sepolia
add-module-to-metabased-sequencerchain :; forge script script/DeployContractsForSequencerChain.s.sol:AddModuleToMetabasedSequencerChain --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeploySealedBidAuctionSequencingModule to Base Sepolia
deploy-sealed-bid-auction-sequencing-module :; forge script script/DeployContractsForSequencerChain.s.sol:DeploySealedBidAuctionSequencingModule --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployTokenBalanceSequencingModule to Base Sepolia
deploy-token-balance-sequencing-module :; forge script script/DeployContractsForSequencerChain.s.sol:DeployTokenBalanceSequencingModule --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployAllowlistSequencingModule to Base Sepolia
deploy-allowlist-sequencing-module :; forge script script/DeployContractsForSequencerChain.s.sol:DeployAllowlistSequencingModule --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployAlwaysAllowedModule to Base Sepolia
deploy-always-allowed-module :; forge script script/DeployContractsForSequencerChain.s.sol:DeployAlwaysAllowedModule --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# run AddBatchTransactionsToMetabasedSequencerChainContract script on Base Sepolia
add-batch-transactions-to-metabased-sequencerchain :; forge script script/AddBatchTransactionsToMetabasedSequencerChainContract.s.sol:AddBatchTransactionsToMetabasedSequencerChainContract --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# run AddBatchTransactionsToMetabasedSequencerChainContract script on localhost
add-batch-transactions-to-metabased-sequencerchain-localhost :; forge script script/AddBatchTransactionsToMetabasedSequencerChainContract.s.sol:AddBatchTransactionsToMetabasedSequencerChainContract --rpc-url localhost --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# run tx limit test against OP Mainnet or Base Sepolia
# and save output to file
# need to change target network in the test file
test-tx-limit :; forge test --match-contract MetabaseSequencerChainIntegrationTest -vv > test/integration/metabase-tx-limit-data/optimism_test_output_$(date +%Y%m%d_%H%M%S).log 2>&1

test-tx-limit :; forge test --match-contract MetabaseSequencerChainIntegrationTest -vv > test/integration/metabase-tx-limit-data/optimism_test_output_$(date +%Y%m%d_%H%M%S).md 2>&1

test-events-limit :; forge test --match-contract MetabaseSequencerChainEventLimitTest -vv > test/integration/metabase-events-limit-data/base_sepolia_test_output_$(date +%Y%m%d_%H%M%S).md 2>&1

# deploy L3BackfillData to Metabased Testnet
deploy-l3-backfill-data :; forge create --rpc-url metabased_testnet --private-key ${ETH_PRIVATE_KEY} src/L3BackfillData.sol:L3BackfillData --constructor-args ${ADMIN} ${MANAGER}