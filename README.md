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

# Finalized Interop for Mandelbrot Grid Calculations  

This document defines a cross-language interop structure for Swift, Rust, and JavaScript to compute and color Mandelbrot sets efficiently.  

## Basic Number Types  

### Integer Types  
- Swift: Int32  
- Rust: i32  
- JavaScript: number (64-bit float, but used as an integer)  

### Floating-Point Types  
- Swift: Double (8 bytes, 64-bit)  
- Rust: f64  
- JavaScript: Float64Array  

## Calc Grid Input Structure  

### Swift  

```
struct ArtImageShapeInputs {  
  let imageHeight: Int  
  let imageWidth: Int  
  let iterationsMax: Double  
  let scale: Double  
  let xCenter: Double  
  let yCenter: Double  
  let theta: Double  
  let dFIterMin: Double  
  let rSqLimit: Double  
  let mandPowerReal: Int  
}  
```


### Rust  

```
struct ArtImageShapeInputs {  
    image_height: i32,  
    image_width: i32,  
    iterations_max: f64,  
    scale: f64,  
    x_center: f64,  
    y_center: f64,  
    theta: f64,  
    d_f_iter_min: f64,  
    r_sq_limit: f64,  
    mand_power_real: i32,  
}  
```

### JavaScript  

```
const artImageShapeInputs = {  
    imageHeight: 800,  
    imageWidth: 600,  
    iterationsMax: 1000.0,  
    scale: 1.5,  
    xCenter: -0.5,  
    yCenter: 0.0,  
    theta: 0.0,  
    dFIterMin: 0.1,  
    rSqLimit: 4.0,  
    mandPowerReal: 2  
};  

```

## Color Grid Input Structure  

### Swift  

```
struct ArtImageColorInputs {  
  let nBlocks: Int  
  let nColors: Int  
  let spacingColorFar: Double  
  let spacingColorNear: Double  
  let yY_input: Double  
  let mandColor: Double  
}  
```

### Rust  

```
struct ArtImageColorInputs {  
    n_blocks: i32,  
    n_colors: i32,  
    spacing_color_far: f64,  
    spacing_color_near: f64,  
    y_y_input: f64,  
    mand_color: f64  
}  

```

### JavaScript  
```
const artImageColorInputs = {  
    nBlocks: 10,  
    nColors: 256,  
    spacingColorFar: 1.0,  
    spacingColorNear: 0.1,  
    yY_input: 0.5,  
    mandColor: 240.0  
};  
```

## Hues (List of Ordered Colors)  

### Swift  
```
final class Hue: Identifiable, Codable, Equatable {  
  var id: UUID  
  var num: Int  
  var r: Double  
  var g: Double  
  var b: Double  
}  
```

### Rust  
```
struct Hue {  
    id: uuid::Uuid,  
    num: i32,  
    r: f64,  
    g: f64,  
    b: f64  
}  
```

### JavaScript  

```
class Hue {  
    constructor(id, num, r, g, b) {  
        this.id = id;  
        this.num = num;  
        this.r = r;  
        this.g = g;  
        this.b = b;  
    }  
}
```  

## I/O for Calc Grid & Color Grid  

### Calc Grid Input (Basic Structure)  
- Swift: shapeInputs: ArtImageShapeInputs  
- Rust: ArtImageShapeInputs struct  
- JavaScript: Object { imageHeight, imageWidth, iterationsMax, scale, xCenter, yCenter, theta, dFIterMin, rSqLimit }  

### Calc Grid Output (Calculated from Mandelbrot)  
- Swift: [[Double]] (2D array of iteration values)  
- Rust: Vec<Vec<f64>>  
- JavaScript: Float64Array[]  
- Holds a floating-point iteration count.  

### Color Grid Input  
- Swift: [[Double]] (2D array of iteration values)  
- Rust: Vec<Vec<f64>>  
- JavaScript: Float64Array[]  
- Holds a floating-point iteration count.  

### Color Grid Output (2D col, row + RGB Hex)  
- Swift: [[String]] ("#RRGGBB")  
- Rust: Vec<Vec<String>> ("#RRGGBB")  
- JavaScript: Array<Array<string>> ("#RRGGBB")  

## How Clients Can Handle It  

| Platform  | Best Conversion |  
|-----------|----------------|  
| Swift     | Convert [[Double]] to UIImage or CGImage |  
| WASM      | Convert [[f64; 3]] to an ImageData buffer |  
| Python    | Convert to NumPy np.array([...], dtype=np.float64) |  

## Clean and Build WASM

```zsh
cargo clean
cargo build --release --features wasm
wasm-pack build --target web --out-dir public/pkg --features wasm
```


