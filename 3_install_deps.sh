#!/bin/bash

echo "🚀 Installing all necessary dependencies for MandArt Engine..."

# --- Install Rust tools globally ---
echo "📦 Installing Rust tools..."
cargo install cargo-outdated wasm-bindgen-cli wasm-pack cargo-audit

# --- Install system dependencies ---
echo "🖼️ Installing image processing libraries..."
brew install libjpeg libpng imagemagick

# --- Add required Rust dependencies ---
echo "📦 Adding core project dependencies..."
cargo add serde --features derive   # Serialization/deserialization
cargo add serde_json                # JSON handling
cargo add toml                       # TOML config parsing
cargo add base64                     # Encoding/decoding
cargo add csv                        # Working with CSV files
cargo add image                      # Image processing (PNG/BMP)
cargo add log env_logger             # Logging
cargo add tempfile                    # Temporary file handling

# --- WASM-specific dependencies ---
echo "🌐 Adding WASM dependencies..."
cargo add wasm-bindgen --features serde-serialize --optional
cargo add serde-wasm-bindgen --optional
cargo add web-sys --features console
cargo add console_error_panic_hook --optional
cargo add console_log --optional

# --- Ensure Rust is set up ---
echo "🔎 Verifying Rust & Cargo installation..."
rustc --version
cargo --version
cargo tree

echo "✅ All dependencies installed successfully!"
exit 0
