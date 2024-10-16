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