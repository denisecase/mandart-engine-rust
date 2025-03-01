//! `api_get_inputs_from_piddef_string.rs` - Extracts inputs from PicDef JSON

use wasm_bindgen::prelude::*;
use serde_json::Value;
use log::info;

use crate::api_types::{JsArtImageShapeInputs, JsArtImageColorInputs};

/// **WASM API: Extract inputs from a PictureDefinition JSON string**
///
/// This function parses a PictureDefinition JSON string and extracts 
/// shape and color inputs for Mandelbrot grid generation.
///
/// @param {string} piddef_json - The JSON string representing a PictureDefinition
/// @returns {[JsArtImageShapeInputs, JsArtImageColorInputs]} A tuple containing:
///   - Shape inputs for grid calculation
///   - Color inputs for grid coloring
///
/// @example
/// ```javascript
/// const [shapeInputs, colorInputs] = mandart.api_get_inputs_from_piddef_string(jsonString);
/// console.log(shapeInputs.image_width); // Access specific properties
/// ```
///
/// @remarks
/// - Provides default values if certain JSON fields are missing
/// - Extracts core parameters for Mandelbrot image generation
/// - Converts Swift-generated PictureDefinition JSON to WebAssembly-compatible inputs
#[wasm_bindgen]
pub fn api_get_inputs_from_picdef_string(piddef_json: String) -> JsValue {
    info!("🔍 WASM: Parsing PictureDefinition from JSON string");
    
    // Parse the JSON string 
    let parsed: Value = serde_json::from_str(&piddef_json).unwrap();

    // Create shape inputs with sensible defaults
    let shape_inputs = JsArtImageShapeInputs::new(
        parsed["imageWidth"].as_u64().unwrap_or(500) as u32,     // Default: 500px width
        parsed["imageHeight"].as_u64().unwrap_or(500) as u32,    // Default: 500px height
        parsed["iterationsMax"].as_f64().unwrap_or(1000.0),      // Default: 1000 max iterations
        parsed["scale"].as_f64().unwrap_or(1.0),                 // Default: 1.0 scale
        parsed["xCenter"].as_f64().unwrap_or(0.0),               // Default: Center X at 0
        parsed["yCenter"].as_f64().unwrap_or(0.0),               // Default: Center Y at 0
        parsed["theta"].as_f64().unwrap_or(0.0),                 // Default: No rotation
        parsed["rSqLimit"].as_f64().unwrap_or(4.0),              // Default: Standard escape radius
        parsed["mandPowerReal"].as_i64().unwrap_or(2) as i32,    // Default: Classic Mandelbrot (power 2)
        parsed["dFIterMin"].as_f64().unwrap_or(0.1),             // Default: Minimum iteration threshold
    );

    // Prepare hues and colors
    let hues_array = parsed["hues"].as_array().cloned().unwrap_or_default();
    
    // Extract colors (RGB values from hues)
    let colors: Vec<[f64; 3]> = hues_array.iter()
        .map(|hue| [
            hue["r"].as_f64().unwrap_or(0.0),
            hue["g"].as_f64().unwrap_or(0.0),
            hue["b"].as_f64().unwrap_or(0.0)
        ])
        .collect();

    // Extract hues with numbered index
    let hues: Vec<[f64; 4]> = hues_array.iter()
        .enumerate()
        .map(|(i, hue)| [
            (i + 1) as f64,  // num starts from 1
            hue["r"].as_f64().unwrap_or(0.0),
            hue["g"].as_f64().unwrap_or(0.0),
            hue["b"].as_f64().unwrap_or(0.0)
        ])
        .collect();

    // Create color inputs with sensible defaults
    let color_inputs = JsArtImageColorInputs {
        n_blocks: parsed["nBlocks"].as_u64().unwrap_or(10) as u32,  // Default: 10 color blocks
        n_colors: hues.len(),                                       // Number of hues
        spacing_color_far: parsed["spacingColorFar"].as_f64().unwrap_or(1.0),   // Default color spacing
        spacing_color_near: parsed["spacingColorNear"].as_f64().unwrap_or(1.0), // Default color spacing
        y_y_input: parsed["yY"].as_f64().unwrap_or(0.5),            // Default Y input
        mand_color: [                                               // Default Mandelbrot color
            parsed["mandColor"]["r"].as_f64().unwrap_or(0.0),
            parsed["mandColor"]["g"].as_f64().unwrap_or(0.0),
            parsed["mandColor"]["b"].as_f64().unwrap_or(0.0),
        ],
        colors,  // Extracted color values
        hues,    // Extracted hue values with index
    };

    // Convert to JS value
    serde_wasm_bindgen::to_value(&(shape_inputs, color_inputs)).unwrap()
}