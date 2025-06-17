# syntax=docker/dockerfile:1.7-labs

FROM debian:bookworm-slim AS brotli-wasm-builder
WORKDIR /workspace
RUN apt-get update && \
    apt-get install -y cmake make git lbzip2 python3 xz-utils && \
    git clone https://github.com/emscripten-core/emsdk.git && \
    cd emsdk && \
    ./emsdk install 3.1.7 && \
    ./emsdk activate 3.1.7
COPY nitro/scripts/build-brotli.sh scripts/
COPY nitro/brotli brotli
RUN cd emsdk && . ./emsdk_env.sh && cd .. && ./scripts/build-brotli.sh -w -t /workspace/install/

FROM scratch AS brotli-wasm-export
COPY --from=brotli-wasm-builder /workspace/install/ /

FROM debian:bookworm-slim AS brotli-library-builder
WORKDIR /workspace
COPY nitro/scripts/build-brotli.sh scripts/
COPY nitro/brotli brotli
RUN apt-get update && \
    apt-get install -y cmake make gcc git && \
    ./scripts/build-brotli.sh -l -t /workspace/install/

FROM scratch AS brotli-library-export
COPY --from=brotli-library-builder /workspace/install/ /

FROM node:18-bookworm-slim AS contracts-builder
RUN apt-get update && \
    apt-get install -y git python3 make g++ curl
RUN curl -L https://foundry.paradigm.xyz | bash && . ~/.bashrc && ~/.foundry/bin/foundryup -i 1.0.0
WORKDIR /workspace
COPY nitro/contracts-legacy/package.json nitro/contracts-legacy/yarn.lock contracts-legacy/
RUN cd contracts-legacy && yarn install
COPY nitro/contracts/package.json nitro/contracts/yarn.lock contracts/
RUN cd contracts && yarn install
COPY nitro/contracts-legacy contracts-legacy/
COPY nitro/contracts contracts/
COPY nitro/safe-smart-account safe-smart-account/
RUN cd safe-smart-account && yarn install
COPY nitro/Makefile .
RUN . ~/.bashrc && NITRO_BUILD_IGNORE_TIMESTAMPS=1 make build-solidity

FROM rust:1.87-slim-bookworm AS prover-header-builder
WORKDIR /workspace
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install -y make clang wabt && \
    cargo install --force cbindgen
COPY nitro/arbitrator/Cargo.* arbitrator/
COPY nitro/Makefile ./
COPY nitro/arbitrator/arbutil arbitrator/arbutil
COPY nitro/arbitrator/bench arbitrator/bench
COPY nitro/arbitrator/brotli arbitrator/brotli
COPY nitro/arbitrator/caller-env arbitrator/caller-env
COPY nitro/arbitrator/prover arbitrator/prover
COPY nitro/arbitrator/wasm-libraries arbitrator/wasm-libraries
COPY nitro/arbitrator/jit arbitrator/jit
COPY nitro/arbitrator/stylus arbitrator/stylus
COPY nitro/arbitrator/tools/wasmer arbitrator/tools/wasmer
COPY --from=brotli-wasm-export / target/
COPY nitro/scripts/build-brotli.sh scripts/
COPY nitro/brotli brotli
RUN apt-get update && apt-get install -y cmake
RUN NITRO_BUILD_IGNORE_TIMESTAMPS=1 make build-prover-header

FROM scratch AS prover-header-export
COPY --from=prover-header-builder /workspace/target/ /

FROM rust:1.87-slim-bookworm AS prover-builder
WORKDIR /workspace
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install -y make wget gpg software-properties-common zlib1g-dev libstdc++-12-dev wabt
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - && \
    add-apt-repository 'deb http://apt.llvm.org/bookworm/ llvm-toolchain-bookworm-15 main' && \
    apt-get update && \
    apt-get install -y llvm-15-dev libclang-common-15-dev
COPY --from=brotli-library-export / target/
COPY nitro/arbitrator/Cargo.* arbitrator/
COPY nitro/arbitrator/arbutil arbitrator/arbutil
COPY nitro/arbitrator/bench arbitrator/bench
COPY nitro/arbitrator/brotli arbitrator/brotli
COPY nitro/arbitrator/caller-env arbitrator/caller-env
COPY nitro/arbitrator/prover/Cargo.toml arbitrator/prover/
COPY nitro/arbitrator/prover/benches arbitrator/prover/benches
COPY nitro/arbitrator/bench/Cargo.toml arbitrator/bench/
COPY nitro/arbitrator/jit/Cargo.toml arbitrator/jit/
COPY nitro/arbitrator/stylus/Cargo.toml arbitrator/stylus/
COPY nitro/arbitrator/tools/wasmer arbitrator/tools/wasmer
COPY nitro/arbitrator/wasm-libraries/user-host-trait/Cargo.toml arbitrator/wasm-libraries/user-host-trait/Cargo.toml
RUN bash -c 'mkdir arbitrator/{prover,jit,stylus}/src arbitrator/wasm-libraries/user-host-trait/src'
RUN echo "fn test() {}" > arbitrator/jit/src/lib.rs && \
    echo "fn test() {}" > arbitrator/prover/src/lib.rs && \
    echo "fn test() {}" > arbitrator/bench/src/lib.rs && \
    echo "fn test() {}" > arbitrator/prover/benches/merkle_bench.rs && \
    echo "fn test() {}" > arbitrator/stylus/src/lib.rs && \
    echo "fn test() {}" > arbitrator/wasm-libraries/user-host-trait/src/lib.rs && \
    cargo build --manifest-path arbitrator/Cargo.toml --release --lib && \
    rm arbitrator/prover/src/lib.rs arbitrator/jit/src/lib.rs arbitrator/stylus/src/lib.rs && \
    rm arbitrator/wasm-libraries/user-host-trait/src/lib.rs && \
    rm arbitrator/prover/benches/merkle_bench.rs && \
    rm arbitrator/bench/src/lib.rs
