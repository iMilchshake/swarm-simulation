#!/bin/bash
set -e

WASM_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$WASM_DIR/.." && pwd)"

echo "Building WASM..."
cargo build --manifest-path "$PROJECT_ROOT/Cargo.toml" --target wasm32-unknown-unknown --bin multi_swarm

echo "Copying files to $WASM_DIR..."
cp "$PROJECT_ROOT/target/wasm32-unknown-unknown/debug/multi_swarm.wasm" "$WASM_DIR/"
ln -sfn "$PROJECT_ROOT/assets" "$WASM_DIR/assets" # symbolic, force overwrite, treat destination as normal file

echo ""
echo "Ready! Starting server on http://localhost:8080"
echo "Press Ctrl+C to stop"
echo ""

python3 -m http.server 8080 --directory "$WASM_DIR"
