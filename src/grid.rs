use serde_json;
use std::fs;
use crate::inputs::{get_shape_inputs, ArtImageShapeInputs};
use crate::calc::calculate_grid;

pub fn get_grid_from_shape_inputs(file_path: &str) -> Result<Vec<Vec<f64>>, String> {
    let shape_inputs = get_shape_inputs(file_path)
        .map_err(|e| format!("Error extracting shape inputs: {}", e))?;
    let grid = calculate_grid(&shape_inputs); 
    Ok(grid) 
}


/// Computes the Mandelbrot grid from a `.mandart` JSON string.
pub fn get_grid_from_mandart_json_string(json_str: &str) -> Result<Vec<Vec<f64>>, String> {
    let shape_inputs: ArtImageShapeInputs = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse Mandart JSON: {}", e))?;
    let grid = calculate_grid(&shape_inputs); 
    Ok(grid) 
}


/// Computes the Mandelbrot grid from a `.mandart` file.
pub fn get_grid_from_mandart_file(file_path: &str) -> Result<Vec<Vec<f64>>, String> {
    let file_content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;
    get_grid_from_mandart_json_string(&file_content) 
}
