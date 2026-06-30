#!/usr/bin/env bash
set -euo pipefail

NETWORK="${1:-}"
CONTRACT_ID="${2:-}"

if [[ -z "$NETWORK" || -z "$CONTRACT_ID" ]]; then
  echo "Usage: $0 <network> <contract-id> [--source <identity>]"
  echo ""
  echo "Available networks: testnet, futurenet, mainnet"
  echo ""
  echo "This script upgrades the contract WASM for an already-deployed contract."
  echo "The contract must support the 'upgrade' function with a new_wasm_hash parameter."
  exit 1
fi

if [[ "$NETWORK" != "testnet" && "$NETWORK" != "futurenet" && "$NETWORK" != "mainnet" ]]; then
  echo "Error: Invalid network '$NETWORK'. Must be one of: testnet, futurenet, mainnet"
  exit 1
fi

SOURCE_IDENTITY="stellarwork-admin"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --source) SOURCE_IDENTITY="$2"; shift 2 ;;
    *) shift ;;
  esac
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
CONTRACT_DIR="$PROJECT_DIR/contracts/escrow"
ADDRESSES_FILE="$PROJECT_DIR/contract-addresses.json"

echo "============================================"
echo " StellarWork Contract Upgrade"
echo " Network:     $NETWORK"
echo " Contract ID: $CONTRACT_ID"
echo "============================================"

echo ""
echo "[1/4] Building new contract WASM..."
cd "$CONTRACT_DIR"
soroban contract build
WASM_PATH="$CONTRACT_DIR/target/wasm32-unknown-unknown/release/escrow.wasm"
if [[ ! -f "$WASM_PATH" ]]; then
  echo "Error: WASM not found at $WASM_PATH"
  exit 1
fi
echo "  WASM built: $WASM_PATH"

echo ""
echo "[2/4] Installing new WASM..."
WASM_HASH=$(soroban contract install \
  --wasm "$WASM_PATH" \
  --source "$SOURCE_IDENTITY" \
  --network "$NETWORK" 2>&1)
echo "  WASM hash: $WASM_HASH"

echo ""
echo "[3/4] Invoking contract upgrade..."
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE_IDENTITY" \
  --network "$NETWORK" \
  -- upgrade \
  --new_wasm_hash "$WASM_HASH"

echo ""
echo "[4/4] Updating contract-addresses.json..."
if [[ -f "$ADDRESSES_FILE" ]]; then
  node -e "
const fs = require('fs');
const data = JSON.parse(fs.readFileSync('$ADDRESSES_FILE', 'utf8'));
data['$NETWORK'] = data['$NETWORK'] || {};
data['$NETWORK'].wasmHash = '$WASM_HASH';
data['$NETWORK'].upgradedAt = new Date().toISOString();
fs.writeFileSync('$ADDRESSES_FILE', JSON.stringify(data, null, 2) + '\n');
"
fi
echo "  Address file updated."

echo ""
echo "============================================"
echo " Upgrade complete!"
echo " Contract ID: $CONTRACT_ID"
echo " WASM hash:   $WASM_HASH"
echo " Network:     $NETWORK"
echo "============================================"
