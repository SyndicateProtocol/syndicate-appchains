create-contract-bindings: create-translator-contract-bindings create-test-contracts-bindings
.PHONY: create-contract-bindings

create-translator-contract-bindings:
	cd contracts && rm -rf out && forge bind --alloy
	cp -r contracts/out/bindings/src/* crates/block-builder/src/contract_bindings 
	mv crates/block-builder/src/contract_bindings/lib.rs crates/block-builder/src/contract_bindings/mod.rs
.PHONY: create-translator-contract-bindings


create-test-contracts-bindings:
	cd ../metabased-contracts && rm -rf out && forge bind --alloy
	cp -r ../metabased-contracts/out/bindings/src/* tests/contract_bindings
	mv tests/contract_bindings/lib.rs tests/contract_bindings/mod.rs
.PHONY: create-test-contracts-bindings