COPY nitro/Makefile ./
COPY nitro/arbitrator/prover arbitrator/prover
COPY nitro/arbitrator/wasm-libraries arbitrator/wasm-libraries
COPY nitro/arbitrator/jit arbitrator/jit
COPY nitro/arbitrator/stylus arbitrator/stylus
COPY --from=brotli-wasm-export / target/
COPY nitro/scripts/build-brotli.sh scripts/
COPY nitro/brotli brotli
RUN touch -a -m arbitrator/prover/src/lib.rs
RUN NITRO_BUILD_IGNORE_TIMESTAMPS=1 make build-prover-lib
RUN NITRO_BUILD_IGNORE_TIMESTAMPS=1 make build-prover-bin
RUN NITRO_BUILD_IGNORE_TIMESTAMPS=1 make build-jit

FROM scratch AS prover-export
COPY --from=prover-builder /workspace/target/ /

# nixos/nix:2.21.4
FROM nixos/nix@sha256:91b689f94a101aa67f95034dffd9a4858e85d0946f67c64dd65ed241644454b9 AS bootstrap

RUN mkdir /build
WORKDIR /build

ENV REPO=https://github.com/aws/aws-nitro-enclaves-sdk-bootstrap.git
ENV COMMIT=7614f19963e4e956493b3260fda4d62834bb281c
RUN git init && \
    git remote add origin $REPO && \
    git fetch --depth=1 origin $COMMIT && \
    git reset --hard FETCH_HEAD

RUN mkdir out
RUN nix-build --option filter-syscalls false -A kernel && cp -r result/* out/
RUN nix-build --option filter-syscalls false -A init && cp -r result/* out/

FROM golang:1.23.1-bookworm AS builder
RUN go install github.com/linuxkit/linuxkit/src/cmd/linuxkit@270fd1c5aa1986977b31af6c743c6a2681f67a29
WORKDIR /build
ARG version=""
ARG datetime=""
ARG modified=""
ENV NITRO_VERSION=$version
ENV NITRO_DATETIME=$datetime
ENV NITRO_MODIFIED=$modified
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install -y wabt
COPY --parents **/go.mod **/go.sum .
RUN go mod download
COPY cmd/enclave/main.go cmd/enclave/main.go
COPY enclave enclave
COPY nitro nitro
WORKDIR /build/nitro
COPY --from=contracts-builder workspace/contracts/build/ contracts/build/
COPY --from=contracts-builder workspace/contracts/out/ contracts/out/
COPY --from=contracts-builder workspace/contracts/node_modules/@offchainlabs/upgrade-executor/build/contracts/src/UpgradeExecutor.sol/UpgradeExecutor.json contracts/node_modules/@offchainlabs/upgrade-executor/build/contracts/src/UpgradeExecutor.sol/
COPY --from=contracts-builder workspace/contracts-legacy/build/ contracts-legacy/build/
COPY --from=contracts-builder workspace/contracts-legacy/out/ contracts-legacy/out/
COPY --from=contracts-builder workspace/safe-smart-account/build/ safe-smart-account/build/
COPY --from=contracts-builder workspace/.make/ .make/
COPY --from=prover-header-export / target/
COPY --from=brotli-library-export / target/
COPY --from=prover-export / target/
RUN mkdir -p target/bin
COPY nitro/.nitro-tag.txt /nitro-tag.txt
RUN NITRO_BUILD_IGNORE_TIMESTAMPS=1 make build-node-deps

WORKDIR /build
RUN go build -ldflags "-linkmode external -extldflags -static" -o bin/enclave ./cmd/enclave

COPY eif/init-ramdisk.yaml eif/user-ramdisk.yaml eif/
COPY --from=bootstrap /build/out bootstrap
RUN linuxkit build --format kernel+initrd --no-sbom --name init-ramdisk ./eif/init-ramdisk.yaml
RUN linuxkit build --format kernel+initrd --no-sbom --name user-ramdisk ./eif/user-ramdisk.yaml

# rust:1.80.1
FROM rust@sha256:29fe4376919e25b7587a1063d7b521d9db735fc137d3cf30ae41eb326d209471 AS eif

RUN mkdir /build
WORKDIR /build

ENV REPO=https://github.com/aws/aws-nitro-enclaves-image-format.git
ENV COMMIT=483114f1da3bad913ad1fb7d5c00dadacc6cbae6
RUN git init && \
    git remote add origin $REPO && \
    git fetch --depth=1 origin $COMMIT && \
    git reset --hard FETCH_HEAD

RUN cargo build --all --release

ARG TARGETARCH
COPY "eif/cmdline-${TARGETARCH}" cmdline
COPY --from=bootstrap /build/out bootstrap
COPY --from=builder /build/init-ramdisk-initrd.img .
COPY --from=builder /build/user-ramdisk-initrd.img .

RUN ./target/release/eif_build \
    --kernel bootstrap/*Image \
    --kernel_config bootstrap/*Image.config \
    --cmdline "$(cat cmdline)" \
    --ramdisk init-ramdisk-initrd.img \
    --ramdisk user-ramdisk-initrd.img \
    --output eif.bin

FROM scratch
COPY --from=eif /build/eif.bin .
