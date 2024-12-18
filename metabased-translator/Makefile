create-contract-bindings: create-translator-contract-bindings create-metabased-contracts-bindings

create-translator-contract-bindings:
	cd contracts && forge bind --alloy
	cp -r contracts/out/bindings/src/* src/contract_bindings


create-metabased-contracts-bindings:
	cd ../metabased-contracts && forge bind --alloy
	cp -r ../metabased-contracts/out/bindings/src/* src/contract_bindings

.PHONY: create-contract-bindings
