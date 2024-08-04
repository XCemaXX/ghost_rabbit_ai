# Ghost Rabbit
[Ghost Rabbit](https://xcemaxx.github.io/ghost_rabbit_ai) is a 2D game like doodle jump. Used WASM and macroquad.

There is legacy C# XNA version: https://github.com/XCemaXX/ghost_rabbit_ai/tree/master/legacy_version

Check runnable version online: [Ghost Rabbit](https://xcemaxx.github.io/ghost_rabbit_ai)

# Building and running
- Go to gui folder of the project: `cd gui`
- Build with: `cargo build --target wasm32-unknown-unknown` or `cargo build --target wasm32-unknown-unknown --release`
- Copy file: ` cp ../target/wasm32-unknown-unknown/debug/ghost_rabbit_ai.wasm ./pkg/ghost_rabbit_ai.wasm` or release
- Run server with: `python -m http.server 7000`
- Open site with: http://localhost:7000

# Show
[Check by yourself online](https://xcemaxx.github.io/ghost_rabbit_ai)  
<img src="./docs/ghost_rabbit_gameplay.gif" width="300" height="457" />

# AI
There was an attempt to make AI to play in this game. It failed.
You can find sources of a genetic algorithm and neural network in libs folder.