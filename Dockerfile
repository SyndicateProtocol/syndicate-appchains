# NOTE: Multi-target Dockerfile for building and running both Metabased Translator and Interceptor
# Stage 1: Prepare cargo-chef
FROM lukemathwalker/cargo-chef:latest-rust-1.84 AS chef
WORKDIR /app

# Install system dependencies
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y \
    libclang-dev \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Stage 2: Generate recipe.json
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies and application
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build configuration
ARG BUILD_PROFILE=release
ENV BUILD_PROFILE=$BUILD_PROFILE

# Use specific rust version from workspace
ENV RUST_VERSION=1.84

# Optional: Extra cargo flags and features
ARG RUSTFLAGS=""
ENV RUSTFLAGS="$RUSTFLAGS"
ARG FEATURES=""
ENV FEATURES=$FEATURES

# Build dependencies first (this layer will be cached if dependencies don't change)
RUN cargo chef cook --profile $BUILD_PROFILE --features "$FEATURES" --recipe-path recipe.json

# Build application
COPY . .

# Build the translator binary
RUN cargo build --profile $BUILD_PROFILE \
    --features "$FEATURES" \
    --locked \
    --bin metabased-translator

# Build interceptor and proxy
RUN cargo build --profile $BUILD_PROFILE \
    --features "$FEATURES" \
    --locked \
    --package interceptor \
    --package proxy

# Copy binaries to known locations
RUN cp /app/target/$BUILD_PROFILE/metabased-translator /app/translator && \
    cp /app/target/$BUILD_PROFILE/interceptor /app/interceptor

# Stage 4: Create metabased-translator runtime image
FROM ubuntu:22.04 AS metabased-translator
WORKDIR /app

# Install metabased-translator dependencies and Foundry prerequisites
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    curl \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash && \
    ~/.foundry/bin/foundryup

# Add foundry binaries to PATH
ENV PATH="/root/.foundry/bin:${PATH}"

# Verify anvil installation
RUN anvil --version

# Copy binary from builder and verify
COPY --from=builder /app/translator /usr/local/bin/metabased-translator

# Create a configuration directory
RUN mkdir -p /etc/metabased

# Optional: Create volume mount points for persistent data
VOLUME ["/data"]

# Expose ports (adjust according to your needs)
EXPOSE 8545 8546

ENTRYPOINT ["/usr/local/bin/metabased-translator"]
# Optional default arguments can be provided via CMD
LABEL service=metabased-translator

# Stage 5: Create interceptor runtime image
FROM ubuntu:22.04 AS interceptor
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    curl \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/*

# NOTE: Foundry is unnecessary for the interceptor. Skipping install

# Copy interceptor binary from builder and verify
COPY --from=builder /app/interceptor /usr/local/bin/interceptor

# Create a configuration directory
RUN mkdir -p /etc/interceptor

# Optional: Create volume mount points for persistent data
VOLUME ["/data"]

# Expose ports (adjust according to your needs)
EXPOSE 8545 8546

ENTRYPOINT ["/usr/local/bin/interceptor"]
# Optional default arguments can be provided via CMD
LABEL service=interceptor