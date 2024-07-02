# Heavily inspired by Reth's Makefile: https://github.com/paradigmxyz/reth/blob/main/Makefile
.DEFAULT_GOAL := help

BIN_DIR = "dist/bin"

BUILD_PATH = "target"

# List of features to use when building. Can be overridden via the environment.
# No jemalloc on Windows
# ifeq ($(OS),Windows_NT)
    FEATURES ?=
# else
#     FEATURES ?= jemalloc asm-keccak
# endif

# Cargo profile for builds. Default is for local builds, CI uses an override.
PROFILE ?= release

# Extra flags for Cargo
CARGO_INSTALL_EXTRA_FLAGS ?=

# Features in reth/op-reth binary crate other than "ethereum" and "optimism"
BIN_OTHER_FEATURES := asm-keccak jemalloc jemalloc-prof min-error-logs min-warn-logs min-info-logs min-debug-logs min-trace-logs

##@ Help

.PHONY: help
help: ## Display this help.
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Build

.PHONY: install
install: ## Build and install the op-reth binary under `~/.cargo/bin`.
	cargo install --path . --bin op-reth-syn --force --locked \
		--features "$(FEATURES)" \
		--profile "$(PROFILE)" \
		$(CARGO_INSTALL_EXTRA_FLAGS)
