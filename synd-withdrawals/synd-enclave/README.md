Local development instructions
1. Checkout submodules with `git submodule --init --recursive`
2. go install github.com/ethereum/go-ethereum/cmd/abigen@latest
- Make sure it is in your PATH via `which abigen || echo "$(go env GOPATH)/bin/abigen"`, and if it isn't you can temporarily add with `export PATH="$(go env GOPATH)/bin:$PATH"`
3. from /shared, run `make create-withdrawal-contract-bindings-go`
4. from /synd-withdrawals/synd-enclave/nitro run `make contracts`. Must use a supported version of npm, like v18
- You may need to modify nitro/Makefile to remove CGO_LDFLAGS linker flag values - -Wl,-no_warn_duplicate_libraries that are unsupported on Mac
5. from /synd-withdrawals/synd-enclave, run `go mod tidy`
6. from /synd-withdrawals/synd-enclave, run `go build -C cmd/enclave` to build the executable

Release build
1. To build the eif.bin file, run `docker build --platform linux/x86_64 --target eif-bin -o . .`
2. To build the local docker image for testing, run `docker build --target local-dev .`
