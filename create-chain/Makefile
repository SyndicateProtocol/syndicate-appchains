.PHONY: generate-contract-abis
generate-contract-abis:
	@echo "Generating contract ABIs and bytecode..."
	@mkdir -p scripts/abi/synd

	# Generate AllowlistSequencingModule
	@echo "Generating AllowlistSequencingModule..."
	@cd ../synd-contracts && \
	  echo "export const AllowlistSequencingModuleABI = $$(forge inspect src/sequencing-modules/AllowlistSequencingModule.sol:AllowlistSequencingModule abi --json) as const" > ../create-chain/scripts/abi/synd/AllowlistSequencingModule.ts && \
	  echo "" >> ../create-chain/scripts/abi/synd/AllowlistSequencingModule.ts && \
	  echo "export const AllowlistSequencingModuleBytecode =" >> ../create-chain/scripts/abi/synd/AllowlistSequencingModule.ts && \
	  echo "  \"$$(forge inspect src/sequencing-modules/AllowlistSequencingModule.sol:AllowlistSequencingModule bytecode)\"" >> ../create-chain/scripts/abi/synd/AllowlistSequencingModule.ts

	# Generate ArbChainConfig
	@echo "Generating ArbChainConfig..."
	@cd ../synd-contracts && \
	  echo "export const ArbChainConfigABI = $$(forge inspect src/config/ArbChainConfig.sol:ArbChainConfig abi --json) as const" > ../create-chain/scripts/abi/synd/ArbChainConfig.ts

	# Generate ArbConfigManager
	@echo "Generating ArbConfigManager..."
	@cd ../synd-contracts && \
	  echo "export const ArbConfigManagerABI = $$(forge inspect src/config/ArbConfigManager.sol:ArbConfigManager abi --json) as const" > ../create-chain/scripts/abi/synd/ArbConfigManager.ts

	# Generate AssertionPoster
	@echo "Generating AssertionPoster..."
	@cd ../synd-contracts && \
	  echo "export const AssertionPosterABI = $$(forge inspect src/withdrawal/AssertionPoster.sol:AssertionPoster abi --json) as const" > ../create-chain/scripts/abi/synd/AssertionPoster.ts && \
	  echo "" >> ../create-chain/scripts/abi/synd/AssertionPoster.ts && \
	  echo "export const assertionPosterBytecode =" >> ../create-chain/scripts/abi/synd/AssertionPoster.ts && \
	  echo "  \"$$(forge inspect src/withdrawal/AssertionPoster.sol:AssertionPoster bytecode)\"" >> ../create-chain/scripts/abi/synd/AssertionPoster.ts

	# Generate RequireAndModule
	@echo "Generating RequireAndModule..."
	@cd ../synd-contracts && \
	  echo "export const RequireAndModuleABI = $$(forge inspect src/requirement-modules/RequireAndModule.sol:RequireAndModule abi --json) as const" > ../create-chain/scripts/abi/synd/RequireAndModule.ts

	# Generate RequireAndModuleFactory
	@echo "Generating RequireAndModuleFactory..."
	@cd ../synd-contracts && \
	  echo "export const RequireAndModuleFactoryABI = $$(forge inspect src/factory/RequireAndModuleFactory.sol:RequireAndModuleFactory abi --json) as const" > ../create-chain/scripts/abi/synd/RequireAndModuleFactory.ts

	# Generate SyndicateFactory
	@echo "Generating SyndicateFactory..."
	@cd ../synd-contracts && \
	  echo "export const SyndicateFactoryABI = $$(forge inspect src/factory/SyndicateFactory.sol:SyndicateFactory abi --json) as const" > ../create-chain/scripts/abi/synd/SyndicateFactory.ts

	# Generate SyndicateSequencingChain
	@echo "Generating SyndicateSequencingChain..."
	@cd ../synd-contracts && \
	  echo "export const SyndicateSequencingChainABI = $$(forge inspect src/SyndicateSequencingChain.sol:SyndicateSequencingChain abi --json) as const" > ../create-chain/scripts/abi/synd/SyndicateSequencingChain.ts

	# Generate TeeKeyManager
	@echo "Generating TeeKeyManager..."
	@cd ../synd-contracts && \
	  echo "export const TeeKeyManagerABI = $$(forge inspect src/withdrawal/TeeKeyManager.sol:TeeKeyManager abi --json) as const" > ../create-chain/scripts/abi/synd/TeeKeyManager.ts

	# Generate TeeModule
	@echo "Generating TeeModule..."
	@cd ../synd-contracts && \
	  echo "export const TeeModuleABI = $$(forge inspect src/withdrawal/TeeModule.sol:TeeModule abi --json) as const" > ../create-chain/scripts/abi/synd/TeeModule.ts && \
	  echo "" >> ../create-chain/scripts/abi/synd/TeeModule.ts && \
	  echo "export const teeModuleBytecode =" >> ../create-chain/scripts/abi/synd/TeeModule.ts && \
	  echo "  \"$$(forge inspect src/withdrawal/TeeModule.sol:TeeModule bytecode)\"" >> ../create-chain/scripts/abi/synd/TeeModule.ts

	@echo "Done! All contract ABIs and bytecode have been generated."
