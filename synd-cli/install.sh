#!/usr/bin/env bash
set -euo pipefail

# Syndicate CLI Installer
# Usage:
#   curl -L https://raw.githubusercontent.com/SyndicateProtocol/syndicate-appchains/main/synd-cli/install.sh | bash
#   curl -L https://raw.githubusercontent.com/SyndicateProtocol/syndicate-appchains/main/synd-cli/install.sh | SYND_VERSION=1.0.0 bash

REPO="SyndicateProtocol/syndicate-appchains"
INSTALL_DIR="${SYND_INSTALL_DIR:-$HOME/.synd/bin}"
BINARY_NAME="synd-cli"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    printf "${GREEN}info${NC}: %s\n" "$1"
}

warn() {
    printf "${YELLOW}warn${NC}: %s\n" "$1"
}

error() {
    printf "${RED}error${NC}: %s\n" "$1" >&2
    exit 1
}

detect_platform() {
    local os arch

    # Detect OS
    case "$(uname -s)" in
        Linux*)  os="linux" ;;
        Darwin*) os="darwin" ;;
        *)       error "Unsupported operating system: $(uname -s)" ;;
    esac

    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64)  arch="x64" ;;
        arm64|aarch64) arch="arm64" ;;
        *)             error "Unsupported architecture: $(uname -m)" ;;
    esac

    echo "${os}-${arch}"
}

get_latest_version() {
    local latest
    # Find the latest synd-cli release (tags starting with "synd-cli-")
    # The "|| true" prevents pipefail from exiting the script when no version is found
    latest=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases" | \
        grep '"tag_name"' | \
        sed -E 's/.*"([^"]+)".*/\1/' | \
        grep '^synd-cli-' | \
        head -n 1 || true)

    if [[ -z "$latest" ]]; then
        error "Failed to fetch latest synd-cli version. Please check your internet connection or specify a version with SYND_VERSION=X.Y.Z"
    fi

    echo "$latest"
}

download_binary() {
    local version="$1"
    local platform="$2"
    local url="https://github.com/${REPO}/releases/download/${version}/${BINARY_NAME}-${platform}-${version}"
    local tmp_file

    tmp_file=$(mktemp)

    info "Downloading synd-cli ${version} for ${platform}..." >&2

    if ! curl -fsSL "$url" -o "$tmp_file"; then
        rm -f "$tmp_file"
        error "Failed to download from ${url}. Please check that version ${version} exists and has binaries for ${platform}."
    fi

    echo "$tmp_file"
}

install_binary() {
    local tmp_file="$1"
    local install_path="${INSTALL_DIR}/${BINARY_NAME}"

    # Create install directory
    mkdir -p "$INSTALL_DIR"

    # Move binary to install location
    mv "$tmp_file" "$install_path"
    chmod +x "$install_path"

    info "Installed synd-cli to ${install_path}"
}

setup_path_instructions() {
    local shell_name
    local rc_file

    shell_name=$(basename "$SHELL")

    case "$shell_name" in
        bash)
            if [[ -f "$HOME/.bash_profile" ]]; then
                rc_file="$HOME/.bash_profile"
            else
                rc_file="$HOME/.bashrc"
            fi
            ;;
        zsh)  rc_file="$HOME/.zshrc" ;;
        fish) rc_file="$HOME/.config/fish/config.fish" ;;
        *)    rc_file="your shell's config file" ;;
    esac

    # Check if already in PATH
    if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
        info "synd-cli is ready to use!"
        return
    fi

    echo ""
    printf "${YELLOW}synd-cli is not in your PATH.${NC}\n"
    echo ""
    printf "Would you like to add it automatically? [Y/n] "

    # Read user input (handle piped input by reading from /dev/tty)
    local response
    if [[ -t 0 ]]; then
        read -r response
    else
        read -r response < /dev/tty
    fi

    case "$response" in
        [Yy]|"")
            # Add to PATH
            if [[ "$shell_name" == "fish" ]]; then
                echo "fish_add_path $INSTALL_DIR" >> "$rc_file"
            else
                echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$rc_file"
            fi
            info "Added synd-cli to PATH in ${rc_file}"
            echo ""
            warn "Restart your terminal or run: source ${rc_file}"
            echo ""
            ;;
        [Nn])
            echo ""
            printf "${YELLOW}========================================${NC}\n"
            warn "ACTION REQUIRED: Add synd-cli to your PATH"
            printf "${YELLOW}========================================${NC}\n"
            echo ""
            printf "Run this command to add synd-cli to your PATH:\n"
            echo ""
            if [[ "$shell_name" == "fish" ]]; then
                printf "  ${GREEN}fish_add_path %s${NC}\n" "$INSTALL_DIR"
            else
                printf "  ${GREEN}echo 'export PATH=\"%s:\$PATH\"' >> %s${NC}\n" "$INSTALL_DIR" "$rc_file"
            fi
            echo ""
            printf "Then restart your terminal or run:\n"
            echo ""
            printf "  ${GREEN}source %s${NC}\n" "$rc_file"
            echo ""
            printf "${YELLOW}========================================${NC}\n"
            echo ""
            ;;
        *)
            warn "Invalid response. Please manually add ${INSTALL_DIR} to your PATH."
            ;;
    esac
}

main() {
    local version="${SYND_VERSION:-}"
    local platform

    info "Syndicate CLI Installer"
    echo ""

    # Detect platform
    platform=$(detect_platform)
    info "Detected platform: ${platform}"

    # Get version (use provided or fetch latest)
    if [[ -z "$version" ]]; then
        info "Fetching latest version..."
        version=$(get_latest_version)
    else
        # User provided version - add the synd-cli-v prefix
        version="synd-cli-v${version}"
    fi
    info "Version: ${version}"

    # Download binary
    local tmp_file
    tmp_file=$(download_binary "$version" "$platform")

    # Install binary
    install_binary "$tmp_file"

    # Show PATH setup instructions
    setup_path_instructions

    info "Installation complete!"
}

main "$@"
