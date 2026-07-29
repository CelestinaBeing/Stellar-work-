#!/usr/bin/env bash
set -euo pipefail

CONTRACT_DIR="$(cd "$(dirname "$0")/contracts/escrow" && pwd)"
WASM="$CONTRACT_DIR/target/wasm32-unknown-unknown/release/escrow.wasm"
SOROBAN="${SOROBAN_CLI:-soroban}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo ""
echo "============================================="
echo "  StellarWork Escrow Contract Gas Benchmark"
echo "============================================="
echo ""

if ! command -v "$SOROBAN" &>/dev/null; then
  echo -e "${RED}Error: soroban CLI not found. Install it with: cargo install soroban-cli${NC}"
  exit 1
fi

echo "[1/3] Building contract in release mode..."
cd "$CONTRACT_DIR"
soroban contract build --package escrow 2>&1

if [ ! -f "$WASM" ]; then
  echo -e "${RED}Error: WASM not found at $WASM. Build may have failed.${NC}"
  exit 1
fi

WASM_SIZE=$(du -h "$WASM" | cut -f1)
echo -e "  WASM size: ${GREEN}$WASM_SIZE${NC}"

echo ""
echo "[2/3] Analyzing contract functions..."

echo ""
echo "  Function                     | Read Bytes | Write Bytes | Approx. Fee (stroops)"
echo "  -----------------------------|------------|-------------|-------------------"

FUNCTIONS=(
  "initialize"
  "post_job"
  "accept_job"
  "submit_work"
  "approve_work"
  "reject_work"
  "cancel_job"
  "freelancer_cancel_job"
  "enforce_deadline"
  "raise_dispute"
  "resolve_dispute"
  "get_job"
  "get_jobs_batch"
  "get_fee_bps"
  "update_fee"
  "withdraw_fees"
)

for func in "${FUNCTIONS[@]}"; do
  OUT=$($SOROBAN contract invoke \
    --wasm "$WASM" \
    --source-account GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF \
    --network standalone \
    --id CABFJAHJOLBPMGMGGQRIXHOPVNLFCLKIHQQCWLGXQCHKJDCHPDLUSADR \
    -- \
    "$func" 2>&1 || true)

  if echo "$OUT" | grep -q "read_bytes\|write_bytes\|cpu_insns\|fee"; then
    READ=$(echo "$OUT" | grep -o '"read_bytes":[0-9]*' | head -1 | cut -d: -f2)
    WRITE=$(echo "$OUT" | grep -o '"write_bytes":[0-9]*' | head -1 | cut -d: -f2)
    CPU=$(echo "$OUT" | grep -o '"cpu_insns":[0-9]*' | head -1 | cut -d: -f2)

    READ=${READ:-0}
    WRITE=${WRITE:-0}
    CPU=${CPU:-0}

    FEE=$(((READ * 6) + (WRITE * 10) + (CPU * 100) / 1000000))
    printf "  %-28s | %10s | %11s | %d\n" "$func" "$READ" "$WRITE" "$FEE"
  else
    printf "  %-28s | ${YELLOW}%10s${NC} | ${YELLOW}%11s${NC} | ${YELLOW}%s${NC}\n" "$func" "N/A" "N/A" "(dry-run)"
  fi
done

echo ""
echo "[3/3] Summary"
echo "  Contract WASM size: $WASM_SIZE"
echo "  Total functions:    ${#FUNCTIONS[@]}"
echo ""
echo "Note: Exact gas costs depend on Soroban network conditions."
echo "      These are approximate values from local simulation."
echo ""
echo "Done."
