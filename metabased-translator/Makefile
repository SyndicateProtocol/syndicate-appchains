create-contract-bindings: create-metabased-contract-bindings create-arbitrum-contract-bindings
.PHONY: create-contract-bindings

create-arbitrum-contract-bindings:
	cd contracts && rm -rf out && forge bind --alloy
	cp -r contracts/out/bindings/src/* contract-bindings/arbitrum
	mv contract-bindings/arbitrum/lib.rs contract-bindings/arbitrum/mod.rs
.PHONY: create-arbitrum-contract-bindings


create-metabased-contract-bindings:
	cd ../metabased-contracts && rm -rf out && forge bind --alloy
	cp -r ../metabased-contracts/out/bindings/src/* contract-bindings/metabased
	mv contract-bindings/metabased/lib.rs contract-bindings/metabased/mod.rs
.PHONY: create-metabased-contracts-bindings

print-sample-command:
	cd ../metabased-translator && cargo run --example generate_command
.PHONY: print-sample-command