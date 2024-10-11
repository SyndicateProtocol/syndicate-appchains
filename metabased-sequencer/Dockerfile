#NOTE: this builds `proxy.rs`
# Stage 1: Build the Rust application
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/metabased-sequencer

# Copy the entire project
COPY . .

# Build the proxy binary
RUN cargo build --release --bin proxy

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

# Install required certificates for HTTPS requests and OpenSSL 3 for cryptography support
RUN apt-get update && apt-get install -y ca-certificates openssl libssl-dev && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/metabased-sequencer/target/release/proxy .

# Ensure the binary has execute permissions
RUN chmod +x proxy

# Expose port 8456
EXPOSE 8456

# Set the entrypoint to the proxy binary
ENTRYPOINT ["./proxy"]

# Define default command-line arguments (optional)
CMD ["--proxy-address", "127.0.0.1:8456", "--target-address", "https://base-sepolia.blockpi.network/v1/rpc/public"]
