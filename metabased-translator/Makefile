create-contract-bindings: create-metabased-contract-bindings create-arbitrum-contract-bindings create-nitro-contract-bindings
.PHONY: create-contract-bindings

create-arbitrum-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf contracts/out contract-bindings/src/arbitrum/*
	cd contracts && forge bind --alloy --force
	cp -r contracts/out/bindings/src/* contract-bindings/src/arbitrum
	mv contract-bindings/src/arbitrum/lib.rs contract-bindings/src/arbitrum/mod.rs
.PHONY: create-arbitrum-contract-bindings


create-nitro-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf contracts/out contract-bindings/src/nitro/*
	cd contracts/nitro-contracts && yarn install && yarn build && forge bind --alloy --force
	cp -r contracts/nitro-contracts/out/bindings/src/* contract-bindings/src/nitro
	mv contract-bindings/src/nitro/lib.rs contract-bindings/src/nitro/mod.rs
.PHONY: create-nitro-contract-bindings


create-metabased-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf ../metabased-contracts/out contract-bindings/src/metabased/*
	cd ../metabased-contracts  && forge bind --alloy --force -C ./sequencing-modules
	cp -r ../metabased-contracts/out/bindings/src/* contract-bindings/src/metabased
	mv contract-bindings/src/metabased/lib.rs contract-bindings/src/metabased/mod.rs
.PHONY: create-metabased-contracts-bindings