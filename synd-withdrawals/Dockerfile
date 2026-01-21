# nixos/nix:2.31.1
FROM nixos/nix@sha256:24196c350d146529a4101edea9c82129308640b500ebbc01d225ad36b6322cb6 AS bootstrap

ENV REPO=https://github.com/aws/aws-nitro-enclaves-sdk-bootstrap.git
ENV COMMIT=7614f19963e4e956493b3260fda4d62834bb281c
RUN git init && \
    git remote add origin $REPO && \
    git fetch --depth=1 origin $COMMIT && \
    git reset --hard FETCH_HEAD

RUN mkdir out
RUN nix-build --option filter-syscalls false -A kernel && cp -r result/* out/
RUN nix-build --option filter-syscalls false -A init && cp -r result/* out/

# node builder
FROM ghcr.io/syndicateprotocol/syndicate-appchains/node-builder AS builder
RUN go install github.com/linuxkit/linuxkit/src/cmd/linuxkit@270fd1c5aa1986977b31af6c743c6a2681f67a29
WORKDIR /build
RUN mv /workspace /build/nitro
RUN ls /build && ls /build/nitro
COPY synd-enclave/cmd/enclave/main.go synd-enclave/cmd/enclave/g1.point cmd/enclave/
COPY synd-enclave/enclave enclave
COPY synd-enclave/teemodule teemodule
COPY synd-enclave/teetypes teetypes
COPY synd-enclave/go.mod synd-enclave/go.sum .
RUN go build -ldflags "-linkmode external -extldflags -static" -o bin/enclave ./cmd/enclave

COPY synd-enclave/eif/init-ramdisk.yaml synd-enclave/eif/user-ramdisk.yaml eif/
COPY --from=bootstrap out bootstrap
RUN linuxkit build --format kernel+initrd --no-sbom --name init-ramdisk ./eif/init-ramdisk.yaml
RUN linuxkit build --format kernel+initrd --no-sbom --name user-ramdisk ./eif/user-ramdisk.yaml

# https://hub.docker.com/layers/library/rust/trixie/images/sha256-155f835173fcc2677a4132b0817d3e4684aeee38806654a4cacb53d6f0e88a83
FROM rust@sha256:bed2d7f8140d73c26f16c298c91ae8487a09f40d3840c0d8d139537e1b51e148  AS eif

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
COPY "synd-enclave/eif/cmdline-${TARGETARCH}" cmdline
COPY --from=bootstrap out bootstrap
COPY --from=builder /build/init-ramdisk-initrd.img .
COPY --from=builder /build/user-ramdisk-initrd.img .

RUN ./target/release/eif_build \
    --kernel bootstrap/*Image \
    --kernel_config bootstrap/*Image.config \
    --cmdline "$(cat cmdline)" \
    --ramdisk init-ramdisk-initrd.img \
    --ramdisk user-ramdisk-initrd.img \
    --output eif.bin

FROM scratch AS eif-bin
COPY --from=eif /build/eif.bin .
