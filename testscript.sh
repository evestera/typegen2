#!/usr/bin/env bash

(cd typegen_wasm; wasm-pack build --release)

echo
echo "Wasm file size (in bytes):"
wc -c typegen_wasm/pkg/typegen_wasm_bg.wasm

# echo
# cargo test --all
