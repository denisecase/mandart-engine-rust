#!/bin/bash

echo "🔍 Checking for existing Rust installations..."

# Common Rust locations
RUST_LOCATIONS=(
    "/usr/local/bin/rustc"
    "/opt/homebrew/bin/rustc"
    "$HOME/.cargo/bin/rustc"
)

# Flag to track if Rust was found
FOUND_RUST=false

# Check for Rust installations
for loc in "${RUST_LOCATIONS[@]}"; do
    if [ -x "$loc" ]; then
        echo "⚠️ Rust found at: $loc"
        FOUND_RUST=true
    fi
done

# Check if Rust is installed via Homebrew
if brew list rust &>/dev/null; then
    echo "⚠️ Rust found via Homebrew!"
    FOUND_RUST=true
fi

# If Rust is not found, exit safely
if [ "$FOUND_RUST" = false ]; then
    echo "✅ No existing Rust installation found. Safe to proceed!"
    exit 0
fi

# Uninstall Rust via Homebrew (if installed)
if brew list rust &>/dev/null; then
    echo "🛑 Removing Rust installed via Homebrew..."
    brew uninstall rust cargo
fi

# Remove Rust manually from common locations
echo "🛑 Removing Rust from common locations..."
rm -rf $HOME/.cargo $HOME/.rustup
sudo rm -f /usr/local/bin/rustc /usr/local/bin/cargo
sudo rm -f /opt/homebrew/bin/rustc /opt/homebrew/bin/cargo

# Verify Rust is gone
if command -v rustc &>/dev/null; then
    echo "❌ Rust removal failed. Please remove manually."
    exit 1
else
    echo "✅ Rust successfully removed! Ready for Rustup installation."
    exit 0
fi
