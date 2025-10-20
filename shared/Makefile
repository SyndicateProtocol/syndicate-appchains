.PHONY: create-contract-bindings-rust
create-contract-bindings-rust:
	git submodule sync --recursive
	git submodule update --init --recursive
	git submodule update --remote ../synd-contracts/lib/nitro-contracts # needs specific branch
	git submodule status --recursive
	rm -rf ../synd-contracts/out contract-bindings/src/synd/*
	cd ../synd-contracts  && forge bind --alloy --module --force --skip test --skip script --skip Mock --skip Helper --skip lib
	cp -r ../synd-contracts/out/bindings/* contract-bindings/src/synd

.PHONY: create-withdrawal-contract-bindings-go
create-withdrawal-contract-bindings-go:
	# NOTE: 99% of the time this task is executed in sequence after `create-contract-bindings-rust`. Let's skip building here to save time.
	# 1) build everything so that Forge knows about TeeModule
	# cd ../synd-contracts && forge build --force --skip test --skip script --skip Mock --skip Helper

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
	# You need to install abigen: `go install github.com/ethereum/go-ethereum/cmd/abigen@latest` and add to your PATH
	abigen \
		--abi=contract-bindings/src/withdrawals-go/TeeModule.abi \
		--bin=contract-bindings/src/withdrawals-go/TeeModule.bin \
		--pkg=teemodule \
		--out=../synd-withdrawals/synd-enclave/teemodule/teeModule.go

	# 5) Clean up
	rm -rf contract-bindings/src/withdrawals-go

.PHONY: create-contract-bindings
create-contract-bindings: create-contract-bindings-rust create-withdrawal-contract-bindings-go
