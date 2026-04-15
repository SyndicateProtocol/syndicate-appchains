#!/usr/bin/env bash
set -euo pipefail

# Resolve the directory this script lives in so it works from any call site
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# ── Preflight ──────────────────────────────────────────────────────────────────
if [ ! -f .env ]; then
  echo "Error: .env file not found."
  echo "  cp .env.example .env"
  echo "  # fill in your values, then re-run this script"
  exit 1
fi

source .env

DATA_DIR="${DATA_DIR:-./data}"

# ── Data directories ───────────────────────────────────────────────────────────
echo "Creating data directories under $DATA_DIR ..."
mkdir -p \
  "$DATA_DIR/valkey" \
  "$DATA_DIR/settlement" \
  "$DATA_DIR/sequencing" \
  "$DATA_DIR/mchain" \
  "$DATA_DIR/nitro"

# ── Snapshots ─────────────────────────────────────────────────────────────────
download_snapshot() {
  local url="$1"
  local dest="$2"
  local label="$3"
  local marker="$dest/.snapshot_downloaded"

  [ -z "$url" ] && return 0

  if [ -f "$marker" ]; then
    echo "Skipping $label snapshot: already downloaded"
    return 0
  fi

  echo "Downloading $label snapshot from $url ..."
  curl -L --progress-bar "$url" | tar -xf - -C "$dest"
  touch "$marker"
  echo "$label snapshot extracted to $dest"
}

download_snapshot "${NITRO_SNAPSHOT_URL:-}"  "$DATA_DIR/nitro"  "nitro"

# ── Forward optional env vars ──────────────────────────────────────────────────
[ -n "${MCHAIN_GENESIS_CONFIG:-}" ] && export GENESIS_CONFIG="$MCHAIN_GENESIS_CONFIG"
[ -n "${MCHAIN_SNAPSHOT_URL:-}" ]   && export SNAPSHOT_URL="$MCHAIN_SNAPSHOT_URL"

# ── Start services ─────────────────────────────────────────────────────────────
echo "Starting services ..."
DOCKER_DEFAULT_PLATFORM=linux/amd64 docker compose --env-file .env up -d

echo ""
echo "All services started."
echo "Appchain JSON-RPC will be available at http://localhost:8545 once nitro is ready."
echo "Run 'docker compose logs -f' to follow logs."
