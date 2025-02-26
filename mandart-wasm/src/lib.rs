//! `lib.rs` - WebAssembly Interface for MandArt Engine

use wasm_bindgen::prelude::*;
use log::info;

// Import all the API modules
mod api_generate_png;
mod api_get_colored_grid;
mod api_load_or_compute_default_grid;
mod api_types;


// Re-export the public API functions
pub use api_generate_png::api_generate_png;
pub use api_get_colored_grid::api_get_colored_grid;
pub use api_load_or_compute_default_grid::api_load_or_compute_default_grid;
pub use api_types::JsArtImageColorInputs;

// WASM initialization function
#[wasm_bindgen(start)]
pub fn init() {
    // Set up panic hook for better error messages
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    // Initialize logging if the feature is enabled
    #[cfg(feature = "console_log")]
    console_log::init_with_level(log::Level::Info)
        .expect("Failed to initialize logger");
    
    info!("MandArt WASM module initialized");
}

// Additional API functions could go here
// For example, a function to get version info

/// Returns version information about the MandArt library
#[wasm_bindgen]
pub fn api_get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    
    format!("{} v{}", name, version)
}