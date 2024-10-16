# include .env file and export its env vars
# (-include to ignore error if it does not exist)
# Note that any unset variables here will wipe the variables if they are set in
# .zshrc or .bashrc. Make sure that the variables are set in .env, especially if
# you're running into issues with fork tests
-include .env

# deploy MetabasedFactory to Base Sepolia
deploy-metabased-sequencerchain :; forge script script/DeployContractsForSequencerChain.s.sol:DeployMetabasedFactory --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployMetabasedSequencerChainPlusSetup to Metabased Testnet
deploy-metabased-sequencerchain-plus-setup :; forge script script/DeployContractsForSequencerChain.s.sol:DeployMetabasedSequencerChainPlusSetup --rpc-url metabased_testnet --private-key ${ETH_PRIVATE_KEY} --broadcast -vv


# run AddBatchTransactionsToMetabasedSequencerChainContract script on Base Sepolia
add-batch-transactions-to-metabased-sequencerchain :; forge script script/AddBatchTransactionsToMetabasedSequencerChainContract.s.sol:AddBatchTransactionsToMetabasedSequencerChainContract --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

## deploy Metafiller related contracts to Metabased Testnet
deploy-l3-backfill-storage :; forge create --rpc-url metabased_testnet --private-key ${ETH_PRIVATE_KEY} src/backfill/L3BackfillStorage.sol:L3BackfillStorage --constructor-args ${ADMIN} ${MANAGER}

deploy-l3-backfill-mapper :; forge create --rpc-url metabased_testnet --private-key ${ETH_PRIVATE_KEY} src/backfill/L3BackfillMapper.sol:L3BackfillMapper --constructor-args ${ADMIN} ${MANAGER}


## Tx Limit Test
# run tx limit test against OP Mainnet or Base Sepolia
# and save output to file
# need to change target network in the test file
test-tx-limit :; forge test --match-contract MetabaseSequencerChainIntegrationTest -vv > test/integration/metabase-tx-limit-data/optimism_test_output_$(date +%Y%m%d_%H%M%S).log 2>&1

test-events-limit :; forge test --match-contract MetabaseSequencerChainEventLimitTest -vv > test/integration/metabase-events-limit-data/base_sepolia_test_output_$(date +%Y%m%d_%H%M%S).md 2>&1



