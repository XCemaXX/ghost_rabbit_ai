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

# Flags match the wasm features rustc enables by default, plus simd128 from .cargo/config.toml.
if [[ "$profile" == release ]] && command -v wasm-opt >/dev/null; then
    wasm-opt -O3 --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext \
        --enable-mutable-globals --enable-reference-types --enable-multivalue --enable-simd \
        pkg/ghost_rabbit_ai.wasm -o pkg/ghost_rabbit_ai.wasm
fi
