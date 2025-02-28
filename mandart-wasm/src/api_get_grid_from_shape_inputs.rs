use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use log::info;

// Import the actual core implementation
use mandart_core::get_grid_from_shape_inputs::get_grid_from_shape_inputs;
use mandart_core::get_inputs_from_picdef_string::ArtImageShapeInputs;

/// Shape inputs for Mandart image calculation (WASM-compatible)
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
    #[serde(default = "default_d_f_iter_min")]
    pub d_f_iter_min: f64,
}

fn default_d_f_iter_min() -> f64 {
    0.1
}

/// **WASM API: Compute a Mandelbrot grid from shape inputs**
/// 
/// Takes shape inputs from JavaScript and returns a 2D grid of iteration values.
///
/// # Arguments
/// * `js_inputs` - Shape inputs in JavaScript format
///
/// # Returns
/// * A 2D array of iteration values as a JsValue
#[wasm_bindgen]
pub fn api_get_grid_from_shape_inputs(js_inputs: JsValue) -> Result<JsValue, JsValue> {
    info!("WASM: Computing Mandelbrot grid from shape inputs");
    
    // Parse the JS inputs
    let js_shape_inputs: JsArtImageShapeInputs = match serde_wasm_bindgen::from_value(js_inputs) {
        Ok(inputs) => inputs,
        Err(err) => return Err(JsValue::from_str(&format!("Failed to parse shape inputs: {}", err))),
    };
    
    // Convert to internal ArtImageShapeInputs
    let shape_inputs = ArtImageShapeInputs {
        image_width: js_shape_inputs.image_width,
        image_height: js_shape_inputs.image_height,
        iterations_max: js_shape_inputs.iterations_max,
        scale: js_shape_inputs.scale,
        x_center: js_shape_inputs.x_center,
        y_center: js_shape_inputs.y_center,
        theta: js_shape_inputs.theta,
        r_sq_limit: js_shape_inputs.r_sq_limit,
        mand_power_real: js_shape_inputs.mand_power_real,
        d_f_iter_min: js_shape_inputs.d_f_iter_min,
    };
    
    // Call the core function to compute the grid
    let iter_grid = get_grid_from_shape_inputs(&shape_inputs);
    
    // Convert the grid to a JS value
    match serde_wasm_bindgen::to_value(&iter_grid) {
        Ok(js_grid) => Ok(js_grid),
        Err(err) => Err(JsValue::from_str(&format!("Failed to serialize grid: {}", err))),
    }
}