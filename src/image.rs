//! `src/image.rs` - Handles Image Generation and Coloring

use crate::calc::calculate_grid;
use crate::inputs::{
    get_color_inputs_from_json_string, get_shape_inputs_from_json_string, ArtImageColorInputs,
};
use std::fs;
use log::info;

/// Colors the grid based on `ArtImageColorInputs`.
pub fn color_grid(grid: &Vec<Vec<f64>>, color_inputs: &ArtImageColorInputs) -> Vec<Vec<[f64; 3]>> {
    let hues = &color_inputs.colors;
    let img_width = grid[0].len();
    let img_height = grid.len();
    let mut img_data = vec![vec![[0.0_f64, 0.0_f64, 0.0_f64]; img_width]; img_height];

    for v in 0..img_height {
        for u in 0..img_width {
            let index = (grid[v][u] as usize) % hues.len();
            img_data[v][u] = hues[index];
        }
    }

    img_data
}

/// Reads a `.mandart` file and generates an image grid.
/// **Does NOT save files** — saving is handled in `main.rs`.
pub fn get_image_from_mandart_file(file_path: &str) -> Result<Vec<Vec<[f64; 3]>>, String> {
    info!("📂 Loading MandArt from file: {}", file_path);

    let file_content = fs::read_to_string(file_path)
        .map_err(|e| format!("❌ Failed to read file {}: {}", file_path, e))?;

    get_image_from_mandart_json_string(&file_content)
}

/// Generates an image grid from a `.mandart` JSON string.
/// **Does NOT save files** — saving is handled in `main.rs`.
pub fn get_image_from_mandart_json_string(
    mandart_json: &str,
) -> Result<Vec<Vec<[f64; 3]>>, String> {
    info!("🖼 Processing MandArt JSON...");

    let shape_inputs = get_shape_inputs_from_json_string(mandart_json)
        .map_err(|e| format!("❌ Failed to extract shape inputs: {}", e))?;
    let color_inputs = get_color_inputs_from_json_string(mandart_json)
        .map_err(|e| format!("❌ Failed to extract color inputs: {}", e))?;

    info!("✅ Extracted Shape Inputs: {:?}", shape_inputs);
    info!("✅ Extracted Color Inputs: {:?}", color_inputs);

    let grid = calculate_grid(&shape_inputs);
    let image_grid = color_grid(&grid, &color_inputs);

    info!("🎨 Coloring complete. Returning Image Grid.");
    Ok(image_grid)
}
