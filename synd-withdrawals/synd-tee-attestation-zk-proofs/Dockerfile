#syntax=docker/dockerfile:1.7-labs

FROM ghcr.io/succinctlabs/sp1:v5.2.2 AS builder

WORKDIR /workspace
COPY --parents **/Cargo.toml **/Cargo.lock **/lib.rs .
RUN mkdir synd-translator/tests/tests && touch synd-translator/tests/tests/lib.rs

WORKDIR synd-withdrawals/synd-tee-attestation-zk-proofs
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/aws-nitro aws-nitro
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/sp1/program sp1/program

# based on https://github.com/succinctlabs/sp1/blob/bb91c6f64f8b0b8acd59558541fce8336e967881/crates/build/src/command/utils.rs#L53
ARG RUSTFLAGS="-C passes=lower-atomic -C link-arg=-Ttext=0x00201000 -C link-arg=--image-base=0x00200800 -C panic=abort --cfg getrandom_backend=\"custom\" -C llvm-args=-misched-prera-direction=bottomup -C llvm-args=-misched-postra-direction=bottomup"

# based on https://github.com/succinctlabs/sp1/blob/bb91c6f64f8b0b8acd59558541fce8336e967881/crates/build/src/command/docker.rs#L134
RUN cd sp1/program && CFLAGS_riscv32im_succinct_zkvm_elf=-D__ILP32__ RUSTFLAGS=${RUSTFLAGS} cargo +succinct build --locked --release --target riscv32im-succinct-zkvm-elf

FROM scratch AS elf-output
COPY --from=builder /workspace/target/riscv32im-succinct-zkvm-elf/release/synd-tee-attestation-zk-proofs-sp1-program .
