//! `api_get_image_from_inputs.rs` - WASM binding for direct image generation from inputs.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use log::info;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::api_types::{JsArtImageShapeInputs, JsArtImageColorInputs};
use mandart_core::get_grid_from_shape_inputs::get_grid_from_shape_inputs;
use mandart_core::get_colored_grid::get_colored_grid;
use mandart_core::get_image_from_colored_grid::get_image_from_colored_grid;

/// Structure for image data to return to JavaScript
#[derive(Serialize, Deserialize)]
pub struct JsImageData {
    pub data: Vec<u8>,      // RGBA pixel data
    pub width: u32,
    pub height: u32,
}

// Simple cache key for grid caching - uses a hash value instead of a complex struct
#[derive(PartialEq, Eq, Hash, Clone)]
struct SimpleGridCacheKey(u64);

impl From<&JsArtImageShapeInputs> for SimpleGridCacheKey {
    fn from(inputs: &JsArtImageShapeInputs) -> Self {
        let mut hasher = DefaultHasher::new();
        
        inputs.image_width.hash(&mut hasher);
        inputs.image_height.hash(&mut hasher);
        
        // Round floating point values to avoid precision issues
        (inputs.iterations_max.round() as u64).hash(&mut hasher);
        ((inputs.scale * 10000.0).round() as u64).hash(&mut hasher);
        ((inputs.x_center * 100000000.0).round() as i64).hash(&mut hasher);
        ((inputs.y_center * 100000000.0).round() as i64).hash(&mut hasher);
        ((inputs.theta * 10000.0).round() as u64).hash(&mut hasher);
        ((inputs.r_sq_limit * 10000.0).round() as u64).hash(&mut hasher);
        inputs.mand_power_real.hash(&mut hasher);
        ((inputs.d_f_iter_min * 10000.0).round() as u64).hash(&mut hasher);
        
        SimpleGridCacheKey(hasher.finish())
    }
}

// Cache size control
const MAX_CACHE_SIZE: usize = 5;  // Maximum number of grids to cache
// Limit for image dimensions to prevent excessive memory allocation
const MAX_IMAGE_DIMENSION: u32 = 10_000;
const MAX_TOTAL_PIXELS: u32 = 100_000_000;

// Thread-local storage for grid cache
thread_local! {
    static GRID_CACHE: std::cell::RefCell<HashMap<SimpleGridCacheKey, Vec<Vec<f64>>>> = 
        std::cell::RefCell::new(HashMap::new());
}

