#!/usr/bin/env bash
set -euo pipefail

CONTAINER_NAME="stellar-devnet"
ENV_FILE="$(dirname "$(dirname "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)")")/.env.dev"

echo "============================================"
echo " Resetting StellarWork DevNet"
echo "============================================"

echo ""
echo "[1/2] Stopping and removing container..."
if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    docker stop "$CONTAINER_NAME" > /dev/null 2>&1 || true
    docker rm "$CONTAINER_NAME" > /dev/null 2>&1 || true
    echo "  Container '$CONTAINER_NAME' removed."
else
    echo "  No container '$CONTAINER_NAME' found."
fi

echo ""
echo "[2/2] Cleaning up environment files..."
if [ -f "$ENV_FILE" ]; then
    rm -f "$ENV_FILE"
    echo "  Removed $ENV_FILE"
else
    echo "  No env file found."
fi

echo ""
echo "============================================"
echo " DevNet reset complete."
echo " Run ./scripts/setup-devnet.sh to start fresh."
echo "============================================"
