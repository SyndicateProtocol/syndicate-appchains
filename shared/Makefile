create-contract-bindings: create-syndicate-contract-bindings create-arbitrum-contract-bindings
.PHONY: create-contract-bindings

create-arbitrum-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf arb-contracts/out contract-bindings/src/arbitrum/*
	cd arb-contracts && forge bind --alloy --force
	cp -r arb-contracts/out/bindings/src/* contract-bindings/src/arbitrum
	mv contract-bindings/src/arbitrum/lib.rs contract-bindings/src/arbitrum/mod.rs
.PHONY: create-arbitrum-contract-bindings


create-syndicate-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf ../syndicate-contracts/out contract-bindings/src/syndicate/*
	cd ../syndicate-contracts  && forge bind --alloy --force -C ./sequencing-modules
	cp -r ../syndicate-contracts/out/bindings/src/* contract-bindings/src/syndicate
	mv contract-bindings/src/syndicate/lib.rs contract-bindings/src/syndicate/mod.rs
.PHONY: create-syndicate-contracts-bindings


