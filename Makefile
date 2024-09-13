test:
	go test -v ./...

lint:
	golangci-lint run --timeout=10m --config=.golangci.yaml --allow-parallel-runners

format:
	go fmt ./...

.PHONY: \
	test