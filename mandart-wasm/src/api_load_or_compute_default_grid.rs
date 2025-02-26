use wasm_bindgen::prelude::*;
use js_sys::Float64Array;
use mandart_core::load_or_compute_default_grid::load_or_compute_default_grid;
use mandart_core::flatten::flatten_grid;

/// Exposes `load_or_compute_default_grid()` to JavaScript.
#[wasm_bindgen]
pub fn api_load_or_compute_default_grid() -> Result<Float64Array, JsValue> {
    // Compute or load the default grid (2D Vec<Vec<f64>>)
    let grid = load_or_compute_default_grid()
        .map_err(|e| JsValue::from_str(&format!("Error: {}", e)))?;

    // Flatten the grid for JS compatibility (Vec<f64>)
    let flat_grid = flatten_grid(grid);

    // Convert to Float64Array for WASM interop
    Ok(Float64Array::from(flat_grid.as_slice()))
}
