# Stage 1: Base image with Rust & cargo-chef
FROM lukemathwalker/cargo-chef:latest-rust-1.84 AS chef
WORKDIR /app

RUN apt-get update && \
    apt-get install -y libclang-dev pkg-config build-essential && \
    rm -rf /var/lib/apt/lists/*

# Stage 2: Prepare cargo-chef recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies and application
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

ARG BUILD_PROFILE=release
ENV BUILD_PROFILE=${BUILD_PROFILE}
ARG FEATURES=""
ENV FEATURES=${FEATURES}

RUN cargo chef cook --profile ${BUILD_PROFILE} --features "${FEATURES}" --recipe-path recipe.json

COPY . .

RUN cargo build --profile ${BUILD_PROFILE} --features "${FEATURES}" --locked --bin metabased-translator
RUN cargo build --profile ${BUILD_PROFILE} --features "${FEATURES}" --locked --package metabased-sequencer
RUN cargo build --profile ${BUILD_PROFILE} --features "${FEATURES}" --locked --package metabased-poster

# Stage 4: Optional Foundry install (in separate stage to avoid bloating runtime image)
FROM debian:bullseye-slim AS foundry
RUN apt-get update && apt-get install -y curl git ca-certificates && \
    curl -L https://foundry.paradigm.xyz | bash && \
    ~/.foundry/bin/foundryup

# Stage 5: Runtime image for metabased-translator
FROM gcr.io/distroless/cc AS metabased-translator
COPY --from=builder /app/target/release/metabased-translator /usr/local/bin/metabased-translator
COPY --from=foundry /root/.foundry /root/.foundry
ENV PATH="/root/.foundry/bin:${PATH}"
ENTRYPOINT ["/usr/local/bin/metabased-translator"]
EXPOSE 8545 8546
LABEL service=metabased-translator

# Stage 6: Runtime image for metabased-sequencer
FROM gcr.io/distroless/cc AS metabased-sequencer
COPY --from=builder /app/target/release/metabased-sequencer /usr/local/bin/metabased-sequencer
ENTRYPOINT ["/usr/local/bin/metabased-sequencer"]
EXPOSE 8545 8546
LABEL service=metabased-sequencer

# Stage 7: Runtime image for metabased-poster
FROM gcr.io/distroless/cc AS metabased-poster
COPY --from=builder /app/target/release/metabased-poster /usr/local/bin/metabased-poster
ENTRYPOINT ["/usr/local/bin/metabased-poster"]
LABEL service=metabased-poster

# --------- Debugging image ---------
FROM ubuntu:22.04 AS metabased-translator-debug
RUN apt-get update && apt-get install -y heaptrack libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/metabased-translator /usr/local/bin/metabased-translator
COPY --from=foundry /root/.foundry /root/.foundry
ENV PATH="/root/.foundry/bin:${PATH}"
ENTRYPOINT ["/usr/local/bin/metabased-translator"]
EXPOSE 8545 8546
LABEL service=metabased-translator

