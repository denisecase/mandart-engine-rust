use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
pub struct ArtImageShapeInputs {
    pub image_height: u32,
    pub image_width: u32,
    pub iterations_max: f64,
    pub scale: f64,
    pub x_center: f64,
    pub y_center: f64,
    pub theta: f64,
    pub d_f_iter_min: f64,
    pub r_sq_limit: f64,
    pub mand_power_real: i32,
}

#[derive(Debug, Deserialize)]
pub struct ArtImageColorInputs {
    pub n_blocks: u32,
    pub n_colors: usize,
    pub spacing_color_far: f64,
    pub spacing_color_near: f64,
    pub y_y_input: f64,
    pub mand_color: [f64; 3],  // Match Swift's `[Double]`
    pub colors: Vec<[f64; 3]>, // [r, g, b] only
}


pub fn get_shape_inputs(input_file: &str) -> io::Result<ArtImageShapeInputs> {
    println!("get_shape_inputs (from file): {}", input_file);
    let file_contents = fs::read_to_string(input_file)?;
    get_shape_inputs_from_json_string(&file_contents) // Use the new JSON parser
}


pub fn get_color_inputs(input_file: &str) -> io::Result<ArtImageColorInputs> {
    println!("get_color_inputs (from file): {}", input_file);
    let file_contents = fs::read_to_string(input_file)?;
    get_color_inputs_from_json_string(&file_contents) // Use the new JSON parser
}


pub fn get_shape_inputs_from_json_string(json_str: &str) -> io::Result<ArtImageShapeInputs> {
    let parsed: Value = serde_json::from_str(json_str)?;

    let shape_inputs = ArtImageShapeInputs {
        image_height: parsed["imageHeight"].as_u64().unwrap_or(500) as u32,
        image_width: parsed["imageWidth"].as_u64().unwrap_or(500) as u32,
        iterations_max: parsed["iterationsMax"].as_f64().unwrap_or(1000.0),
        scale: parsed["scale"].as_f64().unwrap_or(1.0),
        x_center: parsed["xCenter"].as_f64().unwrap_or(0.0),
        y_center: parsed["yCenter"].as_f64().unwrap_or(0.0),
        theta: parsed["theta"].as_f64().unwrap_or(0.0),
        d_f_iter_min: parsed["dFIterMin"].as_f64().unwrap_or(0.1),
        r_sq_limit: parsed["rSqLimit"].as_f64().unwrap_or(4.0),
        mand_power_real: parsed["mandPowerReal"].as_i64().unwrap_or(2) as i32,
    };

    Ok(shape_inputs)
}

//
// NEW: Reads color inputs from a JSON string.
//
pub fn get_color_inputs_from_json_string(json_str: &str) -> io::Result<ArtImageColorInputs> {
    let parsed: Value = serde_json::from_str(json_str)?;

    let colors = parsed["hues"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|hue| Some([hue["r"].as_f64()?, hue["g"].as_f64()?, hue["b"].as_f64()?]))
        .collect::<Vec<[f64; 3]>>();

    let color_inputs = ArtImageColorInputs {
        n_blocks: parsed["nBlocks"].as_u64().unwrap_or(10) as u32,
        n_colors: colors.len(),
        spacing_color_far: parsed["spacingColorFar"].as_f64().unwrap_or(1.0),
        spacing_color_near: parsed["spacingColorNear"].as_f64().unwrap_or(1.0),
        y_y_input: parsed["yY"].as_f64().unwrap_or(0.5),
        mand_color: [
            parsed["mandColor"]["red"].as_f64().unwrap_or(0.0),
            parsed["mandColor"]["green"].as_f64().unwrap_or(0.0),
            parsed["mandColor"]["blue"].as_f64().unwrap_or(0.0),
        ],
        colors,
    };

    Ok(color_inputs)
}
