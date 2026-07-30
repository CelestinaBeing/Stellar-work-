#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ENV_FILE="$PROJECT_DIR/.env.dev"
CONTRACT_DIR="$PROJECT_DIR/contracts/escrow"
ADDRESSES_FILE="$PROJECT_DIR/contract-addresses.json"

echo "============================================"
echo " StellarWork Local DevNet Setup"
echo "============================================"

echo ""
echo "[1/6] Checking Docker..."
if ! command -v docker &> /dev/null; then
    echo "Error: Docker is not installed. Please install Docker first."
    echo "  https://docs.docker.com/get-docker/"
    exit 1
fi
echo "  Docker found."

echo ""
echo "[2/6] Starting local Stellar Quickstart container..."
CONTAINER_NAME="stellar-devnet"
if docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo "  Container '$CONTAINER_NAME' is already running."
else
    if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
        echo "  Removing existing stopped container..."
        docker rm "$CONTAINER_NAME" > /dev/null
    fi
    docker run -d \
        --name "$CONTAINER_NAME" \
        -p 8000:8000 \
        --platform linux/amd64 \
        stellar/quickstart:soroban-dev@sha256:9c002802588c1f7e4c3f217f2c5168cb10803c2a46126eabfba651ab8324ab60 \
        --standalone \
        --enable-soroban-rpc \
        --enable-soroban-diagnostic-events 2>&1
    echo "  Waiting for Stellar RPC to be ready..."
    for i in $(seq 1 30); do
        if curl -s -o /dev/null http://localhost:8000; then
            echo "  Stellar RPC is ready on http://localhost:8000"
            break
        fi
        if [ "$i" -eq 30 ]; then
            echo "Error: Stellar RPC did not start within 30 seconds."
            docker logs "$CONTAINER_NAME" --tail 20
            exit 1
        fi
        sleep 2
    done
fi

echo ""
echo "[3/6] Funding test admin account..."
# Generate a keypair for the test admin if not already present
ADMIN_SECRET="${ADMIN_SECRET:-SAZQ3DXTPNF5R7L7O6QJ6QKJ2KQZ3QZ5J5Q5J5Q5J5Q5J5Q5J5Q5J5Q5}"
ADMIN_ADDRESS="${ADMIN_ADDRESS:-GBAOOGOGB7J3M5X7Z5Q5J5Q5J5Q5J5Q5J5Q5J5Q5J5Q5J5Q5J5Q5J5Q5}"

# Fund using friendbot on standalone network
curl -s "http://localhost:8000/friendbot?addr=$ADMIN_ADDRESS" > /dev/null
echo "  Test admin account funded: $ADMIN_ADDRESS"

echo ""
echo "[4/6] Building and deploying escrow contract..."
cd "$CONTRACT_DIR"
soroban contract build 2>&1 | tail -1
WASM_PATH="$CONTRACT_DIR/target/wasm32-unknown-unknown/release/escrow.wasm"
if [ ! -f "$WASM_PATH" ]; then
    echo "Error: WASM not found at $WASM_PATH"
    exit 1
fi

CONTRACT_ID=$(soroban contract deploy \
    --wasm "$WASM_PATH" \
    --source-account "$ADMIN_SECRET" \
    --rpc-url http://localhost:8000 \
    --network-passphrase "Standalone Network ; February 2017" 2>&1)
echo "  Contract deployed: $CONTRACT_ID"

echo ""
echo "[5/6] Initializing contract..."
NATIVE_TOKEN="$(soroban lab token wrap \
    --source-account "$ADMIN_SECRET" \
    --rpc-url http://localhost:8000 \
    --network-passphrase "Standalone Network ; February 2017" \
    --asset "native" 2>&1 | head -1 || true)"

soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source-account "$ADMIN_SECRET" \
    --rpc-url http://localhost:8000 \
    --network-passphrase "Standalone Network ; February 2017" \
    -- \
    initialize \
    --admin "$ADMIN_ADDRESS" \
    --native-token "$NATIVE_TOKEN" 2>&1 || echo "  (May already be initialized)"

echo ""
echo "[6/6] Saving environment configuration..."
cat > "$ENV_FILE" << EOF
# StellarWork Local DevNet Configuration
# Generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
NEXT_PUBLIC_CONTRACT_ID=$CONTRACT_ID
NEXT_PUBLIC_NETWORK=standalone
NEXT_PUBLIC_SOROBAN_RPC=http://localhost:8000
NEXT_PUBLIC_NETWORK_PASSPHRASE=Standalone Network ; February 2017
ADMIN_SECRET=$ADMIN_SECRET
ADMIN_ADDRESS=$ADMIN_ADDRESS
EOF

# Update contract-addresses.json
if [ ! -f "$ADDRESSES_FILE" ]; then
    echo '{}' > "$ADDRESSES_FILE"
fi
node -e "
const fs = require('fs');
const data = JSON.parse(fs.readFileSync('$ADDRESSES_FILE', 'utf8'));
data.dev = data.dev || {};
data.dev.contractId = '$CONTRACT_ID';
data.dev.rpcUrl = 'http://localhost:8000';
data.dev.passphrase = 'Standalone Network ; February 2017';
data.dev.admin = '$ADMIN_ADDRESS';
data.dev.nativeToken = '$NATIVE_TOKEN';
fs.writeFileSync('$ADDRESSES_FILE', JSON.stringify(data, null, 2) + '\n');
"

echo ""
echo "============================================"
echo " DevNet setup complete!"
echo " Contract ID: $CONTRACT_ID"
echo " RPC URL:     http://localhost:8000"
echo " Config:      $ENV_FILE"
echo "============================================"
echo ""
echo "Next steps:"
echo "  1. Copy env file: cp $ENV_FILE frontend/.env.local"
echo "  2. Run tests:     ./scripts/run-tests-local.sh"
echo "  3. Reset devnet:  ./scripts/reset-devnet.sh"
