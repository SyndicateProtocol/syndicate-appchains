# include .env file and export its env vars
# (-include to ignore error if it does not exist)
# Note that any unset variables here will wipe the variables if they are set in
# .zshrc or .bashrc. Make sure that the variables are set in .env, especially if
# you're running into issues with fork tests
-include .env


# deploy IRLNFT to Syndicate Exo
deploy-irl-nft :; forge create --rpc-url irl --private-key ${ETH_PRIVATE_KEY} src/IRL/IRLNFT.sol:IRLNFT --legacy --broadcast --verify --verifier blockscout --verifier-url 'https://explorer.testnet.irl.syndicate.io/api/' -vvv --constructor-args ${ADMIN_ADDR} ${MINTER}

# verify IRLNFT contract
verify-irl-nft :; forge verify-contract --rpc-url irl --verifier blockscout --verifier-url 'https://explorer.testnet.irl.syndicate.io/api/' ${CONTRACT_ADDRESS} src/IRL/IRLNFT.sol:IRLNFT
