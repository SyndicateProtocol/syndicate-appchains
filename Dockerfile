# Global build arguments
ARG BUILD_PROFILE=release
ARG FEATURES="rocksdb" 

# Stage 1: Base image with Rust
FROM rust:slim-bookworm AS builder
ARG BUILD_PROFILE
ARG FEATURES
WORKDIR /app

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apt \
    --mount=type=cache,target=/var/lib/apt \
    apt-get update && \
    apt-get install -y libclang-dev pkg-config build-essential libssl-dev curl git && \
    rm -rf /var/lib/apt/lists/*

# Stage 2: Build
FROM builder AS build
COPY . .

# Install SP1 toolchain using official installer
RUN curl -L https://sp1up.succinct.xyz | bash
ENV PATH="/root/.sp1/bin:${PATH}"
RUN sp1up

# Verify SP1 installation (optional)
RUN cargo prove --version && \
    rustup toolchain list | grep succinct

# Build SP1 ELF program
RUN cd synd-withdrawals/synd-tee-attestation-zk-proofs/sp1/program && \
    cargo prove build && \
    cd /app

# Perform cargo build with cached Cargo and target directories
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:slim-bookworm,source=/usr/local/cargo \
    cargo build --profile ${BUILD_PROFILE} --features "${FEATURES}" --locked

# --- Go build stage for synd-proposer (fixed brotli-sys) ---
FROM golang:1.23-bookworm AS go-synd-proposer-build

# 1) Install system deps: build tools, Node 18, pkg-config, brotli-dev (just in case)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential cmake lld-14 libudev-dev wabt clang \
        curl git ca-certificates gnupg2 \
        pkg-config libbrotli-dev nodejs \
        npm \
    && rm -rf /var/lib/apt/lists/* \
    && curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && npm install -g yarn

# 2) Force brotli-sys to build its vendored C code
ENV BROTLI_SYS_BUNDLED=1

# 3) In case any other crate still tries pkg-config, make sure it can find brotli.pc
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# 4) Install Rust toolchains, cbindgen & Foundry
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:${PATH}
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain none && \
    rustup toolchain install nightly-2025-02-14 \
        --component rust-src,rustfmt,clippy \
        --target wasm32-wasip1 --target wasm32-unknown-unknown && \
    rustup toolchain install 1.84.1 \
        --component cargo,rustc,clippy \
        --target wasm32-wasip1 --target wasm32-unknown-unknown && \
    rustup default 1.84.1 && \
    cargo install --force cbindgen && \
    curl -L https://foundry.paradigm.xyz | bash && \
    ~/.foundry/bin/foundryup

# 5) Copy in your Go & Nitro code and build the Nitro deps
WORKDIR /go/src
COPY ./synd-withdrawals/synd-enclave ./synd-enclave
COPY ./synd-withdrawals/synd-proposer ./synd-proposer

WORKDIR /go/src/synd-enclave/nitro
# force Nitro to rebuild the vendored brotli C library
RUN ./scripts/build-brotli.sh -l
RUN mkdir -p target/machines/latest \
 && cargo run --manifest-path arbitrator/wasm-libraries/forward/Cargo.toml \
      -- --path arbitrator/wasm-libraries/forward/forward.wat \
 && wat2wasm arbitrator/wasm-libraries/forward/forward.wat \
      -o target/machines/latest/forward.wasm \
 && cargo run --manifest-path arbitrator/wasm-libraries/forward/Cargo.toml \
      -- --path arbitrator/wasm-libraries/forward/forward_stub.wat --stub \
 && wat2wasm arbitrator/wasm-libraries/forward/forward_stub.wat \
      -o target/machines/latest/forward_stub.wasm

# fake out the wasm-build step so no Docker call is made
RUN mkdir -p target/lib-wasm \
 && touch target/lib-wasm/libbrotlicommon-static.a \
 && touch target/lib-wasm/libbrotlienc-static.a \
 && touch target/lib-wasm/libbrotlidec-static.a

RUN make build-node-deps


# 6) Build the Go CLI
WORKDIR /go/src/synd-proposer
RUN go mod download && go mod tidy
RUN CGO_ENABLED=1 go build -o /go/bin/synd-proposer ./cmd/synd-proposer/main.go
    

# Stage 3: Optional Foundry install
FROM debian:bookworm-slim AS foundry
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    git \
    ca-certificates && \
    curl -L https://foundry.paradigm.xyz | bash && \
    ~/.foundry/bin/foundryup && \
    rm -rf /var/lib/apt/lists/*

# Stage 3: Runtime images

# SECURITY WARNING: All runtime containers currently run as root user (UID 0)
# This poses security risks as compromised containers would have root privileges.
# TODO [SEQ-1022]: Add non-root user configuration for better security posture:
# Create a non-root user (or USER 1000:1000) and ensure each application has proper file permissions for non-root execution

FROM gcr.io/distroless/cc AS runtime-base

# Runtime images
FROM runtime-base AS synd-translator
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/synd-translator /usr/local/bin/synd-translator
COPY --from=foundry /root/.foundry /root/.foundry
ENV PATH="/root/.foundry/bin:${PATH}"
ENTRYPOINT ["/usr/local/bin/synd-translator"]
EXPOSE 8545 8546
LABEL service=synd-translator

FROM runtime-base AS synd-proposer
COPY --from=go-synd-proposer-build /go/bin/synd-proposer /usr/local/bin/synd-proposer
ENTRYPOINT ["/usr/local/bin/synd-proposer"]
LABEL service=synd-proposer

FROM runtime-base AS synd-maestro
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/synd-maestro /usr/local/bin/synd-maestro
ENTRYPOINT ["/usr/local/bin/synd-maestro"]
EXPOSE 8545 8546
LABEL service=synd-maestro

FROM runtime-base AS synd-batch-sequencer
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/synd-batch-sequencer /usr/local/bin/synd-batch-sequencer
ENTRYPOINT ["/usr/local/bin/synd-batch-sequencer"]
EXPOSE 8545 8546
LABEL service=synd-batch-sequencer

FROM runtime-base AS synd-mchain
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/synd-mchain /usr/local/bin/synd-mchain
ENTRYPOINT ["/usr/local/bin/synd-mchain"]
EXPOSE 8545 8546
LABEL service=synd-mchain

FROM runtime-base AS synd-chain-ingestor
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/synd-chain-ingestor /usr/local/bin/synd-chain-ingestor
ENTRYPOINT ["/usr/local/bin/synd-chain-ingestor"]
EXPOSE 8545 8546
LABEL service=synd-chain-ingestor

# --------- Debugging image for translator ---------
FROM ubuntu:22.04 AS synd-translator-debug
ARG BUILD_PROFILE
RUN apt-get update && apt-get install -y heaptrack libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/${BUILD_PROFILE}/synd-translator /usr/local/bin/synd-translator
COPY --from=foundry /root/.foundry /root/.foundry
ENV PATH="/root/.foundry/bin:${PATH}"
ENTRYPOINT ["/usr/local/bin/synd-translator"]
EXPOSE 8545 8546
LABEL service=synd-translator
