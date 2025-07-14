## Global build arguments
#ARG BUILD_PROFILE=release
#ARG FEATURES="rocksdb"
#
## Stage 1: Base image with Rust
#FROM rust:slim-bookworm AS builder
#ARG BUILD_PROFILE
#ARG FEATURES
#WORKDIR /app
#
## Install build dependencies
#RUN --mount=type=cache,target=/var/cache/apt \
#    --mount=type=cache,target=/var/lib/apt \
#    apt-get update && \
#    apt-get install -y libclang-dev pkg-config build-essential libssl-dev curl git && \
#    rm -rf /var/lib/apt/lists/*
#
## Stage 2: Build all Rust components including SP1
#FROM builder AS build
#COPY . .
#
## Install SP1 toolchain using official installer
#RUN curl -L https://sp1up.succinct.xyz | bash
#ENV PATH="/root/.sp1/bin:${PATH}"
#RUN sp1up
#
## Verify SP1 installation
#RUN cargo prove --version && \
#    rustup toolchain list | grep succinct
#
## Build SP1 ELF program
#RUN cd synd-withdrawals/synd-tee-attestation-zk-proofs/sp1/program && \
#    cargo prove build && \
#    cd /app
#
## Build other Rust components if needed
#RUN --mount=type=cache,target=/usr/local/cargo,from=rust:slim-bookworm,source=/usr/local/cargo \
#    cargo build --profile ${BUILD_PROFILE} --features "${FEATURES}" --locked

# --- Go build stage for synd-proposer ---
FROM ghcr.io/syndicateprotocol/syndicate-appchains/node-builder AS nitro

FROM golang:1.23.0 AS synd-proposer-build
WORKDIR /

# Copy dependencies
COPY --from=nitro /workspace ./synd-enclave/nitro
COPY ./synd-withdrawals/synd-enclave/enclave ./synd-enclave/enclave
COPY ./synd-withdrawals/synd-enclave/go.mod ./synd-enclave/go.mod
COPY ./synd-withdrawals/synd-enclave/go.sum ./synd-enclave/go.sum
COPY ./synd-withdrawals/synd-proposer ./synd-proposer

# Build the Go binary
WORKDIR /synd-proposer
RUN CGO_ENABLED=1 go build -o /go/bin/synd-proposer ./cmd/synd-proposer/main.go

# Run tests
FROM synd-proposer-build AS synd-proposer-test
WORKDIR /synd-proposer
RUN go test ./...

# --- Runtime base ---
FROM gcr.io/distroless/cc AS runtime-base

# --- Runtime image: synd-proposer ---
FROM runtime-base AS synd-proposer
# Copy Go binary
COPY --from=synd-proposer-build /go/bin/synd-proposer /usr/local/bin/synd-proposer
# Copy SP1 ELF program from Rust build
COPY --from=build /app/synd-withdrawals/synd-tee-attestation-zk-proofs/sp1/program/elf/riscv32im-succinct-zkvm-elf /sp1-elf/
ENV SP1_ELF_PATH=/sp1-elf/riscv32im-succinct-zkvm-elf
ENTRYPOINT ["/usr/local/bin/synd-proposer"]
LABEL service=synd-proposer

# --- Runtime image: synd-tee-attestation (if you have a Rust binary) ---
FROM runtime-base AS synd-tee-attestation
ARG BUILD_PROFILE
# Copy the Rust binary if you built one
COPY --from=build /app/target/${BUILD_PROFILE}/synd-tee-attestation /usr/local/bin/synd-tee-attestation
# Copy SP1 ELF program
COPY --from=build /app/synd-withdrawals/synd-tee-attestation-zk-proofs/sp1/program/elf/riscv32im-succinct-zkvm-elf /sp1-elf/
ENV SP1_ELF_PATH=/sp1-elf/riscv32im-succinct-zkvm-elf
ENTRYPOINT ["/usr/local/bin/synd-tee-attestation"]
LABEL service=synd-tee-attestation