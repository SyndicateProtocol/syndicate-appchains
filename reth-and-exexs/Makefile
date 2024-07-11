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

.PHONY: install-foundry
install-foundry: ## Install Foundry if the forge command is not found or is corrupted.
	@if [ -z "$$(which forge)" ]; then \
		echo "Forge not found. Installing Foundry..."; \
		curl -L https://foundry.paradigm.xyz | bash && bash -c "source ~/.bashrc && foundryup"; \
	fi
	@forge build --help > /dev/null 2>&1 || (echo "Forge is corrupted. Reinstalling Foundry..." && curl -L https://foundry.paradigm.xyz | bash && bash -c "source ~/.bashrc && foundryup")
	@echo "Foundry is installed and ready to use."

.PHONY: build-op-reth-syn
build-op-reth-syn: ## Build and install the op-reth binary under `~/.cargo/bin`.
	cargo install --path . --bin op-reth-syn --force --locked \
		--features "$(FEATURES)" \
		--profile "$(PROFILE)" \
		$(CARGO_INSTALL_EXTRA_FLAGS)

.PHONY: build-contracts
build-contracts: install-foundry ## Build contracts using forge.
	cd ../contracts && forge build
	@echo "Contract compilation complete"

.PHONY: test
test: build-contracts ## Run tests after building the contracts.
	cargo test --all
	@echo "Tests complete"

##@ Custom Build

.PHONY: install
install: build-contracts build-op-reth-syn ## Run build steps.
	@echo "Install complete. You can now run op-reth-syn from your terminal."