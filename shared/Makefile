create-contract-bindings: create-synd-contract-bindings create-arbitrum-contract-bindings
.PHONY: create-contract-bindings

create-arbitrum-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf arb-contracts/out contract-bindings/src/arbitrum/*
	cd arb-contracts && forge bind --alloy --force
	cp -r arb-contracts/out/bindings/src/* contract-bindings/src/arbitrum
	mv contract-bindings/src/arbitrum/lib.rs contract-bindings/src/arbitrum/mod.rs
.PHONY: create-arbitrum-contract-bindings


create-synd-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf ../synd-contracts/out contract-bindings/src/synd/*
	cd ../synd-contracts  && forge bind --alloy --force -C ./sequencing-modules
	cp -r ../synd-contracts/out/bindings/src/* contract-bindings/src/synd
	mv contract-bindings/src/synd/lib.rs contract-bindings/src/synd/mod.rs
.PHONY: create-synd-contract-bindings


