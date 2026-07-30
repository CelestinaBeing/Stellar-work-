#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ENV_FILE="$PROJECT_DIR/.env.dev"

echo "============================================"
echo " Running StellarWork Tests (Local DevNet)"
echo "============================================"

if [ ! -f "$ENV_FILE" ]; then
    echo "Error: DevNet environment file not found at $ENV_FILE"
    echo "Run ./scripts/setup-devnet.sh first to set up the local network."
    exit 1
fi

source "$ENV_FILE"

echo ""
echo "--- Running Contract Tests ---"
cd "$PROJECT_DIR/contracts/escrow"
cargo test 2>&1
CARGO_EXIT=$?
if [ $CARGO_EXIT -ne 0 ]; then
    echo "Error: Contract tests failed (exit code: $CARGO_EXIT)"
    exit $CARGO_EXIT
fi
echo "Contract tests passed."

echo ""
echo "--- Running Frontend Tests ---"
cd "$PROJECT_DIR/frontend"
NEXT_PUBLIC_CONTRACT_ID="$NEXT_PUBLIC_CONTRACT_ID" \
NEXT_PUBLIC_NETWORK="standalone" \
NEXT_PUBLIC_SOROBAN_RPC="http://localhost:8000" \
npm test 2>&1
FRONTEND_EXIT=$?
if [ $FRONTEND_EXIT -ne 0 ]; then
    echo "Warning: Frontend tests had failures (exit code: $FRONTEND_EXIT)"
fi

echo ""
echo "============================================"
echo " Test run complete."
echo " Contract tests: $([ $CARGO_EXIT -eq 0 ] && echo 'PASSED' || echo 'FAILED')"
echo " Frontend tests: $([ $FRONTEND_EXIT -eq 0 ] && echo 'PASSED' || echo 'HAD ISSUES')"
echo "============================================"
