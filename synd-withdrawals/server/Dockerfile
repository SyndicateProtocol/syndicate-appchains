# syntax=docker/dockerfile:1.7-labs

FROM golang:1.24.4-bookworm AS builder
WORKDIR /build
COPY go.mod go.sum .
RUN go mod download
COPY main.go .
RUN go build -ldflags "-linkmode external -extldflags -static" -o server .

FROM scratch
WORKDIR /app
COPY --from=builder /build/server .
ENTRYPOINT ["/app/server"]
