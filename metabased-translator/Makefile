create-contract-bindings: create-translator-contract-bindings create-metabased-contracts-bindings
.PHONY: create-contract-bindings

create-translator-contract-bindings:
	cd contracts && forge bind --alloy
	cp -r contracts/out/bindings/src/* src/contract_bindings/translator_contracts
	mv src/contract_bindings/translator_contracts/lib.rs src/contract_bindings/translator_contracts/mod.rs
.PHONY: create-translator-contract-bindings


create-metabased-contracts-bindings:
	cd ../metabased-contracts && forge bind --alloy
	cp -r ../metabased-contracts/out/bindings/src/* src/contract_bindings/metabased_contracts
	mv src/contract_bindings/metabased_contracts/lib.rs src/contract_bindings/metabased_contracts/mod.rs
.PHONY: create-metabased-contracts-bindings

