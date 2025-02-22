//! `src/image.rs` - Handles Image Generation and Coloring

use crate::calc::calculate_grid;
use crate::inputs::{
    get_color_inputs_from_json_string, get_shape_inputs_from_json_string, ArtImageColorInputs,
};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// Use static for the global file name
static mut SIMPLE_ART_NAME: String = String::new();

pub fn color_grid(grid: &[Vec<f64>], color_inputs: &ArtImageColorInputs) -> Vec<Vec<[f64; 3]>> {
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
    // Extract the simple art name safely
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .and_then(|n| n.split('.').next())
        .ok_or_else(|| "Failed to extract file name".to_string())?;

    // Update the global name safely
    unsafe {
        SIMPLE_ART_NAME = file_name.to_string();
    }
    let file_content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;
    get_image_from_mandart_json_string(&file_content)
}

/// Generates an image grid from a `.mandart` JSON string.
pub fn get_image_from_mandart_json_string(
    mandart_json: &str,
) -> Result<Vec<Vec<[f64; 3]>>, String> {
    println!("get_image_from_mandart_json_string");
    let shape_inputs = get_shape_inputs_from_json_string(mandart_json)
        .map_err(|e| format!("Failed to extract shape inputs: {}", e))?;
    let color_inputs = get_color_inputs_from_json_string(mandart_json)
        .map_err(|e| format!("Failed to extract color inputs: {}", e))?;

    println!("Shape Inputs: {:?}", shape_inputs);
    println!("Color Inputs: {:?}", color_inputs);

    let grid = calculate_grid(&shape_inputs);
    let save_grid_path = unsafe { format!("output/{}.csv", SIMPLE_ART_NAME) };

    save_grid(&grid, &save_grid_path).map_err(|e| format!("Failed to save grid: {}", e))?;
    let image_grid = color_grid(&grid, &color_inputs);
    Ok(image_grid)
}

fn save_grid(grid: &[Vec<f64>], path: &str) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;

    for row in grid {
        let line = row
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(file, "{}", line).map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    Ok(())
}
