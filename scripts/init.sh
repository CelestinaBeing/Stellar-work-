#!/usr/bin/env bash
set -euo pipefail

NETWORK="${1:-}"
CONTRACT_ID=""
ADMIN_ADDRESS=""
NATIVE_TOKEN=""
SOURCE_IDENTITY="stellarwork-admin"

usage() {
  echo "Usage: $0 <network> --contract-id <id> --admin <address> --native-token <address> [--source <identity>]"
  echo ""
  echo "Available networks: testnet, futurenet, mainnet"
  echo ""
  echo "Initializes the escrow contract with admin address and native token."
  exit 1
}

if [[ -z "$NETWORK" ]]; then
  usage
fi

if [[ "$NETWORK" != "testnet" && "$NETWORK" != "futurenet" && "$NETWORK" != "mainnet" ]]; then
  echo "Error: Invalid network '$NETWORK'. Must be one of: testnet, futurenet, mainnet"
  exit 1
fi

shift
while [[ $# -gt 0 ]]; do
  case "$1" in
    --contract-id) CONTRACT_ID="$2"; shift 2 ;;
    --admin) ADMIN_ADDRESS="$2"; shift 2 ;;
    --native-token) NATIVE_TOKEN="$2"; shift 2 ;;
    --source) SOURCE_IDENTITY="$2"; shift 2 ;;
    *) echo "Unknown option: $1"; usage ;;
  esac
done

if [[ -z "$CONTRACT_ID" || -z "$ADMIN_ADDRESS" || -z "$NATIVE_TOKEN" ]]; then
  echo "Error: --contract-id, --admin, and --native-token are all required."
  echo ""
  usage
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ADDRESSES_FILE="$PROJECT_DIR/contract-addresses.json"

echo "============================================"
echo " StellarWork Contract Initialization"
echo " Network:      $NETWORK"
echo " Contract ID:  $CONTRACT_ID"
echo " Admin:        $ADMIN_ADDRESS"
echo " Native Token: $NATIVE_TOKEN"
echo "============================================"

echo ""
echo "[1/2] Calling initialize..."
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE_IDENTITY" \
  --network "$NETWORK" \
  -- initialize \
  --admin "$ADMIN_ADDRESS" \
  --native_token "$NATIVE_TOKEN"

echo "  Contract initialized successfully."

echo ""
echo "[2/2] Updating contract-addresses.json..."
if [[ -f "$ADDRESSES_FILE" ]]; then
  node -e "
const fs = require('fs');
const data = JSON.parse(fs.readFileSync('$ADDRESSES_FILE', 'utf8'));
data['$NETWORK'] = data['$NETWORK'] || {};
data['$NETWORK'].admin = '$ADMIN_ADDRESS';
data['$NETWORK'].nativeToken = '$NATIVE_TOKEN';
data['$NETWORK'].initializedAt = new Date().toISOString();
fs.writeFileSync('$ADDRESSES_FILE', JSON.stringify(data, null, 2) + '\n');
"
fi
echo "  Address file updated."

echo ""
echo "============================================"
echo " Initialization complete!"
echo " Contract ID:  $CONTRACT_ID"
echo " Admin:        $ADMIN_ADDRESS"
echo " Native Token: $NATIVE_TOKEN"
echo "============================================"
