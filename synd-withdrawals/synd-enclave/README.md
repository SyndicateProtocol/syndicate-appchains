Local development instructions
1. Checkout submodules with `git submodule update --init --recursive`
2. go install github.com/ethereum/go-ethereum/cmd/abigen@latest
- Make sure it is in your PATH via `which abigen || echo "$(go env GOPATH)/bin/abigen"`, and if it isn't you can temporarily add with `export PATH="$(go env GOPATH)/bin:$PATH"`
3. from /shared, run `make create-withdrawal-contract-bindings-go`.
- You may need to run `make build-node-deps` as well.
- If you encounter yarn errors, you may need to upgrade yarn to v4.
4. from /synd-withdrawals/synd-enclave/nitro run `make contracts`. Must use a supported version of npm, like v18. Run `nvm use` to select the proper node version.
- You may need to modify nitro/Makefile to remove CGO_LDFLAGS linker flag values - -Wl,-no_warn_duplicate_libraries that are unsupported on Mac
5. from /synd-withdrawals/synd-enclave, run `go mod tidy`
5. from /synd-withdrawals/synd-enclave, run `go build -C cmd/enclave` to build the executable
- run `export MACOSX_DEPLOYMENT_TARGET=$(sw_vers -productVersion)` and `export CGO_LDFLAGS=-Wl,-no_warn_duplicate_libraries` to remove ld warnings on mac
Release build
1. Checkout submodules with `git submodule update --init --recursive`
2. To build the eif.bin file, run `docker build --platform linux/x86_64 --target eif-bin -o . .`
3. To build the local docker image for testing, run `docker build --target local-dev .`
