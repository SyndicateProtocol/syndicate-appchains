create-contract-bindings: create-metabased-contract-bindings create-arbitrum-contract-bindings
.PHONY: create-contract-bindings

create-arbitrum-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf contracts/out contract-bindings/src/arbitrum/*
	cd contracts && forge bind --alloy --force
	cp -r contracts/out/bindings/src/* contract-bindings/src/arbitrum
	mv contract-bindings/src/arbitrum/lib.rs contract-bindings/src/arbitrum/mod-1.rs
	cd contracts/nitro-contracts && yarn install && yarn build && forge bind --alloy --force
	cp -r contracts/nitro-contracts/out/bindings/src/* contract-bindings/src/arbitrum
	mv contract-bindings/src/arbitrum/lib.rs contract-bindings/src/arbitrum/mod-2.rs
	cat contract-bindings/src/arbitrum/mod-2.rs contract-bindings/src/arbitrum/mod-1.rs | sort | uniq > contract-bindings/src/arbitrum/mod.rs 
	rm contract-bindings/src/arbitrum/mod-1.rs
	rm contract-bindings/src/arbitrum/mod-2.rs
.PHONY: create-arbitrum-contract-bindings


create-metabased-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf ../metabased-contracts/out contract-bindings/src/metabased/*
	cd ../metabased-contracts  && forge bind --alloy --force -C ./sequencing-modules
	cp -r ../metabased-contracts/out/bindings/src/* contract-bindings/src/metabased
	mv contract-bindings/src/metabased/lib.rs contract-bindings/src/metabased/mod.rs
.PHONY: create-metabased-contracts-bindings