# include .env file and export its env vars
# (-include to ignore error if it does not exist)
# Note that any unset variables here will wipe the variables if they are set in
# .zshrc or .bashrc. Make sure that the variables are set in .env, especially if
# you're running into issues with fork tests
-include .env

# deploy DeployBasedSequencerChain to SYNDICATE_FRAME
deploy-based-sequencerchain-frame :; forge script script/DeployContractsForSequencerChain.s.sol:DeployBasedSequencerChain --rpc-url ${SYNDICATE_FRAME_RPC_URL}  --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeploySealedBidAuctionSequencingModule to SYNDICATE_FRAME
deploy-sealed-bid-auction-sequencing-module-frame :; forge script script/DeployContractsForSequencerChain.s.sol:DeploySealedBidAuctionSequencingModule --rpc-url ${SYNDICATE_FRAME_RPC_URL}  --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployTokenBalanceSequencingModule to SYNDICATE_FRAME
deploy-token-balance-sequencing-module-frame :; forge script script/DeployContractsForSequencerChain.s.sol:DeployTokenBalanceSequencingModule --rpc-url ${SYNDICATE_FRAME_RPC_URL}  --private-key ${ETH_PRIVATE_KEY} --broadcast -vv

# deploy DeployAllowlistSequencingModule to SYNDICATE_FRAME
deploy-allowlist-sequencing-module-frame :; forge script script/DeployContractsForSequencerChain.s.sol:DeployAllowlistSequencingModule --rpc-url ${SYNDICATE_FRAME_RPC_URL}  --private-key ${ETH_PRIVATE_KEY} --broadcast -vv