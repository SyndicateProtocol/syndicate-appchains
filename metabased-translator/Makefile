create-contract-bindings: create-metabased-contract-bindings create-arbitrum-contract-bindings
.PHONY: create-contract-bindings

create-arbitrum-contract-bindings:
	cd contracts && rm -rf out && forge bind --alloy
	cp -r contracts/out/bindings/src/* contract-bindings/src/arbitrum
	mv contract-bindings/src/arbitrum/lib.rs contract-bindings/src/arbitrum/mod.rs
.PHONY: create-arbitrum-contract-bindings


create-metabased-contract-bindings:
	cd ../metabased-contracts && rm -rf out && forge bind --alloy
	cp -r ../metabased-contracts/out/bindings/src/* contract-bindings/src/metabased
	mv contract-bindings/src/metabased/lib.rs contract-bindings/src/metabased/mod.rs
.PHONY: create-metabased-contracts-bindings
