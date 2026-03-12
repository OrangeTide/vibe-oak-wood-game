#!/bin/bash
set -e

# Ensure WASM target is installed
rustup target add wasm32-unknown-unknown 2>/dev/null || true

# Build
cargo build --release --target wasm32-unknown-unknown

# Prepare dist
mkdir -p dist
cp target/wasm32-unknown-unknown/release/oak_woods_game.wasm dist/
cp web/index.html dist/

# Download macroquad JS bundle if not present
if [ ! -f dist/mq_js_bundle.js ]; then
    echo "Downloading macroquad JS bundle..."
    curl -sSL https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js -o dist/mq_js_bundle.js
fi

# Copy assets
cp -r public/assets dist/

echo ""
echo "Build complete! To run:"
echo "  cd dist && python3 -m http.server 8080"
echo "  Then open http://localhost:8080"
