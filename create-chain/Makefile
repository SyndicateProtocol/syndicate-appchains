.PHONY: generate-contract-abis
generate-contract-abis:
	@echo "Generating contract ABIs and bytecode..."
	@mkdir -p scripts/abi/synd
	
	@echo "Generating AllowlistSequencingModule..."
	@cd ../synd-contracts && \
	  echo "export const allowlistSequencingModuleABI = $$(forge inspect src/sequencing-modules/AllowlistSequencingModule.sol:AllowlistSequencingModule abi --json) as const" > ../create-chain/scripts/abi/synd/AllowlistSequencingModule.ts && \
	  echo "" >> ../create-chain/scripts/abi/synd/AllowlistSequencingModule.ts && \
	  echo "export const allowlistSequencingModuleBytecode =" >> ../create-chain/scripts/abi/synd/AllowlistSequencingModule.ts && \
	  echo "  \"$$(forge inspect src/sequencing-modules/AllowlistSequencingModule.sol:AllowlistSequencingModule bytecode)\"" >> ../create-chain/scripts/abi/synd/AllowlistSequencingModule.ts
	
	@echo "Generating ArbChainConfig..."
	@cd ../synd-contracts && \
	  echo "export const arbChainConfigABI = $$(forge inspect src/config/ArbChainConfig.sol:ArbChainConfig abi --json) as const" > ../create-chain/scripts/abi/synd/ArbChainConfig.ts
	
	@echo "Generating ArbConfigManager..."
	@cd ../synd-contracts && \
	  echo "export const arbConfigManagerABI = $$(forge inspect src/config/ArbConfigManager.sol:ArbConfigManager abi --json) as const" > ../create-chain/scripts/abi/synd/ArbConfigManager.ts
	
	@echo "Generating AssertionPoster..."
	@cd ../synd-contracts && \
	  echo "export const assertionPosterABI = $$(forge inspect src/withdrawal/AssertionPoster.sol:AssertionPoster abi --json) as const" > ../create-chain/scripts/abi/synd/AssertionPoster.ts && \
	  echo "" >> ../create-chain/scripts/abi/synd/AssertionPoster.ts && \
	  echo "export const assertionPosterBytecode =" >> ../create-chain/scripts/abi/synd/AssertionPoster.ts && \
	  echo "  \"$$(forge inspect src/withdrawal/AssertionPoster.sol:AssertionPoster bytecode)\"" >> ../create-chain/scripts/abi/synd/AssertionPoster.ts
	
	@echo "Generating RequireAndModule..."
	@cd ../synd-contracts && \
	  echo "export const requireAndModuleABI = $$(forge inspect src/requirement-modules/RequireAndModule.sol:RequireAndModule abi --json) as const" > ../create-chain/scripts/abi/synd/RequireAndModule.ts
	
	@echo "Generating RequireAndModuleFactory..."
	@cd ../synd-contracts && \
	  echo "export const requireAndModuleFactoryABI = $$(forge inspect src/factory/PermissionModuleFactories.sol:RequireAndModuleFactory abi --json) as const" > ../create-chain/scripts/abi/synd/RequireAndModuleFactory.ts
	
	@echo "Generating SyndicateFactory..."
	@cd ../synd-contracts && \
	  echo "export const syndicateFactoryABI = $$(forge inspect src/factory/SyndicateFactory.sol:SyndicateFactory abi --json) as const" > ../create-chain/scripts/abi/synd/SyndicateFactory.ts
	
	@echo "Generating SyndicateSequencingChain..."
	@cd ../synd-contracts && \
	  echo "export const syndicateSequencingChainABI = $$(forge inspect src/SyndicateSequencingChain.sol:SyndicateSequencingChain abi --json) as const" > ../create-chain/scripts/abi/synd/SyndicateSequencingChain.ts
	
	@echo "Generating TeeKeyManager..."
	@cd ../synd-contracts && \
	  echo "export const teeKeyManagerABI = $$(forge inspect src/withdrawal/TeeKeyManager.sol:TeeKeyManager abi --json) as const" > ../create-chain/scripts/abi/synd/TeeKeyManager.ts
	
	@echo "Generating TeeModule..."
	@cd ../synd-contracts && \
	  echo "export const teeModuleABI = $$(forge inspect src/withdrawal/TeeModule.sol:TeeModule abi --json) as const" > ../create-chain/scripts/abi/synd/TeeModule.ts && \
	  echo "" >> ../create-chain/scripts/abi/synd/TeeModule.ts && \
	  echo "export const teeModuleBytecode =" >> ../create-chain/scripts/abi/synd/TeeModule.ts && \
	  echo "  \"$$(forge inspect src/withdrawal/TeeModule.sol:TeeModule bytecode)\"" >> ../create-chain/scripts/abi/synd/TeeModule.ts
	
	@echo "Done! All contract ABIs and bytecode have been generated."
