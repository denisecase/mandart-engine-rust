//! `api_types.rs` - WASM-compatible types.

use wasm_bindgen::prelude::*;
use mandart_core::get_inputs_from_picdef_string::ArtImageColorInputs;
use serde::{Deserialize, Serialize};
use js_sys::{Array, Float64Array};

// ... your existing JsArtImageShapeInputs ...

/// Represents color inputs in a WASM-compatible structure
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JsArtImageColorInputs {
    pub n_blocks: u32,
    pub n_colors: usize,
    pub spacing_color_far: f64,
    pub spacing_color_near: f64,
    pub y_y_input: f64,
    #[wasm_bindgen(skip)]
    pub mand_color: [f64; 3],
    #[wasm_bindgen(skip)]
    pub colors: Vec<[f64; 3]>,
    #[wasm_bindgen(skip)]
    pub hues: Vec<[f64; 4]>,
}

#[wasm_bindgen]
impl JsArtImageColorInputs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        n_blocks: u32,
        n_colors: usize,
        spacing_color_far: f64,
        spacing_color_near: f64,
        y_y_input: f64,
        mand_color_js: Float64Array,
        colors_js: Array,
        hues_js: Array,
    ) -> Result<JsArtImageColorInputs, JsValue> {
        // Convert JS Float64Array to Rust array
        let mut mand_color = [0.0, 0.0, 0.0];
        if mand_color_js.length() >= 3 {
            for i in 0..3 {
                mand_color[i] = mand_color_js.get_index(i as u32);
            }
        }
        
        // Convert JS Array of Arrays to Rust Vec<[f64; 3]>
        let mut colors = Vec::new();
        for i in 0..colors_js.length() {
            let color_array = js_sys::try_iter(&colors_js.get(i))
                .map_err(|_| JsValue::from_str("Expected iterable for color"))?
                .ok_or(JsValue::from_str("Failed to iterate over color"))?;
                
            let mut color_values = Vec::new();
            for value in color_array {
                let value = value.map_err(|_| JsValue::from_str("Failed to iterate color values"))?;
                let num = js_sys::Number::from(value).value_of();
                color_values.push(num);
            }
            
            if color_values.len() >= 3 {
                colors.push([color_values[0], color_values[1], color_values[2]]);
            }
        }
        
        // Convert JS Array of Arrays to Rust Vec<[f64; 4]>
        let mut hues = Vec::new();
        for i in 0..hues_js.length() {
            let hue_array = js_sys::try_iter(&hues_js.get(i))
                .map_err(|_| JsValue::from_str("Expected iterable for hue"))?
                .ok_or(JsValue::from_str("Failed to iterate over hue"))?;
                
            let mut hue_values = Vec::new();
            for value in hue_array {
                let value = value.map_err(|_| JsValue::from_str("Failed to iterate hue values"))?;
                let num = js_sys::Number::from(value).value_of();
                hue_values.push(num);
            }
            
            if hue_values.len() >= 4 {
                hues.push([hue_values[0], hue_values[1], hue_values[2], hue_values[3]]);
            }
        }
        
        Ok(JsArtImageColorInputs {
            n_blocks,
            n_colors,
            spacing_color_far,
            spacing_color_near,
            y_y_input,
            mand_color,
            colors,
            hues,
        })
    }
    
    /// Get mand_color as Float64Array
    #[wasm_bindgen(getter)]
    pub fn mand_color(&self) -> Float64Array {
        let array = Float64Array::new_with_length(3);
        array.copy_from(&self.mand_color);
        array
    }
    
    /// Set mand_color from Float64Array
    #[wasm_bindgen(setter)]
    pub fn set_mand_color(&mut self, value: Float64Array) {
        for i in 0..3 {
            if i < value.length() as usize {
                self.mand_color[i] = value.get_index(i as u32);
            }
        }
    }
}

// Non-WASM impl for converting to core types
impl JsArtImageColorInputs {
    /// Converts JS structure to Rust structure
    pub fn to_rust(&self) -> ArtImageColorInputs {
        ArtImageColorInputs {
            n_blocks: self.n_blocks,
            n_colors: self.n_colors,
            spacing_color_far: self.spacing_color_far,
            spacing_color_near: self.spacing_color_near,
            y_y_input: self.y_y_input,
            mand_color: self.mand_color,
            colors: self.colors.clone(),
            hues: self.hues.clone(),
        }
    }
}