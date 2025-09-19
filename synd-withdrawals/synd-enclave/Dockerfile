FROM ghcr.io/syndicateprotocol/syndicate-appchains/node-builder AS nitro-node-builder
RUN mv /workspace /nitro
WORKDIR /workspace
RUN mv /nitro /workspace/nitro
COPY go.mod go.sum .
RUN go mod download
COPY cmd/enclave/main.go cmd/enclave/g1.point cmd/enclave/
COPY enclave enclave
COPY teemodule teemodule
COPY teetypes teetypes
RUN go build -ldflags "-linkmode external -extldflags -static" -o bin/enclave ./cmd/enclave
FROM scratch AS test-image
COPY --from=nitro-node-builder /workspace/bin/enclave /bin/enclave
ENTRYPOINT ["/bin/enclave"]
