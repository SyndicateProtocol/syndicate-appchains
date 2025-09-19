# nixos/nix:2.31.1
FROM nixos/nix@sha256:24196c350d146529a4101edea9c82129308640b500ebbc01d225ad36b6322cb6 AS bootstrap
WORKDIR /build
COPY flake.nix flake.lock .
COPY nix nix
COPY synd-enclave/cmd/enclave/main.go synd-enclave/cmd/enclave/g1.point synd-enclave/cmd/enclave/
COPY synd-enclave/enclave synd-enclave/enclave
COPY synd-enclave/teemodule synd-enclave/teemodule
COPY synd-enclave/teetypes synd-enclave/teetypes
COPY synd-enclave/go.mod synd-enclave/go.sum synd-enclave
COPY synd-enclave/eif synd-enclave/eif

#RUN nix --extra-experimental-features "nix-command flakes" --accept-flake-config flake update
RUN nix --extra-experimental-features "nix-command flakes" --accept-flake-config build .#eif-bin --no-update-lock-file

FROM scratch AS eif-bin
COPY --from=bootstrap /build/result eif.bin
