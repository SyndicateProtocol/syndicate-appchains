.PHONY: create-contract-bindings-rust
create-contract-bindings-rust:
	git submodule sync --recursive
	git submodule update --init --recursive
	git submodule update --remote ../synd-contracts/lib/nitro-contracts # needs specific branch
	git submodule status --recursive
	rm -rf ../synd-contracts/out contract-bindings/src/synd/*
	cd ../synd-contracts  && forge bind --alloy --force
	cp -r ../synd-contracts/out/bindings/src/* contract-bindings/src/synd
	mv contract-bindings/src/synd/lib.rs contract-bindings/src/synd/mod.rs
  
	# remove this specific contract binding until this issue is resolved: https://github.com/foundry-rs/foundry/issues/10911
	rm contract-bindings/src/synd/syndicate_token_emission_scheduler.rs
	grep -v 'pub mod r#syndicate_token_emission_scheduler;' contract-bindings/src/synd/mod.rs > contract-bindings/src/synd/mod.rs.tmp && mv contract-bindings/src/synd/mod.rs.tmp contract-bindings/src/synd/mod.rs

.PHONY: create-withdrawal-contract-bindings-go
create-withdrawal-contract-bindings-go:
	# 1) build everything so that Forge knows about TeeModule
	cd ../synd-contracts && forge build --force

	# 2) ensure our build-out dir exists in the appchains repo
	mkdir -p ../shared/contract-bindings/src/withdrawals-go

	# 3) dump the ABI + BIN directly from Forge
	cd ../synd-contracts && \
	  forge inspect src/withdrawal/TeeModule.sol:TeeModule \
	    --format-json \
  		abi \
	    > ../shared/contract-bindings/src/withdrawals-go/TeeModule.abi && \
	  forge inspect src/withdrawal/TeeModule.sol:TeeModule bytecode \
	    > ../shared/contract-bindings/src/withdrawals-go/TeeModule.bin

	# 4) generate Go bindings from those artifacts
	# You need to install abigen: `go install github.com/ethereum/go-ethereum/cmd/abigen@latest`
	abigen \
		--abi=contract-bindings/src/withdrawals-go/TeeModule.abi \
		--bin=contract-bindings/src/withdrawals-go/TeeModule.bin \
		--pkg=teemodule \
		--out=../synd-withdrawals/synd-enclave/teemodule/teeModule.go

	# 5) Clean up
	rm -rf contract-bindings/src/withdrawals-go

.PHONY: create-contract-bindings
create-contract-bindings: create-contract-bindings-rust create-withdrawal-contract-bindings-go