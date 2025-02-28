# mandart-engine-rust

- [MandArt](https://github.com/denisecase/MandArt) - A SwiftUI App
- [MandArt Web](https://github.com/denisecase/mandart-web) - a web app front end (under construction)
- [MandArt Discoveries](https://github.com/denisecase/MandArt-Discoveries) - view discovered MandArt

## Install Rustup

```zsh
chmod +x *.sh
./1_remove_old_rust.sh
./2_setup_rustup.sh
./3_install_deps.sh
./4_build_and_test.sh
./5_run.sh
./6_release_wasm.sh
```

Install VS Code Extension: rust-analyzer


## Modules

- mandart-cli 
- mandart-core
- mandart-wasm


## Interop for Mandelbrot Grid Calculations  

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
Your colors are stored as f64 values ([f64; 3]).
Many PNG libraries expect u8 values ([u8; 3]), so we must scale the colors correctly.
