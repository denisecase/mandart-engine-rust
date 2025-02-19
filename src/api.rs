//! `api.rs` - API functions for MandArt Engine
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;


use crate::config::load_config;
use crate::file_io::{save_grid_to_csv, save_image_to_bmp, save_image_to_png};
use crate::grid::{
    get_grid_from_mandart_file, get_grid_from_mandart_json_string, get_grid_from_shape_inputs,
};
use crate::image::{get_image_from_mandart_file, get_image_from_mandart_json_string};
use serde_json;
use std::collections::HashMap;

pub type ImageGrid = Vec<Vec<[f64; 3]>>; // Standardized Image Representation

/// WASM FUNCTIONS ..............................
/// These functions are used to expose the API to JavaScript.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn api_get_image_from_mandart_file_js(file_path: &str) -> Result<JsValue, JsValue> {
    match get_image_from_mandart_file(file_path) {
        Ok(image_grid) => {
            serde_wasm_bindgen::to_value(&image_grid)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Err(e) => Err(JsValue::from_str(&e)),
    }
}



/// GET GRID ..............................

/// Computes a grid from shape inputs JSON string.
pub fn api_get_grid_from_shape_inputs_json_string(shape_json: &str) -> Result<Vec<Vec<f64>>, String> {
    get_grid_from_shape_inputs(shape_json)
}

/// Computes a grid from a `.mandart` JSON string.
pub fn api_get_grid_from_mandart_json_string(mandart_json: &str) -> Result<Vec<Vec<f64>>, String> {
    get_grid_from_mandart_json_string(mandart_json)
}

/// Computes a grid from a `.mandart` file.
pub fn api_get_grid_from_mandart_file(file_path: &str) -> Result<Vec<Vec<f64>>, String> {
    get_grid_from_mandart_file(file_path)
}


/// GET IMAGE ..............................

/// Computes an image grid from a `.mandart` file.
pub fn api_get_image_from_mandart_file(file_path: &str) -> Result<ImageGrid, String> {
    get_image_from_mandart_file(file_path)
}

/// Computes an image grid from a `.mandart` JSON string.
pub fn api_get_image_from_mandart_json_string(json_str: &str) -> Result<ImageGrid, String> {
    get_image_from_mandart_json_string(json_str)
}

/// FILE IO ..............................

/// Saves a grid to a CSV file.
pub fn api_save_grid_to_csv(grid_json: &str, file_path: &str) -> Result<(), String> {
    let grid: Vec<Vec<f64>> =
        serde_json::from_str(grid_json).map_err(|e| format!("Failed to parse grid JSON: {}", e))?;

    save_grid_to_csv(&grid, file_path)
}

/// Saves an **image grid** to a BMP file.
pub fn api_save_image_to_bmp(image_grid: &ImageGrid, file_path: &str) -> Result<(), String> {
    save_image_to_bmp(image_grid, file_path)
}

/// Saves an **image grid** to a PNG file.
pub fn api_save_image_to_png(image_grid: &ImageGrid, file_path: &str) -> Result<(), String> {
    save_image_to_png(image_grid, file_path)
}

/// Loads the configuration settings.
pub fn api_load_config(config_path: Option<&str>) -> HashMap<String, String> {
    load_config(config_path)
}
