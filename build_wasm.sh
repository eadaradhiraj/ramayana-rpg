#!/usr/bin/env bash
set -e

echo "Building release WASM target (wasm32-unknown-unknown)..."
cargo build --target wasm32-unknown-unknown --release

mkdir -p dist
cp target/wasm32-unknown-unknown/release/ramayana_rpg.wasm dist/
MQ_JS=$(find ~/.cargo/registry/src/ -name "mq_js_bundle.js" | head -n 1)
if [ -n "$MQ_JS" ]; then
    cp "$MQ_JS" dist/
    # Patch undeclared register_plugin in strict mode
    sed -i 's/}(),function(){function i(){}/}(),function(){var register_plugin;function i(){}/g' dist/mq_js_bundle.js
fi

WASM_SIZE=$(ls -lh dist/ramayana_rpg.wasm | awk '{print $5}')
echo "WASM build successfully generated in ./dist (Size: $WASM_SIZE)"
