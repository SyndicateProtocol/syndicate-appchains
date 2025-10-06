FROM rust:1.89.0@sha256:57407b378b2b6e07b48a6135a20c87cc22ea6e249c0acf6cb1833ead3cf116e9 AS builder

RUN curl -L https://sp1.succinct.xyz | bash && ~/.sp1/bin/sp1up --version v5.2.1 

WORKDIR /workspace

COPY . .

RUN cd synd-withdrawals/synd-tee-attestation-zk-proofs/sp1/program && cargo build --locked --ignore-rust-version --release

FROM scratch AS elf-output

COPY --from=builder /workspace/target/release/synd-tee-attestation-zk-proofs-sp1-program /elf-output/
