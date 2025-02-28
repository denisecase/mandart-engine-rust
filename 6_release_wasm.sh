#!/bin/bash

clear

# Ensure build & tests are successful before WASM release
./4_build_and_test.sh || { echo "❌ Build and tests failed. Exiting."; exit 1; }

echo "🚀 Cleaning old builds..."
cargo clean

echo "🔧 Building WASM release..."
cargo build -p mandart-wasm --release --features wasm || { echo "❌ WASM build failed. Exiting."; exit 1; }

# Change to the mandart-wasm directory before running wasm-pack
echo "📦 Packaging WASM with wasm-pack..."
cd mandart-wasm
wasm-pack build --target web --release --out-dir ../public/pkg --features wasm || { 
    echo "❌ WASM pack build failed. Exiting."
    cd ..
    exit 1
}
cd ..

echo "🎉 WASM release is ready!"
exit 0