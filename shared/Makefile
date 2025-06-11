create-contract-bindings: 
	git submodule update --init --recursive
	git submodule update --remote ../synd-contracts/lib/nitro-contracts # needs specific branch
	git submodule status --recursive
	rm -rf ../synd-contracts/out contract-bindings/src/synd/*
	cd ../synd-contracts  && forge bind --alloy --force
	cp -r ../synd-contracts/out/bindings/src/* contract-bindings/src/synd
	mv contract-bindings/src/synd/lib.rs contract-bindings/src/synd/mod.rs
.PHONY: create-contract-bindings