/// **WASM API: Generate an image directly from shape and color inputs**
/// 
/// This function optimizes performance by caching uncolored grids and
/// reusing them when possible.
///
/// # Arguments
/// * `js_shape_inputs` - Shape inputs in JavaScript format
/// * `js_color_inputs` - Color inputs in JavaScript format
///
/// # Returns
/// * An image data structure as a JsValue
#[wasm_bindgen]
pub fn api_get_image_from_inputs(
    js_shape_inputs: JsValue, 
    js_color_inputs: JsValue
) -> Result<JsValue, JsValue> {
    info!("🖼️ WASM: Generating image directly from inputs");
    
    // Parse the JS inputs with more robust error handling
    let shape_inputs: JsArtImageShapeInputs = match serde_wasm_bindgen::from_value(js_shape_inputs) {
        Ok(inputs) => inputs,
        Err(err) => {
            let error_message = format!(
                "Shape input parsing failed: {}. Ensure correct JSON structure and type compatibility.", 
                err
            );
            return Err(JsValue::from_str(&error_message));
        }
    };
    
    let color_inputs: JsArtImageColorInputs = match serde_wasm_bindgen::from_value(js_color_inputs) {
        Ok(inputs) => inputs,
        Err(err) => {
            let error_message = format!(
                "Color input parsing failed: {}. Verify input JSON format.", 
                err
            );
            return Err(JsValue::from_str(&error_message));
        }
    };
    
    // Comprehensive input validation
    if shape_inputs.image_width == 0 || shape_inputs.image_height == 0 {
        return Err(JsValue::from_str("Invalid image dimensions: width and height must be positive"));
    }

    if shape_inputs.image_width > MAX_IMAGE_DIMENSION || shape_inputs.image_height > MAX_IMAGE_DIMENSION {
        let error_message = format!(
            "Image dimensions too large. Max allowed: {}x{}", 
            MAX_IMAGE_DIMENSION, MAX_IMAGE_DIMENSION
        );
        return Err(JsValue::from_str(&error_message));
    }

    let total_pixels = shape_inputs.image_width.saturating_mul(shape_inputs.image_height);
    if total_pixels > MAX_TOTAL_PIXELS {
        let error_message = format!(
            "Total pixels exceed limit. Current: {}, Max: {}", 
            total_pixels, MAX_TOTAL_PIXELS
        );
        return Err(JsValue::from_str(&error_message));
    }

    if shape_inputs.iterations_max <= 0.0 {
        return Err(JsValue::from_str("Iterations must be a positive number"));
    }

    if shape_inputs.scale <= 0.0 {
        return Err(JsValue::from_str("Scale must be a positive number"));
    }

    // Add detailed logging
    info!(
        "Processing image: {}x{}, max_iter: {:.2}, scale: {:.4}", 
        shape_inputs.image_width, 
        shape_inputs.image_height,
        shape_inputs.iterations_max,
        shape_inputs.scale
    );
    
    // Convert to Rust types
    let rust_shape_inputs = shape_inputs.to_rust();
    let rust_color_inputs = color_inputs.to_rust();
    
    // Create a cache key from the shape inputs
    let cache_key = SimpleGridCacheKey::from(&shape_inputs);
    
    // Try to get a cached grid or calculate a new one
    let iter_grid = GRID_CACHE.with(|cache| {
        let mut cache_ref = cache.borrow_mut();
        
        if let Some(cached_grid) = cache_ref.get(&cache_key) {
            info!("Using cached grid for shape inputs");
            cached_grid.clone()
        } else {
            info!("Calculating new grid for shape inputs");
            // Calculate the grid using the core function
            let new_grid = get_grid_from_shape_inputs(&rust_shape_inputs);
            
            // Manage cache size
            if cache_ref.len() >= MAX_CACHE_SIZE {
                // Remove the first entry (arbitrary choice)
                if let Some(key_to_remove) = cache_ref.keys().next().cloned() {
                    cache_ref.remove(&key_to_remove);
                }
            }
            
            // Store the new grid in the cache
            cache_ref.insert(cache_key, new_grid.clone());
            
            new_grid
        }
    });
    
    // Color the grid using the core function
    let colored_grid = get_colored_grid(&iter_grid, &rust_shape_inputs, &rust_color_inputs);
    
    // Generate the image using the core functions
    let img = get_image_from_colored_grid(&colored_grid);
    
    // Convert image to WASM-compatible format
    let width = img.width();
    let height = img.height();
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            data.push(pixel[0]); // R
            data.push(pixel[1]); // G
            data.push(pixel[2]); // B
            data.push(255);      // A (fully opaque)
        }
    }
    
    // Create a JS-compatible structure
    let js_image_data = JsImageData {
        width,
        height,
        data,
    };
    
    // Return as JS value
    match serde_wasm_bindgen::to_value(&js_image_data) {
        Ok(js_image) => Ok(js_image),
        Err(err) => Err(JsValue::from_str(&format!("Failed to convert image to JS: {}", err))),
    }
}

/// **WASM API: Clear the grid cache**
/// 
/// This function allows clients to explicitly clear the grid cache, 
/// which might be useful for low-memory situations.
///
/// # Returns
/// * The number of cache entries that were cleared
#[wasm_bindgen]
pub fn api_clear_grid_cache() -> u32 {
    let size = GRID_CACHE.with(|cache| {
        let mut cache_ref = cache.borrow_mut();
        let size = cache_ref.len();
        cache_ref.clear();
        size
    });
    
    info!("Grid cache cleared, removed {} entries", size);
    size as u32
}