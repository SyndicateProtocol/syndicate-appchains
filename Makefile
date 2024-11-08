test:
	go test -v ./...
.PHONY: test

lint:
	golangci-lint run --timeout=10m --config=.golangci.yaml --allow-parallel-runners
.PHONY: lint

lint-fix:
	golangci-lint run --timeout=10m --config=.golangci.yaml --allow-parallel-runners --fix
.PHONY: lint-fix

format:
	go fmt ./...
.PHONY: format

build:
	op-translator metabased-publisher
.PHONY: build

op-translator:
	make -C ./op-translator op-translator
.PHONY: op-translator

metabased-publisher:
	make -C ./metabased-publisher metabased-publisher
.PHONY: metabased-publisher

curl --location 'http://127.0.0.1:8555' --header 'Content-Type: application/json' --header 'Cookie: lb1=edge-proxyd-proxyd-ankr-http' --data '{"id":1,"jsonrpc":"2.0","method":"eth_chainId","params":[]}'