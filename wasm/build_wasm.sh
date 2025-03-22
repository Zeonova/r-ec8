#!/bin/bash

# Run wasm-pack build
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web

# Move the output files
mv pkg/wasm.js ../web
mv pkg/wasm_bg.wasm ../web

echo "Build and move completed successfully!"
