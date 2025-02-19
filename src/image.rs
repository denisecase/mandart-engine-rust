//! `image.rs` - Handles Image Generation and Coloring

use crate::calc::calculate_grid;
use crate::inputs::{get_shape_inputs_from_json_string, get_color_inputs_from_json_string, ArtImageColorInputs};
use std::fs;

fn color_grid(grid: &[Vec<f64>], color_inputs: &ArtImageColorInputs) -> Vec<Vec<[f64; 3]>> {
    let img_width = grid[0].len();
    let img_height = grid.len();
    let mut img_data = vec![vec![[0.0_f64, 0.0_f64, 0.0_f64]; img_width]; img_height];

    for v in 0..img_height {
        for u in 0..img_width {
            let index = (grid[v][u] as usize) % color_inputs.colors.len();
            let color = color_inputs.colors[index];
            img_data[v][u] = [color[0], color[1], color[2]]; 
        }
    }

    img_data 
}


/// Reads a `.mandart` file and generates an image grid.
pub fn get_image_from_mandart_file(file_path: &str) -> Result<Vec<Vec<[f64; 3]>>, String> {
    println!("get_image_from_mandart_file");
    let file_content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;
    get_image_from_mandart_json_string(&file_content)
}

/// Generates an image grid from a `.mandart` JSON string.
pub fn get_image_from_mandart_json_string(mandart_json: &str) -> Result<Vec<Vec<[f64; 3]>>, String> {
    println!("get_image_from_mandart_json_string");
    let shape_inputs = get_shape_inputs_from_json_string(mandart_json)
        .map_err(|e| format!("Failed to extract shape inputs: {}", e))?;
    let color_inputs = get_color_inputs_from_json_string(mandart_json)
        .map_err(|e| format!("Failed to extract color inputs: {}", e))?;

    println!("Shape Inputs: {:?}", shape_inputs);
    println!("Color Inputs: {:?}", color_inputs);

    let grid = calculate_grid(&shape_inputs);
    let image_grid = color_grid(&grid, &color_inputs);

    Ok(image_grid)
}

