#syntax=docker/dockerfile:1.7-labs

FROM ghcr.io/succinctlabs/sp1:v5.2.2 AS sp1-builder
RUN apt update && apt-get install patch

WORKDIR /workspace
COPY --parents **/Cargo.toml Cargo.lock **/lib.rs .
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/patches/sp1.patch sp1.patch
# see https://docs.succinct.xyz/docs/sp1/optimizing-programs/precompiles for the list of sp1 precompiles
RUN patch -i sp1.patch && mkdir synd-translator/tests/tests && touch synd-translator/tests/tests/lib.rs

WORKDIR /workspace/synd-withdrawals/synd-tee-attestation-zk-proofs
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/aws-nitro aws-nitro
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/program program

# based on https://github.com/succinctlabs/sp1/blob/bb91c6f64f8b0b8acd59558541fce8336e967881/crates/build/src/command/docker.rs#L134
RUN cd program && CFLAGS_riscv32im_succinct_zkvm_elf=-D__ILP32__ cargo +succinct build -F std --locked --profile zkvm --target riscv32im-succinct-zkvm-elf

FROM risczero/risc0-guest-builder:r0.1.88.0 AS risc0-builder
RUN apt update && apt-get install patch

WORKDIR /workspace
COPY --parents **/Cargo.toml Cargo.lock **/lib.rs .
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/patches/risc0.patch risc0.patch
# see https://dev.risczero.com/api/zkvm/precompiles for the list of risc0 precompiles
RUN patch -i risc0.patch && mkdir synd-translator/tests/tests && touch synd-translator/tests/tests/lib.rs

WORKDIR /workspace/synd-withdrawals/synd-tee-attestation-zk-proofs
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/aws-nitro aws-nitro
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/program program

# based on https://docs.rs/risc0-build/3.0.3/src/risc0_build/docker.rs.html#151
RUN cd program && RISC0_FEATURE_bigint2= CC_riscv32im_risc0_zkvm_elf=/root/.risc0/cpp/bin/riscv32-unknown-elf-gcc CFLAGS_riscv32im_risc0_zkvm_elf="-march=rv32im -nostdlib" cargo +risc0 build -F risc0 --locked --profile zkvm --target riscv32im-risc0-zkvm-elf

FROM risczero/risc0-guest-builder:r0.1.88.0 AS openvm-builder
WORKDIR /workspace
COPY --parents **/Cargo.toml Cargo.lock **/lib.rs .
RUN mkdir synd-translator/tests/tests && touch synd-translator/tests/tests/lib.rs
WORKDIR /workspace/synd-withdrawals/synd-tee-attestation-zk-proofs
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/aws-nitro aws-nitro
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/openvm-p384 openvm-p384
COPY synd-withdrawals/synd-tee-attestation-zk-proofs/program program

RUN cd program && cargo +risc0 build -F openvm --locked --profile zkvm --target riscv32im-risc0-zkvm-elf

FROM scratch AS elf-output
COPY --from=sp1-builder /workspace/target/riscv32im-succinct-zkvm-elf/zkvm/synd-tee-attestation-zk-proofs-program synd-tee-attestation-zk-proofs-sp1-program
COPY --from=risc0-builder /workspace/target/riscv32im-risc0-zkvm-elf/zkvm/synd-tee-attestation-zk-proofs-program synd-tee-attestation-zk-proofs-risc0-program
COPY --from=openvm-builder /workspace/target/riscv32im-risc0-zkvm-elf/zkvm/synd-tee-attestation-zk-proofs-program synd-tee-attestation-zk-proofs-openvm-program