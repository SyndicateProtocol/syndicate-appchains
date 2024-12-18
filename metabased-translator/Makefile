create-contract-bindings:
	cd contracts && forge bind --alloy
	cp -r contracts/out/bindings/src/* src/contract-bindings

create-metabased-contracts-bindings:
	cd ../metabased-contracts && forge bind --alloy
	cp -r ../metabased-contracts/out/bindings/src/* src/contract-bindings

.PHONY: create-contract-bindings
