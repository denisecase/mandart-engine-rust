//! `mandart-wasm/lib.rs` - WebAssembly Interface for MandArt Engine

use wasm_bindgen::prelude::*;
use log::info;

// Import all the API modules

mod api_get_grid_from_shape_inputs;
mod api_get_image_from_inputs;
mod api_get_inputs_from_picdef_string;
mod api_load_or_compute_default_grid;
mod api_types;

// Re-export the public API functions
pub use api_get_grid_from_shape_inputs::api_get_grid_from_shape_inputs;
pub use api_get_image_from_inputs::api_get_image_from_inputs;
pub use api_get_inputs_from_picdef_string::api_get_inputs_from_piddef_string;
pub use api_load_or_compute_default_grid::api_load_or_compute_default_grid;
pub use api_types::{JsArtImageColorInputs, JsArtImageShapeInputs};


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

/// Returns version information about the MandArt library
#[wasm_bindgen]
pub fn api_get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    
    format!("{} v{}", name, version)
}

// TypeScript type definitions for better JS integration
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface ArtImageShapeInputs {
    image_width: number;
    image_height: number;
    iterations_max: number;
    scale: number;
    x_center: number;
    y_center: number;
    theta: number;
    r_sq_limit: number;
    mand_power_real: number;
    d_f_iter_min: number;
}

export interface ColorPoint {
    position: number;
    color: string;  // Hex color
}

export interface Hue {
    id: string;
    name: string;
    color_points: ColorPoint[];
}

export interface ArtImageColorInputs {
    n_blocks: number;
    n_colors: number;
    spacing_color_far: number;
    spacing_color_near: number;
    y_y_input: number;
    mand_color: Float64Array;
    // Note: colors and hues are accessed via methods
}

export interface ColoredGrid {
    grid: number[][][];  // 3D array: [x][y][rgb]
    width: number;
    height: number;
}

export interface ImageData {
    data: Uint8Array;  // RGBA data
    width: number;
    height: number;
}

export interface CombinedInputs {
    shape_inputs: ArtImageShapeInputs;
    color_inputs: ArtImageColorInputs;
}

export interface PictureDefinition {
    id: string;
    name: string;
    createdDate: string;
    modifiedDate: string;
    imageWidth: number;
    imageHeight: number;
    iterationsMax: number;
    scale: number;
    xCenter: number;
    yCenter: number;
    theta: number;
    rSqLimit: number;
    mandPowerReal: number;
    hues: Array<{
        id: string;
        name: string;
        num: number;
        r: number;
        g: number;
        b: number;
    }>;
    mandColor: {
        r: number;
        g: number;
        b: number;
    };
}
"#;