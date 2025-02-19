# mandart-engine-rust

Terminal 1:

```zsh
clear
cargo clean
cargo build
cargo tests
```

Terminal 2:

```zsh
cargo run --bin check
```

Check for outdated

```zsh
cargo install cargo-outdated
cargo install wasm-bindgen
cargo add libc --optional
cargo add toml
cargo add base64

cargo outdated
cargo audit
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
