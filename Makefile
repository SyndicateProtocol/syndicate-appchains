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
