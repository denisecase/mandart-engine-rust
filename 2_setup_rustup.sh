#!/bin/bash

echo "🚀 Installing Rustup..."

# Download and install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source Rustup environment
export PATH="$HOME/.cargo/bin:$PATH"
source "$HOME/.cargo/env"

# Verify Rust installation
if ! command -v rustc &>/dev/null; then
    echo "❌ Rust installation failed!"
    exit 1
fi

echo "✅ Rustup installed successfully!"

# Set default Rust version
rustup default stable

# Add necessary targets
echo "🛠 Adding required targets..."
rustup target add wasm32-unknown-unknown
rustup target add x86_64-pc-windows-msvc

# Reinstall global cargo tools
echo "📦 Installing essential Rust tools..."
cargo install cargo-outdated wasm-bindgen-cli wasm-pack cargo-audit

# Verify installation
echo "🔎 Verifying Rust setup..."
rustc --version
cargo --version
rustup show

echo "🎉 Rustup setup complete!"
exit 0
