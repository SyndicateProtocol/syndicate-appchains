test:
	go test -v ./...

.PHONY: \
	test

lint:
	golangci-lint run --timeout=10m --config=.golangci.yaml --allow-parallel-runners

format:
	go fmt ./...