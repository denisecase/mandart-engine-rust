// inputs.rs - process input files to get ready for processing

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
pub struct Hue {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Debug, Deserialize)]
pub struct ArtImageColorInputs {
    pub n_blocks: u32,
    pub n_colors: usize,
    pub spacing_color_far: f64,
    pub spacing_color_near: f64,
    pub y_y_input: f64,
    pub mand_color: Hue,
    pub hues_list: Vec<[f64; 4]>, // [num, r, g, b]
    pub colors: Vec<[f64; 3]>,    // [r, g, b] only
}


/// Reads a `.mandart` file and extracts shape-related parameters.
pub fn get_shape_inputs(input_file: &str) -> io::Result<ArtImageShapeInputs> {
    let file_contents = fs::read_to_string(input_file)?;
    let parsed: Value = serde_json::from_str(&file_contents)?;

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

/// Reads a `.mandart` file and extracts color-related parameters.
pub fn get_color_inputs(input_file: &str) -> io::Result<ArtImageColorInputs> {
    let file_contents = fs::read_to_string(input_file)?;
    let parsed: Value = serde_json::from_str(&file_contents)?;

    let hues = parsed["hues"].as_array().unwrap_or(&vec![]).iter()
        .filter_map(|hue| {
            Some([
                hue["num"].as_f64()?,
                hue["r"].as_f64()?,
                hue["g"].as_f64()?,
                hue["b"].as_f64()?,
            ])
        })
        .collect::<Vec<[f64; 4]>>();

    let colors = hues.iter().map(|h| [h[1], h[2], h[3]]).collect();

    let color_inputs = ArtImageColorInputs {
        n_blocks: parsed["nBlocks"].as_u64().unwrap_or(10) as u32,
        n_colors: hues.len(),
        spacing_color_far: parsed["spacingColorFar"].as_f64().unwrap_or(1.0),
        spacing_color_near: parsed["spacingColorNear"].as_f64().unwrap_or(1.0),
        y_y_input: parsed["yY"].as_f64().unwrap_or(0.5),
        mand_color: Hue {
            r: parsed["mandColor"]["r"].as_f64().unwrap_or(0.0),
            g: parsed["mandColor"]["g"].as_f64().unwrap_or(0.0),
            b: parsed["mandColor"]["b"].as_f64().unwrap_or(0.0),
        },
        hues_list: hues,
        colors,
    };

    Ok(color_inputs)
}
