#!/usr/bin/env bash
# Usage: ./build.sh [--dev]. Output goes to ./pkg.
set -euo pipefail
cd "$(dirname "$0")"

profile=release
cargo_flags=(--release)
if [[ "${1:-}" == "--dev" ]]; then
    profile=debug
    cargo_flags=()
fi

cargo build -p ghost_rabbit_ai --target wasm32-unknown-unknown "${cargo_flags[@]}"
cp "../target/wasm32-unknown-unknown/$profile/ghost_rabbit_ai.wasm" pkg/ghost_rabbit_ai.wasm

# Flags match the wasm features rustc enables by default.
if [[ "$profile" == release ]] && command -v wasm-opt >/dev/null; then
    wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext \
        --enable-mutable-globals --enable-reference-types --enable-multivalue \
        pkg/ghost_rabbit_ai.wasm -o pkg/ghost_rabbit_ai.wasm
fi
