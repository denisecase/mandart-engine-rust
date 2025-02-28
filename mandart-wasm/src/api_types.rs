//! `api_types.rs` - WASM-compatible types.

use wasm_bindgen::prelude::*;
use mandart_core::get_inputs_from_picdef_string::{ArtImageColorInputs, ArtImageShapeInputs};
use serde::{Deserialize, Serialize};
use js_sys::{Array, Float64Array};

/// Represents shape inputs in a WASM-compatible structure
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JsArtImageShapeInputs {
    pub image_width: u32,
    pub image_height: u32,
    pub iterations_max: f64,
    pub scale: f64,
    pub x_center: f64,
    pub y_center: f64,
    pub theta: f64,
    pub r_sq_limit: f64,
    pub mand_power_real: i32,
    pub d_f_iter_min: f64,
}

#[wasm_bindgen]
impl JsArtImageShapeInputs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        image_width: u32,
        image_height: u32,
        iterations_max: f64,
        scale: f64,
        x_center: f64,
        y_center: f64,
        theta: f64,
        r_sq_limit: f64,
        mand_power_real: i32,
        d_f_iter_min: f64,
    ) -> JsArtImageShapeInputs {
        JsArtImageShapeInputs {
            image_width,
            image_height,
            iterations_max,
            scale,
            x_center,
            y_center,
            theta,
            r_sq_limit,
            mand_power_real,
            d_f_iter_min,
        }
    }
}

// Non-WASM impl for converting to core types
impl JsArtImageShapeInputs {
    /// Convert JS structure to Rust structure
    pub fn to_rust(&self) -> ArtImageShapeInputs {
        ArtImageShapeInputs {
            image_width: self.image_width,
            image_height: self.image_height,
            iterations_max: self.iterations_max,
            scale: self.scale,
            x_center: self.x_center,
            y_center: self.y_center,
            theta: self.theta,
            r_sq_limit: self.r_sq_limit,
            mand_power_real: self.mand_power_real,
            d_f_iter_min: self.d_f_iter_min,
        }
    }
    
    /// Create JS structure from Rust structure
    pub fn from_rust(rust_inputs: &ArtImageShapeInputs) -> Self {
        JsArtImageShapeInputs {
            image_width: rust_inputs.image_width,
            image_height: rust_inputs.image_height,
            iterations_max: rust_inputs.iterations_max,
            scale: rust_inputs.scale,
            x_center: rust_inputs.x_center,
            y_center: rust_inputs.y_center,
            theta: rust_inputs.theta,
            r_sq_limit: rust_inputs.r_sq_limit,
            mand_power_real: rust_inputs.mand_power_real,
            d_f_iter_min: rust_inputs.d_f_iter_min,
        }
    }
}

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
    
    /// Create JS structure from Rust structure
    pub fn from_rust(rust_inputs: &ArtImageColorInputs) -> Self {
        JsArtImageColorInputs {
            n_blocks: rust_inputs.n_blocks,
            n_colors: rust_inputs.n_colors,
            spacing_color_far: rust_inputs.spacing_color_far,
            spacing_color_near: rust_inputs.spacing_color_near,
            y_y_input: rust_inputs.y_y_input,
            mand_color: rust_inputs.mand_color,
            colors: rust_inputs.colors.clone(),
            hues: rust_inputs.hues.clone(),
        }
    }
}