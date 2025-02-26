#!/bin/bash
clear

echo "🚀 Cleaning project..."
cargo clean

echo "🚀 Building project..."
cargo build

echo "✅ Running tests..."
cargo test --all-features

echo "🎉 Build and test complete!"
exit 0
