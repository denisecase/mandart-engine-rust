use wasm_bindgen::prelude::*;
use js_sys::Float64Array;
use web_sys::console;
use mandart_core::flatten::unflatten_grid;
use mandart_core::get_colored_grid::get_colored_grid;
use mandart_core::get_inputs_from_picdef_string::{get_shape_inputs_from_picdef_string, get_color_inputs_from_picdef_string};
use image::{RgbImage, Rgb};
use base64::{Engine as _, engine::general_purpose};
use std::io::Cursor;

/// Generates a PNG file from a grid and returns it as a base64 string
#[wasm_bindgen]
pub fn api_generate_png(
    flat_grid: &Float64Array,
    width: u32,
    height: u32,
    picdef_json: &str
) -> Result<String, JsValue> {
    console::log_1(&JsValue::from_str("📊 Generating PNG..."));
    
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
    
    // Color the grid
    let colored_grid = get_colored_grid(&grid, &shape_inputs, &color_inputs);
    
    // Create an RgbImage
    let mut img = RgbImage::new(width, height);
    
    // Fill the image with colors from our grid
    for y in 0..height {
        for x in 0..width {
            if x < width && y < height &&
               (x as usize) < colored_grid.len() && 
               (y as usize) < colored_grid[x as usize].len() {
                let r = colored_grid[x as usize][y as usize][0].clamp(0.0, 255.0) as u8;
                let g = colored_grid[x as usize][y as usize][1].clamp(0.0, 255.0) as u8;
                let b = colored_grid[x as usize][y as usize][2].clamp(0.0, 255.0) as u8;
                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
    }
    
    // Create an in-memory buffer to hold the PNG
    let mut png_buffer = Cursor::new(Vec::new());
    
    // Write the image to the buffer in PNG format
    img.write_to(&mut png_buffer, image::ImageFormat::Png)
        .map_err(|e| JsValue::from_str(&format!("Failed to encode PNG: {}", e)))?;
    
    // Convert to base64
    let base64_png = general_purpose::STANDARD.encode(png_buffer.into_inner());
    
    // Return as a data URL
    let data_url = format!("data:image/png;base64,{}", base64_png);
    
    console::log_1(&JsValue::from_str("✅ PNG generated successfully"));
    
    Ok(data_url)
}