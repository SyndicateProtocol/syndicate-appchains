# Building synd-enclave

## Local development instructions

1. Checkout submodules with `git submodule update --init --recursive`
2. go install github.com/ethereum/go-ethereum/cmd/abigen@latest
- Make sure it is in your PATH via `which abigen || echo "$(go env GOPATH)/bin/abigen"`, and if it isn't you can temporarily add with `export PATH="$(go env GOPATH)/bin:$PATH"`
3. from /shared, run `make create-withdrawal-contract-bindings-go`.
- You may need to run `make build-node-deps` as well.
- If you encounter yarn errors, you may need to upgrade yarn to v4.
4. from /synd-withdrawals/synd-enclave/nitro run `make contracts`. Must use a supported version of npm, like v18. Run `nvm use` to select the proper node version.
- You may need to modify nitro/Makefile to remove CGO_LDFLAGS linker flag values - -Wl,-no_warn_duplicate_libraries that are unsupported on Mac
5. from /synd-withdrawals/synd-enclave, run `go mod tidy`
6. from /synd-withdrawals/synd-enclave, run `go build -C cmd/enclave` to build the executable
- run `export MACOSX_DEPLOYMENT_TARGET=$(sw_vers -productVersion)` and `export CGO_LDFLAGS=-Wl,-no_warn_duplicate_libraries` to remove ld warnings on mac

## Release build

1. Checkout submodules with `git submodule update --init --recursive`
2. To build the eif.bin file, run `docker build --platform linux/x86_64 --target eif-bin -o . .`
3. To build the local docker image for testing, run `docker build --target local-dev .`

## Reproducible builds with Nix

1. Install Nix using https://github.com/DeterminateSystems/nix-installer:

```sh
curl -fsSL https://install.determinate.systems/nix | sh -s -- install
```

2. Change to the `synd-withdrawals/synd-enclave` directory (containing `flake.nix` file).

3. Because Nix might take a long time to fetch all the dependencies (like nitro and its submodules),
I recommend running `nix flake archive` first, so you know you're not stuck on a build, but rather fetching dependencies.

4. Run the following command to build the `eif.bin` file:

```sh
nix build .#eif-bin --print-out-paths
```

In the end, you should get a Nix path for `eif.bin` with a hash in the name, e.g.
`/nix/store/b0v83kjd7r5993gl4mk96zcrlaam7swq-eif.bin`.

The path should be the same for everyone building on all machines with the same architecture.
E.g., when using a fixed commit of this repo, the resulting path should be the same for `x86_64-linux` or `aarch64-linux` respectively.
- In the event of differing hashes between machines, start by running the build again with `--print-build-logs`. `eif.bin` build should report the hashes of the resulting file in the form of `PCR0`/`PCR1`/`PCR2` hashes. Compare these hashes between machines.
  - If only the Nix store path differs, but the PCR hashes are the same, then the resulting file is identical, but Nix noticed a change of dependency code used to build it. Consider using the same technique to see whether the dependencies used to build `eif.bin` are reproducible (especially ones based on the code from this repo, like `synd-enclave-server` and `enclave-src-with-generated`).
  - If the PCR hashes differ, then there is an issue with the reproducibility of the build.
