//! `get_inputs_from_picdef.rs` - Extracts **shape, color, and power inputs** from PicDef JSON.

use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::io::{self, Error, ErrorKind};
use log::info;

/// **Inputs required for calculating the Mandelbrot grid.**
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

/// **Inputs required only for recoloring an already calculated grid.**
#[derive(Debug, Deserialize)]
pub struct ArtImageColorInputs {
    pub n_blocks: u32,
    pub n_colors: usize,
    pub spacing_color_far: f64,
    pub spacing_color_near: f64,
    pub y_y_input: f64,
    pub mand_color: [f64; 3],  // a hue
    pub colors: Vec<[f64; 3]>,  // sorted by hue.num
    pub hues: Vec<[f64; 4]>,  // match type to swift use FLOATING POINT NUMBERS
}

// note that the `ArtImageColorInputs` struct has a `colors` field that is not present in the JSON schema
// this is because the `colors` field is used to store the hues in the order they are provided in the JSON
// the `hues` field is used to store the hues in the order they are provided in the JSON, but sorted by hue.num
// in swift, we do this 
//         var colors: [[Double]] = appState.picdef.hues.map { [$0.r, $0.g, $0.b] }
// in swift 
// huesData = (try? JSONEncoder().encode(hues)) ?? Data()
// mandColorData = (try? JSONEncoder().encode(mandColor)) ?? Data()

// In swift (to help with types)
//  var mandColor: Hue {
//     get { (try? JSONDecoder().decode(Hue.self, from: mandColorData)) ?? Hue.defaultHue }
//     set {
//       mandColorData = (try? JSONEncoder().encode(newValue)) ?? Data()
//       objectWillChange.send()
//       saveToSwiftData()
//     }
//   }


//   var mandPowerReal: Int

//   /// Default hues used for initialization
//   static let defaultHues: [Hue] = [
//     Hue(num: 1, r: 0.0, g: 255.0, b: 0.0),
//     Hue(num: 2, r: 255.0, g: 255.0, b: 0.0),
//     Hue(num: 3, r: 255.0, g: 0.0, b: 0.0),
//     Hue(num: 4, r: 255.0, g: 0.0, b: 255.0),
//     Hue(num: 5, r: 0.0, g: 0.0, b: 255.0),
//     Hue(num: 6, r: 0.0, g: 255.0, b: 255.0),
//   ]


/// **Extracts shape inputs from a PicDef JSON string (`picdef_string`).**
pub fn get_shape_inputs_from_picdef_string(picdef_string: &str) -> io::Result<ArtImageShapeInputs> {
    info!("Extracting shape inputs from PicDef JSON...");

    let parsed: Value = serde_json::from_str(picdef_string)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to parse PicDef JSON: {}", e)))?;

    Ok(ArtImageShapeInputs {
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
    })
}

/// **Extracts color inputs from a PicDef JSON string (`picdef_string`).**
/// **Extracts color inputs from a PicDef JSON string (`picdef_string`).**
pub fn get_color_inputs_from_picdef_string(picdef_string: &str) -> io::Result<ArtImageColorInputs> {
    info!("Extracting color inputs from PicDef JSON...");

    let parsed: Value = serde_json::from_str(picdef_string)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to parse PicDef JSON: {}", e)))?;

    // Extract hues and ensure they are sorted by hue.num
    let mut hues = parsed["hues"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|hue| {
            Some([
                hue["num"].as_f64()?,   // Use hue.num for sorting
                hue["r"].as_f64()?,
                hue["g"].as_f64()?,
                hue["b"].as_f64()?,
            ])
        })
        .collect::<Vec<[f64; 4]>>();

    // Sort hues by `hue.num`
    hues.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap_or(std::cmp::Ordering::Equal));
    
    // Convert hues to colors (extract r, g, b components) - equivalent to Swift's map operation
    let colors = hues.iter().map(|hue| [hue[1], hue[2], hue[3]]).collect::<Vec<[f64; 3]>>();

    Ok(ArtImageColorInputs {
        n_blocks: parsed["nBlocks"].as_u64().unwrap_or(10) as u32,
        n_colors: hues.len(),
        spacing_color_far: parsed["spacingColorFar"].as_f64().unwrap_or(1.0),
        spacing_color_near: parsed["spacingColorNear"].as_f64().unwrap_or(1.0),
        y_y_input: parsed["yY"].as_f64().unwrap_or(0.5),
        mand_color: [
            parsed["mandColor"]["r"].as_f64().unwrap_or(0.0),
            parsed["mandColor"]["g"].as_f64().unwrap_or(0.0),
            parsed["mandColor"]["b"].as_f64().unwrap_or(0.0),
        ],
        colors, // Add the generated colors field
        hues,   // Keep the sorted hues
    })
}
/// **Reads a `.mandart` file and extracts the PicDef JSON string.**
pub fn read_picdef_from_mandart_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
        .map_err(|e| Error::new(ErrorKind::NotFound, format!("Failed to read .mandart file: {}", e)))
}

/// **Reads a `.mandart` file and extracts shape inputs from its PicDef JSON string.**
pub fn get_shape_inputs_from_mandart_file(file_path: &str) -> io::Result<ArtImageShapeInputs> {
    let picdef_string = read_picdef_from_mandart_file(file_path)?;
    get_shape_inputs_from_picdef_string(&picdef_string)
}

/// **Reads a `.mandart` file and extracts color inputs from its PicDef JSON string.**
pub fn get_color_inputs_from_mandart_file(file_path: &str) -> io::Result<ArtImageColorInputs> {
    let picdef_string = read_picdef_from_mandart_file(file_path)?;
    get_color_inputs_from_picdef_string(&picdef_string)
}
