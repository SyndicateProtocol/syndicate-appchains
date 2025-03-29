create-contract-bindings: create-metabased-contract-bindings create-arbitrum-contract-bindings
.PHONY: create-contract-bindings

create-arbitrum-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf ../rollup-contracts/out contract-bindings/src/arbitrum/*
	cd ../rollup-contracts && forge bind --alloy --force
	cp -r ../rollup-contracts/out/bindings/src/* contract-bindings/src/arbitrum
	mv contract-bindings/src/arbitrum/lib.rs contract-bindings/src/arbitrum/mod.rs
.PHONY: create-arbitrum-contract-bindings


create-metabased-contract-bindings:
	git submodule update --init --recursive
	git submodule status --recursive
	rm -rf ../metabased-contracts/out contract-bindings/src/metabased/*
	cd ../metabased-contracts  && forge bind --alloy --force -C ./sequencing-modules --libraries src/use-cases/dream/RLP/RLPTxDecoder.sol:RLPTxDecoder:0x1234567890123456789012345678901234567890
	cp -r ../metabased-contracts/out/bindings/src/* contract-bindings/src/metabased
	mv contract-bindings/src/metabased/lib.rs contract-bindings/src/metabased/mod.rs
.PHONY: create-metabased-contracts-bindings


