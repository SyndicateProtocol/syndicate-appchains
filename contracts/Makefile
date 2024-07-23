# include .env file and export its env vars
# (-include to ignore error if it does not exist)
# Note that any unset variables here will wipe the variables if they are set in
# .zshrc or .bashrc. Make sure that the variables are set in .env, especially if
# you're running into issues with fork tests
-include .env

# deploy DeployBasedSequencerChain to Base Sepolia
deploy-based-sequencerchain :; forge script script/DeployContractsForSequencerChain.s.sol:DeployBasedSequencerChain --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# run AddModuleToBasedSequencerChain script on Base Sepolia
add-module-to-based-sequencerchain :; forge script script/DeployContractsForSequencerChain.s.sol:AddModuleToBasedSequencerChain --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeploySealedBidAuctionSequencingModule to Base Sepolia
deploy-sealed-bid-auction-sequencing-module :; forge script script/DeployContractsForSequencerChain.s.sol:DeploySealedBidAuctionSequencingModule --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployTokenBalanceSequencingModule to Base Sepolia
deploy-token-balance-sequencing-module :; forge script script/DeployContractsForSequencerChain.s.sol:DeployTokenBalanceSequencingModule --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployAllowlistSequencingModule to Base Sepolia
deploy-allowlist-sequencing-module :; forge script script/DeployContractsForSequencerChain.s.sol:DeployAllowlistSequencingModule --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployAlwaysAllowedModule to Base Sepolia
deploy-always-allowed-module :; forge script script/DeployContractsForSequencerChain.s.sol:DeployAlwaysAllowedModule --rpc-url base_sepolia --private-key ${ETH_PRIVATE_KEY} --broadcast -vv