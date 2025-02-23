# mandart-engine-rust

## Install Cargo with brew

```
brew install cargo
brew install rust
```

Alternatively (not tested) install Rustup (official) 

```zsh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

Install VS Code Extension: rust-analyzer

After installation, restart your terminal and verify:

```zsh
rustc --version
cargo --version
```

Get tools and keep them updated.

```zsh
cargo install cargo-outdated    # Check outdated dependencies
cargo install wasm-bindgen-cli  # Required for WebAssembly
cargo install wasm-pack         # Pack WebAssembly projects
cargo install cargo-audit

brew install libjpeg libpng     # Required for the `image` crate
brew install imagemagick        # Extra image format support

cargo add libc --optional       # Optional for FFI compatibility
cargo add toml                  # For parsing TOML files
cargo add base64                # For encoding/decoding
cargo add csv                   # For working with CSV files

cargo outdated                  # Check for outdated dependencies
cargo audit                     # Check for security vulnerabilities
cargo tree                      # Verify dependency tree

cargo test --features wasm       # Run all tests with WASM feature
cargo clippy --all-features      # Lint the project for issues

cargo check --features wasm      # Verify WASM build works
cargo check --target x86_64-pc-windows-msvc  # Ensure Win compat (may need Rustup)

cargo build --features wasm                             # Standard build for Rust/WASM
cargo build --target wasm32-unknown-unknown --features wasm  # WASM-specific build

```

## Development

Terminal 1: Build and Test

```zsh
clear
cargo clean
cargo build
cargo test
```

Terminal 2: Debugging Checks

```zsh
cargo run --bin check
```

## Prep For Release

Build and optimize for relase (ensures optimized performance with WebAssembly (WASM) enabled).
Verify first with the non release and then optimize with --release
Copy the output .wasm file to .wasm.txt for web app use.

```zsh
cargo clean
cargo build --release --features wasm
wasm-pack build --target web --out-dir public/pkg --features wasm
wasm-pack build --target web --release --out-dir public/pkg --features wasm

ls -lh public/pkg/*.wasm
cp public/pkg/*.wasm public/pkg/mandart-engine-rust.wasm.txt
```

## Core Rust Library (src/)

- api.rs - Implements all API functions (get_grid_*, get_image_*, etc.).
- config.rs - Reads and manages constants
- file_io.rs - Handles loading/saving CSV, JSON, PNG.
- grid.rs - Handles grid computation (Mandelbrot calculations).
- image.rs - Handles image coloring and saving (PNG).
- lib.rs - Exposes the public API for Rust, WASM, and Swift.
- main.rs - CLI test app.
- utils.rs - General helper functions.

## Folders

- input - .mandart files and their associated MandArt-generated .png files (for testing)
- input_swift - csv grids output from swift MandArt (for testing)
- output - put generated files here

Note: these folders are temporary and used for testing. They might not be distributed, although a catalog of existing .mandart files and their .png thumbnails are valuable in the host web app or MandArt SwiftUI app, so maybe?

## Types

Grid Data (Calculated from Mandelbrot)
- Type: Vec<Vec<f64>>
- Why? Each pixel in the grid holds a floating-point iteration count.
- Used in: calculate_grid(), save_grid_to_csv(), get_grid_from_mandart_json_string(), etc.

Color Data (RGB Values)
- Type: Vec<[f64; 3]>
- Why? Each color is an RGB triplet (matching Swift's [Double] arrays).
- Used in: color_grid(), ArtImageColorInputs, etc.

ImageGrid Representation of the Image
- Type: Vec<Vec<[f64; 3]>>
- Why? Preserves precision and keeps it easy for clients
- Used in: get_image...functions

### How Clients Can Handle It  

| Platform | Best Conversion |
|-------------|----------------------|
| Swift | Convert `[[Double]]` to `UIImage` or `CGImage` |
| WASM | Convert `[[f32; 3]]` to an `ImageData` buffer |
| Python | Convert to NumPy `np.array([...], dtype=np.float64)` |

## Clean and Build WASM

```zsh
cargo clean
cargo build --release --features wasm
wasm-pack build --target web --out-dir public/pkg --features wasm
```
