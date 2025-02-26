use wasm_bindgen::prelude::*;
use js_sys::{Float64Array, Uint8Array};
use web_sys::console;
use mandart_core::get_colored_grid::get_colored_grid;
use mandart_core::flatten::unflatten_grid;
use mandart_core::get_inputs_from_picdef_string::{get_shape_inputs_from_picdef_string, get_color_inputs_from_picdef_string};

/// **Colors a Mandelbrot iteration grid and returns RGBA data.**
#[wasm_bindgen]
pub fn api_get_colored_grid(
    flat_grid: &Float64Array,
    width: u32,
    height: u32,
    picdef_json: &str
) -> Result<Uint8Array, JsValue> {
    console::log_1(&JsValue::from_str("📊 Coloring Mandelbrot grid..."));
    
    // Convert flat JS array to Rust Vec
    let mut flat_rust_grid = vec![0.0; flat_grid.length() as usize];
    flat_grid.copy_to(&mut flat_rust_grid);
    
    // Unflatten the grid (1D → 2D)
    let grid = unflatten_grid(flat_rust_grid, width as usize, height as usize);
    
    // Parse shape and color inputs from the picdef JSON
    let shape_inputs = get_shape_inputs_from_picdef_string(picdef_json)
        .map_err(|e| JsValue::from_str(&format!("Error parsing shape inputs: {}", e)))?;
        
    let color_inputs = get_color_inputs_from_picdef_string(picdef_json)
        .map_err(|e| JsValue::from_str(&format!("Error parsing color inputs: {}", e)))?;
    
    // Color the grid using the core function
    let colored_grid = get_colored_grid(&grid, &shape_inputs, &color_inputs);
    
    // Convert to RGBA format
    let rgba_size = (width * height * 4) as usize;
    let mut rgba_data = vec![0u8; rgba_size];
    
    let mut idx = 0;
    for y in 0..height as usize {
        for x in 0..width as usize {
            if x < colored_grid.len() && y < colored_grid[x].len() {
                rgba_data[idx] = colored_grid[x][y][0].clamp(0.0, 255.0) as u8;     // R
                rgba_data[idx + 1] = colored_grid[x][y][1].clamp(0.0, 255.0) as u8; // G
                rgba_data[idx + 2] = colored_grid[x][y][2].clamp(0.0, 255.0) as u8; // B
                rgba_data[idx + 3] = 255;                                           // A (opaque)
            }
            idx += 4;
        }
    }
    
    // Create the JS Uint8Array
    let js_rgba = Uint8Array::new_with_length(rgba_size as u32);
    js_rgba.copy_from(&rgba_data);
    
    console::log_1(&JsValue::from_str("✅ Grid colored successfully"));
    
    Ok(js_rgba)
}