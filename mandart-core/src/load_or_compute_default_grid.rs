// mandart-core/src/load_or_compute_default_grid.rs

use crate::config_settings::load_config;
use crate::get_grid_from_shape_inputs::get_grid_from_shape_inputs;
use crate::get_inputs_from_picdef_string::get_shape_inputs_from_picdef_string;
use crate::get_picdef_from_mandart_file::get_picdef_from_mandart_file;
use crate::io_csv::{read_grid_from_csv, save_grid_to_csv};
use log::{info, warn};

/// Loads a precomputed iteration grid from CSV or computes a new one from `Default.mandart`.
/// Returns `Ok(Vec<Vec<f64>>)` if successful, or `Err(String)` if an error occurs.
pub fn load_or_compute_default_grid() -> Result<Vec<Vec<f64>>, String> {
    let config = load_config(None);

    // Get paths from config
    let csv_grid_path = config
        .get("csv_grid_path")
        .cloned()
        .unwrap_or_else(|| "assets/MandArt_Catalog/Default.csv".to_string());

    let mandart_file_path = config
        .get("mandart_file_path")
        .cloned()
        .unwrap_or_else(|| "assets/MandArt_Catalog/Default.mandart".to_string());

    // Try to load the precomputed iteration grid from CSV
    if let Ok(grid) = read_grid_from_csv(&csv_grid_path) {
        info!("✅ Loaded precomputed iteration grid from CSV: {}", &csv_grid_path);
        return Ok(grid); // ✅ Returning the precomputed 2D f64 grid
    }

    warn!("⚠️ Precomputed CSV grid not found. Computing from Default.mandart...");

    // Read the default `Default.mandart` file (JSON format)
    let picdef_json = get_picdef_from_mandart_file(&mandart_file_path)
        .map_err(|e| format!("Failed to read Default.mandart: {}", e))?;

    // Extract shape inputs from PicDef JSON
    let shape_inputs = get_shape_inputs_from_picdef_string(&picdef_json)
        .map_err(|e| format!("Failed to extract shape inputs: {}", e))?;

    // Compute the 2D iteration grid from shape inputs
    let computed_grid = get_grid_from_shape_inputs(&shape_inputs);

    // Save computed iteration grid to CSV for future use
    if let Err(e) = save_grid_to_csv(&computed_grid, &csv_grid_path) {
        warn!("⚠️ Failed to save computed iteration grid to CSV: {}", e);
    }

    info!("✅ Computed new Mandelbrot iteration grid.");
    Ok(computed_grid) // ✅ Returning the correct 2D f64 grid
}
