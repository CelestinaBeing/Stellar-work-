#!/bin/bash
set -e

# Navigate to the contract directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/escrow"

echo "=================================================="
echo "Running Rust Contract Test Coverage (Tarpaulin)"
echo "=================================================="

# Check if cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "cargo-tarpaulin is not installed. Installing it now..."
    echo "This may take a few minutes..."
    cargo install cargo-tarpaulin
fi

# Create coverage output directory
mkdir -p "../../coverage"

# Run tarpaulin
# --out Lcov: Generates lcov.info for CI integrations (Codecov/Coveralls)
# --out Html: Generates an interactive HTML report for local viewing
# --output-dir: Specifies where to save the reports
# --fail-under 80: Enforces a minimum 80% coverage threshold
cargo tarpaulin \
    --out Lcov \
    --out Html \
    --output-dir ../../coverage \
    --fail-under 80 \
    --verbose

echo "=================================================="
echo "Coverage reports generated successfully:"
echo "  - LCOV: coverage/lcov.info"
echo "  - HTML: coverage/tarpaulin-report.html"
echo "=================================================="
