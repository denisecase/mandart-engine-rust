//! `api.rs` - API functions for MandArt Engine

use wasm_bindgen::prelude::*;
use serde_json;
use std::collections::HashMap;
use crate::config::load_config;
use crate::grid:: get_grid_from_mandart_json_string;
use crate::image::{color_grid, get_image_from_mandart_json_string, get_image_from_mandart_file};
use crate::inputs::ArtImageColorInputs;

/// Define standardized image representation
pub type ImageGrid = Vec<Vec<[f64; 3]>>;

#[cfg(feature = "wasm")]
pub fn setup_logging() {
    use log::Level;
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    
    INIT.call_once(|| {
        console_log::init_with_level(Level::Debug).expect("Failed to initialize console_log.");
        console_error_panic_hook::set_once();
    });
}

/// ========================
/// WASM FUNCTIONS
/// ========================

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn api_calc_grid_js(json_str: &str) -> Result<JsValue, JsValue> {
    setup_logging();
    match get_grid_from_mandart_json_string(json_str) {
        Ok(grid) => serde_wasm_bindgen::to_value(&grid)
            .map_err(|e| JsValue::from_str(&e.to_string())),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn api_color_grid_js(grid_csv: &str, hues_json: &str) -> Result<JsValue, JsValue> {
    setup_logging();

    // Deserialize CSV into a grid
    let mut rdr = csv::Reader::from_reader(grid_csv.as_bytes());
    let grid: Vec<Vec<f64>> = rdr
        .deserialize()
        .map(|result| result.map_err(|e| JsValue::from_str(&format!("❌ CSV parsing error: {}", e))))
        .collect::<Result<_, _>>()?;

    // Deserialize JSON into color inputs
    let color_inputs: ArtImageColorInputs = serde_json::from_str(hues_json)
        .map_err(|e| JsValue::from_str(&format!("❌ Color Inputs JSON parsing error: {}", e)))?;

    // Apply coloring
    let colored_grid = color_grid(&grid, &color_inputs);

    // Convert processed grid back to CSV
    let mut wtr = csv::Writer::from_writer(vec![]);
    for row in &colored_grid {
        if let Err(e) = wtr.serialize(row) {
            return Err(JsValue::from_str(&format!("❌ CSV serialization error: {}", e)));
        }
    }
    let csv_output = String::from_utf8(wtr.into_inner().unwrap())
        .map_err(|e| JsValue::from_str(&format!("❌ CSV conversion error: {}", e)))?;

    Ok(JsValue::from_str(&csv_output))
}

/// ========================
/// GRID COMPUTATION FUNCTIONS
/// ========================

pub use crate::grid::get_grid_from_mandart_json_string as api_get_grid_from_mandart_json_string;

/// ========================
/// IMAGE COMPUTATION FUNCTIONS
/// ========================

pub fn api_compute_image_from_mandart_file(file_path: &str) -> Result<ImageGrid, String> {
    get_image_from_mandart_file(file_path)
}

pub fn api_compute_image_from_mandart_json(json_str: &str) -> Result<ImageGrid, String> {
    get_image_from_mandart_json_string(json_str)
}

/// ========================
/// CONFIGURATION LOADING
/// ========================

pub fn api_load_config(config_path: Option<&str>) -> HashMap<String, String> {
    load_config(config_path)
}
