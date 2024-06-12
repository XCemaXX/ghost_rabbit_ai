cargo build --target wasm32-unknown-unknown
cp ../target/wasm32-unknown-unknown/debug/ghost_rabbit_ai.wasm ./pkg/ghost_rabbit_ai.wasm

# cargo build --target wasm32-unknown-unknown --release
# cp ../target/wasm32-unknown-unknown/release/ghost_rabbit_ai.wasm ./pkg/ghost_rabbit_ai.wasm

# run
# python -m http.server 7102