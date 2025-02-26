#!/bin/bash

set -e  # Exit script immediately if a command fails

echo "🔍 Checking if build is up to date..."
if ! cargo build -p mandart-cli --bin main --quiet 2>/dev/null; then
    echo "🔨 Building mandart-cli/main.rs..."
    cargo build -p mandart-cli --bin main
else
    echo "✅ Build is already up to date."
fi

echo "🏁 Running mandart-cli/main.rs..."
cargo run -p mandart-cli --bin main

echo "🎉 Done!"
exit 0
