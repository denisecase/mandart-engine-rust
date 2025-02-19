//! `api.rs` - API functions for MandArt Engine

#[cfg(feature = "wasm")]
extern crate wasm_bindgen;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
use serde_json;
use std::collections::HashMap;

use crate::config::load_config;
use crate::file_io::{save_grid_to_csv, save_image_to_bmp, save_image_to_png};
use crate::grid::{
    get_grid_from_mandart_file, get_grid_from_mandart_json_string, get_grid_from_shape_inputs,
};
use crate::image::{get_image_from_mandart_file, get_image_from_mandart_json_string, color_grid};
use crate::inputs::ArtImageColorInputs;

/// Define standardized image representation
pub type ImageGrid = Vec<Vec<[f64; 3]>>;

/// ========================
/// WASM FUNCTIONS
/// ========================

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn api_get_image_from_mandart_file_js(file_path: &str, color_inputs: JsValue) -> Result<JsValue, JsValue> {
    match get_grid_from_mandart_file(file_path) {
        Ok(grid) => {
            let width = grid.len();
            let height = grid.get(0).map_or(0, |row| row.len());

            web_sys::console::log_1(&format!("Rust: Grid Size = [{}][{}]", width, height).into());

            let color_inputs: ArtImageColorInputs = serde_wasm_bindgen::from_value(color_inputs)
                .map_err(|e| JsValue::from_str(&format!("Invalid color inputs: {}", e)))?;

            // ✅ Convert grid to colored image
            let colorized_grid = color_grid(&grid, &color_inputs);
            web_sys::console::log_1(&format!("Rust: Colorized Grid Size = [{}][{}]", colorized_grid.len(), colorized_grid[0].len()).into());

            // ✅ Debug: Log first 10 values
            if colorized_grid.len() > 0 {
                web_sys::console::log_1(&format!("Rust: First Row Sample: {:?}", &colorized_grid[0][..10]).into());
            }

            // ✅ Flatten grid
            let mut image_data = Vec::with_capacity(width * height * 4);
            for row in colorized_grid.iter() {
                for &pixel in row.iter() {
                    image_data.push((pixel[0] * 255.0) as u8); // Red
                    image_data.push((pixel[1] * 255.0) as u8); // Green
                    image_data.push((pixel[2] * 255.0) as u8); // Blue
                    image_data.push(255);                     // Alpha (fully opaque)
                }
            }

            web_sys::console::log_1(&format!("Rust: Final ImageData Length = {}", image_data.len()).into());

            serde_wasm_bindgen::to_value(&image_data)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Err(e) => Err(JsValue::from_str(&e)),
    }
}



#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn api_get_image_from_mandart_json_string_js(json_str: &str) -> Result<JsValue, JsValue> {
    match get_image_from_mandart_json_string(json_str) {
        Ok(image_grid) => serde_wasm_bindgen::to_value(&image_grid)
            .map_err(|e| JsValue::from_str(&e.to_string())),
        Err(e) => Err(JsValue::from_str(&e)),
    }
}

/// ========================
/// GRID COMPUTATION FUNCTIONS
/// ========================

pub fn api_get_grid_from_shape_inputs_json_string(
    shape_json: &str,
) -> Result<Vec<Vec<f64>>, String> {
    get_grid_from_shape_inputs(shape_json)
}

pub fn api_get_grid_from_mandart_json_string(
    mandart_json: &str,
) -> Result<Vec<Vec<f64>>, String> {
    get_grid_from_mandart_json_string(mandart_json)
}

pub fn api_get_grid_from_mandart_file(file_path: &str) -> Result<Vec<Vec<f64>>, String> {
    get_grid_from_mandart_file(file_path)
}

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
/// FILE I/O FUNCTIONS
/// ========================

pub fn api_save_grid_to_csv(grid_json: &str, file_path: &str) -> Result<(), String> {
    let grid: Vec<Vec<f64>> =
        serde_json::from_str(grid_json).map_err(|e| format!("Failed to parse grid JSON: {}", e))?;
    save_grid_to_csv(&grid, file_path)
}

pub fn api_save_image_to_bmp(image_grid: &ImageGrid, file_path: &str) -> Result<(), String> {
    save_image_to_bmp(image_grid, file_path)
}

pub fn api_save_image_to_png(image_grid: &ImageGrid, file_path: &str) -> Result<(), String> {
    save_image_to_png(image_grid, file_path)
}

/// ========================
/// CONFIGURATION LOADING
/// ========================

pub fn api_load_config(config_path: Option<&str>) -> HashMap<String, String> {
    load_config(config_path)
}
