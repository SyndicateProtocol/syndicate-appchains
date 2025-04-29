# Global build arguments
ARG BUILD_PROFILE=release
ARG FEATURES=""

# Stage 1: Base image with Rust
FROM rust:slim-bookworm AS builder
ARG BUILD_PROFILE
ARG FEATURES
WORKDIR /app

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apt \
    --mount=type=cache,target=/var/lib/apt \
    apt-get update && \
    apt-get install -y libclang-dev pkg-config build-essential libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Stage 2: Build
FROM builder AS build
COPY . .

# Perform cargo build with cached Cargo and target directories
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:slim-bookworm,source=/usr/local/cargo \
    cargo build --profile ${BUILD_PROFILE} --features "${FEATURES}" --locked --workspace


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
FROM gcr.io/distroless/cc AS runtime-base

# Runtime images
FROM runtime-base AS metabased-translator
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/metabased-translator /usr/local/bin/metabased-translator
COPY --from=foundry /root/.foundry /root/.foundry
ENV PATH="/root/.foundry/bin:${PATH}"
ENTRYPOINT ["/usr/local/bin/metabased-translator"]
EXPOSE 8545 8546
LABEL service=metabased-translator

FROM runtime-base AS metabased-sequencer
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/metabased-sequencer /usr/local/bin/metabased-sequencer
ENTRYPOINT ["/usr/local/bin/metabased-sequencer"]
EXPOSE 8545 8546
LABEL service=metabased-sequencer

FROM runtime-base AS metabased-poster
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/metabased-poster /usr/local/bin/metabased-poster
ENTRYPOINT ["/usr/local/bin/metabased-poster"]
LABEL service=metabased-poster

FROM runtime-base AS maestro
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/maestro /usr/local/bin/maestro
ENTRYPOINT ["/usr/local/bin/maestro"]
EXPOSE 8545 8546
LABEL service=maestro

FROM runtime-base AS batch-sequencer
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/batch-sequencer /usr/local/bin/batch-sequencer
ENTRYPOINT ["/usr/local/bin/batch-sequencer"]
EXPOSE 8545 8546
LABEL service=batch-sequencer

FROM runtime-base AS mchain
ARG BUILD_PROFILE
COPY --from=build /app/target/${BUILD_PROFILE}/mchain /usr/local/bin/mchain
ENTRYPOINT ["/usr/local/bin/mchain"]
EXPOSE 8545 8546
LABEL service=mchain

# --------- Debugging image for translator ---------
FROM ubuntu:22.04 AS metabased-translator-debug
ARG BUILD_PROFILE
RUN apt-get update && apt-get install -y heaptrack libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/${BUILD_PROFILE}/metabased-translator /usr/local/bin/metabased-translator
COPY --from=foundry /root/.foundry /root/.foundry
ENV PATH="/root/.foundry/bin:${PATH}"
ENTRYPOINT ["/usr/local/bin/metabased-translator"]
EXPOSE 8545 8546
LABEL service=metabased-translator
